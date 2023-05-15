/*
 * File: silencer.sv
 * Project: silent
 * Created Date: 22/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

module silencer #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK,
    input var DIN_VALID,
    input var [WIDTH-1:0] STEP,
    input var [WIDTH-1:0] CYCLE[DEPTH],
    input var [WIDTH-1:0] DUTY[DEPTH],
    input var [WIDTH-1:0] PHASE[DEPTH],
    output var [WIDTH-1:0] DUTY_S[DEPTH],
    output var [WIDTH-1:0] PHASE_S[DEPTH],
    output var DOUT_VALID
);

  localparam int AddSubLatency = 2;

  bit signed [WIDTH:0] step, step_n;
  bit signed [WIDTH:0] cycle_buf[6], cycle_n_buf[3];

  bit signed [WIDTH:0] current_duty [DEPTH] = '{DEPTH{0}};
  bit signed [WIDTH:0] current_phase[DEPTH] = '{DEPTH{0}};
  bit signed [WIDTH:0] duty_step, phase_step;
  bit signed [WIDTH:0] a_duty_step, b_duty_step;
  bit signed [WIDTH:0] a_phase_step, b_phase_step;
  bit signed [WIDTH:0] a_duty, b_duty, s_duty;
  bit signed [WIDTH:0] a_phase, b_phase, s_phase;
  bit signed [WIDTH:0] a_phase_fold, b_phase_fold, s_phase_fold;
  bit add, add_fold;
  bit [$clog2(DEPTH+(AddSubLatency+1)*3)-1:0] calc_cnt, calc_step_cnt, fold_cnt, set_cnt;

  bit dout_valid = 0;

  assign DOUT_VALID = dout_valid;

  typedef enum bit {
    WAITING,
    RUN
  } state_t;

  state_t state = WAITING;

  for (genvar i = 0; i < DEPTH; i++) begin : gen_duty_phase
    assign DUTY_S[i]  = current_duty[i][WIDTH-1:0];
    assign PHASE_S[i] = current_phase[i][WIDTH-1:0];
  end

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

  always_ff @(posedge CLK) begin
    case (state)
      WAITING: begin
        if (DIN_VALID) begin
          step <= {1'b0, STEP};
          step_n <= -{1'b0, STEP};

          calc_step_cnt <= 0;
          calc_cnt <= 0;
          fold_cnt <= 0;
          set_cnt <= 0;

          state <= RUN;
        end
      end
      RUN: begin
        dout_valid <= 1'b0;
        cycle_buf[0] <= {1'b0, CYCLE[calc_step_cnt]};
        cycle_buf[1] <= cycle_buf[0];
        cycle_buf[2] <= cycle_buf[1];
        cycle_buf[3] <= cycle_buf[2];
        cycle_buf[4] <= cycle_buf[3];
        cycle_buf[5] <= cycle_buf[4];
        cycle_n_buf[0] <= -{1'b0, CYCLE[calc_step_cnt]};
        cycle_n_buf[1] <= cycle_n_buf[0];
        cycle_n_buf[2] <= cycle_n_buf[1];

        // step 1: calculate duty/phase step
        a_duty_step <= {1'b0, DUTY[calc_step_cnt]};
        b_duty_step <= current_duty[calc_step_cnt];
        a_phase_step <= {1'b0, PHASE[calc_step_cnt]};
        b_phase_step <= current_phase[calc_step_cnt];
        calc_step_cnt <= calc_step_cnt + 1;

        // step 2: calculate next duty/phase
        a_duty <= current_duty[calc_cnt];
        if (duty_step[WIDTH] == 1'b0) begin
          b_duty <= (duty_step < step) ? duty_step : step;
        end else begin
          b_duty <= (step_n < duty_step) ? duty_step : step_n;
        end
        a_phase <= current_phase[calc_cnt];
        if (phase_step[WIDTH] == 1'b0) begin
          b_phase <= (phase_step < step) ? phase_step : step;
          add <= (phase_step <= {1'b0, cycle_buf[AddSubLatency][WIDTH:1]});
        end else begin
          b_phase <= (step_n < phase_step) ? phase_step : step_n;
          add <= ({1'b1, cycle_n_buf[AddSubLatency][WIDTH:1]} <= phase_step);
        end
        if (calc_step_cnt > AddSubLatency) begin
          calc_cnt <= calc_cnt + 1;
        end

        // step 3: make phase be in [0, T-1]
        a_phase_fold <= s_phase;
        if (s_phase >= cycle_buf[1+AddSubLatency+AddSubLatency]) begin
          b_phase_fold <= cycle_buf[1+AddSubLatency+AddSubLatency];
          add_fold <= 1'b0;
        end else if (s_phase[WIDTH] == 1'b1) begin
          b_phase_fold <= cycle_buf[1+AddSubLatency+AddSubLatency];
          add_fold <= 1'b1;
        end else begin
          b_phase_fold <= '0;
          add_fold <= 1'b1;
        end

        if (calc_cnt > AddSubLatency) begin
          if (fold_cnt <= DEPTH - 1) begin
            current_duty[fold_cnt] <= s_duty;
          end
          fold_cnt <= fold_cnt + 1;
        end
        if (fold_cnt > AddSubLatency) begin
          dout_valid <= 1'b1;
          current_phase[set_cnt] <= s_phase_fold;
          set_cnt <= set_cnt + 1;
          if (set_cnt == DEPTH - 1) begin
            state <= WAITING;
          end
        end
      end
      default: begin
      end
    endcase
  end

endmodule
