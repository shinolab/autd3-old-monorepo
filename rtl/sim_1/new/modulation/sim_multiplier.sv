/*
 * File: sim_multiplier.sv
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

module sim_modulation_multiplier();

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

bit start;
bit done;
bit [7:0] m;
bit [WIDTH-1:0] duty[0:DEPTH-1];
bit [WIDTH-1:0] duty_out[0:DEPTH-1];

modulation_multiplier#(
                         .WIDTH(),
                         .DEPTH()
                     ) modulation_multiplier (
                         .CLK(CLK_20P48M),
                         .START(start),
                         .M(m),
                         .DUTY_IN(duty),
                         .DUTY_OUT(duty_out),
                         .DONE(done)
                     );

task set_random();
    @(posedge CLK_20P48M);
    for (int j = 0; j < DEPTH; j++) begin
        duty[j] = sim_helper_random.range(8000, 0);
    end
    m = sim_helper_random.range(8'hFF, 0);
    start = 1;

    @(posedge done);

    for (int j = 0; j < DEPTH; j++) begin
        if (duty_out[j] != (duty[j] * m / 255)) begin
            $error("Failed at d=%d, m=%d, d_m=%d", duty[j], m, duty_out[j]);
            $finish();
        end
    end
endtask

initial begin
    start = 0;
    m = 0;
    duty = '{DEPTH{0}};
    sim_helper_random.init();
    @(posedge locked);

    for(int i = 0; i < 5000; i++) begin
        $display("check %d", i);
        set_random();
    end

    $display("OK! sim_modulation_multiplier");
    $finish();
end


endmodule
