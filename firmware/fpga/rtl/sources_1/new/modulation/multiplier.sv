/*
 * File: multiplier.sv
 * Project: modulation
 * Created Date: 24/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

`timescale 1ns / 1ps
module modulation_multiplier #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK,
    input var START,
    input var [7:0] M,
    input var [WIDTH-1:0] DUTY_IN[0:DEPTH-1],
    output var [WIDTH-1:0] DUTY_OUT[0:DEPTH-1],
    output var DONE
);

  localparam int MULT_LATENCY = 3;
  localparam int DIV_LATENCY = 34;
  localparam int LATENCY = MULT_LATENCY + DIV_LATENCY;

  bit [WIDTH-1:0] duty[0:DEPTH-1];
  bit [WIDTH-1:0] duty_m[0:DEPTH-1];

  bit signed [WIDTH:0] a;
  bit signed [8:0] b;
  bit signed [WIDTH+9:0] p;

  bit [31:0] quo;
  bit [7:0] _unused;

  bit [$clog2(DEPTH+(LATENCY+1))-1:0] calc_cnt, set_cnt;
  bit done = 0;

  assign DONE = done;
  for (genvar i = 0; i < DEPTH; i++) begin
    assign DUTY_OUT[i] = duty_m[i];
  end

  mult #(
      .WIDTH_A(WIDTH + 1),
      .WIDTH_B(9)
  ) mult (
      .CLK(CLK),
      .A  (a),
      .B  (b),
      .P  (p)
  );

  div_32_8 div_32_8 (
      .s_axis_dividend_tdata({10'd0, p[WIDTH+8:0]}),
      .s_axis_dividend_tvalid(1'b1),
      .s_axis_divisor_tdata(8'hFF),
      .s_axis_divisor_tvalid(1'b1),
      .aclk(CLK),
      .m_axis_dout_tdata({quo, _unused}),
      .m_axis_dout_tvalid()
  );

  enum bit {
    IDLE,
    PROCESS
  } state = IDLE;

  for (genvar i = 0; i < DEPTH; i++) begin
    always_ff @(posedge CLK) begin
      case (state)
        IDLE: begin
          if (START) begin
            duty[i] <= DUTY_IN[i];
          end
        end
      endcase
    end
  end

  always_ff @(posedge CLK) begin
    case (state)
      IDLE: begin
        if (START) begin
          calc_cnt <= 0;
          set_cnt <= 0;
          b <= M;
          state <= PROCESS;
        end
        done <= 0;
      end
      PROCESS: begin
        b <= M;
        a <= {1'b0, duty[calc_cnt]};
        calc_cnt <= calc_cnt + 1;

        if (calc_cnt > LATENCY) begin
          duty_m[set_cnt] <= quo[WIDTH-1:0];
          set_cnt <= set_cnt + 1;
          if (set_cnt == DEPTH - 1) begin
            done  <= 1;
            state <= IDLE;
          end
        end
      end
    endcase
  end

endmodule
