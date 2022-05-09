/*
 * File: silencer.sv
 * Project: silent
 * Created Date: 22/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */

module silencer#(
           parameter int WIDTH = 13,
           parameter int DEPTH = 249
       )(
           input var CLK,
           input var [63:0] SYS_TIME,
           input var [15:0] CYCLE_S,
           input var [WIDTH-1:0] STEP,
           input var [WIDTH-1:0] CYCLE[0:DEPTH-1],
           input var [WIDTH-1:0] DUTY[0:DEPTH-1],
           input var [WIDTH-1:0] PHASE[0:DEPTH-1],
           output var [WIDTH-1:0] DUTY_S[0:DEPTH-1],
           output var [WIDTH-1:0] PHASE_S[0:DEPTH-1],
           output var DONE
       );

bit update;
bit [WIDTH-1:0] duty_s_tmp[0:DEPTH-1];
bit [WIDTH-1:0] phase_s_tmp[0:DEPTH-1];
bit done;

assign DONE = done;

silent_timer#(
                .WIDTH(WIDTH)
            ) silent_timer (
                .CLK(CLK),
                .SYS_TIME(SYS_TIME),
                .CYCLE(CYCLE_S),
                .UPDATE(update)
            );

silent_filter#(
                 .WIDTH(WIDTH),
                 .DEPTH(DEPTH)
             ) silent_filter (
                 .CLK(CLK),
                 .UPDATE(update),
                 .STEP(STEP),
                 .CYCLE(CYCLE),
                 .DUTY(DUTY),
                 .PHASE(PHASE),
                 .DUTY_S(duty_s_tmp),
                 .PHASE_S(phase_s_tmp),
                 .DONE(done)
             );

silent_buffer#(
                 .WIDTH(WIDTH),
                 .DEPTH(DEPTH)
             ) silent_buffer (
                 .CLK(CLK),
                 .DONE(done),
                 .DUTY_IN(duty_s_tmp),
                 .PHASE_IN(phase_s_tmp),
                 .DUTY_OUT(DUTY_S),
                 .PHASE_OUT(PHASE_S)
             );

endmodule
