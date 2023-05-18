/*
 * File: mux.sv
 * Project: new
 * Created Date: 18/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module mux #(
    parameter int WIDTH = 13
) (
    input var CLK_L,
    input var OP_MODE,
    input var [WIDTH-1:0] DUTY_NORMAL,
    input var [WIDTH-1:0] PHASE_NORMAL,
    input var DOUT_VALID_NORMAL,
    input var [WIDTH-1:0] DUTY_STM,
    input var [WIDTH-1:0] PHASE_STM,
    input var DOUT_VALID_STM,
    input var [15:0] STM_IDX,
    input var USE_STM_START_IDX,
    input var USE_STM_FINISH_IDX,
    input var [15:0] STM_START_IDX,
    input var [15:0] STM_FINISH_IDX,
    output var [WIDTH-1:0] DUTY,
    output var [WIDTH-1:0] PHASE,
    output var DOUT_VALID
);

  bit [WIDTH-1:0] duty_stm_buf;
  bit [WIDTH-1:0] phase_stm_buf;
  bit dout_valid_stm_buf;
  bit [WIDTH-1:0] duty_normal_buf;
  bit [WIDTH-1:0] phase_normal_buf;
  bit dout_valid_normal_buf;

  typedef enum bit [1:0] {
    NORMAL,
    WAIT_START_STM,
    STM,
    WAIT_FINISH_STM
  } stm_state_t;

  stm_state_t stm_state = NORMAL;

  bit output_stm;
  assign output_stm = (stm_state == STM) | (stm_state == WAIT_FINISH_STM);
  assign DUTY = output_stm ? duty_stm_buf : duty_normal_buf;
  assign PHASE = output_stm ? phase_stm_buf : phase_normal_buf;
  assign DOUT_VALID = output_stm ? dout_valid_stm_buf : dout_valid_normal_buf;

  always_ff @(posedge CLK_L) begin
    duty_stm_buf <= DUTY_STM;
    phase_stm_buf <= PHASE_STM;
    dout_valid_stm_buf <= DOUT_VALID_STM;
    duty_normal_buf <= DUTY_NORMAL;
    phase_normal_buf <= PHASE_NORMAL;
    dout_valid_normal_buf <= DOUT_VALID_NORMAL;
  end

  always_ff @(posedge CLK_L) begin
    case (stm_state)
      NORMAL: begin
        if (OP_MODE) begin
          stm_state <= USE_STM_START_IDX ? WAIT_START_STM : STM;
        end
      end
      WAIT_START_STM: begin
        if (OP_MODE) begin
          stm_state <= DOUT_VALID_STM & (STM_IDX == STM_START_IDX) ? STM : stm_state;
        end else begin
          stm_state <= NORMAL;
        end
      end
      STM: begin
        if (~OP_MODE) begin
          stm_state <= USE_STM_FINISH_IDX ? WAIT_FINISH_STM : NORMAL;
        end
      end
      WAIT_FINISH_STM: begin
        if (~OP_MODE) begin
          stm_state <= DOUT_VALID_STM & (STM_IDX == STM_FINISH_IDX) ? NORMAL : stm_state;
        end else begin
          stm_state <= STM;
        end
      end
      default: begin
      end
    endcase
  end

endmodule
