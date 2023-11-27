/*
 * File: stm_focus_operator.sv
 * Project: stm
 * Created Date: 13/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module stm_focus_operator #(
    parameter int DEPTH = 249
) (
    input var CLK,
    input var [15:0] IDX,
    input var UPDATE,
    stm_bus_if.focus_port STM_BUS,
    input var [31:0] SOUND_SPEED,
    output var [7:0] INTENSITY,
    output var [7:0] PHASE,
    output var DOUT_VALID
);

  localparam int DivLatency = 2 + 2 + 2 + 2 + 10 + 66 + 1;
  localparam int Latency = DivLatency + 249;

  logic [7:0] intensity;
  logic [7:0] phase;

  logic [15:0] addr;
  logic [127:0] data_out;
  logic dout_valid;

  logic [7:0] intensity_buf;
  logic signed [17:0] focus_x, focus_y, focus_z;
  logic signed [15:0] trans_x, trans_y;
  logic signed [17:0] dx, dy;
  logic [35:0] dx2, dy2, dz2;
  logic [36:0] dxy2;
  logic [37:0] d2;
  logic [23:0] sqrt_dout;

  logic [63:0] quo;
  logic [31:0] _unused_rem;

  logic [$clog2(Latency)-1:0] cnt;
  logic [$clog2(DEPTH)-1:0] set_cnt;

  logic [$clog2(DEPTH)-1:0] tr_idx;

  typedef enum logic [2:0] {
    WAITING,
    BRAM_WAIT_0,
    BRAM_WAIT_1,
    LOAD,
    CALC
  } state_t;

  state_t state = WAITING;

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

  div_64_32 div_64_32_quo (
      .s_axis_dividend_tdata({22'd0, sqrt_dout, 18'd0}),
      .s_axis_dividend_tvalid(1'b1),
      .s_axis_divisor_tdata(SOUND_SPEED),
      .s_axis_divisor_tvalid(1'b1),
      .aclk(CLK),
      .m_axis_dout_tdata({quo, _unused_rem}),
      .m_axis_dout_tvalid()
  );

  assign STM_BUS.FOCUS_ADDR = addr;
  assign data_out = STM_BUS.DATA_OUT;

  assign INTENSITY = intensity;
  assign PHASE = phase;
  assign DOUT_VALID = dout_valid;

  always_ff @(posedge CLK) begin
    case (state)
      WAITING: begin
        dout_valid <= 1'b0;
        if (UPDATE) begin
          addr  <= IDX;
          state <= BRAM_WAIT_0;
        end
      end
      BRAM_WAIT_0: begin
        state <= BRAM_WAIT_1;
      end
      BRAM_WAIT_1: begin
        state <= LOAD;
      end
      LOAD: begin
        focus_x <= data_out[17:0];
        focus_y <= data_out[35:18];
        focus_z <= data_out[53:36];
        intensity_buf <= data_out[61:54];
        tr_idx <= 0;
        cnt <= 0;
        set_cnt <= 0;

        state <= CALC;
      end
      CALC: begin
        tr_idx <= tr_idx + 1;
        cnt <= cnt + 1;

        if (cnt >= DivLatency) begin
          dout_valid <= 1'b1;
          set_cnt <= set_cnt + 1;

          phase <= quo[7:0];
          intensity <= intensity_buf;
        end

        if (set_cnt == DEPTH - 1) begin
          state <= WAITING;
        end
      end
      default: begin
      end
    endcase
  end

endmodule
