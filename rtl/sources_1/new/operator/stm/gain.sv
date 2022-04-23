/*
 * File: gain.sv
 * Project: stm
 * Created Date: 13/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */

`timescale 1ns / 1ps
module stm_gain_operator#(
           parameter int WIDTH = 13,
           parameter int DEPTH = 249
       )(
           input var CLK,
           input var RST,
           input var [15:0] IDX,
           ss_bus_if.gain_port SS_BUS,
           input var [1:0] LOAD_MODE,
           input var [WIDTH-1:0] CYCLE[0:DEPTH-1],
           output var [WIDTH-1:0] DUTY[0:DEPTH-1],
           output var [WIDTH-1:0] PHASE[0:DEPTH-1],
           output var START,
           output var DONE
       );

`include "params.vh"

bit [WIDTH-1:0] duty[0:DEPTH-1];
bit [WIDTH-1:0] phase[0:DEPTH-1];
bit [WIDTH-1:0] duty_buf[0:DEPTH-1];
bit [WIDTH-1:0] phase_buf[0:DEPTH-1];

bit [15:0] idx;
bit [127:0] data_out;
bit [15:0] idx_old;
bit start;
bit done;

bit [9:0] gain_addr_base;
bit [5:0] gain_addr_offset;
bit [5:0] set_cnt;

enum bit [2:0] {
         IDLE,
         WAIT_0,
         WAIT_1,
         SET,
         BUF
     } state = IDLE;

for (genvar i = 0; i < DEPTH; i++) begin
    assign DUTY[i] = duty[i];
    assign PHASE[i] = phase[i];
end

assign idx = IDX;
assign SS_BUS.GAIN_ADDR = {gain_addr_base, gain_addr_offset};
assign data_out = SS_BUS.DATA_OUT;

assign START = start;
assign DONE = done;

always_ff @(posedge CLK) begin
    idx_old <= idx;
    start <= idx != idx_old;
end

always_ff @(posedge CLK) begin
    if (RST) begin
        duty <= '{DEPTH{0}};
        phase <= '{DEPTH{0}};
    end
    else begin
        case(state)
            IDLE: begin
                if (start) begin
                    done <= 0;
                    gain_addr_base <= idx[9:0];
                    gain_addr_offset <= 0;
                    state <= WAIT_0;
                end
            end
            WAIT_0: begin
                gain_addr_offset <= gain_addr_offset + 1;
                state <= WAIT_1;
            end
            WAIT_1: begin
                gain_addr_offset <= gain_addr_offset + 1;
                set_cnt <= 0;
                state <= SET;
            end
            SET: begin
                if (set_cnt < DEPTH[7:2]) begin
                    case(LOAD_MODE)
                        LOAD_LEGACY: begin
                            phase_buf[{set_cnt, 2'b00}] <= {data_out[7:0], 5'h00};
                            duty_buf[{set_cnt, 2'b00}] <= {2'b00, data_out[15:8], 3'h7} + 1;
                            phase_buf[{set_cnt, 2'b00}+1] <= {data_out[39:32], 5'h00};
                            duty_buf[{set_cnt, 2'b00}+1] <= {2'b00, data_out[47:40], 3'h7} + 1;
                            phase_buf[{set_cnt, 2'b00}+2] <= {data_out[71:64], 5'h00};
                            duty_buf[{set_cnt, 2'b00}+2] <= {2'b00, data_out[79:72], 3'h7} + 1;
                            phase_buf[{set_cnt, 2'b00}+3] <= {data_out[103:96], 5'h00};
                            duty_buf[{set_cnt, 2'b00}+3] <= {2'b00, data_out[111:104], 3'h7} + 1;
                        end
                        LOAD_RAW: begin
                            phase_buf[{set_cnt, 2'b00}] <= data_out[WIDTH-1:0];
                            duty_buf[{set_cnt, 2'b00}] <= data_out[WIDTH-1+16:16];
                            phase_buf[{set_cnt, 2'b00}+1] <= data_out[WIDTH-1+32:32];
                            duty_buf[{set_cnt, 2'b00}+1] <= data_out[WIDTH-1+48:48];
                            phase_buf[{set_cnt, 2'b00}+2] <= data_out[WIDTH-1+64:64];
                            duty_buf[{set_cnt, 2'b00}+2] <= data_out[WIDTH-1+80:80];
                            phase_buf[{set_cnt, 2'b00}+3] <= data_out[WIDTH-1+96:96];
                            duty_buf[{set_cnt, 2'b00}+3] <= data_out[WIDTH-1+112:112];
                        end
                        LOAD_DUTY_SHIFT_RAW_PHASE: begin
                            phase_buf[{set_cnt, 2'b00}] <= data_out[WIDTH-1:0];
                            duty_buf[{set_cnt, 2'b00}] <= CYCLE[{set_cnt, 2'b00}][WIDTH-1:1] >> data_out[19:16];
                            phase_buf[{set_cnt, 2'b00}+1] <= data_out[WIDTH-1+32:32];
                            duty_buf[{set_cnt, 2'b00}+1] <= CYCLE[{set_cnt, 2'b00}+1][WIDTH-1:1] >> data_out[51:48];
                            phase_buf[{set_cnt, 2'b00}+2] <= data_out[WIDTH-1+64:64];
                            duty_buf[{set_cnt, 2'b00}+2] <= CYCLE[{set_cnt, 2'b00}+2][WIDTH-1:1] >> data_out[83:80];
                            phase_buf[{set_cnt, 2'b00}+3] <= data_out[WIDTH-1+96:96];
                            duty_buf[{set_cnt, 2'b00}+3] <= CYCLE[{set_cnt, 2'b00}+3][WIDTH-1:1] >> data_out[115:112];
                        end
                    endcase
                    gain_addr_offset <= gain_addr_offset + 1;
                    set_cnt <= set_cnt + 1;
                end
                else begin
                    case(LOAD_MODE)
                        LOAD_LEGACY: begin
                            phase_buf[{set_cnt, 2'b00}] <= {data_out[7:0], 5'h00};
                            duty_buf[{set_cnt, 2'b00}] <= {2'b00, data_out[15:8], 3'h7} + 1;
                        end
                        LOAD_RAW: begin
                            phase_buf[{set_cnt, 2'b00}] <= data_out[WIDTH-1:0];
                            duty_buf[{set_cnt, 2'b00}] <= data_out[WIDTH-1+16:16];
                        end
                        LOAD_DUTY_SHIFT_RAW_PHASE: begin
                            phase_buf[{set_cnt, 2'b00}] <= data_out[WIDTH-1:0];
                            duty_buf[{set_cnt, 2'b00}] <= CYCLE[{set_cnt, 2'b00}][WIDTH-1:1] >> data_out[19:16];
                        end
                    endcase
                    state <= BUF;
                end
            end
            BUF: begin
                phase <= phase_buf;
                duty <= duty_buf;
                done <= 1;
                state <= IDLE;
            end
        endcase
    end
end

endmodule
