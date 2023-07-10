/*
 * File: cycle_buffer.sv
 * Project: pwm
 * Created Date: 31/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */



`timescale 1ns / 1ps
module cycle_buffer #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK,
    input var [WIDTH-1:0] CYCLE[DEPTH],
    output var [WIDTH-1:0] CYCLE_M1[DEPTH],
    output var [WIDTH-1:0] CYCLE_M2[DEPTH]
);

  bit [WIDTH-1:0] cycle_m1[DEPTH];
  bit [WIDTH-1:0] cycle_m2[DEPTH];

  for (genvar i = 0; i < DEPTH; i++) begin : gen_cycle_buffer
    assign CYCLE_M1[i] = cycle_m1[i];
    assign CYCLE_M2[i] = cycle_m2[i];
    always_ff @(posedge CLK) begin
      cycle_m1[i] <= CYCLE[i] - 1;
      cycle_m2[i] <= cycle_m1[i] - 1;
    end
  end

endmodule
