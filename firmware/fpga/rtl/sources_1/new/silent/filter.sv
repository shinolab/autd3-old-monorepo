/*
 * File: filter.sv
 * Project: silent
 * Created Date: 21/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */


`timescale 1ns / 1ps
module silent_filter #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK,
    input var UPDATE,
    input var [WIDTH-1:0] STEP,
    input var [WIDTH-1:0] CYCLE[0:DEPTH-1],
    input var [WIDTH-1:0] DUTY[0:DEPTH-1],
    input var [WIDTH-1:0] PHASE[0:DEPTH-1],
    output var [WIDTH-1:0] DUTY_S[0:DEPTH-1],
    output var [WIDTH-1:0] PHASE_S[0:DEPTH-1],
    output var DONE
);

  localparam int ADDSUB_LATENCY = 2;

  bit signed [WIDTH:0] step, step_n;
  bit signed [WIDTH:0] duty [0:DEPTH-1];
  bit signed [WIDTH:0] phase[0:DEPTH-1];
  bit signed [WIDTH:0] cycle[0:DEPTH-1], cycle_n[0:DEPTH-1];
  bit signed [WIDTH:0] current_duty [0:DEPTH-1] = '{DEPTH{0}};
  bit signed [WIDTH:0] current_phase[0:DEPTH-1] = '{DEPTH{0}};
  bit signed [WIDTH:0] duty_step, phase_step;
  bit signed [WIDTH:0] a_duty_step, b_duty_step;
  bit signed [WIDTH:0] a_phase_step, b_phase_step;
  bit signed [WIDTH:0] a_duty, b_duty, s_duty;
  bit signed [WIDTH:0] a_phase, b_phase, s_phase;
  bit signed [WIDTH:0] a_phase_fold, b_phase_fold, s_phase_fold;
  bit add, add_fold;
  bit [$clog2(DEPTH+(ADDSUB_LATENCY+1)*3)-1:0] calc_cnt, calc_step_cnt, fold_cnt, set_cnt;
  bit done = 0;

  enum bit {
    IDLE,
    PROCESS
  } state = IDLE;

  for (genvar i = 0; i < DEPTH; i++) begin
    assign DUTY_S[i]  = current_duty[i][WIDTH-1:0];
    assign PHASE_S[i] = current_phase[i][WIDTH-1:0];
  end
  assign DONE = done;

  addsub #(
      .WIDTH(WIDTH + 1)
  ) sub_duty_step (
      .CLK(CLK),
      .A  (a_duty_step),
      .B  (b_duty_step),
      .ADD(1'b0),
      .S  (duty_step)
  );

  addsub #(
      .WIDTH(WIDTH + 1)
  ) sub_phase_step (
      .CLK(CLK),
      .A  (a_phase_step),
      .B  (b_phase_step),
      .ADD(1'b0),
      .S  (phase_step)
  );

  addsub #(
      .WIDTH(WIDTH + 1)
  ) add_duty (
      .CLK(CLK),
      .A  (a_duty),
      .B  (b_duty),
      .ADD(1'b1),
      .S  (s_duty)
  );

  addsub #(
      .WIDTH(WIDTH + 1)
  ) addsub_phase (
      .CLK(CLK),
      .A  (a_phase),
      .B  (b_phase),
      .ADD(add),
      .S  (s_phase)
  );

  addsub #(
      .WIDTH(WIDTH + 1)
  ) addsub_phase_fold (
      .CLK(CLK),
      .A  (a_phase_fold),
      .B  (b_phase_fold),
      .ADD(add_fold),
      .S  (s_phase_fold)
  );

  for (genvar i = 0; i < DEPTH; i++) begin
    always_ff @(posedge CLK) begin
      case (state)
        IDLE: begin
          if (UPDATE) begin
            cycle[i] <= {1'b0, CYCLE[i]};
            cycle_n[i] <= -{1'b0, CYCLE[i]};
            duty[i] <= {1'b0, DUTY[i]};
            phase[i] <= {1'b0, PHASE[i]};
          end
        end
      endcase
    end
  end

  always_ff @(posedge CLK) begin
    case (state)
      IDLE: begin
        if (UPDATE) begin
          step <= {1'b0, STEP};
          step_n <= -{1'b0, STEP};

          calc_step_cnt <= 0;
          calc_cnt <= 0;
          fold_cnt <= 0;
          set_cnt <= 0;

          state <= PROCESS;
        end
        done <= 0;
      end
      PROCESS: begin
        // calculate duty/phase step
        a_duty_step <= duty[calc_step_cnt];
        b_duty_step <= current_duty[calc_step_cnt];
        a_phase_step <= phase[calc_step_cnt];
        b_phase_step <= current_phase[calc_step_cnt];
        calc_step_cnt <= calc_step_cnt + 1;

        // calculate next duty
        a_duty <= current_duty[calc_cnt];
        if (duty_step[WIDTH] == 1'b0) begin
          b_duty <= (duty_step < step) ? duty_step : step;
        end else begin
          b_duty <= (step_n < duty_step) ? duty_step : step_n;
        end
        // calculate next phase
        a_phase <= current_phase[calc_cnt];
        if (phase_step[WIDTH] == 1'b0) begin
          b_phase <= (phase_step < step) ? phase_step : step;
          add <= (phase_step <= {1'b0, cycle[calc_cnt][WIDTH:1]});
        end else begin
          b_phase <= (step_n < phase_step) ? phase_step : step_n;
          add <= ({1'b1, cycle_n[calc_cnt][WIDTH:1]} <= phase_step);
        end
        if (calc_step_cnt > ADDSUB_LATENCY) begin
          calc_cnt <= calc_cnt + 1;
        end

        // make phase be in [0, cycle-1]
        a_phase_fold <= s_phase;
        if (s_phase >= cycle[fold_cnt]) begin
          b_phase_fold <= cycle[fold_cnt];
          add_fold <= 1'b0;
        end else if (s_phase[WIDTH] == 1'b1) begin
          b_phase_fold <= cycle[fold_cnt];
          add_fold <= 1'b1;
        end else begin
          b_phase_fold <= '0;
          add_fold <= 1'b1;
        end
        if (calc_cnt > ADDSUB_LATENCY) begin
          // set duty
          if (fold_cnt <= DEPTH - 1) begin
            current_duty[fold_cnt] <= s_duty;
          end

          fold_cnt <= fold_cnt + 1;
        end

        if (fold_cnt > ADDSUB_LATENCY) begin
          // set phase
          current_phase[set_cnt] <= s_phase_fold;

          set_cnt <= set_cnt + 1;
          if (set_cnt == DEPTH - 1) begin
            done  <= 1;
            state <= IDLE;
          end
        end
      end
    endcase
  end

endmodule
