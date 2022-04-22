/*
 * File: sim_gain.sv
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

module sim_operator_stm_gain();

bit CLK_20P48M;
bit locked;
bit [63:0] SYS_TIME;
sim_helper_clk sim_helper_clk(
                   .CLK_163P84M(),
                   .CLK_20P48M(CLK_20P48M),
                   .LOCKED(locked),
                   .SYS_TIME(SYS_TIME)
               );

localparam int WIDTH = 13;
localparam int DEPTH = 249;
localparam bit [WIDTH-1:0] MAX = (1 << WIDTH) - 1;

sim_helper_bram sim_helper_bram();
sim_helper_random sim_helper_random();

bit [15:0] cycle_s;
bit [31:0] freq_div_s;

bit start, done;
bit [15:0] idx;

bit [WIDTH-1:0] duty_buf[0:1023][0:DEPTH-1];
bit [WIDTH-1:0] phase_buf[0:1023][0:DEPTH-1];
bit [WIDTH-1:0] duty[0:DEPTH-1];
bit [WIDTH-1:0] phase[0:DEPTH-1];

stm_operator#(
                .WIDTH(WIDTH),
                .DEPTH(DEPTH)
            ) stm_operator (
                .CLK(CLK_20P48M),
                .SYS_TIME(SYS_TIME),
                .ULTRASOUND_CYCLE(),
                .CYCLE(cycle_s),
                .FREQ_DIV(freq_div_s),
                .SOUND_SPEED(),
                .STM_GAIN_MODE(1'b1),
                .CPU_BUS(sim_helper_bram.cpu_bus.stm_port),
                .DUTY(duty),
                .PHASE(phase),
                .START(start),
                .DONE(done),
                .IDX(idx)
            );

bit [15:0] idx_buf;
initial begin
    cycle_s = 33 - 1;
    freq_div_s = 8*(1 + DEPTH / 4 + 3 + 2);
    @(posedge locked);

    sim_helper_random.init();
    for (int i = 0; i < cycle_s + 1; i++) begin
        for (int j = 0; j < DEPTH; j++) begin
            duty_buf[i][j] = sim_helper_random.range(MAX, 0);
            phase_buf[i][j] = sim_helper_random.range(MAX, 0);
        end
        sim_helper_bram.write_stm_gain_duty_phase(i, duty_buf[i], phase_buf[i]);
    end

    for (int j = 0; j < cycle_s + 1; j++) begin
        @(posedge start);
        idx_buf = idx;
        $display("check %d @%d", idx_buf, SYS_TIME);
        @(posedge done);
        for (int i = 0; i < DEPTH; i++) begin
            if (duty_buf[idx_buf][i] != duty[i]) begin
                $error("Failed at d_in=%d, d_out=%d", duty_buf[idx_buf][i], duty[i]);
                $finish();
            end
            if (phase_buf[idx_buf][i] != phase[i]) begin
                $error("Failed at p_in=%d, p_out=%d", phase_buf[idx_buf][i], phase[i]);
                $finish();
            end
        end
    end

    $display("OK!");
    $finish();
end

endmodule
