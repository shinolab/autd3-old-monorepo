/*
 * File: main.sv
 * Project: new
 * Created Date: 18/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module main #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK,
    input var CLK_L,
    input var CAT_SYNC0,
    cpu_bus_if.ctl_port CPU_BUS_CTL,
    cpu_bus_if.normal_port CPU_BUS_NORMAL,
    cpu_bus_if.stm_port CPU_BUS_STM,
    cpu_bus_if.mod_port CPU_BUS_MOD,
    input var THERMO,
    output var FORCE_FAN,
    output var PWM_OUT[DEPTH]
);

  `include "params.vh"

  bit [63:0] sys_time;

  bit [63:0] ecat_sync_time;
  bit sync_set;

  bit trig_40khz;

  bit legacy_mode;

  bit [WIDTH-1:0] duty;
  bit [WIDTH-1:0] phase;
  bit [WIDTH-1:0] cycle[DEPTH];
  bit dout_valid;

  bit op_mode;
  bit [WIDTH-1:0] duty_normal;
  bit [WIDTH-1:0] phase_normal;
  bit dout_valid_normal;

  bit stm_gain_mode;
  bit [15:0] cycle_stm;
  bit [31:0] freq_div_stm;
  bit [31:0] sound_speed;
  bit [15:0] stm_start_idx;
  bit [15:0] stm_finish_idx;
  bit use_stm_start_idx, use_stm_finish_idx;

  bit [15:0] cycle_m;
  bit [31:0] freq_div_m;
  bit [WIDTH-1:0] duty_m;
  bit [WIDTH-1:0] phase_m;
  bit [15:0] delay_m[DEPTH];
  bit dout_valid_m;

  bit [WIDTH-1:0] step_s;
  bit [WIDTH-1:0] duty_s;
  bit [WIDTH-1:0] phase_s;
  bit dout_valid_s;

  synchronizer synchronizer (
      .CLK(CLK),
      .ECAT_SYNC_TIME(ecat_sync_time),
      .SET(sync_set),
      .ECAT_SYNC(CAT_SYNC0),
      .SYS_TIME(sys_time),
      .SYNC()
  );

  controller #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) controller (
      .CLK(CLK_L),
      .THERMO(THERMO),
      .FORCE_FAN(FORCE_FAN),
      .CPU_BUS(CPU_BUS_CTL),
      .ECAT_SYNC_TIME(ecat_sync_time),
      .SYNC_SET(sync_set),
      .OP_MODE(op_mode),
      .STM_GAIN_MODE(stm_gain_mode),
      .CYCLE_M(cycle_m),
      .FREQ_DIV_M(freq_div_m),
      .DELAY_M(delay_m),
      .STEP_S(step_s),
      .CYCLE_STM(cycle_stm),
      .FREQ_DIV_STM(freq_div_stm),
      .SOUND_SPEED(sound_speed),
      .STM_START_IDX(stm_start_idx),
      .USE_STM_START_IDX(use_stm_start_idx),
      .STM_FINISH_IDX(stm_finish_idx),
      .USE_STM_FINISH_IDX(use_stm_finish_idx),
      .CYCLE(cycle),
      .LEGACY_MODE(legacy_mode)
  );

  timer_40kHz timer_40kHz (
      .CLK_L(CLK_L),
      .SYS_TIME(sys_time),
      .TRIG_40KHZ(trig_40khz)
  );

  normal_operator #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) normal_operator (
      .CLK_L(CLK_L),
      .CPU_BUS(CPU_BUS_NORMAL),
      .LEGACY_MODE(legacy_mode),
      .TRIG_40KHZ(trig_40khz),
      .DUTY(duty_normal),
      .PHASE(phase_normal),
      .DOUT_VALID(dout_valid_normal)
  );

  if (ENABLE_STM == "TRUE") begin : gen_stm
    bit duty_stm;
    bit phase_stm;
    bit dout_valid_stm;
    bit [15:0] stm_idx;

    stm_operator #(
        .WIDTH(WIDTH),
        .DEPTH(DEPTH)
    ) stm_operator (
        .CLK_L(CLK_L),
        .SYS_TIME(sys_time),
        .TRIG_40KHZ(trig_40khz),
        .LEGACY_MODE(legacy_mode),
        .CYCLE(cycle),
        .CYCLE_STM(cycle_stm),
        .FREQ_DIV_STM(freq_div_stm),
        .SOUND_SPEED(sound_speed),
        .STM_GAIN_MODE(stm_gain_mode),
        .CPU_BUS(CPU_BUS_STM),
        .DUTY(duty_stm),
        .PHASE(phase_stm),
        .DOUT_VALID(dout_valid_stm),
        .IDX(stm_idx)
    );

    mux mux (
        .CLK_L(CLK_L),
        .OP_MODE(op_mode),
        .DUTY_NORMAL(duty_normal),
        .PHASE_NORMAL(phase_normal),
        .DOUT_VALID_NORMAL(dout_valid_normal),
        .DUTY_STM(duty_stm),
        .PHASE_STM(phase_stm),
        .DOUT_VALID_STM(dout_valid_stm),
        .STM_IDX(stm_idx),
        .USE_STM_START_IDX(use_stm_start_idx),
        .USE_STM_FINISH_IDX(use_stm_finish_idx),
        .STM_START_IDX(stm_start_idx),
        .STM_FINISH_IDX(stm_finish_idx),
        .DUTY(duty),
        .PHASE(phase),
        .DOUT_VALID(dout_valid)
    );

  end else begin : gen_stm_false
    assign duty = duty_normal;
    assign phase = phase_normal;
    assign dout_valid = dout_valid_normal;
  end

  if (ENABLE_MODULATOR == "TRUE") begin : gen_modulator
    modulator #(
        .WIDTH(WIDTH),
        .DEPTH(DEPTH)
    ) modulator (
        .CLK(CLK_L),
        .SYS_TIME(sys_time),
        .CYCLE_M(cycle_m),
        .FREQ_DIV_M(freq_div_m),
        .CPU_BUS(CPU_BUS_MOD),
        .DIN_VALID(dout_valid),
        .DUTY_IN(duty),
        .PHASE_IN(phase),
        .DELAY_M(delay_m),
        .DUTY_OUT(duty_m),
        .PHASE_OUT(phase_m),
        .DOUT_VALID(dout_valid_m),
        .IDX()
    );
  end else begin : gen_modulator_false
    assign duty_m = duty;
    assign phase_m = phase;
    assign dout_valid_m = dout_valid;
  end

  if (ENABLE_SILENCER == "TRUE") begin : gen_silencer
    silencer #(
        .WIDTH(WIDTH),
        .DEPTH(DEPTH)
    ) silencer (
        .CLK(CLK_L),
        .DIN_VALID(dout_valid_m),
        .STEP(step_s),
        .CYCLE(cycle),
        .DUTY(duty_m),
        .PHASE(phase_m),
        .DUTY_S(duty_s),
        .PHASE_S(phase_s),
        .DOUT_VALID(dout_valid_s)
    );
  end else begin : gen_silencer_false
    assign duty_s = duty_m;
    assign phase_s = phase_m;
    assign dout_valid_s = dout_valid_m;
  end

  pwm #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) pwm (
      .CLK(CLK),
      .CLK_L(CLK_L),
      .SYS_TIME(sys_time),
      .DIN_VALID(dout_valid_s),
      .CYCLE(cycle),
      .DUTY(duty_s),
      .PHASE(phase_s),
      .PWM_OUT(PWM_OUT),
      .TIME_CNT(),
      .DOUT_VALID()
  );

endmodule
