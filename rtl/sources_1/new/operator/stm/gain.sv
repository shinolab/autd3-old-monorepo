/*
 * File: gain.sv
 * Project: stm
 * Created Date: 13/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/09/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

`timescale 1ns / 1ps
module stm_gain_operator #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK,
    input var [15:0] IDX,
    ss_bus_if.gain_port SS_BUS,
    input var LEGACY_MODE,
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
    assign DUTY[i]  = duty[i];
    assign PHASE[i] = phase[i];
  end

  assign idx = IDX;
  assign SS_BUS.GAIN_ADDR = {gain_addr_base, gain_addr_offset};
  assign data_out = SS_BUS.DATA_OUT;

  assign START = start;
  assign DONE = done;

  always_ff @(posedge CLK) begin
    idx_old <= idx;
    start   <= idx != idx_old;
  end

  always_ff @(posedge CLK) begin
    if (LEGACY_MODE) begin
      case (state)
        IDLE: begin
          if (start) begin
            done <= 0;
            gain_addr_base <= idx[10:1];
            gain_addr_offset <= idx[0] ? 6'h20 : 0;
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
          // if (set_cnt < DEPTH[7:3]) begin
          //   phase_buf[{set_cnt, 3'h0}] <= {1'b0, data_out[7:0], 4'h00};
          //   duty_buf[{set_cnt, 3'h0}] <= {2'b00, data_out[15:8], 3'h7} + 1;
          //   phase_buf[{set_cnt, 3'h1}] <= {1'b0, data_out[23:16], 4'h00};
          //   duty_buf[{set_cnt, 3'h1}] <= {2'b00, data_out[31:24], 3'h7} + 1;
          //   phase_buf[{set_cnt, 3'h2}] <= {1'b0, data_out[39:32], 4'h00};
          //   duty_buf[{set_cnt, 3'h2}] <= {2'b00, data_out[47:40], 3'h7} + 1;
          //   phase_buf[{set_cnt, 3'h3}] <= {1'b0, data_out[55:48], 4'h00};
          //   duty_buf[{set_cnt, 3'h3}] <= {2'b00, data_out[63:56], 3'h7} + 1;
          //   phase_buf[{set_cnt, 3'h4}] <= {1'b0, data_out[71:64], 4'h00};
          //   duty_buf[{set_cnt, 3'h4}] <= {2'b00, data_out[79:72], 3'h7} + 1;
          //   phase_buf[{set_cnt, 3'h5}] <= {1'b0, data_out[87:80], 4'h00};
          //   duty_buf[{set_cnt, 3'h5}] <= {2'b00, data_out[95:88], 3'h7} + 1;
          //   phase_buf[{set_cnt, 3'h6}] <= {1'b0, data_out[103:96], 4'h00};
          //   duty_buf[{set_cnt, 3'h6}] <= {2'b00, data_out[111:104], 3'h7} + 1;
          //   phase_buf[{set_cnt, 3'h7}] <= {1'b0, data_out[119:112], 4'h00};
          //   duty_buf[{set_cnt, 3'h7}] <= {2'b00, data_out[127:120], 3'h7} + 1;
          //   gain_addr_offset <= gain_addr_offset + 1;
          //   set_cnt <= set_cnt + 1;
          // end else begin
          //   phase_buf[{set_cnt, 3'h0}] <= {1'b0, data_out[7:0], 4'h00};
          //   duty_buf[{set_cnt, 3'h0}] <= {2'b00, data_out[15:8], 3'h7} + 1;
          //   state <= BUF;
          // end
          // unroll to meet timing
          case (set_cnt)
            6'd0: begin
              phase_buf[0] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[0] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[1] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[1] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[2] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[2] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[3] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[3] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[4] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[4] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[5] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[5] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[6] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[6] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[7] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[7] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd1: begin
              phase_buf[8] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[8] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[9] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[9] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[10] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[10] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[11] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[11] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[12] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[12] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[13] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[13] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[14] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[14] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[15] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[15] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd2: begin
              phase_buf[16] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[16] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[17] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[17] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[18] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[18] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[19] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[19] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[20] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[20] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[21] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[21] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[22] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[22] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[23] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[23] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd3: begin
              phase_buf[24] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[24] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[25] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[25] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[26] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[26] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[27] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[27] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[28] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[28] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[29] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[29] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[30] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[30] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[31] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[31] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd4: begin
              phase_buf[32] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[32] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[33] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[33] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[34] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[34] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[35] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[35] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[36] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[36] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[37] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[37] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[38] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[38] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[39] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[39] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd5: begin
              phase_buf[40] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[40] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[41] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[41] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[42] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[42] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[43] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[43] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[44] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[44] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[45] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[45] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[46] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[46] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[47] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[47] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd6: begin
              phase_buf[48] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[48] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[49] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[49] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[50] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[50] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[51] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[51] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[52] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[52] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[53] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[53] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[54] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[54] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[55] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[55] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd7: begin
              phase_buf[56] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[56] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[57] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[57] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[58] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[58] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[59] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[59] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[60] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[60] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[61] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[61] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[62] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[62] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[63] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[63] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd8: begin
              phase_buf[64] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[64] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[65] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[65] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[66] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[66] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[67] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[67] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[68] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[68] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[69] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[69] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[70] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[70] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[71] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[71] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd9: begin
              phase_buf[72] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[72] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[73] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[73] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[74] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[74] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[75] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[75] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[76] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[76] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[77] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[77] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[78] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[78] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[79] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[79] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd10: begin
              phase_buf[80] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[80] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[81] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[81] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[82] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[82] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[83] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[83] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[84] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[84] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[85] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[85] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[86] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[86] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[87] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[87] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd11: begin
              phase_buf[88] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[88] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[89] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[89] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[90] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[90] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[91] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[91] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[92] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[92] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[93] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[93] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[94] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[94] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[95] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[95] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd12: begin
              phase_buf[96] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[96] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[97] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[97] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[98] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[98] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[99] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[99] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[100] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[100] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[101] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[101] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[102] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[102] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[103] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[103] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd13: begin
              phase_buf[104] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[104] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[105] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[105] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[106] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[106] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[107] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[107] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[108] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[108] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[109] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[109] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[110] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[110] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[111] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[111] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd14: begin
              phase_buf[112] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[112] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[113] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[113] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[114] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[114] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[115] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[115] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[116] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[116] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[117] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[117] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[118] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[118] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[119] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[119] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd15: begin
              phase_buf[120] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[120] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[121] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[121] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[122] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[122] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[123] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[123] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[124] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[124] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[125] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[125] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[126] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[126] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[127] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[127] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd16: begin
              phase_buf[128] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[128] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[129] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[129] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[130] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[130] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[131] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[131] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[132] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[132] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[133] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[133] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[134] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[134] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[135] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[135] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd17: begin
              phase_buf[136] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[136] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[137] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[137] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[138] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[138] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[139] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[139] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[140] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[140] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[141] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[141] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[142] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[142] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[143] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[143] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd18: begin
              phase_buf[144] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[144] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[145] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[145] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[146] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[146] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[147] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[147] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[148] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[148] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[149] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[149] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[150] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[150] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[151] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[151] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd19: begin
              phase_buf[152] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[152] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[153] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[153] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[154] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[154] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[155] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[155] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[156] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[156] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[157] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[157] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[158] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[158] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[159] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[159] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd20: begin
              phase_buf[160] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[160] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[161] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[161] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[162] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[162] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[163] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[163] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[164] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[164] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[165] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[165] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[166] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[166] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[167] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[167] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd21: begin
              phase_buf[168] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[168] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[169] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[169] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[170] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[170] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[171] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[171] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[172] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[172] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[173] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[173] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[174] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[174] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[175] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[175] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd22: begin
              phase_buf[176] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[176] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[177] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[177] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[178] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[178] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[179] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[179] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[180] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[180] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[181] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[181] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[182] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[182] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[183] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[183] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd23: begin
              phase_buf[184] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[184] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[185] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[185] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[186] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[186] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[187] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[187] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[188] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[188] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[189] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[189] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[190] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[190] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[191] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[191] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd24: begin
              phase_buf[192] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[192] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[193] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[193] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[194] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[194] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[195] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[195] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[196] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[196] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[197] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[197] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[198] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[198] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[199] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[199] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd25: begin
              phase_buf[200] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[200] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[201] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[201] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[202] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[202] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[203] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[203] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[204] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[204] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[205] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[205] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[206] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[206] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[207] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[207] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd26: begin
              phase_buf[208] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[208] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[209] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[209] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[210] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[210] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[211] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[211] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[212] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[212] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[213] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[213] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[214] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[214] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[215] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[215] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd27: begin
              phase_buf[216] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[216] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[217] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[217] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[218] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[218] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[219] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[219] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[220] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[220] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[221] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[221] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[222] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[222] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[223] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[223] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd28: begin
              phase_buf[224] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[224] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[225] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[225] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[226] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[226] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[227] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[227] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[228] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[228] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[229] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[229] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[230] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[230] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[231] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[231] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd29: begin
              phase_buf[232] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[232] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[233] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[233] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[234] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[234] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[235] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[235] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[236] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[236] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[237] <= {1'b0, data_out[87:80], 4'h00};
              duty_buf[237] <= {2'b00, data_out[95:88], 3'h7} + 1;
              phase_buf[238] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[238] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[239] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[239] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd30: begin
              phase_buf[240] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[240] <= {2'b00, data_out[15:8], 3'h7} + 1;
              phase_buf[241] <= {1'b0, data_out[23:16], 4'h00};
              duty_buf[241] <= {2'b00, data_out[31:24], 3'h7} + 1;
              phase_buf[242] <= {1'b0, data_out[39:32], 4'h00};
              duty_buf[242] <= {2'b00, data_out[47:40], 3'h7} + 1;
              phase_buf[243] <= {1'b0, data_out[55:48], 4'h00};
              duty_buf[243] <= {2'b00, data_out[63:56], 3'h7} + 1;
              phase_buf[244] <= {1'b0, data_out[71:64], 4'h00};
              duty_buf[244] <= {2'b00, data_out[79:72], 3'h7} + 1;
              phase_buf[245] <= {1'b0, data_out[87:80], 4'h00};
              phase_buf[246] <= {1'b0, data_out[103:96], 4'h00};
              duty_buf[246] <= {2'b00, data_out[111:104], 3'h7} + 1;
              phase_buf[247] <= {1'b0, data_out[119:112], 4'h00};
              duty_buf[247] <= {2'b00, data_out[127:120], 3'h7} + 1;
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd31: begin
              phase_buf[248] <= {1'b0, data_out[7:0], 4'h00};
              duty_buf[248] <= {2'b00, data_out[15:8], 3'h7} + 1;
              state <= BUF;
            end
          endcase
        end
        BUF: begin
          phase <= phase_buf;
          duty  <= duty_buf;
          done  <= 1;
          state <= IDLE;
        end
      endcase
    end else begin
      case (state)
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
          // if (set_cnt < DEPTH[7:2]) begin
          //   phase_buf[{set_cnt, 2'h0}] <= data_out[0+WIDTH-1:0];
          //   duty_buf[{set_cnt, 2'h0}] <= data_out[16+WIDTH-1:16];
          //   phase_buf[{set_cnt, 2'h1}] <= data_out[32+WIDTH-1:32];
          //   duty_buf[{set_cnt, 2'h1}] <= data_out[48+WIDTH-1:48];
          //   phase_buf[{set_cnt, 2'h2}] <= data_out[64+WIDTH-1:64];
          //   duty_buf[{set_cnt, 2'h2}] <= data_out[80+WIDTH-1:80];
          //   phase_buf[{set_cnt, 2'h3}] <= data_out[96+WIDTH-1:96];
          //   duty_buf[{set_cnt, 2'h3}] <= data_out[112+WIDTH-1:112];
          //   gain_addr_offset <= gain_addr_offset + 1;
          //   set_cnt <= set_cnt + 1;
          // end else begin
          //   phase_buf[{set_cnt, 2'h0}] <= data_out[0+WIDTH-1:0];
          //   duty_buf[{set_cnt, 2'h0}] <= data_out[0+WIDTH-1+16:0+16];
          //   state <= BUF;
          // end
          // unroll to meet timing
          case (set_cnt)
            6'd0: begin
              phase_buf[0] <= data_out[0+WIDTH-1:0];
              duty_buf[0] <= data_out[16+WIDTH-1:16];
              phase_buf[1] <= data_out[32+WIDTH-1:32];
              duty_buf[1] <= data_out[48+WIDTH-1:48];
              phase_buf[2] <= data_out[64+WIDTH-1:64];
              duty_buf[2] <= data_out[80+WIDTH-1:80];
              phase_buf[3] <= data_out[96+WIDTH-1:96];
              duty_buf[3] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd1: begin
              phase_buf[4] <= data_out[0+WIDTH-1:0];
              duty_buf[4] <= data_out[16+WIDTH-1:16];
              phase_buf[5] <= data_out[32+WIDTH-1:32];
              duty_buf[5] <= data_out[48+WIDTH-1:48];
              phase_buf[6] <= data_out[64+WIDTH-1:64];
              duty_buf[6] <= data_out[80+WIDTH-1:80];
              phase_buf[7] <= data_out[96+WIDTH-1:96];
              duty_buf[7] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd2: begin
              phase_buf[8] <= data_out[0+WIDTH-1:0];
              duty_buf[8] <= data_out[16+WIDTH-1:16];
              phase_buf[9] <= data_out[32+WIDTH-1:32];
              duty_buf[9] <= data_out[48+WIDTH-1:48];
              phase_buf[10] <= data_out[64+WIDTH-1:64];
              duty_buf[10] <= data_out[80+WIDTH-1:80];
              phase_buf[11] <= data_out[96+WIDTH-1:96];
              duty_buf[11] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd3: begin
              phase_buf[12] <= data_out[0+WIDTH-1:0];
              duty_buf[12] <= data_out[16+WIDTH-1:16];
              phase_buf[13] <= data_out[32+WIDTH-1:32];
              duty_buf[13] <= data_out[48+WIDTH-1:48];
              phase_buf[14] <= data_out[64+WIDTH-1:64];
              duty_buf[14] <= data_out[80+WIDTH-1:80];
              phase_buf[15] <= data_out[96+WIDTH-1:96];
              duty_buf[15] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd4: begin
              phase_buf[16] <= data_out[0+WIDTH-1:0];
              duty_buf[16] <= data_out[16+WIDTH-1:16];
              phase_buf[17] <= data_out[32+WIDTH-1:32];
              duty_buf[17] <= data_out[48+WIDTH-1:48];
              phase_buf[18] <= data_out[64+WIDTH-1:64];
              duty_buf[18] <= data_out[80+WIDTH-1:80];
              phase_buf[19] <= data_out[96+WIDTH-1:96];
              duty_buf[19] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd5: begin
              phase_buf[20] <= data_out[0+WIDTH-1:0];
              duty_buf[20] <= data_out[16+WIDTH-1:16];
              phase_buf[21] <= data_out[32+WIDTH-1:32];
              duty_buf[21] <= data_out[48+WIDTH-1:48];
              phase_buf[22] <= data_out[64+WIDTH-1:64];
              duty_buf[22] <= data_out[80+WIDTH-1:80];
              phase_buf[23] <= data_out[96+WIDTH-1:96];
              duty_buf[23] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd6: begin
              phase_buf[24] <= data_out[0+WIDTH-1:0];
              duty_buf[24] <= data_out[16+WIDTH-1:16];
              phase_buf[25] <= data_out[32+WIDTH-1:32];
              duty_buf[25] <= data_out[48+WIDTH-1:48];
              phase_buf[26] <= data_out[64+WIDTH-1:64];
              duty_buf[26] <= data_out[80+WIDTH-1:80];
              phase_buf[27] <= data_out[96+WIDTH-1:96];
              duty_buf[27] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd7: begin
              phase_buf[28] <= data_out[0+WIDTH-1:0];
              duty_buf[28] <= data_out[16+WIDTH-1:16];
              phase_buf[29] <= data_out[32+WIDTH-1:32];
              duty_buf[29] <= data_out[48+WIDTH-1:48];
              phase_buf[30] <= data_out[64+WIDTH-1:64];
              duty_buf[30] <= data_out[80+WIDTH-1:80];
              phase_buf[31] <= data_out[96+WIDTH-1:96];
              duty_buf[31] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd8: begin
              phase_buf[32] <= data_out[0+WIDTH-1:0];
              duty_buf[32] <= data_out[16+WIDTH-1:16];
              phase_buf[33] <= data_out[32+WIDTH-1:32];
              duty_buf[33] <= data_out[48+WIDTH-1:48];
              phase_buf[34] <= data_out[64+WIDTH-1:64];
              duty_buf[34] <= data_out[80+WIDTH-1:80];
              phase_buf[35] <= data_out[96+WIDTH-1:96];
              duty_buf[35] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd9: begin
              phase_buf[36] <= data_out[0+WIDTH-1:0];
              duty_buf[36] <= data_out[16+WIDTH-1:16];
              phase_buf[37] <= data_out[32+WIDTH-1:32];
              duty_buf[37] <= data_out[48+WIDTH-1:48];
              phase_buf[38] <= data_out[64+WIDTH-1:64];
              duty_buf[38] <= data_out[80+WIDTH-1:80];
              phase_buf[39] <= data_out[96+WIDTH-1:96];
              duty_buf[39] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd10: begin
              phase_buf[40] <= data_out[0+WIDTH-1:0];
              duty_buf[40] <= data_out[16+WIDTH-1:16];
              phase_buf[41] <= data_out[32+WIDTH-1:32];
              duty_buf[41] <= data_out[48+WIDTH-1:48];
              phase_buf[42] <= data_out[64+WIDTH-1:64];
              duty_buf[42] <= data_out[80+WIDTH-1:80];
              phase_buf[43] <= data_out[96+WIDTH-1:96];
              duty_buf[43] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd11: begin
              phase_buf[44] <= data_out[0+WIDTH-1:0];
              duty_buf[44] <= data_out[16+WIDTH-1:16];
              phase_buf[45] <= data_out[32+WIDTH-1:32];
              duty_buf[45] <= data_out[48+WIDTH-1:48];
              phase_buf[46] <= data_out[64+WIDTH-1:64];
              duty_buf[46] <= data_out[80+WIDTH-1:80];
              phase_buf[47] <= data_out[96+WIDTH-1:96];
              duty_buf[47] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd12: begin
              phase_buf[48] <= data_out[0+WIDTH-1:0];
              duty_buf[48] <= data_out[16+WIDTH-1:16];
              phase_buf[49] <= data_out[32+WIDTH-1:32];
              duty_buf[49] <= data_out[48+WIDTH-1:48];
              phase_buf[50] <= data_out[64+WIDTH-1:64];
              duty_buf[50] <= data_out[80+WIDTH-1:80];
              phase_buf[51] <= data_out[96+WIDTH-1:96];
              duty_buf[51] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd13: begin
              phase_buf[52] <= data_out[0+WIDTH-1:0];
              duty_buf[52] <= data_out[16+WIDTH-1:16];
              phase_buf[53] <= data_out[32+WIDTH-1:32];
              duty_buf[53] <= data_out[48+WIDTH-1:48];
              phase_buf[54] <= data_out[64+WIDTH-1:64];
              duty_buf[54] <= data_out[80+WIDTH-1:80];
              phase_buf[55] <= data_out[96+WIDTH-1:96];
              duty_buf[55] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd14: begin
              phase_buf[56] <= data_out[0+WIDTH-1:0];
              duty_buf[56] <= data_out[16+WIDTH-1:16];
              phase_buf[57] <= data_out[32+WIDTH-1:32];
              duty_buf[57] <= data_out[48+WIDTH-1:48];
              phase_buf[58] <= data_out[64+WIDTH-1:64];
              duty_buf[58] <= data_out[80+WIDTH-1:80];
              phase_buf[59] <= data_out[96+WIDTH-1:96];
              duty_buf[59] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd15: begin
              phase_buf[60] <= data_out[0+WIDTH-1:0];
              duty_buf[60] <= data_out[16+WIDTH-1:16];
              phase_buf[61] <= data_out[32+WIDTH-1:32];
              duty_buf[61] <= data_out[48+WIDTH-1:48];
              phase_buf[62] <= data_out[64+WIDTH-1:64];
              duty_buf[62] <= data_out[80+WIDTH-1:80];
              phase_buf[63] <= data_out[96+WIDTH-1:96];
              duty_buf[63] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd16: begin
              phase_buf[64] <= data_out[0+WIDTH-1:0];
              duty_buf[64] <= data_out[16+WIDTH-1:16];
              phase_buf[65] <= data_out[32+WIDTH-1:32];
              duty_buf[65] <= data_out[48+WIDTH-1:48];
              phase_buf[66] <= data_out[64+WIDTH-1:64];
              duty_buf[66] <= data_out[80+WIDTH-1:80];
              phase_buf[67] <= data_out[96+WIDTH-1:96];
              duty_buf[67] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd17: begin
              phase_buf[68] <= data_out[0+WIDTH-1:0];
              duty_buf[68] <= data_out[16+WIDTH-1:16];
              phase_buf[69] <= data_out[32+WIDTH-1:32];
              duty_buf[69] <= data_out[48+WIDTH-1:48];
              phase_buf[70] <= data_out[64+WIDTH-1:64];
              duty_buf[70] <= data_out[80+WIDTH-1:80];
              phase_buf[71] <= data_out[96+WIDTH-1:96];
              duty_buf[71] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd18: begin
              phase_buf[72] <= data_out[0+WIDTH-1:0];
              duty_buf[72] <= data_out[16+WIDTH-1:16];
              phase_buf[73] <= data_out[32+WIDTH-1:32];
              duty_buf[73] <= data_out[48+WIDTH-1:48];
              phase_buf[74] <= data_out[64+WIDTH-1:64];
              duty_buf[74] <= data_out[80+WIDTH-1:80];
              phase_buf[75] <= data_out[96+WIDTH-1:96];
              duty_buf[75] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd19: begin
              phase_buf[76] <= data_out[0+WIDTH-1:0];
              duty_buf[76] <= data_out[16+WIDTH-1:16];
              phase_buf[77] <= data_out[32+WIDTH-1:32];
              duty_buf[77] <= data_out[48+WIDTH-1:48];
              phase_buf[78] <= data_out[64+WIDTH-1:64];
              duty_buf[78] <= data_out[80+WIDTH-1:80];
              phase_buf[79] <= data_out[96+WIDTH-1:96];
              duty_buf[79] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd20: begin
              phase_buf[80] <= data_out[0+WIDTH-1:0];
              duty_buf[80] <= data_out[16+WIDTH-1:16];
              phase_buf[81] <= data_out[32+WIDTH-1:32];
              duty_buf[81] <= data_out[48+WIDTH-1:48];
              phase_buf[82] <= data_out[64+WIDTH-1:64];
              duty_buf[82] <= data_out[80+WIDTH-1:80];
              phase_buf[83] <= data_out[96+WIDTH-1:96];
              duty_buf[83] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd21: begin
              phase_buf[84] <= data_out[0+WIDTH-1:0];
              duty_buf[84] <= data_out[16+WIDTH-1:16];
              phase_buf[85] <= data_out[32+WIDTH-1:32];
              duty_buf[85] <= data_out[48+WIDTH-1:48];
              phase_buf[86] <= data_out[64+WIDTH-1:64];
              duty_buf[86] <= data_out[80+WIDTH-1:80];
              phase_buf[87] <= data_out[96+WIDTH-1:96];
              duty_buf[87] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd22: begin
              phase_buf[88] <= data_out[0+WIDTH-1:0];
              duty_buf[88] <= data_out[16+WIDTH-1:16];
              phase_buf[89] <= data_out[32+WIDTH-1:32];
              duty_buf[89] <= data_out[48+WIDTH-1:48];
              phase_buf[90] <= data_out[64+WIDTH-1:64];
              duty_buf[90] <= data_out[80+WIDTH-1:80];
              phase_buf[91] <= data_out[96+WIDTH-1:96];
              duty_buf[91] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd23: begin
              phase_buf[92] <= data_out[0+WIDTH-1:0];
              duty_buf[92] <= data_out[16+WIDTH-1:16];
              phase_buf[93] <= data_out[32+WIDTH-1:32];
              duty_buf[93] <= data_out[48+WIDTH-1:48];
              phase_buf[94] <= data_out[64+WIDTH-1:64];
              duty_buf[94] <= data_out[80+WIDTH-1:80];
              phase_buf[95] <= data_out[96+WIDTH-1:96];
              duty_buf[95] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd24: begin
              phase_buf[96] <= data_out[0+WIDTH-1:0];
              duty_buf[96] <= data_out[16+WIDTH-1:16];
              phase_buf[97] <= data_out[32+WIDTH-1:32];
              duty_buf[97] <= data_out[48+WIDTH-1:48];
              phase_buf[98] <= data_out[64+WIDTH-1:64];
              duty_buf[98] <= data_out[80+WIDTH-1:80];
              phase_buf[99] <= data_out[96+WIDTH-1:96];
              duty_buf[99] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd25: begin
              phase_buf[100] <= data_out[0+WIDTH-1:0];
              duty_buf[100] <= data_out[16+WIDTH-1:16];
              phase_buf[101] <= data_out[32+WIDTH-1:32];
              duty_buf[101] <= data_out[48+WIDTH-1:48];
              phase_buf[102] <= data_out[64+WIDTH-1:64];
              duty_buf[102] <= data_out[80+WIDTH-1:80];
              phase_buf[103] <= data_out[96+WIDTH-1:96];
              duty_buf[103] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd26: begin
              phase_buf[104] <= data_out[0+WIDTH-1:0];
              duty_buf[104] <= data_out[16+WIDTH-1:16];
              phase_buf[105] <= data_out[32+WIDTH-1:32];
              duty_buf[105] <= data_out[48+WIDTH-1:48];
              phase_buf[106] <= data_out[64+WIDTH-1:64];
              duty_buf[106] <= data_out[80+WIDTH-1:80];
              phase_buf[107] <= data_out[96+WIDTH-1:96];
              duty_buf[107] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd27: begin
              phase_buf[108] <= data_out[0+WIDTH-1:0];
              duty_buf[108] <= data_out[16+WIDTH-1:16];
              phase_buf[109] <= data_out[32+WIDTH-1:32];
              duty_buf[109] <= data_out[48+WIDTH-1:48];
              phase_buf[110] <= data_out[64+WIDTH-1:64];
              duty_buf[110] <= data_out[80+WIDTH-1:80];
              phase_buf[111] <= data_out[96+WIDTH-1:96];
              duty_buf[111] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd28: begin
              phase_buf[112] <= data_out[0+WIDTH-1:0];
              duty_buf[112] <= data_out[16+WIDTH-1:16];
              phase_buf[113] <= data_out[32+WIDTH-1:32];
              duty_buf[113] <= data_out[48+WIDTH-1:48];
              phase_buf[114] <= data_out[64+WIDTH-1:64];
              duty_buf[114] <= data_out[80+WIDTH-1:80];
              phase_buf[115] <= data_out[96+WIDTH-1:96];
              duty_buf[115] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd29: begin
              phase_buf[116] <= data_out[0+WIDTH-1:0];
              duty_buf[116] <= data_out[16+WIDTH-1:16];
              phase_buf[117] <= data_out[32+WIDTH-1:32];
              duty_buf[117] <= data_out[48+WIDTH-1:48];
              phase_buf[118] <= data_out[64+WIDTH-1:64];
              duty_buf[118] <= data_out[80+WIDTH-1:80];
              phase_buf[119] <= data_out[96+WIDTH-1:96];
              duty_buf[119] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd30: begin
              phase_buf[120] <= data_out[0+WIDTH-1:0];
              duty_buf[120] <= data_out[16+WIDTH-1:16];
              phase_buf[121] <= data_out[32+WIDTH-1:32];
              duty_buf[121] <= data_out[48+WIDTH-1:48];
              phase_buf[122] <= data_out[64+WIDTH-1:64];
              duty_buf[122] <= data_out[80+WIDTH-1:80];
              phase_buf[123] <= data_out[96+WIDTH-1:96];
              duty_buf[123] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd31: begin
              phase_buf[124] <= data_out[0+WIDTH-1:0];
              duty_buf[124] <= data_out[16+WIDTH-1:16];
              phase_buf[125] <= data_out[32+WIDTH-1:32];
              duty_buf[125] <= data_out[48+WIDTH-1:48];
              phase_buf[126] <= data_out[64+WIDTH-1:64];
              duty_buf[126] <= data_out[80+WIDTH-1:80];
              phase_buf[127] <= data_out[96+WIDTH-1:96];
              duty_buf[127] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd32: begin
              phase_buf[128] <= data_out[0+WIDTH-1:0];
              duty_buf[128] <= data_out[16+WIDTH-1:16];
              phase_buf[129] <= data_out[32+WIDTH-1:32];
              duty_buf[129] <= data_out[48+WIDTH-1:48];
              phase_buf[130] <= data_out[64+WIDTH-1:64];
              duty_buf[130] <= data_out[80+WIDTH-1:80];
              phase_buf[131] <= data_out[96+WIDTH-1:96];
              duty_buf[131] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd33: begin
              phase_buf[132] <= data_out[0+WIDTH-1:0];
              duty_buf[132] <= data_out[16+WIDTH-1:16];
              phase_buf[133] <= data_out[32+WIDTH-1:32];
              duty_buf[133] <= data_out[48+WIDTH-1:48];
              phase_buf[134] <= data_out[64+WIDTH-1:64];
              duty_buf[134] <= data_out[80+WIDTH-1:80];
              phase_buf[135] <= data_out[96+WIDTH-1:96];
              duty_buf[135] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd34: begin
              phase_buf[136] <= data_out[0+WIDTH-1:0];
              duty_buf[136] <= data_out[16+WIDTH-1:16];
              phase_buf[137] <= data_out[32+WIDTH-1:32];
              duty_buf[137] <= data_out[48+WIDTH-1:48];
              phase_buf[138] <= data_out[64+WIDTH-1:64];
              duty_buf[138] <= data_out[80+WIDTH-1:80];
              phase_buf[139] <= data_out[96+WIDTH-1:96];
              duty_buf[139] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd35: begin
              phase_buf[140] <= data_out[0+WIDTH-1:0];
              duty_buf[140] <= data_out[16+WIDTH-1:16];
              phase_buf[141] <= data_out[32+WIDTH-1:32];
              duty_buf[141] <= data_out[48+WIDTH-1:48];
              phase_buf[142] <= data_out[64+WIDTH-1:64];
              duty_buf[142] <= data_out[80+WIDTH-1:80];
              phase_buf[143] <= data_out[96+WIDTH-1:96];
              duty_buf[143] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd36: begin
              phase_buf[144] <= data_out[0+WIDTH-1:0];
              duty_buf[144] <= data_out[16+WIDTH-1:16];
              phase_buf[145] <= data_out[32+WIDTH-1:32];
              duty_buf[145] <= data_out[48+WIDTH-1:48];
              phase_buf[146] <= data_out[64+WIDTH-1:64];
              duty_buf[146] <= data_out[80+WIDTH-1:80];
              phase_buf[147] <= data_out[96+WIDTH-1:96];
              duty_buf[147] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd37: begin
              phase_buf[148] <= data_out[0+WIDTH-1:0];
              duty_buf[148] <= data_out[16+WIDTH-1:16];
              phase_buf[149] <= data_out[32+WIDTH-1:32];
              duty_buf[149] <= data_out[48+WIDTH-1:48];
              phase_buf[150] <= data_out[64+WIDTH-1:64];
              duty_buf[150] <= data_out[80+WIDTH-1:80];
              phase_buf[151] <= data_out[96+WIDTH-1:96];
              duty_buf[151] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd38: begin
              phase_buf[152] <= data_out[0+WIDTH-1:0];
              duty_buf[152] <= data_out[16+WIDTH-1:16];
              phase_buf[153] <= data_out[32+WIDTH-1:32];
              duty_buf[153] <= data_out[48+WIDTH-1:48];
              phase_buf[154] <= data_out[64+WIDTH-1:64];
              duty_buf[154] <= data_out[80+WIDTH-1:80];
              phase_buf[155] <= data_out[96+WIDTH-1:96];
              duty_buf[155] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd39: begin
              phase_buf[156] <= data_out[0+WIDTH-1:0];
              duty_buf[156] <= data_out[16+WIDTH-1:16];
              phase_buf[157] <= data_out[32+WIDTH-1:32];
              duty_buf[157] <= data_out[48+WIDTH-1:48];
              phase_buf[158] <= data_out[64+WIDTH-1:64];
              duty_buf[158] <= data_out[80+WIDTH-1:80];
              phase_buf[159] <= data_out[96+WIDTH-1:96];
              duty_buf[159] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd40: begin
              phase_buf[160] <= data_out[0+WIDTH-1:0];
              duty_buf[160] <= data_out[16+WIDTH-1:16];
              phase_buf[161] <= data_out[32+WIDTH-1:32];
              duty_buf[161] <= data_out[48+WIDTH-1:48];
              phase_buf[162] <= data_out[64+WIDTH-1:64];
              duty_buf[162] <= data_out[80+WIDTH-1:80];
              phase_buf[163] <= data_out[96+WIDTH-1:96];
              duty_buf[163] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd41: begin
              phase_buf[164] <= data_out[0+WIDTH-1:0];
              duty_buf[164] <= data_out[16+WIDTH-1:16];
              phase_buf[165] <= data_out[32+WIDTH-1:32];
              duty_buf[165] <= data_out[48+WIDTH-1:48];
              phase_buf[166] <= data_out[64+WIDTH-1:64];
              duty_buf[166] <= data_out[80+WIDTH-1:80];
              phase_buf[167] <= data_out[96+WIDTH-1:96];
              duty_buf[167] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd42: begin
              phase_buf[168] <= data_out[0+WIDTH-1:0];
              duty_buf[168] <= data_out[16+WIDTH-1:16];
              phase_buf[169] <= data_out[32+WIDTH-1:32];
              duty_buf[169] <= data_out[48+WIDTH-1:48];
              phase_buf[170] <= data_out[64+WIDTH-1:64];
              duty_buf[170] <= data_out[80+WIDTH-1:80];
              phase_buf[171] <= data_out[96+WIDTH-1:96];
              duty_buf[171] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd43: begin
              phase_buf[172] <= data_out[0+WIDTH-1:0];
              duty_buf[172] <= data_out[16+WIDTH-1:16];
              phase_buf[173] <= data_out[32+WIDTH-1:32];
              duty_buf[173] <= data_out[48+WIDTH-1:48];
              phase_buf[174] <= data_out[64+WIDTH-1:64];
              duty_buf[174] <= data_out[80+WIDTH-1:80];
              phase_buf[175] <= data_out[96+WIDTH-1:96];
              duty_buf[175] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd44: begin
              phase_buf[176] <= data_out[0+WIDTH-1:0];
              duty_buf[176] <= data_out[16+WIDTH-1:16];
              phase_buf[177] <= data_out[32+WIDTH-1:32];
              duty_buf[177] <= data_out[48+WIDTH-1:48];
              phase_buf[178] <= data_out[64+WIDTH-1:64];
              duty_buf[178] <= data_out[80+WIDTH-1:80];
              phase_buf[179] <= data_out[96+WIDTH-1:96];
              duty_buf[179] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd45: begin
              phase_buf[180] <= data_out[0+WIDTH-1:0];
              duty_buf[180] <= data_out[16+WIDTH-1:16];
              phase_buf[181] <= data_out[32+WIDTH-1:32];
              duty_buf[181] <= data_out[48+WIDTH-1:48];
              phase_buf[182] <= data_out[64+WIDTH-1:64];
              duty_buf[182] <= data_out[80+WIDTH-1:80];
              phase_buf[183] <= data_out[96+WIDTH-1:96];
              duty_buf[183] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd46: begin
              phase_buf[184] <= data_out[0+WIDTH-1:0];
              duty_buf[184] <= data_out[16+WIDTH-1:16];
              phase_buf[185] <= data_out[32+WIDTH-1:32];
              duty_buf[185] <= data_out[48+WIDTH-1:48];
              phase_buf[186] <= data_out[64+WIDTH-1:64];
              duty_buf[186] <= data_out[80+WIDTH-1:80];
              phase_buf[187] <= data_out[96+WIDTH-1:96];
              duty_buf[187] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd47: begin
              phase_buf[188] <= data_out[0+WIDTH-1:0];
              duty_buf[188] <= data_out[16+WIDTH-1:16];
              phase_buf[189] <= data_out[32+WIDTH-1:32];
              duty_buf[189] <= data_out[48+WIDTH-1:48];
              phase_buf[190] <= data_out[64+WIDTH-1:64];
              duty_buf[190] <= data_out[80+WIDTH-1:80];
              phase_buf[191] <= data_out[96+WIDTH-1:96];
              duty_buf[191] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd48: begin
              phase_buf[192] <= data_out[0+WIDTH-1:0];
              duty_buf[192] <= data_out[16+WIDTH-1:16];
              phase_buf[193] <= data_out[32+WIDTH-1:32];
              duty_buf[193] <= data_out[48+WIDTH-1:48];
              phase_buf[194] <= data_out[64+WIDTH-1:64];
              duty_buf[194] <= data_out[80+WIDTH-1:80];
              phase_buf[195] <= data_out[96+WIDTH-1:96];
              duty_buf[195] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd49: begin
              phase_buf[196] <= data_out[0+WIDTH-1:0];
              duty_buf[196] <= data_out[16+WIDTH-1:16];
              phase_buf[197] <= data_out[32+WIDTH-1:32];
              duty_buf[197] <= data_out[48+WIDTH-1:48];
              phase_buf[198] <= data_out[64+WIDTH-1:64];
              duty_buf[198] <= data_out[80+WIDTH-1:80];
              phase_buf[199] <= data_out[96+WIDTH-1:96];
              duty_buf[199] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd50: begin
              phase_buf[200] <= data_out[0+WIDTH-1:0];
              duty_buf[200] <= data_out[16+WIDTH-1:16];
              phase_buf[201] <= data_out[32+WIDTH-1:32];
              duty_buf[201] <= data_out[48+WIDTH-1:48];
              phase_buf[202] <= data_out[64+WIDTH-1:64];
              duty_buf[202] <= data_out[80+WIDTH-1:80];
              phase_buf[203] <= data_out[96+WIDTH-1:96];
              duty_buf[203] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd51: begin
              phase_buf[204] <= data_out[0+WIDTH-1:0];
              duty_buf[204] <= data_out[16+WIDTH-1:16];
              phase_buf[205] <= data_out[32+WIDTH-1:32];
              duty_buf[205] <= data_out[48+WIDTH-1:48];
              phase_buf[206] <= data_out[64+WIDTH-1:64];
              duty_buf[206] <= data_out[80+WIDTH-1:80];
              phase_buf[207] <= data_out[96+WIDTH-1:96];
              duty_buf[207] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd52: begin
              phase_buf[208] <= data_out[0+WIDTH-1:0];
              duty_buf[208] <= data_out[16+WIDTH-1:16];
              phase_buf[209] <= data_out[32+WIDTH-1:32];
              duty_buf[209] <= data_out[48+WIDTH-1:48];
              phase_buf[210] <= data_out[64+WIDTH-1:64];
              duty_buf[210] <= data_out[80+WIDTH-1:80];
              phase_buf[211] <= data_out[96+WIDTH-1:96];
              duty_buf[211] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd53: begin
              phase_buf[212] <= data_out[0+WIDTH-1:0];
              duty_buf[212] <= data_out[16+WIDTH-1:16];
              phase_buf[213] <= data_out[32+WIDTH-1:32];
              duty_buf[213] <= data_out[48+WIDTH-1:48];
              phase_buf[214] <= data_out[64+WIDTH-1:64];
              duty_buf[214] <= data_out[80+WIDTH-1:80];
              phase_buf[215] <= data_out[96+WIDTH-1:96];
              duty_buf[215] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd54: begin
              phase_buf[216] <= data_out[0+WIDTH-1:0];
              duty_buf[216] <= data_out[16+WIDTH-1:16];
              phase_buf[217] <= data_out[32+WIDTH-1:32];
              duty_buf[217] <= data_out[48+WIDTH-1:48];
              phase_buf[218] <= data_out[64+WIDTH-1:64];
              duty_buf[218] <= data_out[80+WIDTH-1:80];
              phase_buf[219] <= data_out[96+WIDTH-1:96];
              duty_buf[219] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd55: begin
              phase_buf[220] <= data_out[0+WIDTH-1:0];
              duty_buf[220] <= data_out[16+WIDTH-1:16];
              phase_buf[221] <= data_out[32+WIDTH-1:32];
              duty_buf[221] <= data_out[48+WIDTH-1:48];
              phase_buf[222] <= data_out[64+WIDTH-1:64];
              duty_buf[222] <= data_out[80+WIDTH-1:80];
              phase_buf[223] <= data_out[96+WIDTH-1:96];
              duty_buf[223] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd56: begin
              phase_buf[224] <= data_out[0+WIDTH-1:0];
              duty_buf[224] <= data_out[16+WIDTH-1:16];
              phase_buf[225] <= data_out[32+WIDTH-1:32];
              duty_buf[225] <= data_out[48+WIDTH-1:48];
              phase_buf[226] <= data_out[64+WIDTH-1:64];
              duty_buf[226] <= data_out[80+WIDTH-1:80];
              phase_buf[227] <= data_out[96+WIDTH-1:96];
              duty_buf[227] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd57: begin
              phase_buf[228] <= data_out[0+WIDTH-1:0];
              duty_buf[228] <= data_out[16+WIDTH-1:16];
              phase_buf[229] <= data_out[32+WIDTH-1:32];
              duty_buf[229] <= data_out[48+WIDTH-1:48];
              phase_buf[230] <= data_out[64+WIDTH-1:64];
              duty_buf[230] <= data_out[80+WIDTH-1:80];
              phase_buf[231] <= data_out[96+WIDTH-1:96];
              duty_buf[231] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd58: begin
              phase_buf[232] <= data_out[0+WIDTH-1:0];
              duty_buf[232] <= data_out[16+WIDTH-1:16];
              phase_buf[233] <= data_out[32+WIDTH-1:32];
              duty_buf[233] <= data_out[48+WIDTH-1:48];
              phase_buf[234] <= data_out[64+WIDTH-1:64];
              duty_buf[234] <= data_out[80+WIDTH-1:80];
              phase_buf[235] <= data_out[96+WIDTH-1:96];
              duty_buf[235] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd59: begin
              phase_buf[236] <= data_out[0+WIDTH-1:0];
              duty_buf[236] <= data_out[16+WIDTH-1:16];
              phase_buf[237] <= data_out[32+WIDTH-1:32];
              duty_buf[237] <= data_out[48+WIDTH-1:48];
              phase_buf[238] <= data_out[64+WIDTH-1:64];
              duty_buf[238] <= data_out[80+WIDTH-1:80];
              phase_buf[239] <= data_out[96+WIDTH-1:96];
              duty_buf[239] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd60: begin
              phase_buf[240] <= data_out[0+WIDTH-1:0];
              duty_buf[240] <= data_out[16+WIDTH-1:16];
              phase_buf[241] <= data_out[32+WIDTH-1:32];
              duty_buf[241] <= data_out[48+WIDTH-1:48];
              phase_buf[242] <= data_out[64+WIDTH-1:64];
              duty_buf[242] <= data_out[80+WIDTH-1:80];
              phase_buf[243] <= data_out[96+WIDTH-1:96];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd61: begin
              phase_buf[244] <= data_out[0+WIDTH-1:0];
              duty_buf[244] <= data_out[16+WIDTH-1:16];
              phase_buf[245] <= data_out[32+WIDTH-1:32];
              duty_buf[245] <= data_out[48+WIDTH-1:48];
              phase_buf[246] <= data_out[64+WIDTH-1:64];
              duty_buf[246] <= data_out[80+WIDTH-1:80];
              phase_buf[247] <= data_out[96+WIDTH-1:96];
              duty_buf[247] <= data_out[112+WIDTH-1:112];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            6'd62: begin
              phase_buf[248] <= data_out[0+WIDTH-1:0];
              duty_buf[248] <= data_out[16+WIDTH-1:16];
              state <= BUF;
            end
          endcase
        end
        BUF: begin
          phase <= phase_buf;
          duty  <= duty_buf;
          done  <= 1;
          state <= IDLE;
        end
      endcase
    end
  end

endmodule
