/*
 * File: sampler.sv
 * Project: stm
 * Created Date: 13/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */

`timescale 1ns / 1ps
module stm_sampler(
           input var CLK,
           input var [63:0] SYS_TIME,
           input var [15:0] CYCLE,
           input var [31:0] FREQ_DIV,
           output var [15:0] IDX
       );

bit [63:0] divined;
bit [31:0] freq_div;
bit [63:0] quo;
bit [31:0] _unused_rem;
bit [63:0] _unused_quo;
bit [31:0] cycle;
bit [31:0] rem;
bit rem_tvalid;

assign IDX = rem[15:0];

div_64_32 div_64_32_quo(
              .s_axis_dividend_tdata(divined),
              .s_axis_dividend_tvalid(1'b1),
              .s_axis_divisor_tdata(freq_div),
              .s_axis_divisor_tvalid(1'b1),
              .aclk(CLK),
              .m_axis_dout_tdata({quo, _unused_rem}),
              .m_axis_dout_tvalid()
          );

div_64_32 div_64_32_rem(
              .s_axis_dividend_tdata(quo),
              .s_axis_dividend_tvalid(1'b1),
              .s_axis_divisor_tdata(cycle),
              .s_axis_divisor_tvalid(1'b1),
              .aclk(CLK),
              .m_axis_dout_tdata({_unused_quo, rem}),
              .m_axis_dout_tvalid()
          );

always_ff @(posedge CLK) begin
    divined <= SYS_TIME;
    freq_div <= FREQ_DIV;
    cycle <= CYCLE + 1;
end

endmodule
