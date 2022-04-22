/*
 * File: sim_modulator.sv
 * Project: modulation
 * Created Date: 25/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */

module sim_modulator();

bit [63:0] SYS_TIME;
bit locked;
sim_helper_clk sim_helper_clk(
                   .CLK_163P84M(),
                   .CLK_20P48M(CLK_20P48M),
                   .LOCKED(locked),
                   .SYS_TIME(SYS_TIME)
               );

sim_helper_random sim_helper_random();
sim_helper_bram sim_helper_bram();

localparam int WIDTH = 13;
localparam int DEPTH = 10;

bit [15:0] cycle;
bit [31:0] freq_div;
bit [WIDTH-1:0] duty[0:DEPTH-1];
bit [WIDTH-1:0] phase[0:DEPTH-1];
bit [WIDTH-1:0] duty_out[0:DEPTH-1];
bit [WIDTH-1:0] phase_out[0:DEPTH-1];
bit start;
bit done;
bit [15:0] idx;

bit [7:0] mod_data[0:65535];

modulator#(
             .WIDTH(WIDTH),
             .DEPTH(DEPTH)
         ) modulator (
             .CLK(CLK_20P48M),
             .SYS_TIME(SYS_TIME),
             .CYCLE(cycle),
             .FREQ_DIV(freq_div),
             .CPU_BUS(sim_helper_bram.cpu_bus.mod_port),
             .DUTY_IN(duty),
             .PHASE_IN(phase),
             .DUTY_OUT(duty_out),
             .PHASE_OUT(phase_out),
             .START(start),
             .DONE(done),
             .IDX(idx)
         );

localparam int DIV_LATENCY = 66;

bit [15:0] idx_buf;
task set_random();
    @(posedge CLK_20P48M);
    cycle = 16'hFFFF;
    // cycle = 16'd999;
    for (int i = 0; i < DEPTH; i++) begin
        duty[i] = sim_helper_random.range(8000, 0);
    end
    for (int j = 0; j < cycle + 1; j++) begin
        mod_data[j] = sim_helper_random.range(8'hFF, 0);
    end
    sim_helper_bram.write_mod(mod_data, cycle + 1);

    for (int j = 0; j < cycle + 1; j++) begin
        @(posedge start);
        idx_buf = idx;
        $display("check %d @%d", idx_buf, SYS_TIME);
        @(posedge done);
        @(posedge CLK_20P48M);
        @(posedge CLK_20P48M);
        for (int i = 0; i < DEPTH; i++) begin
            if (duty_out[i] != (duty[i] * mod_data[idx_buf] / 255)) begin
                $error("Failed at d=%d, m=%d, d_m=%d", duty[i], mod_data[idx_buf], duty_out[i]);
                $finish();
            end
        end
    end
endtask

localparam int MULT_LATENCY = 38;

initial begin
    cycle = 0;
    freq_div = 8*(1 + MULT_LATENCY + DEPTH + 2);
    duty = '{DEPTH{2500}};
    phase = '{DEPTH{2500}};
    sim_helper_random.init();
    @(posedge locked);

    set_random();

    $display("OK!");
    $finish();
end

endmodule
