/*
 * File: time_cnt_generator.sv
 * Project: new
 * Created Date: 17/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module time_cnt_generator #(
    parameter int DEPTH = 249
) (
    input var CLK,
    input var [63:0] SYS_TIME,
    input var SKIP_ONE_ASSERT,
    output var [8:0] TIME_CNT,
    output var UPDATE
);
  logic [8:0] t_buf0, t_buf1, t;
  logic update;

  assign t_buf0   = SYS_TIME[8:0];

  assign UPDATE   = update;
  assign TIME_CNT = t;

  logic cross0;
  assign cross0 = t_buf0 == 9'd0;

  logic cross0_with_skip;
  assign cross0_with_skip = (t_buf0 == 9'd1) & SKIP_ONE_ASSERT;

  always_ff @(posedge CLK) begin
    t_buf1 <= t_buf0;
    t <= t_buf1;
    if (cross0 | cross0_with_skip) begin
      update <= 1'b1;
    end else begin
      update <= 1'b0;
    end
  end

endmodule
