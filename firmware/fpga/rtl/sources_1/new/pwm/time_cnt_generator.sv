/*
 * File: time_cnt_generator.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module time_cnt_generator #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK,
    input var [63:0] SYS_TIME,
    input var [WIDTH-1:0] CYCLE[DEPTH],
    input var [WIDTH-1:0] CYCLE_M1[DEPTH],
    input var [WIDTH-1:0] CYCLE_M2[DEPTH],
    output var [WIDTH-1:0] TIME_CNT[DEPTH]
);

  localparam int DivLatency = 66 + 1;

  bit [63:0] divined;
  bit [15:0] divisor;
  bit [63:0] _unused;
  bit [15:0] rem;

  bit [WIDTH-1:0] t[DEPTH] = '{DEPTH{0}};

  bit [$clog2(DEPTH)-1:0] sync_cnt = DivLatency % DEPTH;
  bit [$clog2(DEPTH)-1:0] set_cnt = 0;

  div_64_16 div_64_16 (
      .s_axis_dividend_tdata(divined),
      .s_axis_dividend_tvalid(1'b1),
      .s_axis_divisor_tdata(divisor),
      .s_axis_divisor_tvalid(1'b1),
      .aclk(CLK),
      .m_axis_dout_tdata({_unused, rem}),
      .m_axis_dout_tvalid()
  );

  for (genvar i = 0; i < DEPTH; i++) begin : gen_time_cnt
    always_ff @(posedge CLK) begin
      if (i == set_cnt) begin
        t[i] <= (t[i] == CYCLE_M2[i]) && (rem[WIDTH-1:0] == 0) ? CYCLE_M1[i] : rem[WIDTH-1:0];
      end else begin
        t[i] <= (t[i] == CYCLE_M1[i]) ? 0 : t[i] + 1;
      end
    end
    assign TIME_CNT[i] = t[i];
  end

  always_ff @(posedge CLK) begin
    divined  <= SYS_TIME[63:0];
    divisor  <= CYCLE[sync_cnt];

    sync_cnt <= (sync_cnt == DEPTH - 1) ? 0 : sync_cnt + 1;
    set_cnt  <= (set_cnt == DEPTH - 1) ? 0 : set_cnt + 1;
  end

endmodule
