/*
 * File: main.sv
 * Project: new
 * Created Date: 18/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module main #(
    parameter int DEPTH = 249
) (
    input var CLK,
    input var CAT_SYNC0,
    cpu_bus_if.ctl_port CPU_BUS_CTL,
    cpu_bus_if.normal_port CPU_BUS_NORMAL,
    cpu_bus_if.stm_port CPU_BUS_STM,
    cpu_bus_if.mod_port CPU_BUS_MOD,
    input var THERMO,
    output var FORCE_FAN,
    output var PWM_OUT[DEPTH],
    output var GPIO_OUT[2]
);

  `include "params.vh"

  logic [63:0] sys_time;
  logic skip_one_assert;

  logic [8:0] time_cnt;
  logic update;

  logic [63:0] ecat_sync_time;
  logic sync_set;

  logic [7:0] intensity;
  logic [7:0] phase;
  logic dout_valid;

  logic op_mode;
  logic [7:0] intensity_normal;
  logic [7:0] phase_normal;
  logic dout_valid_normal;

  logic [7:0] intensity_stm;
  logic [7:0] phase_stm;
  logic dout_valid_stm;
  logic [15:0] stm_idx;
  logic stm_gain_mode;
  logic [15:0] cycle_stm;
  logic [31:0] freq_div_stm;
  logic [31:0] sound_speed;
  logic [15:0] stm_start_idx;
  logic [15:0] stm_finish_idx;
  logic use_stm_start_idx, use_stm_finish_idx;

  logic [15:0] cycle_m;
  logic [31:0] freq_div_m;
  logic [15:0] intensity_m;
  logic [7:0] phase_m;
  logic [15:0] delay_m[DEPTH];
  logic dout_valid_m;

  logic [15:0] step_intensity_s;
  logic [15:0] step_phase_s;
  logic [15:0] intensity_s;
  logic [7:0] phase_s;
  logic dout_valid_s;

  logic [8:0] pulse_width_e;
  logic [7:0] phase_e;
  logic dout_valid_e;

  logic [7:0] debug_output_idx;

  synchronizer synchronizer (
      .CLK(CLK),
      .ECAT_SYNC_TIME(ecat_sync_time),
      .SET(sync_set),
      .ECAT_SYNC(CAT_SYNC0),
      .SYS_TIME(sys_time),
      .SYNC(),
      .SKIP_ONE_ASSERT(skip_one_assert)
  );

  controller #(
      .DEPTH(DEPTH)
  ) controller (
      .CLK(CLK),
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
      .STEP_INTENSITY_S(step_intensity_s),
      .STEP_PHASE_S(step_phase_s),
      .CYCLE_STM(cycle_stm),
      .FREQ_DIV_STM(freq_div_stm),
      .SOUND_SPEED(sound_speed),
      .STM_START_IDX(stm_start_idx),
      .USE_STM_START_IDX(use_stm_start_idx),
      .STM_FINISH_IDX(stm_finish_idx),
      .USE_STM_FINISH_IDX(use_stm_finish_idx),
      .DEBUG_OUTPUT_IDX(debug_output_idx)
  );

  time_cnt_generator #(
      .DEPTH(DEPTH)
  ) time_cnt_generator (
      .CLK(CLK),
      .SYS_TIME(sys_time),
      .SKIP_ONE_ASSERT(skip_one_assert),
      .TIME_CNT(time_cnt),
      .UPDATE(update)
  );

  normal_operator #(
      .DEPTH(DEPTH)
  ) normal_operator (
      .CLK(CLK),
      .CPU_BUS(CPU_BUS_NORMAL),
      .UPDATE(update),
      .INTENSITY(intensity_normal),
      .PHASE(phase_normal),
      .DOUT_VALID(dout_valid_normal)
  );

  stm_operator #(
      .DEPTH(DEPTH)
  ) stm_operator (
      .CLK(CLK),
      .SYS_TIME(sys_time),
      .UPDATE(update),
      .CYCLE_STM(cycle_stm),
      .FREQ_DIV_STM(freq_div_stm),
      .SOUND_SPEED(sound_speed),
      .STM_GAIN_MODE(stm_gain_mode),
      .CPU_BUS(CPU_BUS_STM),
      .INTENSITY(intensity_stm),
      .PHASE(phase_stm),
      .DOUT_VALID(dout_valid_stm),
      .IDX(stm_idx)
  );

  mux mux (
      .CLK(CLK),
      .OP_MODE(op_mode),
      .INTENSITY_NORMAL(intensity_normal),
      .PHASE_NORMAL(phase_normal),
      .DOUT_VALID_NORMAL(dout_valid_normal),
      .INTENSITY_STM(intensity_stm),
      .PHASE_STM(phase_stm),
      .DOUT_VALID_STM(dout_valid_stm),
      .STM_IDX(stm_idx),
      .USE_STM_START_IDX(use_stm_start_idx),
      .USE_STM_FINISH_IDX(use_stm_finish_idx),
      .STM_START_IDX(stm_start_idx),
      .STM_FINISH_IDX(stm_finish_idx),
      .INTENSITY(intensity),
      .PHASE(phase),
      .DOUT_VALID(dout_valid)
  );

  modulator #(
      .DEPTH(DEPTH)
  ) modulator (
      .CLK(CLK),
      .SYS_TIME(sys_time),
      .CYCLE_M(cycle_m),
      .FREQ_DIV_M(freq_div_m),
      .CPU_BUS(CPU_BUS_MOD),
      .DIN_VALID(dout_valid),
      .INTENSITY_IN(intensity),
      .PHASE_IN(phase),
      .DELAY_M(delay_m),
      .INTENSITY_OUT(intensity_m),
      .PHASE_OUT(phase_m),
      .DOUT_VALID(dout_valid_m),
      .IDX()
  );

  silencer #(
      .DEPTH(DEPTH)
  ) silencer (
      .CLK(CLK),
      .DIN_VALID(dout_valid_m),
      .STEP_INTENSITY(step_intensity_s),
      .STEP_PHASE(step_phase_s),
      .INTENSITY_IN(intensity_m),
      .PHASE_IN(phase_m),
      .INTENSITY_OUT(intensity_s),
      .PHASE_OUT(phase_s),
      .DOUT_VALID(dout_valid_s)
  );

  pulse_width_encoder #(
      .DEPTH(DEPTH)
  ) pulse_width_encoder (
      .CLK(CLK),
      .DIN_VALID(dout_valid_s),
      .INTENSITY_IN(intensity_s),
      .PHASE_IN(phase_s),
      .PULSE_WIDTH_OUT(pulse_width_e),
      .PHASE_OUT(phase_e),
      .DOUT_VALID(dout_valid_e)
  );

  pwm #(
      .DEPTH(DEPTH)
  ) pwm (
      .CLK(CLK),
      .TIME_CNT(time_cnt),
      .UPDATE(update),
      .DIN_VALID(dout_valid_e),
      .PULSE_WIDTH(pulse_width_e),
      .PHASE(phase_e),
      .PWM_OUT(PWM_OUT)
  );

  logic gpio_out_0;
  logic gpio_out_1;

  assign GPIO_OUT[0] = gpio_out_0;
  assign GPIO_OUT[1] = gpio_out_1;

  always_ff @(posedge CLK) begin
    gpio_out_0 <= time_cnt < 9'd256;
    gpio_out_1 <= debug_output_idx == 8'hFF ? 1'b0 : PWM_OUT[debug_output_idx];
  end

endmodule
