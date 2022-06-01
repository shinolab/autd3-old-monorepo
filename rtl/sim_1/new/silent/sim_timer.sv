/*
 * File: sim_timer.sv
 * Project: silent
 * Created Date: 22/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

`timescale 1ns / 1ps
module sim_silent_timer();

localparam bit [63:0] CLK_FREQ = 163840000;

bit CLK_20P48M;
bit [63:0] SYS_TIME;
bit locked;
sim_helper_clk sim_helper_clk(
                   .CLK_163P84M(),
                   .CLK_20P48M(CLK_20P48M),
                   .LOCKED(locked),
                   .SYS_TIME(SYS_TIME)
               );

localparam int WIDTH = 13;

bit [15:0] cycle;
bit update;

silent_timer#(
                .WIDTH(WIDTH)
            ) silent_timer (
                .CLK(CLK_20P48M),
                .SYS_TIME(SYS_TIME),
                .CYCLE(cycle),
                .UPDATE(update)
            );

bit [63:0] time_t;
initial begin
    cycle = 4096;

    @(posedge locked);

    @(posedge update);
    time_t = $time();

    for(int i = 0; i < 3; i++) begin
        $display("check %d", i);
        @(posedge update);
        if (1000000000/ (CLK_FREQ / cycle) != $time() - time_t) begin
            $display("Failed! %d != %d", 1000000000/ (CLK_FREQ / cycle), $time() - time_t);
            $finish();
        end
        time_t = $time();
    end

    $display("OK!");
    $finish();
end

endmodule
