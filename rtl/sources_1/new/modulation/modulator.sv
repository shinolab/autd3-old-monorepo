/*
 * File: modulator.sv
 * Project: modulation
 * Created Date: 24/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */

`timescale 1ns / 1ps
module modulator#(
           parameter int WIDTH = 13,
           parameter int DEPTH = 249
       )(
           input var CLK,
           input var [63:0] SYS_TIME,
           input var [15:0] CYCLE,
           input var [31:0] FREQ_DIV,
           cpu_bus_if.mod_port CPU_BUS,
           input var [WIDTH-1:0] DUTY_IN[0:DEPTH-1],
           input var [WIDTH-1:0] PHASE_IN[0:DEPTH-1],
           output var [WIDTH-1:0] DUTY_OUT[0:DEPTH-1],
           output var [WIDTH-1:0] PHASE_OUT[0:DEPTH-1],
           output var START,
           output var DONE,
           output var [15:0] IDX
       );

ms_bus_if ms_bus_if();

bit [7:0] m;
bit start;
bit done;
bit [15:0] idx;

bit [WIDTH-1:0] duty_tmp[0:DEPTH-1];

assign START = start;
assign DONE = done;
assign IDX = idx;

modulation_memory modulation_memory(
                      .CLK(CLK),
                      .CPU_BUS(CPU_BUS),
                      .MS_BUS(ms_bus_if.memory_port)
                  );

modulation_sampler modulation_sampler(
                       .CLK(CLK),
                       .SYS_TIME(SYS_TIME),
                       .CYCLE(CYCLE),
                       .FREQ_DIV(FREQ_DIV),
                       .MS_BUS(ms_bus_if.sampler_port),
                       .M(m),
                       .START(start),
                       .IDX(idx)
                   );

modulation_multiplier#(
                         .WIDTH(WIDTH),
                         .DEPTH(DEPTH)
                     ) modulation_multiplier (
                         .CLK(CLK),
                         .START(start),
                         .M(m),
                         .DUTY_IN(DUTY_IN),
                         .DUTY_OUT(duty_tmp),
                         .DONE(done)
                     );

modulation_buffer#(
                     .WIDTH(WIDTH),
                     .DEPTH(DEPTH)
                 ) modulation_buffer (
                     .CLK(CLK),
                     .START(start),
                     .DONE(done),
                     .DUTY_IN(duty_tmp),
                     .PHASE_IN(PHASE_IN),
                     .DUTY_OUT(DUTY_OUT),
                     .PHASE_OUT(PHASE_OUT)
                 );

endmodule
