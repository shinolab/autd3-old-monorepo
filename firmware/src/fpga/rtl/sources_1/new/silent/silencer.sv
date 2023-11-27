/*
 * File: silencer.sv
 * Project: silent
 * Created Date: 22/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
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
    input var [15:0] STEP_INTENSITY,
    input var [15:0] STEP_PHASE,
    input var [15:0] INTENSITY_IN,
    input var [7:0] PHASE_IN,
    output var [15:0] INTENSITY_OUT,
    output var [7:0] PHASE_OUT,
    output var DOUT_VALID
);

  localparam int AddSubLatency = 2;

  logic signed [17:0] step_intensity, step_intensity_n;
  logic signed [17:0] step_phase, step_phase_n;
  logic [15:0] intensity_buf;
  logic [15:0] phase_buf;

  logic signed [17:0] current_intensity[DEPTH] = '{DEPTH{0}};
  logic signed [17:0] current_phase[DEPTH] = '{DEPTH{0}};
  logic signed [17:0] a_intensity_step, b_intensity_step, intensity_step;
  logic signed [17:0] intensity_step_buf[3];
  logic signed [17:0] a_phase_step, b_phase_step, phase_step;
  logic signed [17:0] a_phase_fg, b_phase_fg, s_phase_fg;
  logic add_phase_fg;
  logic signed [17:0] a_intensity, b_intensity, s_intensity;
  logic signed [17:0] a_phase, b_phase, s_phase;
  logic signed [17:0] a_phase_fold, b_phase_fold, s_phase_fold;
  logic add_fold;
  logic [$clog2(DEPTH+(AddSubLatency+1)*4)-1:0] calc_cnt, calc_step_cnt, fold_cnt, set_cnt;

  logic [15:0] intensity_s;
  logic [7:0] phase_s;

  logic dout_valid = 0;

  assign INTENSITY_OUT = intensity_s;
  assign PHASE_OUT = phase_s;
  assign DOUT_VALID = dout_valid;

  typedef enum logic {
    WAITING,
    RUN
  } state_t;

  state_t state = WAITING;

  addsub #(
      .WIDTH(18)
  ) sub_intensity_step (
      .CLK(CLK),
      .A  (a_intensity_step),
      .B  (b_intensity_step),
      .ADD(1'b0),
      .S  (intensity_step)
  );

  addsub #(
      .WIDTH(18)
  ) sub_phase_step (
      .CLK(CLK),
      .A  (a_phase_step),
      .B  (b_phase_step),
      .ADD(1'b0),
      .S  (phase_step)
  );

  addsub #(
      .WIDTH(18)
  ) phase_fg (
      .CLK(CLK),
      .A  (a_phase_fg),
      .B  (b_phase_fg),
      .ADD(add_phase_fg),
      .S  (s_phase_fg)
  );

  addsub #(
      .WIDTH(18)
  ) add_intensity (
      .CLK(CLK),
      .A  (a_intensity),
      .B  (b_intensity),
      .ADD(1'b1),
      .S  (s_intensity)
  );

  addsub #(
      .WIDTH(18)
  ) addsub_phase (
      .CLK(CLK),
      .A  (a_phase),
      .B  (b_phase),
      .ADD(1'b1),
      .S  (s_phase)
  );

  addsub #(
      .WIDTH(18)
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
          step_intensity <= {2'b00, STEP_INTENSITY};
          step_intensity_n <= -{2'b00, STEP_INTENSITY};
          step_phase <= {2'b00, STEP_PHASE};
          step_phase_n <= -{2'b00, STEP_PHASE};

          calc_step_cnt <= 0;
          calc_cnt <= 0;
          fold_cnt <= 0;
          set_cnt <= 0;

          state <= RUN;
        end
      end
      RUN: begin
        // intensity 1: calculate step
        a_intensity_step <= {2'b00, intensity_buf};
        b_intensity_step <= current_intensity[calc_step_cnt];

        // intensity 2: wait phase
        intensity_step_buf[0] <= intensity_step;
        intensity_step_buf[1] <= intensity_step_buf[0];
        intensity_step_buf[2] <= intensity_step_buf[1];

        // intensity 3: calculate next intensity
        a_intensity <= current_intensity[calc_cnt];
        if (intensity_step_buf[2][17] == 1'b0) begin
          b_intensity <= (intensity_step_buf[2] < step_intensity) ? intensity_step_buf[2] : step_intensity;
        end else begin
          b_intensity <= (step_intensity_n < intensity_step_buf[2]) ? intensity_step_buf[2] : step_intensity_n;
        end

        // phase 1: calculate step
        a_phase_step <= {2'b00, phase_buf};
        b_phase_step <= current_phase[calc_step_cnt];

        // phase 2: should phase go forward or back?
        a_phase_fg   <= phase_step;
        if (phase_step[17] == 1'b0) begin
          if (phase_step <= 18'sd32768) begin
            b_phase_fg <= '0;
          end else begin
            b_phase_fg   <= 18'sd65536;
            add_phase_fg <= 1'b0;
          end
        end else begin
          if (-18'sd65536 <= phase_step) begin
            b_phase_fg <= '0;
          end else begin
            b_phase_fg   <= 18'sd65536;
            add_phase_fg <= 1'b1;
          end
        end

        // phase 3: calculate next phase
        a_phase <= current_phase[calc_cnt];
        if (s_phase_fg[17] == 1'b0) begin
          b_phase <= (s_phase_fg < step_phase) ? s_phase_fg : step_phase;
        end else begin
          b_phase <= (step_phase_n < s_phase_fg) ? s_phase_fg : step_phase_n;
        end

        // phase 4: make phase be in [0, T-1]
        a_phase_fold <= s_phase;
        if (s_phase >= 18'sd65536) begin
          b_phase_fold <= 18'sd65536;
          add_fold <= 1'b0;
        end else if (s_phase[17] == 1'b1) begin
          b_phase_fold <= 18'sd65536;
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
            current_intensity[fold_cnt] <= s_intensity;
          end
          fold_cnt <= fold_cnt + 1;
        end
        if (fold_cnt > AddSubLatency) begin
          dout_valid <= 1'b1;
          current_phase[set_cnt] <= s_phase_fold;
          intensity_s <= current_intensity[set_cnt];
          phase_s <= s_phase_fold[15:8];
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
    intensity_buf <= INTENSITY_IN;
    phase_buf <= {PHASE_IN, 8'h00};
  end

endmodule
