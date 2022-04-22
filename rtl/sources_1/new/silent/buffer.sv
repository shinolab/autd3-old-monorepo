/*
 * File: buffer.sv
 * Project: silent
 * Created Date: 22/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/03/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */


`timescale 1ns / 1ps
module silent_buffer#(
           parameter int WIDTH = 13,
           parameter int DEPTH = 249
       )(
           input var CLK,
           input var DONE,
           input var [WIDTH-1:0] DUTY_IN[0:DEPTH-1],
           input var [WIDTH-1:0] PHASE_IN[0:DEPTH-1],
           output var [WIDTH-1:0] DUTY_OUT[0:DEPTH-1],
           output var [WIDTH-1:0] PHASE_OUT[0:DEPTH-1]
       );

bit [WIDTH-1:0] duty[0:DEPTH-1];
bit [WIDTH-1:0] phase[0:DEPTH-1];

for (genvar i = 0; i < DEPTH; i++) begin
    assign DUTY_OUT[i] = duty[i];
    assign PHASE_OUT[i] = phase[i];

    always_ff @(posedge CLK) begin
        if (DONE) begin
            duty[i] <= DUTY_IN[i];
            phase[i] <= PHASE_IN[i];
        end
    end
end

endmodule
