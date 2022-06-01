/*
 * File: mult.sv
 * Project: common
 * Created Date: 07/01/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

`timescale 1ns / 1ps
module mult#(
           parameter int WIDTH_A = 16,
           parameter int WIDTH_B = 16
       )(
           input var CLK,
           input var signed [WIDTH_A-1:0] A,
           input var signed [WIDTH_B-1:0] B,
           output var signed [WIDTH_A+WIDTH_B-1:0] P
       );

MULT_MACRO #(
               .DEVICE("7SERIES"),
               .LATENCY(3),
               .WIDTH_A(WIDTH_A),
               .WIDTH_B(WIDTH_B)
           ) MULT_MACRO_inst (
               .P(P),
               .A(A),
               .B(B),
               .CE(1'b1),
               .CLK(CLK),
               .RST()
           );

endmodule
