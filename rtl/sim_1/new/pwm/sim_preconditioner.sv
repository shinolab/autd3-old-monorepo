/*
 * File: sim_preconditioner.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */

module sim_pwm_preconditioner();

bit CLK_20P48M;
bit locked;
sim_helper_clk sim_helper_clk(
                   .CLK_163P84M(),
                   .CLK_20P48M(CLK_20P48M),
                   .LOCKED(locked),
                   .SYS_TIME()
               );

sim_helper_random sim_helper_random();

localparam int WIDTH = 13;
localparam int DEPTH = 249;
localparam int CYCLE = 4096;

bit [WIDTH-1:0] cycle[0:DEPTH-1];
bit [WIDTH-1:0] duty[0:DEPTH-1];
bit [WIDTH-1:0] phase[0:DEPTH-1];

bit [WIDTH-1:0] rise[0:DEPTH-1];
bit [WIDTH-1:0] fall[0:DEPTH-1];
bit done;

pwm_preconditioner#(
                      .WIDTH(WIDTH),
                      .DEPTH(DEPTH)
                  ) pwm_preconditioner (
                      .CLK(CLK_20P48M),
                      .CYCLE(cycle),
                      .DUTY(duty),
                      .PHASE(phase),
                      .RISE(rise),
                      .FALL(fall),
                      .DONE(done)
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

    if ((rise[idx] != ((cycle[idx]-p-d/2+cycle[idx])%cycle[idx])) & (fall[idx] == ((cycle[idx]-p+(d+1)/2)%cycle[idx]))) begin
        $error("Failed at idx=%d, d=%d, p=%d, R=%d, F=%d", idx, d, p, rise[idx], fall[idx]);
        $finish();
    end
endtask

task set_random();
    @(posedge CLK_20P48M);
    for (int j = 0; j < DEPTH; j++) begin
        cycle[j] = sim_helper_random.range(8000, 2000);
        duty[j] = sim_helper_random.range(cycle[j], 0);
        phase[j] = sim_helper_random.range(cycle[j], 0);
    end

    wait_calc();

    for (int j = 0; j < DEPTH; j++) begin
        if ((rise[j] != ((cycle[j]-phase[j]-duty[j]/2+cycle[j])%cycle[j])) | (fall[j] != ((cycle[j]-phase[j]+(duty[j]+1)/2)%cycle[j]))) begin
            $error("Failed at T=%d, d=%d, p=%d, R=%d, F=%d", cycle[j], duty[j], phase[j], rise[j], fall[j]);
            $finish();
        end
    end
endtask

initial begin
    cycle = '{DEPTH{CYCLE}};
    duty = '{DEPTH{0}};
    phase = '{DEPTH{0}};
    @(posedge locked);

    set(0, CYCLE/2, CYCLE/2); // normal, D=CYCLE/2
    set(0, CYCLE, CYCLE/2); // normal, D=CYCLE
    set(0, 0, CYCLE/2); // normal, D=0
    set(0, CYCLE/2, CYCLE/2-CYCLE/4); // normal, D=CYCLE/2, left edge
    set(0, CYCLE/2, CYCLE/2+CYCLE/4); // normal, D=CYCLE/2, right edge

    set(0, CYCLE/2, 0); // over, D=CYCLE/2
    set(0, CYCLE/2, CYCLE); // over, D=CYCLE/2
    set(0, 0, CYCLE); // over, D=0

    // at random
    sim_helper_random.init();
    for(int i = 0; i < 5000; i++) begin
        $display("check start @%d", i);
        set_random();
        $display("check finish @%d", i);
    end

    $display("OK!");
    $finish();
end

endmodule
