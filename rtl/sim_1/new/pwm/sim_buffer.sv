/*
 * File: sim_buffer.sv
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



module sim_pwm_buffer();

bit CLK_163P84M;
bit locked;
sim_helper_clk sim_helper_clk(
                   .CLK_163P84M(CLK_163P84M),
                   .CLK_20P48M(),
                   .LOCKED(locked),
                   .SYS_TIME()
               );

localparam int WIDTH = 13;

bit [WIDTH-1:0] cycle;
bit [WIDTH-1:0] time_cnt;
bit [WIDTH-1:0] rise_in;
bit [WIDTH-1:0] fall_in;

bit [WIDTH-1:0] rise;
bit [WIDTH-1:0] fall;

pwm_buffer #(
               .WIDTH(WIDTH)
           ) pwm_buffer(
               .CLK(CLK_163P84M),
               .CYCLE(cycle),
               .TIME_CNT(time_cnt),
               .RISE_IN(rise_in),
               .FALL_IN(fall_in),
               .RISE_OUT(rise),
               .FALL_OUT(fall)
           );


initial begin
    cycle = 4096;
    time_cnt = 0;
    rise_in = 0;
    fall_in = 0;
    @(posedge locked);

    rise_in = 100;
    fall_in = 200;

    while(1) begin
        @(posedge CLK_163P84M);
        $display("check @%d", time_cnt);
        if (time_cnt != 0) begin
            if (rise != 0 || fall != 0) begin
                $display("Failed!");
                $finish();
            end
        end
        else begin
            if (rise != rise_in || fall != fall_in) begin
                $display("Failed!");
                $finish();
            end
            break;
        end
    end

    $display("OK!");
    $finish();
end

always @(posedge CLK_163P84M) begin
    time_cnt = (time_cnt == cycle - 1) ? 0 : time_cnt + 1;
end

endmodule
