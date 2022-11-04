/*
 * File: buffer.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
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
    output var [WIDTH-1:0] RISE_OUT,
    output var [WIDTH-1:0] FALL_OUT
);

  bit [WIDTH-1:0] t;
  bit [WIDTH-1:0] R;
  bit [WIDTH-1:0] F;

  assign t = TIME_CNT;
  assign RISE_OUT = R;
  assign FALL_OUT = F;

  always_ff @(posedge CLK) begin
    if (t == CYCLE_M1) begin
      R <= RISE_IN;
      F <= FALL_IN;
    end
  end

endmodule
