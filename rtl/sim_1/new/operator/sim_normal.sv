/*
 * File: sim_normal.sv
 * Project: operator
 * Created Date: 12/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */

module sim_operator_normal();

bit CLK_20P48M;
bit locked;
sim_helper_clk sim_helper_clk(
                   .CLK_163P84M(),
                   .CLK_20P48M(CLK_20P48M),
                   .LOCKED(locked),
                   .SYS_TIME()
               );

localparam int WIDTH = 13;
localparam int DEPTH = 249;
localparam bit [WIDTH-1:0] MAX = (1 << WIDTH) - 1;

sim_helper_bram sim_helper_bram();
sim_helper_random sim_helper_random();

bit [WIDTH-1:0] duty_buf[0:DEPTH-1];
bit [WIDTH-1:0] phase_buf[0:DEPTH-1];
bit [WIDTH-1:0] duty[0:DEPTH-1];
bit [WIDTH-1:0] phase[0:DEPTH-1];

normal_operator#(
                   .WIDTH(WIDTH),
                   .DEPTH(DEPTH)
               ) normal_operator (
                   .CLK(CLK_20P48M),
                   .CPU_BUS(sim_helper_bram.cpu_bus.normal_port),
                   .DUTY(duty),
                   .PHASE(phase)
               );

initial begin
    @(posedge locked);

    sim_helper_random.init();
    for (int i = 0; i < DEPTH; i++) begin
        duty_buf[i] = sim_helper_random.range(MAX, 0);
        phase_buf[i] = sim_helper_random.range(MAX, 0);
        sim_helper_bram.write_duty_phase(i, duty_buf[i], phase_buf[i]);
    end

    for (int i = 0; i < DEPTH * 2; i++) begin
        @(posedge CLK_20P48M);
    end

    for (int i = 0; i < DEPTH; i++) begin
        if (duty_buf[i] != duty[i]) begin
            $display("failed at duty[%d], %d!=%d", i, duty_buf[i], duty[i]);
            $finish();
        end
        if (phase_buf[i] != phase[i]) begin
            $display("failed at phase[%d], %d!=%d", i, phase_buf[i], phase[i]);
            $finish();
        end
    end

    $display("OK!");
    $finish();
end

endmodule
