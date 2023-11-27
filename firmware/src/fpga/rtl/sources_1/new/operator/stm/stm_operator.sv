/*
 * File: stm_operator.sv
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
module stm_operator #(
    parameter int DEPTH = 249
) (
    input var CLK,
    input var [63:0] SYS_TIME,
    input var UPDATE,
    input var [15:0] CYCLE_STM,
    input var [31:0] FREQ_DIV_STM,
    input var [31:0] SOUND_SPEED,
    input var STM_GAIN_MODE,
    cpu_bus_if.stm_port CPU_BUS,
    output var [7:0] INTENSITY,
    output var [7:0] PHASE,
    output var DOUT_VALID,
    output var [15:0] IDX
);

  stm_bus_if stm_bus_if ();
  assign stm_bus_if.STM_GAIN_MODE = STM_GAIN_MODE;

  logic [ 7:0] intensity_gain;
  logic [ 7:0] phase_gain;
  logic [ 7:0] intensity_focus;
  logic [ 7:0] phase_focus;
  logic [15:0] idx;
  logic dout_valid_gain, dout_valid_focus;

  assign INTENSITY = STM_GAIN_MODE ? intensity_gain : intensity_focus;
  assign PHASE = STM_GAIN_MODE ? phase_gain : phase_focus;
  assign DOUT_VALID = STM_GAIN_MODE ? dout_valid_gain : dout_valid_focus;
  assign IDX = idx;

  stm_memory stm_memory (
      .CLK(CLK),
      .CPU_BUS(CPU_BUS),
      .STM_BUS(stm_bus_if.memory_port)
  );

  stm_sampler stm_sampler (
      .CLK(CLK),
      .SYS_TIME(SYS_TIME),
      .CYCLE_STM(CYCLE_STM),
      .FREQ_DIV_STM(FREQ_DIV_STM),
      .IDX(idx)
  );

  stm_gain_operator #(
      .DEPTH(DEPTH)
  ) stm_gain_operator (
      .CLK(CLK),
      .UPDATE(UPDATE),
      .IDX(idx),
      .STM_BUS(stm_bus_if.gain_port),
      .INTENSITY(intensity_gain),
      .PHASE(phase_gain),
      .DOUT_VALID(dout_valid_gain)
  );

  stm_focus_operator #(
      .DEPTH(DEPTH)
  ) stm_focus_operator (
      .CLK(CLK),
      .UPDATE(UPDATE),
      .IDX(idx),
      .STM_BUS(stm_bus_if.focus_port),
      .SOUND_SPEED(SOUND_SPEED),
      .INTENSITY(intensity_focus),
      .PHASE(phase_focus),
      .DOUT_VALID(dout_valid_focus)
  );

endmodule
