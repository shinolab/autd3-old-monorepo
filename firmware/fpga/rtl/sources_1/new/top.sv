/*
 * File: top.sv
 * Project: new
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module top (
    input var [16:1] CPU_ADDR,
    inout tri [15:0] CPU_DATA,
    input var CPU_CKIO,
    input var CPU_CS1_N,
    input var RESET_N,
    input var CPU_WE0_N,
    input var CPU_RD_N,
    input var CPU_RDWR,
    input var MRCC_25P6M,
    input var CAT_SYNC0,
    output var FORCE_FAN,
    input var THERMO,
    output var [252:1] XDCR_OUT
);

  `include "cvt_uid.vh"
  `include "params.vh"

  bit clk, clk_l;
  bit reset;

  bit PWM_OUT[DEPTH];

  assign reset = ~RESET_N;

  for (genvar i = 0; i < DEPTH; i++) begin : gen_output
    assign XDCR_OUT[cvt_uid(i)+1] = PWM_OUT[i];
  end

  cpu_bus_if cpu_bus ();
  assign cpu_bus.BUS_CLK = CPU_CKIO;
  assign cpu_bus.EN = ~CPU_CS1_N;
  assign cpu_bus.RD = ~CPU_RD_N;
  assign cpu_bus.RDWR = CPU_RDWR;
  assign cpu_bus.WE = ~CPU_WE0_N;
  assign cpu_bus.BRAM_SELECT = CPU_ADDR[16:15];
  assign cpu_bus.BRAM_ADDR = CPU_ADDR[14:1];
  assign cpu_bus.CPU_DATA = CPU_DATA;

  ultrasound_cnt_clk_gen ultrasound_cnt_clk_gen (
      .clk_in1(MRCC_25P6M),
      .reset(reset),
      .clk_out1(clk),
      .clk_out2(clk_l),
      .locked()
  );

  main #(
      .WIDTH(13),
      .DEPTH(249)
  ) main (
      .CLK(clk),
      .CLK_L(clk_l),
      .CAT_SYNC0(CAT_SYNC0),
      .CPU_BUS_CTL(cpu_bus.ctl_port),
      .CPU_BUS_NORMAL(cpu_bus.normal_port),
      .CPU_BUS_STM(cpu_bus.stm_port),
      .CPU_BUS_MOD(cpu_bus.mod_port),
      .THERMO(THERMO),
      .FORCE_FAN(FORCE_FAN),
      .PWM_OUT(PWM_OUT)
  );

endmodule
