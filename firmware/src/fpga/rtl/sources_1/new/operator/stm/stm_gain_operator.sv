/*
 * File: stm_gain_operator.sv
 * Project: stm
 * Created Date: 13/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module stm_gain_operator #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK_L,
    input var TRIG_40KHZ,
    input var [15:0] IDX,
    stm_bus_if.gain_port STM_BUS,
    input var LEGACY_MODE,
    output var [WIDTH-1:0] DUTY,
    output var [WIDTH-1:0] PHASE,
    output var DOUT_VALID
);

  bit [WIDTH-1:0] duty;
  bit [WIDTH-1:0] phase;

  bit [127:0] data_out;

  bit [15:0] idx;
  bit dout_valid;

  bit [9:0] gain_addr_base;
  bit [5:0] gain_addr_offset;
  bit [2:0] set_cnt;
  bit [$clog2(DEPTH):0] cnt;

  typedef enum bit [1:0] {
    WAITING,
    BRAM_WAIT_0,
    BRAM_WAIT_1,
    RUN
  } state_t;

  state_t state = WAITING;

  assign DUTY = duty;
  assign PHASE = phase;

  assign idx = IDX;
  assign STM_BUS.GAIN_ADDR = {gain_addr_base, gain_addr_offset};
  assign data_out = STM_BUS.DATA_OUT;

  assign DOUT_VALID = dout_valid;

  always_ff @(posedge CLK_L) begin
    if (LEGACY_MODE) begin
      case (state)
        WAITING: begin
          dout_valid <= 0;
          if (TRIG_40KHZ) begin
            gain_addr_base <= idx[10:1];
            gain_addr_offset <= idx[0] ? 6'h20 : 0;
            state <= BRAM_WAIT_0;
          end
        end
        BRAM_WAIT_0: begin
          state <= BRAM_WAIT_1;
        end
        BRAM_WAIT_1: begin
          cnt <= 0;
          set_cnt <= 0;
          state <= RUN;
        end
        RUN: begin
          dout_valid <= 1;
          case (set_cnt)
            0: begin
              phase <= {1'b0, data_out[7:0], 4'h00};
              duty <= {2'b00, data_out[15:8], 3'h7} + 1;
              set_cnt <= set_cnt + 1;
            end
            1: begin
              phase <= {1'b0, data_out[23:16], 4'h00};
              duty <= {2'b00, data_out[31:24], 3'h7} + 1;
              set_cnt <= set_cnt + 1;
            end
            2: begin
              phase <= {1'b0, data_out[39:32], 4'h00};
              duty <= {2'b00, data_out[47:40], 3'h7} + 1;
              set_cnt <= set_cnt + 1;
            end
            3: begin
              phase <= {1'b0, data_out[55:48], 4'h00};
              duty <= {2'b00, data_out[63:56], 3'h7} + 1;
              set_cnt <= set_cnt + 1;
            end
            4: begin
              phase <= {1'b0, data_out[71:64], 4'h00};
              duty <= {2'b00, data_out[79:72], 3'h7} + 1;
              set_cnt <= set_cnt + 1;
            end
            5: begin
              phase <= {1'b0, data_out[87:80], 4'h00};
              duty <= {2'b00, data_out[95:88], 3'h7} + 1;
              set_cnt <= set_cnt + 1;
              gain_addr_offset <= gain_addr_offset + 1;
            end
            6: begin
              phase <= {1'b0, data_out[103:96], 4'h00};
              duty <= {2'b00, data_out[111:104], 3'h7} + 1;
              set_cnt <= set_cnt + 1;
            end
            7: begin
              phase <= {1'b0, data_out[119:112], 4'h00};
              duty <= {2'b00, data_out[127:120], 3'h7} + 1;
              set_cnt <= 0;
            end
            default: begin
            end
          endcase
          cnt <= cnt + 1;
          if (cnt == DEPTH - 1) begin
            state <= WAITING;
          end
        end
        default: begin
        end
      endcase
    end else begin
      case (state)
        WAITING: begin
          dout_valid <= 0;
          if (TRIG_40KHZ) begin
            gain_addr_base <= idx[9:0];
            gain_addr_offset <= 0;
            state <= BRAM_WAIT_0;
          end
        end
        BRAM_WAIT_0: begin
          state <= BRAM_WAIT_1;
        end
        BRAM_WAIT_1: begin
          cnt <= 0;
          set_cnt <= 0;
          state <= RUN;
        end
        RUN: begin
          dout_valid <= 1;
          case (set_cnt)
            0: begin
              phase <= data_out[0+WIDTH-1:0];
              duty <= data_out[16+WIDTH-1:16];
              set_cnt <= set_cnt + 1;
            end
            1: begin
              phase <= data_out[32+WIDTH-1:32];
              duty <= data_out[48+WIDTH-1:48];
              gain_addr_offset <= gain_addr_offset + 1;
              set_cnt <= set_cnt + 1;
            end
            2: begin
              phase <= data_out[64+WIDTH-1:64];
              duty <= data_out[80+WIDTH-1:80];
              set_cnt <= set_cnt + 1;
            end
            3: begin
              phase <= data_out[96+WIDTH-1:96];
              duty <= data_out[112+WIDTH-1:112];
              set_cnt <= 0;
            end
            default: begin
            end
          endcase
          cnt <= cnt + 1;
          if (cnt == DEPTH - 1) begin
            state <= WAITING;
          end
        end
        default: begin
        end
      endcase
    end
  end

endmodule
