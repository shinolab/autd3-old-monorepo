/*
 * File: pwm_buffer.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module pwm_buffer #(
    parameter int WIDTH = 13
) (
    input var CLK,
    input var [WIDTH-1:0] CYCLE_M1,
    input var [WIDTH-1:0] TIME_CNT,
    input var [WIDTH-1:0] RISE_IN,
    input var [WIDTH-1:0] FALL_IN,
    input var FULL_WIDTH_IN,
    output var [WIDTH-1:0] RISE_OUT,
    output var [WIDTH-1:0] FALL_OUT,
    output var FULL_WIDTH_OUT
);

  bit [WIDTH-1:0] t;
  bit [WIDTH-1:0] R;
  bit [WIDTH-1:0] F;
  bit FULL_WIDTH;

  assign t = TIME_CNT;
  assign RISE_OUT = R;
  assign FALL_OUT = F;
  assign FULL_WIDTH_OUT = FULL_WIDTH;

  always_ff @(posedge CLK) begin
    if (t == CYCLE_M1) begin
      R <= RISE_IN;
      F <= FALL_IN;
      FULL_WIDTH <= FULL_WIDTH_IN;
    end
  end

endmodule
