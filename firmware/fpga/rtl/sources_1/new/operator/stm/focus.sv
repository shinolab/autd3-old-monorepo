/*
 * File: focus.sv
 * Project: stm
 * Created Date: 13/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/09/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

`timescale 1ns / 1ps
module stm_focus_operator #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK,
    input var [15:0] IDX,
    ss_bus_if.focus_port SS_BUS,
    input var [31:0] SOUND_SPEED,
    input var [WIDTH-1:0] ULTRASOUND_CYCLE[0:DEPTH-1],
    output var [WIDTH-2:0] DUTY[0:DEPTH-1],
    output var [WIDTH-1:0] PHASE[0:DEPTH-1],
    output var START,
    output var DONE
);

  localparam int SQRT_LATENCY = 2 + 2 + 2 + 2 + 10 + 66;
  localparam int DIV_LATENCY = SQRT_LATENCY + 66 + 1;
  localparam int LATENCY = DIV_LATENCY + 249;

  bit [WIDTH-2:0] duty[0:DEPTH-1];
  bit [WIDTH-1:0] phase[0:DEPTH-1];
  bit [WIDTH-2:0] duty_buf[0:DEPTH-1];
  bit [WIDTH-1:0] phase_buf[0:DEPTH-1];

  bit [15:0] idx;
  bit [127:0] data_out;
  bit [15:0] idx_old;
  bit start_buf, start;
  bit done;

  bit [3:0] duty_shift;
  bit signed [17:0] focus_x, focus_y, focus_z;
  bit signed [15:0] trans_x, trans_y;
  bit signed [17:0] dx, dy;
  bit [35:0] dx2, dy2, dz2;
  bit [36:0] dxy2;
  bit [37:0] d2;
  bit [23:0] sqrt_dout;

  bit [63:0] quo;
  bit [31:0] _unused_rem;
  bit [15:0] divined;
  bit [63:0] _unused_quo;
  bit [15:0] rem;

  bit [$clog2(LATENCY)-1:0] cnt;
  bit [$clog2(DEPTH)-1:0] cycle_load_cnt;
  bit [$clog2(DEPTH)-1:0] set_cnt;

  bit [$clog2(DEPTH)-1:0] tr_idx;

  enum bit [2:0] {
    IDLE,
    CALC,
    BUF
  } state = IDLE;

  dist_mem_tr dist_mem_tr (
      .a  (tr_idx),
      .spo({trans_x, trans_y})
  );

  addsub #(
      .WIDTH(18)
  ) addsub_x (
      .CLK(CLK),
      .A  (focus_x),
      .B  ({2'b00, trans_x}),
      .ADD(1'b0),
      .S  (dx)
  );

  addsub #(
      .WIDTH(18)
  ) addsub_y (
      .CLK(CLK),
      .A  (focus_y),
      .B  ({2'b00, trans_y}),
      .ADD(1'b0),
      .S  (dy)
  );

  mult #(
      .WIDTH_A(18),
      .WIDTH_B(18)
  ) mult_x (
      .CLK(CLK),
      .A  (dx),
      .B  (dx),
      .P  (dx2)
  );

  mult #(
      .WIDTH_A(18),
      .WIDTH_B(18)
  ) mult_y (
      .CLK(CLK),
      .A  (dy),
      .B  (dy),
      .P  (dy2)
  );

  mult #(
      .WIDTH_A(18),
      .WIDTH_B(18)
  ) mult_z (
      .CLK(CLK),
      .A  (focus_z),
      .B  (focus_z),
      .P  (dz2)
  );

  addsub #(
      .WIDTH(37)
  ) addsub_xy2 (
      .CLK(CLK),
      .A  ({1'b0, dx2}),
      .B  ({1'b0, dy2}),
      .ADD(1'b1),
      .S  (dxy2)
  );

  addsub #(
      .WIDTH(38)
  ) addsub_xyz2 (
      .CLK(CLK),
      .A  ({1'b0, dxy2}),
      .B  ({2'b00, dz2}),
      .ADD(1'b1),
      .S  (d2)
  );

  sqrt_38 sqrt_38 (
      .aclk(CLK),
      .s_axis_cartesian_tvalid(1'b1),
      .s_axis_cartesian_tdata({2'b00, d2}),
      .m_axis_dout_tvalid(),
      .m_axis_dout_tdata(sqrt_dout)
  );

  div_64_32_l div_64_32_quo (
      .s_axis_dividend_tdata({18'd0, sqrt_dout, 22'd0}),
      .s_axis_dividend_tvalid(1'b1),
      .s_axis_divisor_tdata(SOUND_SPEED),
      .s_axis_divisor_tvalid(1'b1),
      .aclk(CLK),
      .m_axis_dout_tdata({quo, _unused_rem}),
      .m_axis_dout_tvalid()
  );

  div_64_16_l div_64_16_rem (
      .s_axis_dividend_tdata(quo),
      .s_axis_dividend_tvalid(1'b1),
      .s_axis_divisor_tdata(divined),
      .s_axis_divisor_tvalid(1'b1),
      .aclk(CLK),
      .m_axis_dout_tdata({_unused_quo, rem}),
      .m_axis_dout_tvalid()
  );

  assign idx = IDX;
  assign SS_BUS.FOCUS_ADDR = idx;
  assign data_out = SS_BUS.DATA_OUT;

  for (genvar i = 0; i < DEPTH; i++) begin
    assign DUTY[i]  = duty[i];
    assign PHASE[i] = phase[i];
  end

  assign START = start;
  assign DONE  = done;

  always_ff @(posedge CLK) begin
    idx_old <= idx;
    start_buf <= idx != idx_old;
    start <= start_buf;
  end

  always_ff @(posedge CLK) begin
    case (state)
      IDLE: begin
        done <= 0;
        if (start) begin
          focus_x <= data_out[17:0];
          focus_y <= data_out[35:18];
          focus_z <= data_out[53:36];
          duty_shift <= data_out[57:54];
          tr_idx <= 0;
          cnt <= 0;
          cycle_load_cnt <= 0;
          set_cnt <= 0;
          state <= CALC;
        end
      end
      CALC: begin
        tr_idx <= tr_idx == DEPTH - 1 ? tr_idx : tr_idx + 1;
        cnt <= cnt + 1;
        if (cnt >= SQRT_LATENCY) begin
          cycle_load_cnt <= cycle_load_cnt == DEPTH - 1 ? cycle_load_cnt : cycle_load_cnt + 1;
          divined <= ULTRASOUND_CYCLE[cycle_load_cnt];
        end
        if (cnt >= DIV_LATENCY) begin
          if (set_cnt == DEPTH - 1) begin
            state <= BUF;
          end else begin
            set_cnt <= set_cnt + 1;
          end
          phase_buf[set_cnt] <= rem[WIDTH-1:0];
          duty_buf[set_cnt]  <= ULTRASOUND_CYCLE[set_cnt][WIDTH-1:1] >> duty_shift;
        end
      end
      BUF: begin
        phase <= phase_buf;
        duty  <= duty_buf;
        done  <= 1;
        state <= IDLE;
      end
    endcase
  end
endmodule
