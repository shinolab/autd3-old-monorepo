/*
 * File: top.sv
 * Project: new
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/05/2023
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

  localparam int WIDTH = 13;
  localparam int DEPTH = 249;

  bit clk, clk_l;
  bit reset;

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

  synchronizer synchronizer (
      .CLK(clk),
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
      .CLK(clk_l),
      .THERMO(THERMO),
      .FORCE_FAN(FORCE_FAN),
      .CPU_BUS(cpu_bus.ctl_port),
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
      .CLK_L(clk_l),
      .SYS_TIME(sys_time),
      .TRIG_40KHZ(trig_40khz)
  );

  normal_operator #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) normal_operator (
      .CLK_L(clk_l),
      .CPU_BUS(cpu_bus.normal_port),
      .LEGACY_MODE(legacy_mode),
      .TRIG_40KHZ(trig_40khz),
      .DUTY(duty_normal),
      .PHASE(phase_normal),
      .DOUT_VALID(dout_valid_normal)
  );

  if (ENABLE_STM == "TRUE") begin : gen_stm
    bit [WIDTH-1:0] duty_stm;
    bit [WIDTH-1:0] phase_stm;
    bit dout_valid_stm;

    bit [WIDTH-1:0] duty_stm_buf;
    bit [WIDTH-1:0] phase_stm_buf;
    bit dout_valid_stm_buf;
    bit [WIDTH-1:0] duty_normal_buf;
    bit [WIDTH-1:0] phase_normal_buf;
    bit dout_valid_normal_buf;

    always_ff @(posedge clk_l) begin
      duty_stm_buf <= duty_stm;
      phase_stm_buf <= phase_stm;
      dout_valid_stm_buf <= dout_valid_stm;
      duty_normal_buf <= duty_normal;
      phase_normal_buf <= phase_normal;
      dout_valid_normal_buf <= dout_valid_normal;
    end

    bit [15:0] stm_idx;

    typedef enum bit [1:0] {
      NORMAL,
      WAIT_START_STM,
      STM,
      WAIT_FINISH_STM
    } stm_state_t;

    stm_state_t stm_state = NORMAL;

    bit output_stm;
    assign output_stm = (stm_state == STM) | (stm_state == WAIT_FINISH_STM);
    assign duty = output_stm ? duty_stm_buf : duty_normal_buf;
    assign phase = output_stm ? phase_stm_buf : phase_normal_buf;
    assign dout_valid = output_stm ? dout_valid_stm_buf : dout_valid_normal_buf;

    stm_operator #(
        .WIDTH(WIDTH),
        .DEPTH(DEPTH)
    ) stm_operator (
        .CLK_L(clk_l),
        .SYS_TIME(sys_time),
        .TRIG_40KHZ(trig_40khz),
        .LEGACY_MODE(legacy_mode),
        .CYCLE(cycle),
        .CYCLE_STM(cycle_stm),
        .FREQ_DIV_STM(freq_div_stm),
        .SOUND_SPEED(sound_speed),
        .STM_GAIN_MODE(stm_gain_mode),
        .CPU_BUS(cpu_bus.stm_port),
        .DUTY(duty_stm),
        .PHASE(phase_stm),
        .DOUT_VALID(dout_valid_stm),
        .IDX(stm_idx)
    );

    always_ff @(posedge clk_l) begin
      case (stm_state)
        NORMAL: begin
          if (op_mode) begin
            stm_state <= use_stm_start_idx ? WAIT_START_STM : STM;
          end
        end
        WAIT_START_STM: begin
          if (op_mode) begin
            stm_state <= dout_valid_stm & (stm_idx == stm_start_idx) ? STM : stm_state;
          end else begin
            stm_state <= NORMAL;
          end
        end
        STM: begin
          if (~op_mode) begin
            stm_state <= use_stm_finish_idx ? WAIT_FINISH_STM : NORMAL;
          end
        end
        WAIT_FINISH_STM: begin
          if (~op_mode) begin
            stm_state <= dout_valid_stm & (stm_idx == stm_finish_idx) ? NORMAL : stm_state;
          end else begin
            stm_state <= STM;
          end
        end
        default: begin
        end
      endcase
    end
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
        .CLK(clk_l),
        .SYS_TIME(sys_time),
        .CYCLE_M(cycle_m),
        .FREQ_DIV_M(freq_div_m),
        .CPU_BUS(cpu_bus.mod_port),
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
        .CLK(clk_l),
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
      .CLK(clk),
      .CLK_L(clk_l),
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
