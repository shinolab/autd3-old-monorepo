/*
 * File: stm_gain_operator.sv
 * Project: stm
 * Created Date: 13/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module stm_gain_operator #(
    parameter int DEPTH = 249
) (
    input var CLK,
    input var UPDATE,
    input var [15:0] IDX,
    stm_bus_if.gain_port STM_BUS,
    output var [7:0] INTENSITY,
    output var [7:0] PHASE,
    output var DOUT_VALID
);

  logic [7:0] intensity;
  logic [7:0] phase;

  logic [127:0] data_out;

  logic [15:0] idx;
  logic dout_valid;

  logic [9:0] gain_addr_base;
  logic [5:0] gain_addr_offset;
  logic [2:0] set_cnt;
  logic [$clog2(DEPTH):0] cnt;

  typedef enum logic [1:0] {
    WAITING,
    BRAM_WAIT_0,
    BRAM_WAIT_1,
    RUN
  } state_t;

  state_t state = WAITING;

  assign INTENSITY = intensity;
  assign PHASE = phase;

  assign idx = IDX;
  assign STM_BUS.GAIN_ADDR = {gain_addr_base, gain_addr_offset};
  assign data_out = STM_BUS.DATA_OUT;

  assign DOUT_VALID = dout_valid;

  always_ff @(posedge CLK) begin
    case (state)
      WAITING: begin
        dout_valid <= 0;
        if (UPDATE) begin
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
            phase <= data_out[7:0];
            intensity <= data_out[15:8];
            set_cnt <= set_cnt + 1;
          end
          1: begin
            phase <= data_out[23:16];
            intensity <= data_out[31:24];
            set_cnt <= set_cnt + 1;
          end
          2: begin
            phase <= data_out[39:32];
            intensity <= data_out[47:40];
            set_cnt <= set_cnt + 1;
          end
          3: begin
            phase <= data_out[55:48];
            intensity <= data_out[63:56];
            set_cnt <= set_cnt + 1;
          end
          4: begin
            phase <= data_out[71:64];
            intensity <= data_out[79:72];
            set_cnt <= set_cnt + 1;
          end
          5: begin
            phase <= data_out[87:80];
            intensity <= data_out[95:88];
            set_cnt <= set_cnt + 1;
            gain_addr_offset <= gain_addr_offset + 1;
          end
          6: begin
            phase <= data_out[103:96];
            intensity <= data_out[111:104];
            set_cnt <= set_cnt + 1;
          end
          7: begin
            phase <= data_out[119:112];
            intensity <= data_out[127:120];
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

endmodule
