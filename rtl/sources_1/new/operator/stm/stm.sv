/*
 * File: stm.sv
 * Project: stm
 * Created Date: 13/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */

`timescale 1ns / 1ps
module stm_operator#(
           parameter int WIDTH = 13,
           parameter int DEPTH = 249
       )(
           input var CLK,
           input var RST,
           input var [63:0] SYS_TIME,
           input var [WIDTH-1:0] ULTRASOUND_CYCLE[0:DEPTH-1],
           input var [15:0] CYCLE,
           input var [31:0] FREQ_DIV,
           input var [31:0] SOUND_SPEED,
           input var STM_GAIN_MODE,
           cpu_bus_if.stm_port CPU_BUS,
           output var [WIDTH-1:0] DUTY[0:DEPTH-1],
           output var [WIDTH-1:0] PHASE[0:DEPTH-1],
           output var START,
           output var DONE,
           output var [15:0] IDX
       );

ss_bus_if ss_bus_if();
assign ss_bus_if.STM_GAIN_MODE = STM_GAIN_MODE;

bit [WIDTH-1:0] duty_gain[0:DEPTH-1];
bit [WIDTH-1:0] phase_gain[0:DEPTH-1];
bit [WIDTH-1:0] duty_focus[0:DEPTH-1];
bit [WIDTH-1:0] phase_focus[0:DEPTH-1];
bit [15:0] idx;
bit start_gain, done_gain;
bit start_focus, done_focus;

for (genvar i = 0; i < DEPTH; i++) begin
    assign DUTY[i] = STM_GAIN_MODE ? duty_gain[i] : duty_focus[i];
    assign PHASE[i] = STM_GAIN_MODE ? phase_gain[i] : phase_focus[i];
end

assign IDX = idx;
assign START = STM_GAIN_MODE ? start_gain : start_focus;
assign DONE = STM_GAIN_MODE ? done_gain : done_focus;

stm_memory stm_memory(
               .CLK(CLK),
               .CPU_BUS(CPU_BUS),
               .SS_BUS(ss_bus_if.memory_port)
           );

stm_sampler stm_sampler(
                .CLK(CLK),
                .SYS_TIME(SYS_TIME),
                .CYCLE(CYCLE),
                .FREQ_DIV(FREQ_DIV),
                .IDX(idx)
            );

stm_gain_operator#(
                     .WIDTH(WIDTH),
                     .DEPTH(DEPTH)
                 ) stm_gain_operator (
                     .CLK(CLK),
                     .RST(RST),
                     .IDX(idx),
                     .SS_BUS(ss_bus_if.gain_port),
                     .DUTY(duty_gain),
                     .PHASE(phase_gain),
                     .START(start_gain),
                     .DONE(done_gain)
                 );

stm_focus_operator#(
                      .WIDTH(WIDTH),
                      .DEPTH(DEPTH)
                  ) stm_focus_operator (
                      .CLK(CLK),
                      .RST(RST),
                      .IDX(idx),
                      .SS_BUS(ss_bus_if.focus_port),
                      .SOUND_SPEED(SOUND_SPEED),
                      .ULTRASOUND_CYCLE(ULTRASOUND_CYCLE),
                      .DUTY(duty_focus),
                      .PHASE(phase_focus),
                      .START(start_focus),
                      .DONE(done_focus)
                  );

endmodule
