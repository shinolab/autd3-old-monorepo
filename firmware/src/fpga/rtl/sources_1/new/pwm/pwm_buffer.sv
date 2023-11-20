/*
 * File: pwm_buffer.sv
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
module pwm_buffer #(
    parameter int WIDTH = 9
) (
    input var CLK,
    input var UPDATE,
    input var [WIDTH-1:0] RISE_IN,
    input var [WIDTH-1:0] FALL_IN,
    output var [WIDTH-1:0] RISE_OUT,
    output var [WIDTH-1:0] FALL_OUT
);

  logic [WIDTH-1:0] R;
  logic [WIDTH-1:0] F;

  assign RISE_OUT = R;
  assign FALL_OUT = F;

  always_ff @(posedge CLK) begin
    if (UPDATE) begin
      R <= RISE_IN;
      F <= FALL_IN;
    end
  end

endmodule
