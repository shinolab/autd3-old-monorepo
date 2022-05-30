/*
 * File: sim_sampler.sv
 * Project: modulation
 * Created Date: 25/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

module sim_modulation_sampler();

localparam bit [63:0] CLK_FREQ = 163840000;
localparam int DIV_LATENCY = 66;

bit CLK_20P48M;
bit [63:0] SYS_TIME;
bit locked;
sim_helper_clk sim_helper_clk(
                   .CLK_163P84M(),
                   .CLK_20P48M(CLK_20P48M),
                   .LOCKED(locked),
                   .SYS_TIME(SYS_TIME)
               );

sim_helper_random sim_helper_random();

localparam int WIDTH = 13;
localparam int DEPTH = 249;

bit [15:0] cycle;
bit [31:0] freq_div;
bit [7:0] m;
bit start;
bit [15:0] idx;

ms_bus_if ms_bus_if();

modulation_sampler modulation_sampler(
                       .CLK(CLK_20P48M),
                       .SYS_TIME(SYS_TIME),
                       .CYCLE(cycle),
                       .FREQ_DIV(freq_div),
                       .MS_BUS(ms_bus_if.sampler_port),
                       .M(m),
                       .START(start),
                       .IDX(idx)
                   );

bit [63:0] time_t;
initial begin
    cycle = 16'hFFFF;
    freq_div = 4096;
    @(posedge locked);

    @(posedge start);
    time_t = $time();

    for(int i = 0; i < cycle; i++) begin
        $display("check %d", i);
        @(posedge start);
        if (1000000000/ (CLK_FREQ / freq_div) != $time() - time_t) begin
            $display("Sample timing failed! %d != %d", 1000000000/ (CLK_FREQ / freq_div), $time() - time_t);
            $finish();
        end
        if (((SYS_TIME - 8*DIV_LATENCY*2 - 2 - 1) / freq_div) % (cycle + 1) != idx) begin
            $display("Index failed! %d != %d", ((SYS_TIME - 8*DIV_LATENCY*2 - 2 - 1) / freq_div) % (cycle + 1) , idx);
            $finish();
        end
        time_t = $time();
    end

    $display("OK!");
    $finish();
end

endmodule
