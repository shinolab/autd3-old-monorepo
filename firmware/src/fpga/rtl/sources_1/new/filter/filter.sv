/*
 * File: filter.sv
 * Project: filter
 * Created Date: 28/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

module filter #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK,
    input var DIN_VALID,
    input var signed [WIDTH:0] FILTER_DUTY[DEPTH],
    input var signed [WIDTH:0] FILTER_PHASE[DEPTH],
    input var [WIDTH-1:0] CYCLE[DEPTH],
    input var [WIDTH-1:0] DUTY,
    input var [WIDTH-1:0] PHASE,
    output var [WIDTH-1:0] DUTY_F,
    output var [WIDTH-1:0] PHASE_F,
    output var DOUT_VALID
);

  localparam int AddSubLatency = 2;

  bit signed [WIDTH:0] cycle_buf[3];
  bit [WIDTH-1:0] duty_buf, phase_buf;

  bit signed [WIDTH+1:0] a_duty, b_duty, s_duty;
  bit signed [WIDTH-1:0] s_duty_buf[3];
  
  bit signed [WIDTH+1:0] a_phase, b_phase, s_phase;
  bit signed [WIDTH+1:0] a_phase_fold, b_phase_fold, s_phase_fold;
  bit add, add_fold;
  bit [$clog2(DEPTH+(AddSubLatency+1)*2)-1:0] calc_cnt, fold_cnt, set_cnt;

  bit [WIDTH-1:0] duty_f, phase_f;

  bit dout_valid = 0;

  assign DUTY_F = duty_f;
  assign PHASE_F = phase_f;
  assign DOUT_VALID = dout_valid;

  typedef enum bit {
    WAITING,
    RUN
  } state_t;

  state_t state = WAITING;

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
          calc_cnt <= 0;
          fold_cnt <= 0;
          set_cnt <= 0;

          state <= RUN;
        end
      end
      RUN: begin
        // step 1: calculate next duty/phase
        a_duty <= {1'b0, duty_buf};
        b_duty <= FILTER_DUTY[calc_cnt];
        a_phase <= {1'b0, phase_buf};
        b_phase <= FILTER_PHASE[calc_cnt];
        calc_cnt <= calc_cnt + 1;

        // step 2: make duty/phase be in [0, T-1]
        if (s_duty > cycle_buf[AddSubLatency]) begin
          s_duty_buf[0] <= cycle_buf[AddSubLatency][WIDTH-1:0];
        end else if (s_duty[WIDTH] == 1'b1) begin
          s_duty_buf[0] <= '0;
        end else begin
          s_duty_buf[0] <= s_duty[WIDTH-1:0];
        end
        a_phase_fold <= s_phase;
        if (s_phase >= cycle_buf[AddSubLatency]) begin
          b_phase_fold <= cycle_buf[AddSubLatency];
          add_fold <= 1'b0;
        end else if (s_phase[WIDTH] == 1'b1) begin
          b_phase_fold <= cycle_buf[AddSubLatency];
          add_fold <= 1'b1;
        end else begin
          b_phase_fold <= '0;
          add_fold <= 1'b1;
        end

        if (calc_cnt > AddSubLatency) begin
          fold_cnt <= fold_cnt + 1;
        end

        if (fold_cnt > AddSubLatency) begin
          dout_valid <= 1'b1;
          duty_f <= s_duty_buf[2][WIDTH-1:0];
          phase_f <= s_phase_fold[WIDTH-1:0];
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

    cycle_buf[0] <= {1'b0, CYCLE[calc_cnt]};
    cycle_buf[1] <= cycle_buf[0];
    cycle_buf[2] <= cycle_buf[1];

    s_duty_buf[1] <= s_duty_buf[0];
    s_duty_buf[2] <= s_duty_buf[1];
  end

endmodule
