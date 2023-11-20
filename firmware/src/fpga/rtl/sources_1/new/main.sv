/*
 * File: main.sv
 * Project: new
 * Created Date: 18/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module main #(
    parameter int WIDTH = 9,
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
    output var GPIO_OUT
);

  `include "params.vh"

  logic [63:0] sys_time;
  logic skip_one_assert;

  logic [WIDTH-1:0] time_cnt;
  logic update;

  logic [63:0] ecat_sync_time;
  logic sync_set;

  logic [WIDTH-1:0] duty;
  logic [WIDTH-1:0] phase;
  logic dout_valid;

  logic op_mode;
  logic [WIDTH-1:0] duty_normal;
  logic [WIDTH-1:0] phase_normal;
  logic dout_valid_normal;

  logic stm_gain_mode;
  logic [15:0] cycle_stm;
  logic [31:0] freq_div_stm;
  logic [31:0] sound_speed;
  logic [15:0] stm_start_idx;
  logic [15:0] stm_finish_idx;
  logic use_stm_start_idx, use_stm_finish_idx;

  logic [15:0] cycle_m;
  logic [31:0] freq_div_m;
  logic [WIDTH-1:0] duty_m;
  logic [WIDTH-1:0] phase_m;
  logic [15:0] delay_m[DEPTH];
  logic dout_valid_m;

  logic [WIDTH-1:0] step_s;
  logic [WIDTH-1:0] duty_s;
  logic [WIDTH-1:0] phase_s;
  logic dout_valid_s;

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
      .WIDTH(WIDTH),
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
      .STEP_S(step_s),
      .CYCLE_STM(cycle_stm),
      .FREQ_DIV_STM(freq_div_stm),
      .SOUND_SPEED(sound_speed),
      .STM_START_IDX(stm_start_idx),
      .USE_STM_START_IDX(use_stm_start_idx),
      .STM_FINISH_IDX(stm_finish_idx),
      .USE_STM_FINISH_IDX(use_stm_finish_idx)
  );

  time_cnt_generator #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) time_cnt_generator (
      .CLK(CLK),
      .SYS_TIME(sys_time),
      .SKIP_ONE_ASSERT(skip_one_assert),
      .TIME_CNT(time_cnt),
      .UPDATE(update)
  );

  normal_operator #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) normal_operator (
      .CLK(CLK),
      .CPU_BUS(CPU_BUS_NORMAL),
      .UPDATE(update),
      .DUTY(duty_normal),
      .PHASE(phase_normal),
      .DOUT_VALID(dout_valid_normal)
  );

  if (ENABLE_STM == "TRUE") begin : gen_stm
    logic [WIDTH-1:0] duty_stm;
    logic [WIDTH-1:0] phase_stm;
    logic dout_valid_stm;
    logic [15:0] stm_idx;

    stm_operator #(
        .WIDTH(WIDTH),
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
        .DUTY(duty_stm),
        .PHASE(phase_stm),
        .DOUT_VALID(dout_valid_stm),
        .IDX(stm_idx)
    );

    mux #(
        .WIDTH(WIDTH)
    ) mux (
        .CLK(CLK),
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
        .CLK(CLK),
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
        .CLK(CLK),
        .DIN_VALID(dout_valid_m),
        .STEP(step_s),
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
      .TIME_CNT(time_cnt),
      .UPDATE(update),
      .DIN_VALID(dout_valid_s),
      .DUTY(duty_s),
      .PHASE(phase_s),
      .PWM_OUT(PWM_OUT)
  );

  logic gpio_out;

  assign GPIO_OUT = gpio_out;

  always_ff @(posedge CLK) begin
    gpio_out <= time_cnt < 9'd256;
  end

endmodule
