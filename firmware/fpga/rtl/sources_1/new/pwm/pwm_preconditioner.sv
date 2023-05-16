/*
 * File: pwm_preconditioner.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */


`timescale 1ns / 1ps
module pwm_preconditioner #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK,
    input var DIN_VALID,
    input var [WIDTH-1:0] CYCLE[DEPTH],
    input var [WIDTH-1:0] DUTY,
    input var [WIDTH-1:0] PHASE,
    output var [WIDTH-1:0] RISE[DEPTH],
    output var [WIDTH-1:0] FALL[DEPTH],
    output var DOUT_VALID
);

  localparam int AddSubLatency = 2;

  bit [WIDTH-1:0] rise[DEPTH], fall[DEPTH];

  bit signed [WIDTH-1:0] cycle_buf[6], duty_buf[3], phase_buf;
  bit [WIDTH-1:0] rise_buf[DEPTH], fall_buf[DEPTH];

  bit signed [WIDTH+1:0] a_phase, b_phase, s_phase;
  bit signed [WIDTH+1:0] a_duty_r, b_duty_r, s_duty_r;
  bit signed [WIDTH+1:0] a_rise, b_rise, s_rise;
  bit signed [WIDTH+1:0] a_fall, b_fall, s_fall;
  bit signed [WIDTH+1:0] a_fold_rise, b_fold_rise, s_fold_rise;
  bit fold_rise_addsub;
  bit signed [WIDTH+1:0] a_fold_fall, b_fold_fall, s_fold_fall;

  bit [$clog2(DEPTH+(AddSubLatency+1)*3)-1:0] cnt, lr_cnt, fold_cnt, set_cnt;

  bit dout_valid;

  assign DOUT_VALID = dout_valid;

  for (genvar i = 0; i < DEPTH; i++) begin : gen_rise_fall
    assign RISE[i] = rise[i];
    assign FALL[i] = fall[i];
  end

  addsub #(
      .WIDTH(WIDTH + 2)
  ) sub_phase (
      .CLK(CLK),
      .A  (a_phase),
      .B  (b_phase),
      .ADD(1'b0),
      .S  (s_phase)
  );
  addsub #(
      .WIDTH(WIDTH + 2)
  ) add_duty_r (
      .CLK(CLK),
      .A  (a_duty_r),
      .B  (b_duty_r),
      .ADD(1'b1),
      .S  (s_duty_r)
  );

  addsub #(
      .WIDTH(WIDTH + 2)
  ) sub_rise (
      .CLK(CLK),
      .A  (a_rise),
      .B  (b_rise),
      .ADD(1'b0),
      .S  (s_rise)
  );
  addsub #(
      .WIDTH(WIDTH + 2)
  ) add_fall (
      .CLK(CLK),
      .A  (a_fall),
      .B  (b_fall),
      .ADD(1'b1),
      .S  (s_fall)
  );

  addsub #(
      .WIDTH(WIDTH + 2)
  ) add_fold_rise (
      .CLK(CLK),
      .A  (a_fold_rise),
      .B  (b_fold_rise),
      .ADD(fold_rise_addsub),
      .S  (s_fold_rise)
  );
  addsub #(
      .WIDTH(WIDTH + 2)
  ) sub_fold_fall (
      .CLK(CLK),
      .A  (a_fold_fall),
      .B  (b_fold_fall),
      .ADD(1'b0),
      .S  (s_fold_fall)
  );

  typedef enum bit [2:0] {
    WAITING,
    RUN,
    DONE
  } state_t;

  state_t state = WAITING;

  always_ff @(posedge CLK) begin
    case (state)
      WAITING: begin
        if (DIN_VALID) begin
          cnt <= 0;
          lr_cnt <= 0;
          fold_cnt <= 0;
          set_cnt <= 0;
          dout_valid <= 1'b0;

          state <= RUN;
        end
      end
      RUN: begin
        // step 1
        a_phase <= {2'b00, CYCLE[cnt]};
        b_phase <= {2'b00, phase_buf};
        a_duty_r <= {3'b000, duty_buf[0][WIDTH-1:1]};
        b_duty_r <= duty_buf[0][0];
        cnt <= cnt + 1;

        // step 2
        a_rise <= s_phase;
        b_rise <= {3'b000, duty_buf[AddSubLatency][WIDTH-1:1]};
        a_fall <= s_phase;
        b_fall <= s_duty_r;
        if (cnt > AddSubLatency) begin
          lr_cnt <= lr_cnt + 1;
        end

        // step 3
        a_fold_rise <= s_rise;
        if (s_rise[WIDTH+1] == 1'b1) begin
          b_fold_rise <= cycle_buf[1+AddSubLatency+AddSubLatency];
          fold_rise_addsub <= 1'b1;
        end else if (cycle_buf[1+AddSubLatency+AddSubLatency] <= s_rise) begin
          b_fold_rise <= cycle_buf[1+AddSubLatency+AddSubLatency];
          fold_rise_addsub <= 1'b0;
        end else begin
          b_fold_rise <= 0;
          fold_rise_addsub <= 1'b1;
        end
        a_fold_fall <= s_fall;
        if (cycle_buf[1+AddSubLatency+AddSubLatency] <= s_fall) begin
          b_fold_fall <= cycle_buf[1+AddSubLatency+AddSubLatency];
        end else begin
          b_fold_fall <= 0;
        end
        if (lr_cnt > AddSubLatency) begin
          fold_cnt <= fold_cnt + 1;
        end

        // step 4
        if (fold_cnt > AddSubLatency) begin
          rise_buf[set_cnt] <= s_fold_rise[WIDTH-1:0];
          fall_buf[set_cnt] <= s_fold_fall[WIDTH-1:0];

          set_cnt <= set_cnt + 1;
        end

        if (set_cnt == DEPTH - 1) begin
          state <= DONE;
        end
      end
      DONE: begin
        dout_valid <= 1'b1;

        state <= WAITING;
      end
      default: begin
      end
    endcase
  end

  always_ff @(posedge CLK) begin
    duty_buf[0] <= DUTY;
    duty_buf[1] <= duty_buf[0];
    duty_buf[2] <= duty_buf[1];

    phase_buf <= PHASE;

    cycle_buf[0] <= CYCLE[cnt];
    cycle_buf[1] <= cycle_buf[0];
    cycle_buf[2] <= cycle_buf[1];
    cycle_buf[3] <= cycle_buf[2];
    cycle_buf[4] <= cycle_buf[3];
    cycle_buf[5] <= cycle_buf[4];
  end

  for (genvar i = 0; i < DEPTH; i++) begin : gen_copy_buf
    always_ff @(posedge CLK) begin
      if (state == DONE) begin
        rise[i] <= rise_buf[i];
        fall[i] <= fall_buf[i];
      end
    end
  end

endmodule
