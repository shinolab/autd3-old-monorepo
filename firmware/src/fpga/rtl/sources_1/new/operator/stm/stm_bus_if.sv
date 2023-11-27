/*
 * File: stm_bus_if.sv
 * Project: stm
 * Created Date: 13/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
interface stm_bus_if ();

  logic [15:0] ADDR;
  logic [127:0] DATA_OUT;
  logic STM_GAIN_MODE;

  logic [15:0] GAIN_ADDR;
  logic [15:0] FOCUS_ADDR;

  assign ADDR = STM_GAIN_MODE ? GAIN_ADDR : FOCUS_ADDR;

  modport memory_port(input ADDR, output DATA_OUT);
  modport gain_port(output GAIN_ADDR, input DATA_OUT);
  modport focus_port(output FOCUS_ADDR, input DATA_OUT);

endinterface
