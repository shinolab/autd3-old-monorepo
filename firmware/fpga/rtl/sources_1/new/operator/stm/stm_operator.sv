/*
 * File: stm_operator.sv
 * Project: stm
 * Created Date: 13/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module stm_operator #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK_L,
    input var [63:0] SYS_TIME,
    input var TRIG_40KHZ,
    input var LEGACY_MODE,
    input var [WIDTH-1:0] CYCLE[DEPTH],
    input var [15:0] CYCLE_STM,
    input var [31:0] FREQ_DIV_STM,
    input var [31:0] SOUND_SPEED,
    input var STM_GAIN_MODE,
    cpu_bus_if.stm_port CPU_BUS,
    output var [WIDTH-1:0] DUTY,
    output var [WIDTH-1:0] PHASE,
    output var DOUT_VALID,
    output var [15:0] IDX
);

  stm_bus_if stm_bus_if ();
  assign stm_bus_if.STM_GAIN_MODE = STM_GAIN_MODE;

  bit [WIDTH-1:0] duty_gain;
  bit [WIDTH-1:0] phase_gain;
  bit [WIDTH-1:0] duty_focus;
  bit [WIDTH-1:0] phase_focus;
  bit [15:0] idx;
  bit start_gain, done_gain;
  bit start_focus, done_focus;

  assign DUTY = STM_GAIN_MODE ? duty_gain : duty_focus;
  assign PHASE = STM_GAIN_MODE ? phase_gain : phase_focus;
  assign DOUT_VALID = STM_GAIN_MODE ? dout_valid_gain : dout_valid_focus;
  assign IDX = idx;

  stm_memory stm_memory (
      .CLK_L  (CLK_L),
      .CPU_BUS(CPU_BUS),
      .STM_BUS(stm_bus_if.memory_port)
  );

  stm_sampler stm_sampler (
      .CLK_L(CLK_L),
      .SYS_TIME(SYS_TIME),
      .CYCLE_STM(CYCLE_STM),
      .FREQ_DIV_STM(FREQ_DIV_STM),
      .IDX(idx)
  );

  stm_gain_operator #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) stm_gain_operator (
      .CLK_L(CLK_L),
      .TRIG_40KHZ(TRIG_40KHZ),
      .IDX(idx),
      .STM_BUS(stm_bus_if.gain_port),
      .LEGACY_MODE(LEGACY_MODE),
      .CYCLE(CYCLE),
      .DUTY(duty_gain),
      .PHASE(phase_gain),
      .DOUT_VALID(dout_valid_gain)
  );

  stm_focus_operator #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) stm_focus_operator (
      .CLK_L(CLK_L),
      .TRIG_40KHZ(TRIG_40KHZ),
      .IDX(idx),
      .STM_BUS(stm_bus_if.focus_port),
      .SOUND_SPEED(SOUND_SPEED),
      .CYCLE(CYCLE),
      .DUTY(duty_focus),
      .PHASE(phase_focus),
      .DOUT_VALID(dout_valid_focus)
  );

endmodule
