/*
 * File: sim_helper_clk.sv
 * Project: helper
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */


`timescale 1ns / 1ps
module sim_helper_clk (
    output var CLK_163P84M,
    output var CLK_20P48M,
    output var LOCKED,
    output var [63:0] SYS_TIME
);

  bit MRCC_25P6M;

  bit clk_163P84M, clk_20P48M;
  bit locked;
  bit reset;
  bit [63:0] sys_time;

  ultrasound_cnt_clk_gen ultrasound_cnt_clk_gen (
      .clk_in1(MRCC_25P6M),
      .reset(reset),
      .locked(locked),
      .clk_out1(clk_163P84M),
      .clk_out2(clk_20P48M)
  );

  assign CLK_163P84M = clk_163P84M;
  assign CLK_20P48M = clk_20P48M;
  assign LOCKED = locked;
  assign SYS_TIME = sys_time;

  initial begin
    MRCC_25P6M = 0;
    reset = 1;
    #1000;
    reset = 0;
    sys_time = 1;
  end

  // main clock 25.6MHz
  always begin
    #19.531 MRCC_25P6M = !MRCC_25P6M;
    #19.531 MRCC_25P6M = !MRCC_25P6M;
    #19.531 MRCC_25P6M = !MRCC_25P6M;
    #19.532 MRCC_25P6M = !MRCC_25P6M;
  end

  always @(posedge CLK_163P84M) begin
    sys_time = sys_time + 1;
  end

endmodule
