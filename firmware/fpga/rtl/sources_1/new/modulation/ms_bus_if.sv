/*
 * File: ms_bus_if.sv
 * Project: modulation
 * Created Date: 24/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

`timescale 1ns / 1ps
interface ms_bus_if ();

  bit [15:0] ADDR;
  bit [ 7:0] M;

  modport memory_port(input ADDR, output M);
  modport sampler_port(output ADDR, input M);

endinterface
