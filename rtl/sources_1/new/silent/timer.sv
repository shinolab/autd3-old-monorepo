/*
 * File: timer.sv
 * Project: silent
 * Created Date: 22/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

`timescale 1ns / 1ps
module silent_timer#(
           parameter int WIDTH = 13
       )(
           input var CLK,
           input var [63:0] SYS_TIME,
           input var [15:0] CYCLE,
           output var UPDATE
       );

bit [63:0] divined;
bit [15:0] cycle;
bit [63:0] _unused;
bit [15:0] rem;

bit [1:0] zero_cross;
bit update;

assign UPDATE = update;

div_64_16 div_64_16(
              .s_axis_dividend_tdata(divined),
              .s_axis_dividend_tvalid(1'b1),
              .s_axis_divisor_tdata(cycle),
              .s_axis_divisor_tvalid(1'b1),
              .aclk(CLK),
              .m_axis_dout_tdata({_unused, rem}),
              .m_axis_dout_tvalid()
          );

always_ff @(posedge CLK) begin
    zero_cross <= {zero_cross[0], rem < {1'b0, cycle[15:1]}};
    update <= zero_cross == 2'b01;
end

always_ff @(posedge CLK) begin
    divined <= SYS_TIME[63:0];
    cycle <= CYCLE;
end

endmodule
