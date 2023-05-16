/*
 * File: modulator.sv
 * Project: modulation
 * Created Date: 24/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module modulator #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK,
    input var [63:0] SYS_TIME,
    input var [15:0] CYCLE_M,
    input var [31:0] FREQ_DIV_M,
    cpu_bus_if.mod_port CPU_BUS,
    input var DIN_VALID,
    input var [WIDTH-1:0] DUTY_IN,
    input var [WIDTH-1:0] PHASE_IN,
    input var [15:0] DELAY_M[DEPTH],
    output var [WIDTH-1:0] DUTY_OUT,
    output var [WIDTH-1:0] PHASE_OUT,
    output var DOUT_VALID,
    output var [15:0] IDX
);

  modulation_bus_if m_bus_if ();

  modulation_memory modulation_memory (
      .CLK(CLK),
      .CPU_BUS(CPU_BUS),
      .M_BUS(m_bus_if.memory_port)
  );

  modulation_sampler #(
      .DEPTH(DEPTH)
  ) modulation_sampler (
      .CLK(CLK),
      .SYS_TIME(SYS_TIME),
      .CYCLE_M(CYCLE_M),
      .FREQ_DIV_M(FREQ_DIV_M),
      .IDX(IDX)
  );

  modulation_multiplier #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) modulation_multiplier (
      .CLK(CLK),
      .CYCLE_M(CYCLE_M),
      .DIN_VALID(DIN_VALID),
      .IDX(IDX),
      .M_BUS(m_bus_if.sampler_port),
      .DELAY_M(DELAY_M),
      .DUTY_IN(DUTY_IN),
      .PHASE_IN(PHASE_IN),
      .DUTY_OUT(DUTY_OUT),
      .PHASE_OUT(PHASE_OUT),
      .DOUT_VALID(DOUT_VALID)
  );

endmodule
