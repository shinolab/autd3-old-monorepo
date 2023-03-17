/*
 * File: top.sv
 * Project: new
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
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
    output var [252:1] XDCR_OUT,
    output var [3:0] GPIO_OUT,
    input var [3:0] GPIO_IN
);

  `include "cvt_uid.vh"
  `include "params.vh"

  localparam int WIDTH = 13;
  localparam int TRANS_NUM = 249;

  bit clk, clk_l;
  bit reset;

  bit [63:0] sys_time;

  bit [63:0] ecat_sync_time;
  bit sync_set;

  bit legacy_mode;

  bit op_mode;
  bit [WIDTH-1:0] duty_normal[0:TRANS_NUM-1];
  bit [WIDTH-1:0] phase_normal[0:TRANS_NUM-1];
  bit [WIDTH-1:0] duty_stm[0:TRANS_NUM-1];
  bit [WIDTH-1:0] phase_stm[0:TRANS_NUM-1];
  bit [WIDTH-1:0] duty[0:TRANS_NUM-1];
  bit [WIDTH-1:0] phase[0:TRANS_NUM-1];

  bit [WIDTH-1:0] cycle[0:TRANS_NUM-1];

  bit [15:0] cycle_m;
  bit [31:0] freq_div_m;
  bit [WIDTH-1:0] duty_m[0:TRANS_NUM-1];
  bit [WIDTH-1:0] phase_m[0:TRANS_NUM-1];
  bit [15:0] delay_m[0:TRANS_NUM-1];

  bit [15:0] cycle_s;
  bit [WIDTH-1:0] step_s;
  bit [WIDTH-1:0] duty_s[0:TRANS_NUM-1];
  bit [WIDTH-1:0] phase_s[0:TRANS_NUM-1];

  bit stm_gain_mode;
  bit [15:0] cycle_stm;
  bit [31:0] freq_div_stm;
  bit [31:0] sound_speed;
  bit [15:0] stm_start_idx;
  bit [15:0] stm_finish_idx;
  bit use_stm_start_idx;

  bit PWM_OUT[0:TRANS_NUM-1];

  bit stm_done, mod_done, silencer_done;
  bit gpo_0 = 0;
  bit gpo_1 = 0;
  bit gpo_2 = 0;
  bit gpo_3 = 0;

  assign GPIO_OUT = {gpo_3, gpo_2, gpo_1, gpo_0};

  bit sync_dbg;
  bit [2:0] sync_tri_dbg = 0;
  always_ff @(posedge clk_l) begin
    sync_tri_dbg <= {sync_tri_dbg[1:0], CAT_SYNC0};
  end
  assign sync_dbg = sync_tri_dbg == 3'b011;

  always_ff @(posedge clk_l) begin
    gpo_0 <= stm_done ? ~gpo_0 : gpo_0;
    gpo_1 <= mod_done ? ~gpo_1 : gpo_1;
    gpo_2 <= silencer_done ? ~gpo_2 : gpo_2;
    gpo_3 <= sync_dbg ? ~gpo_3 : gpo_3;
  end

  assign reset = ~RESET_N;

  for (genvar i = 0; i < TRANS_NUM; i++) begin
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

  controller #(
      .WIDTH(WIDTH),
      .DEPTH(TRANS_NUM)
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
      .CYCLE_S(cycle_s),
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

  normal_operator #(
      .WIDTH(WIDTH),
      .DEPTH(TRANS_NUM)
  ) normal_operator (
      .CLK(clk_l),
      .CPU_BUS(cpu_bus.normal_port),
      .CYCLE(cycle),
      .LEGACY_MODE(legacy_mode),
      .DUTY(duty_normal),
      .PHASE(phase_normal)
  );

  if (ENABLE_STM == "TRUE") begin
    bit [15:0] stm_idx;
    enum bit [1:0] {
      NORMAL,
      WAIT_START_STM,
      STM,
      WAIT_FINISH_STM
    } stm_state = NORMAL;

    for (genvar i = 0; i < TRANS_NUM; i++) begin
      assign duty[i]  = (stm_state == STM) | (stm_state == WAIT_FINISH_STM) ? duty_stm[i] : duty_normal[i];
      assign phase[i] = (stm_state == STM) | (stm_state == WAIT_FINISH_STM) ? phase_stm[i] : phase_normal[i];
    end
    stm_operator #(
        .WIDTH(WIDTH),
        .DEPTH(TRANS_NUM)
    ) stm_operator (
        .CLK(clk_l),
        .SYS_TIME(sys_time),
        .LEGACY_MODE(legacy_mode),
        .ULTRASOUND_CYCLE(cycle),
        .CYCLE(cycle_stm),
        .FREQ_DIV(freq_div_stm),
        .SOUND_SPEED(sound_speed),
        .STM_GAIN_MODE(stm_gain_mode),
        .CPU_BUS(cpu_bus.stm_port),
        .DUTY(duty_stm),
        .PHASE(phase_stm),
        .START(),
        .DONE(stm_done),
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
            stm_state <= stm_done & (stm_idx == stm_start_idx) ? STM : stm_state;
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
            stm_state <= stm_done & (stm_idx == stm_finish_idx) ? NORMAL : stm_state;
          end else begin
            stm_state <= STM;
          end
        end
      endcase
    end
  end else begin
    for (genvar i = 0; i < TRANS_NUM; i++) begin
      assign duty[i]  = duty_normal[i];
      assign phase[i] = phase_normal[i];
    end
    assign stm_done = 0;
  end

  synchronizer synchronizer (
      .CLK(clk),
      .ECAT_SYNC_TIME(ecat_sync_time),
      .SET(sync_set),
      .ECAT_SYNC(CAT_SYNC0),
      .SYS_TIME(sys_time),
      .SYNC()
  );

  if (ENABLE_MODULATOR == "TRUE") begin
    modulator #(
        .WIDTH(WIDTH),
        .DEPTH(TRANS_NUM)
    ) modulator (
        .CLK(clk_l),
        .SYS_TIME(sys_time),
        .CYCLE(cycle_m),
        .FREQ_DIV(freq_div_m),
        .DELAY_M(delay_m),
        .CPU_BUS(cpu_bus.mod_port),
        .DUTY_IN(duty),
        .PHASE_IN(phase),
        .DUTY_OUT(duty_m),
        .PHASE_OUT(phase_m),
        .START(),
        .DONE(mod_done)
    );
  end else begin
    assign duty_m   = duty;
    assign phase_m  = phase;
    assign mod_done = 0;
  end

  if (ENABLE_SILENCER == "TRUE") begin
    silencer #(
        .WIDTH(WIDTH),
        .DEPTH(TRANS_NUM)
    ) silencer (
        .CLK(clk_l),
        .SYS_TIME(sys_time),
        .CYCLE_S(cycle_s),
        .STEP(step_s),
        .CYCLE(cycle),
        .DUTY(duty_m),
        .PHASE(phase_m),
        .DUTY_S(duty_s),
        .PHASE_S(phase_s),
        .DONE(silencer_done)
    );
  end else begin
    assign duty_s = duty_m;
    assign phase_s = phase_m;
    assign silencer_done = 0;
  end

  pwm #(
      .WIDTH(WIDTH),
      .TRANS_NUM(TRANS_NUM)
  ) pwm (
      .CLK(clk),
      .CLK_L(clk_l),
      .SYS_TIME(sys_time),
      .CYCLE(cycle),
      .DUTY(duty_s),
      .PHASE(phase_s),
      .PWM_OUT(PWM_OUT),
      .DONE(),
      .TIME_CNT()
  );

endmodule
