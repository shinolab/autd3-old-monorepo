/*
 * File: modulation_multiplier.sv
 * Project: modulation
 * Created Date: 24/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module modulation_multiplier #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK,
    input var [15:0] CYCLE_M,
    input var DIN_VALID,
    input var [15:0] IDX,
    modulation_bus_if.sampler_port M_BUS,
    input var [15:0] DELAY_M[DEPTH],
    input var [WIDTH-1:0] DUTY_IN,
    output var [WIDTH-1:0] DUTY_OUT,
    input var [WIDTH-1:0] PHASE_IN,
    output var [WIDTH-1:0] PHASE_OUT,
    output var DOUT_VALID
);

  localparam int BRAMLatency = 2;
  localparam int AddSubLatency = 2;
  localparam int MultLatency = 3;
  localparam int DivLatency = 34;
  localparam int Latency = BRAMLatency + 2 * AddSubLatency + MultLatency + DivLatency;

  bit [WIDTH-1:0] phase_buf[DEPTH];
  bit [$clog2(DEPTH):0] phase_set_cnt = 0;
  bit [WIDTH-1:0] phase_out;

  bit [31:0] cycle;
  bit [WIDTH-1:0] duty_buf[8];

  bit [7:0] mod;

  bit [WIDTH-1:0] duty_m;

  bit [17:0] idx_offset_a, idx_offset_b, idx_offset_s;
  bit [17:0] idx_oc_a, idx_oc_b, idx_oc_s;

  bit signed [WIDTH+9:0] p;

  bit [31:0] quo;
  bit [7:0] _unused;

  bit [$clog2(DEPTH+(Latency+1))-1:0] cnt, delay_cnt, calc_cnt, set_cnt;
  bit dout_valid = 0;

  assign M_BUS.ADDR = idx_oc_s[15:0];
  assign mod = M_BUS.M;
  assign DOUT_VALID = dout_valid;
  assign DUTY_OUT = duty_m;
  assign PHASE_OUT = phase_out;

  addsub #(
      .WIDTH(18)
  ) addsub_o (
      .CLK(CLK),
      .A  (idx_offset_a),
      .B  (idx_offset_b),
      .ADD(1'b0),
      .S  (idx_offset_s)
  );

  addsub #(
      .WIDTH(18)
  ) addsub_oc (
      .CLK(CLK),
      .A  (idx_oc_a),
      .B  (idx_oc_b),
      .ADD(1'b1),
      .S  (idx_oc_s)
  );

  mult #(
      .WIDTH_A(WIDTH + 1),
      .WIDTH_B(9)
  ) mult (
      .CLK(CLK),
      .A  ({1'b0, duty_buf[7]}),
      .B  ({1'b0, mod}),
      .P  (p)
  );

  div_32_8 div_32_8 (
      .s_axis_dividend_tdata({10'd0, p[WIDTH+8:0]}),
      .s_axis_dividend_tvalid(1'b1),
      .s_axis_divisor_tdata(8'hFF),
      .s_axis_divisor_tvalid(1'b1),
      .aclk(CLK),
      .m_axis_dout_tdata({quo, _unused}),
      .m_axis_dout_tvalid()
  );

  typedef enum bit {
    WAITING,
    RUN
  } state_t;

  state_t state = WAITING;

  always_ff @(posedge CLK) begin
    case (state)
      WAITING: begin
        dout_valid <= 1'b0;
        if (DIN_VALID) begin
          cnt <= 0;
          calc_cnt <= 0;
          set_cnt <= 0;

          phase_buf[0] <= PHASE_IN;
          phase_set_cnt <= 1;

          idx_offset_a <= {2'b00, IDX};
          idx_offset_b <= {2'b00, DELAY_M[0]};
          delay_cnt <= 1;
          cycle <= CYCLE_M + 1;

          state <= RUN;
        end
      end
      RUN: begin
        if (phase_set_cnt < DEPTH) begin
          phase_buf[phase_set_cnt] <= PHASE_IN;
          phase_set_cnt <= phase_set_cnt + 1;
        end

        delay_cnt <= delay_cnt + 1;
        idx_offset_b <= {2'b00, DELAY_M[delay_cnt]};
        idx_oc_a <= idx_offset_s;
        idx_oc_b <= idx_offset_s[17] == 1'b1 ? {1'b0, cycle[16:0]} : 0;
        cnt <= cnt + 1;

        if (cnt > Latency) begin
          dout_valid <= 1'b1;
          duty_m <= quo[WIDTH-1:0];
          phase_out <= phase_buf[set_cnt];
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
    duty_buf[0] <= DUTY_IN;
    duty_buf[1] <= duty_buf[0];
    duty_buf[2] <= duty_buf[1];
    duty_buf[3] <= duty_buf[2];
    duty_buf[4] <= duty_buf[3];
    duty_buf[5] <= duty_buf[4];
    duty_buf[6] <= duty_buf[5];
    duty_buf[7] <= duty_buf[6];
  end

endmodule
