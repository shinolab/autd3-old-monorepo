/*
 * File: silencer.sv
 * Project: silent
 * Created Date: 22/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

module silencer #(
    parameter int DEPTH = 249
) (
    input var CLK,
    input var DIN_VALID,
    input var [8:0] STEP,
    input var [8:0] PULSE_WIDTH_IN,
    input var [7:0] PHASE_IN,
    output var [8:0] PULSE_WIDTH_OUT,
    output var [7:0] PHASE_OUT,
    output var DOUT_VALID
);

  localparam int AddSubLatency = 2;

  logic signed [10:0] step, step_n;
  logic [8:0] pulse_width_buf;
  logic [7:0] phase_buf;

  logic signed [10:0] current_pulse_width[DEPTH] = '{DEPTH{0}};
  logic signed [10:0] current_phase[DEPTH] = '{DEPTH{0}};
  logic signed [10:0] a_pulse_width_step, b_pulse_width_step, pulse_width_step;
  logic signed [10:0] pulse_width_step_buf[3];
  logic signed [10:0] a_phase_step, b_phase_step, phase_step;
  logic signed [10:0] a_phase_fg, b_phase_fg, s_phase_fg;
  logic add_phase_fg;
  logic signed [10:0] a_pulse_width, b_pulse_width, s_pulse_width;
  logic signed [10:0] a_phase, b_phase, s_phase;
  logic signed [10:0] a_phase_fold, b_phase_fold, s_phase_fold;
  logic add_fold;
  logic [$clog2(DEPTH+(AddSubLatency+1)*4)-1:0] calc_cnt, calc_step_cnt, fold_cnt, set_cnt;

  logic [8:0] pulse_width_s;
  logic [7:0] phase_s;

  logic dout_valid = 0;

  assign PULSE_WIDTH_OUT = pulse_width_s;
  assign PHASE_OUT = phase_s;
  assign DOUT_VALID = dout_valid;

  typedef enum logic {
    WAITING,
    RUN
  } state_t;

  state_t state = WAITING;

  addsub #(
      .WIDTH(11)
  ) sub_pulse_width_step (
      .CLK(CLK),
      .A  (a_pulse_width_step),
      .B  (b_pulse_width_step),
      .ADD(1'b0),
      .S  (pulse_width_step)
  );

  addsub #(
      .WIDTH(11)
  ) sub_phase_step (
      .CLK(CLK),
      .A  (a_phase_step),
      .B  (b_phase_step),
      .ADD(1'b0),
      .S  (phase_step)
  );

  addsub #(
      .WIDTH(11)
  ) phase_fg (
      .CLK(CLK),
      .A  (a_phase_fg),
      .B  (b_phase_fg),
      .ADD(add_phase_fg),
      .S  (s_phase_fg)
  );

  addsub #(
      .WIDTH(11)
  ) add_pulse_width (
      .CLK(CLK),
      .A  (a_pulse_width),
      .B  (b_pulse_width),
      .ADD(1'b1),
      .S  (s_pulse_width)
  );

  addsub #(
      .WIDTH(11)
  ) addsub_phase (
      .CLK(CLK),
      .A  (a_phase),
      .B  (b_phase),
      .ADD(1'b1),
      .S  (s_phase)
  );

  addsub #(
      .WIDTH(11)
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
        // pulse_width 1: calculate step
        a_pulse_width_step <= {2'b00, pulse_width_buf};
        b_pulse_width_step <= current_pulse_width[calc_step_cnt];

        // pulse_width 2: wait phase
        pulse_width_step_buf[0] <= pulse_width_step;
        pulse_width_step_buf[1] <= pulse_width_step_buf[0];
        pulse_width_step_buf[2] <= pulse_width_step_buf[1];

        // pulse_width 3: calculate next pulse_width
        a_pulse_width <= current_pulse_width[calc_cnt];
        if (pulse_width_step_buf[2][10] == 1'b0) begin
          b_pulse_width <= (pulse_width_step_buf[2] < step) ? pulse_width_step_buf[2] : step;
        end else begin
          b_pulse_width <= (step_n < pulse_width_step_buf[2]) ? pulse_width_step_buf[2] : step_n;
        end

        // phase 1: calculate step
        a_phase_step <= {2'b00, phase_buf};
        b_phase_step <= current_phase[calc_step_cnt];

        // phase 2: should phase go forward or back?
        a_phase_fg   <= phase_step;
        if (phase_step[10] == 1'b0) begin
          if (phase_step <= 11'sd128) begin
            b_phase_fg <= '0;
          end else begin
            b_phase_fg   <= 11'sd256;
            add_phase_fg <= 1'b0;
          end
        end else begin
          if (-11'sd256 <= phase_step) begin
            b_phase_fg <= '0;
          end else begin
            b_phase_fg   <= 11'sd256;
            add_phase_fg <= 1'b1;
          end
        end

        // phase 3: calculate next phase
        a_phase <= current_phase[calc_cnt];
        if (s_phase_fg[10] == 1'b0) begin
          b_phase <= (s_phase_fg < step) ? s_phase_fg : step;
        end else begin
          b_phase <= (step_n < s_phase_fg) ? s_phase_fg : step_n;
        end

        // phase 4: make phase be in [0, T-1]
        a_phase_fold <= s_phase;
        if (s_phase >= 11'sd256) begin
          b_phase_fold <= 11'sd256;
          add_fold <= 1'b0;
        end else if (s_phase[10] == 1'b1) begin
          b_phase_fold <= 11'sd256;
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
            current_pulse_width[fold_cnt] <= s_pulse_width;
          end
          fold_cnt <= fold_cnt + 1;
        end
        if (fold_cnt > AddSubLatency) begin
          dout_valid <= 1'b1;
          current_phase[set_cnt] <= s_phase_fold;
          pulse_width_s <= current_pulse_width[set_cnt][8:0];
          phase_s <= s_phase_fold[7:0];
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
    pulse_width_buf <= PULSE_WIDTH_IN;
    phase_buf <= PHASE_IN;
  end

endmodule
