/*
 * File: generator.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */


`timescale 1ns / 1ps
module pwm_generator#(
           parameter int WIDTH = 13
       )(
           input var CLK,
           input var [WIDTH-1:0] TIME_CNT,
           input var [WIDTH-1:0] RISE,
           input var [WIDTH-1:0] FALL,
           output var PWM_OUT
       );

bit [WIDTH-1:0] t;
bit [WIDTH-1:0] R;
bit [WIDTH-1:0] F;
bit v;

assign t = TIME_CNT;
assign R = RISE;
assign F = FALL;
// assign PWM_OUT = (R <= F) ? ((R <= t) & (t < F)) : ((t < F) | (R <= t));
assign PWM_OUT = v;

always_ff @(posedge CLK) begin
    if (R <= F) begin
        v <= (R <= t) & (t < F);
    end
    else begin
        v <= (t < F) | (R <= t);
    end
end

endmodule
