/*
 * File: sim_pwm.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */


module sim_pwm();

bit CLK_163P84M;
bit CLK_20P48M;
bit [63:0] SYS_TIME;
bit locked;
sim_helper_clk sim_helper_clk(
                   .CLK_163P84M(CLK_163P84M),
                   .CLK_20P48M(CLK_20P48M),
                   .LOCKED(locked),
                   .SYS_TIME(SYS_TIME)
               );

sim_helper_random sim_helper_random();

localparam int WIDTH = 13;
localparam int DEPTH = 249;
localparam int CYCLE = 4096;

bit [WIDTH-1:0] cycle[0:DEPTH-1];
bit [WIDTH-1:0] duty[0:DEPTH-1];
bit [WIDTH-1:0] phase[0:DEPTH-1];
bit pwm_out[0:DEPTH-1];
bit done;

bit [WIDTH-1:0] time_cnt[0:DEPTH-1];

pwm#(
       .WIDTH(WIDTH),
       .TRANS_NUM(DEPTH)
   ) pwm(
       .CLK(CLK_163P84M),
       .CLK_L(CLK_20P48M),
       .SYS_TIME(SYS_TIME),
       .CYCLE(cycle),
       .DUTY(duty),
       .PHASE(phase),
       .PWM_OUT(pwm_out),
       .DONE(done),
       .TIME_CNT(time_cnt)
   );

task wait_calc();
    // capture values
    while(1) begin
        @(posedge CLK_20P48M);
        if (done) begin
            break;
        end
    end

    // wait calc
    while(1) begin
        @(posedge CLK_20P48M);
        if (done) begin
            break;
        end
    end

    @(posedge CLK_20P48M);
endtask

task set(int idx, bit [WIDTH-1:0] d,  bit [WIDTH-1:0] p);
    @(posedge CLK_20P48M);
    duty[idx] = d;
    phase[idx] = p;

    wait_calc();

    // wait buffer
    while (time_cnt[idx] != cycle[idx] - 1)
        @(posedge CLK_163P84M);
    @(posedge CLK_163P84M);
endtask

task set_and_check(int idx, bit [WIDTH-1:0] d,  bit [WIDTH-1:0] p);
    set(idx, d, p);

    $display("check start\tidx=%d, duty=%d, phase=%d \t@t=%d", idx[$clog2(DEPTH)-1:0], d, p, SYS_TIME);
    while (1) begin
        automatic int r = (cycle[idx]-phase[idx]-duty[idx]/2+cycle[idx])%cycle[idx];
        automatic int f = (cycle[idx]-phase[idx]+(duty[idx]+1)/2)%cycle[idx];
        automatic int t = time_cnt[idx];
        @(posedge CLK_163P84M);
        if (pwm_out[idx] != (((r <= f) & ((r <= t) & (t < f))) | ((f < r) & ((r <= t) | (t < f))))) begin
            $error("\tFailed at v=%u, t=%d, T=%d, duty=%d, phase=%d, R=%d, F=%d", pwm_out[idx], t, cycle[idx], duty[idx], phase[idx], r, f);
            $finish();
        end
        if (t == cycle[idx] - 1) begin
            break;
        end
    end
endtask

task set_and_check_random();
    automatic int idx = sim_helper_random.range(DEPTH-1, 0);
    automatic int d = sim_helper_random.range(cycle[idx], 0);
    automatic int p = sim_helper_random.range(cycle[idx], 0);
    set_and_check(idx, d, p);
endtask

initial begin
    sim_helper_random.init();
    cycle[0] = CYCLE;
    for (int i = 1; i < DEPTH; i++) begin
        cycle[i] = sim_helper_random.range(8000, 2000);
    end
    duty = '{DEPTH{0}};
    phase = '{DEPTH{0}};
    @(posedge locked);

    // set(0, CYCLE, CYCLE/2);
    // set(0, 1000, CYCLE/2);
    // set(0, 0, 0);

    // at random
    for(int i = 0; i < 100; i++) begin
        set_and_check_random();
    end

    $display("OK! sim_pwm");
    $finish();
end

endmodule
