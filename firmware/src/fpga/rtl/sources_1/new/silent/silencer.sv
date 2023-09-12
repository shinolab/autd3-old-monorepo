/*
 * File: silencer.sv
 * Project: silent
 * Created Date: 22/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/08/2023
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
    input var [WIDTH-1:0] DUTY,
    input var [WIDTH-1:0] PHASE,
    output var [WIDTH-1:0] DUTY_S,
    output var [WIDTH-1:0] PHASE_S,
    output var DOUT_VALID
);

  localparam int AddSubLatency = 2;

  bit signed [WIDTH+1:0] step, step_n;
  bit signed [WIDTH+1:0] cycle_buf[9], cycle_n_buf[3];
  bit [WIDTH-1:0] duty_buf, phase_buf;

  bit signed [WIDTH+1:0] current_duty [DEPTH] = '{DEPTH{0}};
  bit signed [WIDTH+1:0] current_phase[DEPTH] = '{DEPTH{0}};
  bit signed [WIDTH+1:0] a_duty_step, b_duty_step, duty_step;
  bit signed [WIDTH+1:0] duty_step_buf[3];
  bit signed [WIDTH+1:0] a_phase_step, b_phase_step, phase_step;
  bit signed [WIDTH+1:0] a_phase_fg, b_phase_fg, s_phase_fg;
  bit add_phase_fg;
  bit signed [WIDTH+1:0] a_duty, b_duty, s_duty;
  bit signed [WIDTH+1:0] a_phase, b_phase, s_phase;
  bit signed [WIDTH+1:0] a_phase_fold, b_phase_fold, s_phase_fold;
  bit add_fold;
  bit [$clog2(DEPTH+(AddSubLatency+1)*4)-1:0] calc_cnt, calc_step_cnt, fold_cnt, set_cnt;

  bit [WIDTH-1:0] duty_s, phase_s;

  bit dout_valid = 0;

  assign DUTY_S = duty_s;
  assign PHASE_S = phase_s;
  assign DOUT_VALID = dout_valid;

  typedef enum bit {
    WAITING,
    RUN
  } state_t;

  state_t state = WAITING;

  addsub #(
      .WIDTH(WIDTH + 2)
  ) sub_duty_step (
      .CLK(CLK),
      .A  (a_duty_step),
      .B  (b_duty_step),
      .ADD(1'b0),
      .S  (duty_step)
  );

  addsub #(
      .WIDTH(WIDTH + 2)
  ) sub_phase_step (
      .CLK(CLK),
      .A  (a_phase_step),
      .B  (b_phase_step),
      .ADD(1'b0),
      .S  (phase_step)
  );

  addsub #(
      .WIDTH(WIDTH + 2)
  ) phase_fg (
      .CLK(CLK),
      .A  (a_phase_fg),
      .B  (b_phase_fg),
      .ADD(add_phase_fg),
      .S  (s_phase_fg)
  );

  addsub #(
      .WIDTH(WIDTH + 2)
  ) add_duty (
      .CLK(CLK),
      .A  (a_duty),
      .B  (b_duty),
      .ADD(1'b1),
      .S  (s_duty)
  );

  addsub #(
      .WIDTH(WIDTH + 2)
  ) addsub_phase (
      .CLK(CLK),
      .A  (a_phase),
      .B  (b_phase),
      .ADD(1'b1),
      .S  (s_phase)
  );

  addsub #(
      .WIDTH(WIDTH + 2)
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
        dout_valid <= 1'b0;
        if (DIN_VALID) begin
          step <= {2'b00, STEP};
          step_n <= -{2'b00, STEP};

          calc_step_cnt <= 0;
          calc_cnt <= 0;
          fold_cnt <= 0;
          set_cnt <= 0;

          state <= RUN;
        end
      end
      RUN: begin
        // duty 1: calculate step
        a_duty_step <= {2'b00, duty_buf};
        b_duty_step <= current_duty[calc_step_cnt];

        // duty 2: wait phase
        duty_step_buf[0] <= duty_step;
        duty_step_buf[1] <= duty_step_buf[0];
        duty_step_buf[2] <= duty_step_buf[1];

        // duty 3: calculate next duty
        a_duty <= current_duty[calc_cnt];
        if (duty_step_buf[2][WIDTH+1] == 1'b0) begin
          b_duty <= (duty_step_buf[2] < step) ? duty_step_buf[2] : step;
        end else begin
          b_duty <= (step_n < duty_step_buf[2]) ? duty_step_buf[2] : step_n;
        end

        // phase 1: calculate step
        a_phase_step <= {2'b00, phase_buf};
        b_phase_step <= current_phase[calc_step_cnt];

        // phase 2: should phase go forward or back?
        a_phase_fg <= phase_step;
        if (phase_step[WIDTH+1] == 1'b0) begin
          if (phase_step <= {1'b0, cycle_buf[AddSubLatency][WIDTH+1:1]}) begin
            b_phase_fg <= '0;
          end else begin
            b_phase_fg <= cycle_buf[AddSubLatency];
            add_phase_fg <= 1'b0;
          end
        end else begin
          if ({1'b1, cycle_n_buf[AddSubLatency][WIDTH+1:1]} <= phase_step) begin
            b_phase_fg <= '0;
          end else begin
            b_phase_fg <= cycle_buf[AddSubLatency];
            add_phase_fg <= 1'b1;
          end
        end

        // phase 3: calculate next phase
        a_phase <= current_phase[calc_cnt];
        if (s_phase_fg[WIDTH+1] == 1'b0) begin
          b_phase <= (s_phase_fg < step) ? s_phase_fg : step;
        end else begin
          b_phase <= (step_n < s_phase_fg) ? s_phase_fg : step_n;
        end

        // phase 4: make phase be in [0, T-1]
        a_phase_fold <= s_phase;
        if (s_phase >= cycle_buf[1+AddSubLatency+1+AddSubLatency+AddSubLatency]) begin
          b_phase_fold <= cycle_buf[1+AddSubLatency+1+AddSubLatency+AddSubLatency];
          add_fold <= 1'b0;
        end else if (s_phase[WIDTH+1] == 1'b1) begin
          b_phase_fold <= cycle_buf[1+AddSubLatency+1+AddSubLatency+AddSubLatency];
          add_fold <= 1'b1;
        end else begin
          b_phase_fold <= '0;
          add_fold <= 1'b1;
        end

        calc_step_cnt <= calc_step_cnt + 1;
        if (calc_step_cnt > 1 + AddSubLatency + AddSubLatency) begin
          calc_cnt <= calc_cnt + 1;
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
          duty_s <= current_duty[set_cnt][WIDTH-1:0];
          phase_s <= s_phase_fold[WIDTH-1:0];
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

  always_ff @(posedge CLK) begin
    duty_buf <= DUTY;
    phase_buf <= PHASE;

    cycle_buf[0] <= {2'b00, CYCLE[calc_step_cnt]};
    cycle_buf[1] <= cycle_buf[0];
    cycle_buf[2] <= cycle_buf[1];
    cycle_buf[3] <= cycle_buf[2];
    cycle_buf[4] <= cycle_buf[3];
    cycle_buf[5] <= cycle_buf[4];
    cycle_buf[6] <= cycle_buf[5];
    cycle_buf[7] <= cycle_buf[6];
    cycle_buf[8] <= cycle_buf[7];
    cycle_n_buf[0] <= -{2'b00, CYCLE[calc_step_cnt]};
    cycle_n_buf[1] <= cycle_n_buf[0];
    cycle_n_buf[2] <= cycle_n_buf[1];
  end

endmodule
