/*
 * File: preconditioner.sv
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


`timescale 1ns / 1ps
module pwm_preconditioner#(
           parameter int WIDTH = 13,
           parameter int DEPTH = 249
       )(
           input var CLK,
           input var [WIDTH-1:0] CYCLE[0:DEPTH-1],
           input var [WIDTH-1:0] DUTY[0:DEPTH-1],
           input var [WIDTH-1:0] PHASE[0:DEPTH-1],
           output var [WIDTH-1:0] RISE[0:DEPTH-1],
           output var [WIDTH-1:0] FALL[0:DEPTH-1],
           output var DONE
       );

localparam int ADDSUB_LATENCY = 2;

bit signed [WIDTH+1:0] cycle[0:DEPTH-1];
bit signed [WIDTH+1:0] duty[0:DEPTH-1];
bit signed [WIDTH+1:0] phase[0:DEPTH-1];
bit [WIDTH-1:0] rise_buf[0:DEPTH-1], fall_buf[0:DEPTH-1];

bit signed [WIDTH+1:0] a_phase, b_phase, s_phase;
bit signed [WIDTH+1:0] a_duty_r, b_duty_r, s_duty_r;
bit signed [WIDTH+1:0] a_rise, b_rise, s_rise;
bit signed [WIDTH+1:0] a_fall, b_fall, s_fall;
bit signed [WIDTH+1:0] a_fold_rise, b_fold_rise, s_fold_rise;
bit fold_rise_addsub;
bit signed [WIDTH+1:0] a_fold_fall, b_fold_fall, s_fold_fall;

bit [$clog2(DEPTH+(ADDSUB_LATENCY+1)*3)-1:0] cnt, lr_cnt, fold_cnt, set_cnt;

bit done = 0;

assign DONE = done;

for (genvar i = 0; i < DEPTH; i++) begin
    assign RISE[i] = rise_buf[i];
    assign FALL[i] = fall_buf[i];
end

addsub #(
           .WIDTH(WIDTH+2)
       ) sub_phase(
           .CLK(CLK),
           .A(a_phase),
           .B(b_phase),
           .ADD(1'b0),
           .S(s_phase)
       );
addsub #(
           .WIDTH(WIDTH+2)
       ) add_duty_r(
           .CLK(CLK),
           .A(a_duty_r),
           .B(b_duty_r),
           .ADD(1'b1),
           .S(s_duty_r)
       );

addsub #(
           .WIDTH(WIDTH+2)
       ) sub_rise(
           .CLK(CLK),
           .A(a_rise),
           .B(b_rise),
           .ADD(1'b0),
           .S(s_rise)
       );
addsub #(
           .WIDTH(WIDTH+2)
       ) add_fall(
           .CLK(CLK),
           .A(a_fall),
           .B(b_fall),
           .ADD(1'b1),
           .S(s_fall)
       );

addsub #(
           .WIDTH(WIDTH+2)
       ) add_fold_rise(
           .CLK(CLK),
           .A(a_fold_rise),
           .B(b_fold_rise),
           .ADD(fold_rise_addsub),
           .S(s_fold_rise)
       );
addsub #(
           .WIDTH(WIDTH+2)
       ) sub_fold_fall(
           .CLK(CLK),
           .A(a_fold_fall),
           .B(b_fold_fall),
           .ADD(1'b0),
           .S(s_fold_fall)
       );

for (genvar i = 0; i < DEPTH; i++) begin
    always_ff @(posedge CLK) begin
        if (set_cnt == DEPTH - 1) begin
            cycle[i] <= {2'b00, CYCLE[i]};
            duty[i] <= {2'b00, DUTY[i]};
            phase[i] <= {2'b00, PHASE[i]};
        end
    end
end

always_ff @(posedge CLK) begin
    if (set_cnt == DEPTH - 1) begin
        rise_buf[set_cnt] <= s_fold_rise[WIDTH-1:0];
        fall_buf[set_cnt] <= s_fold_fall[WIDTH-1:0];

        cnt <= 0;
        lr_cnt <= 0;
        fold_cnt <= 0;
        set_cnt <= 0;

        done <= 1;
    end
    else begin
        // calc (cycle-phase), calc duty_r
        a_phase <= cycle[cnt];
        b_phase <= phase[cnt];
        a_duty_r <= {1'b0, duty[cnt][WIDTH+1:1]};
        b_duty_r <= duty[cnt][0];
        cnt <= cnt + 1;

        // calc rise/fall
        a_rise <= s_phase;
        b_rise <= {1'b0, duty[lr_cnt][WIDTH+1:1]};
        a_fall <= s_phase;
        b_fall <= s_duty_r;
        if (cnt > ADDSUB_LATENCY) begin
            lr_cnt <= lr_cnt + 1;
        end

        // make rise/fall be in [0, cycle-1]
        a_fold_rise <= s_rise;
        if (s_rise[WIDTH+1] == 1'b1) begin
            b_fold_rise <= cycle[fold_cnt];
            fold_rise_addsub <= 1'b1;
        end
        else if (cycle[fold_cnt] <= s_rise) begin
            b_fold_rise <= cycle[fold_cnt];
            fold_rise_addsub <= 1'b0;
        end
        else begin
            b_fold_rise <= 0;
            fold_rise_addsub <= 1'b1;
        end
        a_fold_fall <= s_fall;
        if (cycle[fold_cnt] <= s_fall) begin
            b_fold_fall <= cycle[fold_cnt];
        end
        else begin
            b_fold_fall <= 0;
        end
        if (lr_cnt > ADDSUB_LATENCY) begin
            fold_cnt <= fold_cnt + 1;
        end

        if (fold_cnt > ADDSUB_LATENCY) begin
            rise_buf[set_cnt] <= s_fold_rise[WIDTH-1:0];
            fall_buf[set_cnt] <= s_fold_fall[WIDTH-1:0];

            set_cnt <= set_cnt + 1;
        end

        done <= 0;
    end
end

endmodule
