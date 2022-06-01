/*
 * File: sim_generator.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

module sim_pwm_generator();

bit CLK_163P84M;
bit locked;
bit [63:0] SYS_TIME;
sim_helper_clk sim_helper_clk(
                   .CLK_163P84M(CLK_163P84M),
                   .CLK_20P48M(),
                   .LOCKED(locked),
                   .SYS_TIME(SYS_TIME)
               );

localparam int WIDTH = 13;
localparam int CYCLE = 4096;

bit [WIDTH-1:0] time_cnt;
assign time_cnt = SYS_TIME % CYCLE;

bit [WIDTH-1:0] rise, fall;

bit pwm_out;

pwm_generator#(
                 .WIDTH(WIDTH)
             ) pwm_generator (
                 .CLK(CLK_163P84M),
                 .TIME_CNT(time_cnt),
                 .RISE(rise),
                 .FALL(fall),
                 .PWM_OUT(pwm_out)
             );

task set(bit [WIDTH-1:0] r, bit [WIDTH-1:0] f);
    while (time_cnt != CYCLE - 1)
        @(posedge CLK_163P84M);
    rise = r;
    fall = f;
    @(posedge CLK_163P84M);
    $display("check start\t@t=%d", SYS_TIME);
    while (1) begin
        automatic int t = time_cnt;
        @(posedge CLK_163P84M);
        if (pwm_out != (((r <= f) & ((r <= t) & (t < f))) | ((f < r) & ((r <= t) | (t < f))))) begin
            $error("Failed at v=%u, t=%d, R=%d, F=%d", pwm_out, time_cnt, rise, fall);
            $finish();
        end
        if (time_cnt == CYCLE - 1) begin
            break;
        end
    end
    $display("check done\t@t=%d", SYS_TIME);
endtask

initial begin
    time_cnt = 0;
    rise = 0;
    fall = 0;
    @(posedge locked);

    set(CYCLE/2-CYCLE/4, CYCLE/2+CYCLE/4); // normal, D=CYCLE/2
    set(0, CYCLE); // normal, D=CYCLE
    set(CYCLE/2, CYCLE/2); // normal, D=0
    set(0, CYCLE/2); // normal, D=CYCLE/2, left edge
    set(CYCLE-CYCLE/2, CYCLE); // normal, D=CYCLE/2, right edge

    set(CYCLE-CYCLE/4, CYCLE/4); // over, D=CYCLE/2
    set(CYCLE, 0); // over, D=0
    set(CYCLE, CYCLE/2); // over, D=CYCLE/2, right edge
    set(CYCLE-CYCLE/2, 0); // over, D=CYCLE/2, left edge

    set(0, 0);

    $display("OK!");
    $finish();
end

endmodule
