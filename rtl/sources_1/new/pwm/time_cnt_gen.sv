/*
 * File: time_cnt_gen.sv
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

`timescale 1ns / 1ps
module time_cnt_generator#(
           parameter int WIDTH = 13,
           parameter int DEPTH = 249
       )(
           input var CLK,
           input var [63:0] SYS_TIME,
           input var [WIDTH-1:0] CYCLE[0:DEPTH-1],
           output var [WIDTH-1:0] TIME_CNT[0:DEPTH-1]
       );

localparam int DIV_LATENCY = 66 + 1;

bit [63:0] divined;
bit [15:0] divisor;
bit [63:0] _unused;
bit [15:0] rem;

bit [WIDTH-1:0] t[0:DEPTH-1] = '{DEPTH{0}};

bit [$clog2(DEPTH)-1:0] sync_cnt = DIV_LATENCY % DEPTH;
bit [$clog2(DEPTH)-1:0] set_cnt = 0;

bit [WIDTH-1:0] cycle_m1[0:DEPTH];
bit [WIDTH-1:0] cycle_m2[0:DEPTH];

div_64_16 div_64_16(
              .s_axis_dividend_tdata(divined),
              .s_axis_dividend_tvalid(1'b1),
              .s_axis_divisor_tdata(divisor),
              .s_axis_divisor_tvalid(1'b1),
              .aclk(CLK),
              .m_axis_dout_tdata({_unused, rem}),
              .m_axis_dout_tvalid()
          );

for (genvar i = 0; i < DEPTH; i++) begin
    always_ff @(posedge CLK) begin
        if (i == set_cnt) begin
            t[i] <= (t[i] == cycle_m2[i]) && (rem[WIDTH-1:0] == 0) ? t[i] + 1 : rem[WIDTH-1:0]; // make sure t be T-1
        end
        else begin
            t[i] <= (t[i] == cycle_m1[i]) ? 0 : t[i] + 1;
        end
    end
    assign TIME_CNT[i] = t[i];
end

always_ff @(posedge CLK) begin
    divined <= SYS_TIME[63:0];
    divisor <= CYCLE[sync_cnt];

    sync_cnt <= (sync_cnt == DEPTH - 1) ? 0 : sync_cnt + 1;
    set_cnt <= (set_cnt == DEPTH - 1) ? 0 : set_cnt + 1;
end

for (genvar i = 0; i < DEPTH; i++) begin
    always_ff @(posedge CLK) begin
        cycle_m1[i] <= CYCLE[i] - 1;
        cycle_m2[i] <= cycle_m1[i] - 1;
    end
end

endmodule
