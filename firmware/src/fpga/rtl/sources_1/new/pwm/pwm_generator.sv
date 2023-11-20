/*
 * File: pwm_generator.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */


`timescale 1ns / 1ps
module pwm_generator #(
    parameter int WIDTH = 9
) (
    input var CLK,
    input var [WIDTH-1:0] TIME_CNT,
    input var [WIDTH-1:0] RISE,
    input var [WIDTH-1:0] FALL,
    output var PWM_OUT
);

  logic [WIDTH-1:0] t;
  logic [WIDTH-1:0] R;
  logic [WIDTH-1:0] F;
  logic v;

  assign t = TIME_CNT;
  assign R = RISE;
  assign F = FALL;
  assign PWM_OUT = v;

  always_ff @(posedge CLK) begin
    if (R <= F) begin
      v <= (R <= t) & (t < F);
    end else begin
      v <= (t < F) | (R <= t);
    end
  end

endmodule
