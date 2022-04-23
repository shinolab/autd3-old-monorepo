/*
 * File: top.sv
 * Project: new
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */

`timescale 1ns / 1ps
module top(
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
           //    output var [3:0] GPIO_OUT
           input var [3:0] GPIO_IN
       );

`include "cvt_uid.vh"
`include "params.vh"

localparam int WIDTH = 13;
localparam int TRANS_NUM = 249;

bit clk, clk_l, clk_ctl;
bit reset;

bit [63:0] sys_time;

bit [63:0] ecat_sync_time;
bit [15:0] ecat_sync_cycle_ticks;
bit sync_set;

bit [1:0] load_mode;

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

bit [15:0] cycle_s;
bit [WIDTH-1:0] step_s;
bit [WIDTH-1:0] duty_s[0:TRANS_NUM-1];
bit [WIDTH-1:0] phase_s[0:TRANS_NUM-1];

bit [15:0] cycle_stm;
bit [31:0] freq_div_stm;
bit [31:0] sound_speed;

bit PWM_OUT[0:TRANS_NUM-1];

bit wdt_rst;
bit wst_assert;

assign reset = ~RESET_N;
if (ENABLE_STM == "TRUE") begin
    for (genvar i = 0; i < TRANS_NUM; i++) begin
        assign duty[i] = op_mode ? duty_stm[i] : duty_normal[i];
        assign phase[i] = op_mode ? phase_stm[i] : phase_normal[i];
    end
end
else begin
    for (genvar i = 0; i < TRANS_NUM; i++) begin
        assign duty[i] = duty_normal[i];
        assign phase[i] = phase_normal[i];
    end
end
for (genvar i = 0; i < TRANS_NUM; i++) begin
    assign XDCR_OUT[cvt_uid(i) + 1] = PWM_OUT[i];
end

if (ENABLE_MODULATOR != "TRUE") begin
    assign duty_m = duty;
    assign phase_m = phase;
end

if (ENABLE_SILENCER != "TRUE") begin
    assign duty_s = duty_m;
    assign phase_s = phase_m;
end

cpu_bus_if cpu_bus();
assign cpu_bus.BUS_CLK = CPU_CKIO;
assign cpu_bus.EN = ~CPU_CS1_N;
assign cpu_bus.RD = ~CPU_RD_N;
assign cpu_bus.RDWR = CPU_RDWR;
assign cpu_bus.WE = ~CPU_WE0_N;
assign cpu_bus.BRAM_SELECT = CPU_ADDR[16:15];
assign cpu_bus.BRAM_ADDR = CPU_ADDR[14:1];
assign cpu_bus.CPU_DATA = CPU_DATA;

ultrasound_cnt_clk_gen ultrasound_cnt_clk_gen(
                           .clk_in1(MRCC_25P6M),
                           .reset(reset),
                           .clk_out1(clk),
                           .clk_out2(clk_l),
                           .locked()
                       );

wdt wdt(
        .CLK(clk_l),
        .RST(wdt_rst),
        .ASSERT(wst_assert)
    );

controller#(
              .WIDTH(WIDTH),
              .DEPTH(TRANS_NUM)
          ) controller (
              .CLK(clk_l),
              .THERMO(THERMO),
              .FORCE_FAN(FORCE_FAN),
              .CPU_BUS(cpu_bus.ctl_port),
              .ECAT_SYNC_TIME(ecat_sync_time),
              .ECAT_SYNC_CYCLE_TICKS(ecat_sync_cycle_ticks),
              .SYNC_SET(sync_set),
              .OP_MODE(op_mode),
              .STM_GAIN_MODE(stm_gain_mode),
              .CYCLE_M(cycle_m),
              .FREQ_DIV_M(freq_div_m),
              .CYCLE_S(cycle_s),
              .STEP_S(step_s),
              .CYCLE_STM(cycle_stm),
              .FREQ_DIV_STM(freq_div_stm),
              .SOUND_SPEED(sound_speed),
              .CYCLE(cycle),
              .LOAD_MODE(load_mode),
              .WDT_RST(wdt_rst)
          );

normal_operator#(
                   .WIDTH(WIDTH),
                   .DEPTH(TRANS_NUM)
               ) normal_operator (
                   .CLK(clk_l),
                   .RST(wst_assert),
                   .CPU_BUS(cpu_bus.normal_port),
                   .CYCLE(cycle),
                   .LOAD_MODE(load_mode),
                   .DUTY(duty_normal),
                   .PHASE(phase_normal)
               );

if (ENABLE_STM == "TRUE") begin
    stm_operator#(
                    .WIDTH(WIDTH),
                    .DEPTH(TRANS_NUM)
                ) stm_operator (
                    .CLK(clk_l),
                    .RST(wst_assert),
                    .SYS_TIME(sys_time),
                    .LOAD_MODE(load_mode),
                    .ULTRASOUND_CYCLE(cycle),
                    .CYCLE(cycle_stm),
                    .FREQ_DIV(freq_div_stm),
                    .SOUND_SPEED(sound_speed),
                    .STM_GAIN_MODE(stm_gain_mode),
                    .CPU_BUS(cpu_bus.stm_port),
                    .DUTY(duty_stm),
                    .PHASE(phase_stm),
                    .START(),
                    .DONE(),
                    .IDX()
                );
end

synchronizer synchronizer(
                 .CLK(clk),
                 .ECAT_SYNC_TIME(ecat_sync_time),
                 .ECAT_SYNC_CYCLE_TICKS(ecat_sync_cycle_ticks),
                 .SET(sync_set),
                 .ECAT_SYNC(CAT_SYNC0),
                 .SYS_TIME(sys_time)
             );

if (ENABLE_MODULATOR == "TRUE") begin
    modulator#(
                 .WIDTH(WIDTH),
                 .DEPTH(TRANS_NUM)
             ) modulator (
                 .CLK(clk_l),
                 .SYS_TIME(sys_time),
                 .CYCLE(cycle_m),
                 .FREQ_DIV(freq_div_m),
                 .CPU_BUS(cpu_bus.mod_port),
                 .DUTY_IN(duty),
                 .PHASE_IN(phase),
                 .DUTY_OUT(duty_m),
                 .PHASE_OUT(phase_m),
                 .START(),
                 .DONE()
             );
end

if (ENABLE_SILENCER == "TRUE") begin
    silencer#(
                .WIDTH(WIDTH),
                .DEPTH(TRANS_NUM)
            ) silencer (
                .CLK(clk_l),
                .RST(wst_assert),
                .SYS_TIME(sys_time),
                .CYCLE_S(cycle_s),
                .STEP(step_s),
                .CYCLE(cycle),
                .DUTY(duty_m),
                .PHASE(phase_m),
                .DUTY_S(duty_s),
                .PHASE_S(phase_s),
                .DONE()
            );
end

pwm #(
        .WIDTH(WIDTH),
        .TRANS_NUM(TRANS_NUM)
    ) pwm (
        .CLK(clk),
        .CLK_L(clk_l),
        .SET(sync_set),
        .SYS_TIME(sys_time),
        .CYCLE(cycle),
        .DUTY(duty_s),
        .PHASE(phase_s),
        .PWM_OUT(PWM_OUT),
        .DONE(),
        .TIME_CNT()
    );

endmodule
