/*
 * File: sim_mux.sv
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
module sim_mux ();

  bit CLK_20P48M;
  bit locked;
  sim_helper_clk sim_helper_clk (
      .CLK_163P84M(),
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME()
  );

  localparam int WIDTH = 13;
  localparam int DEPTH = 249;

  sim_helper_random sim_helper_random ();

  bit op_mode;
  bit [WIDTH-1:0] duty_normal, phase_normal;
  bit dout_valid_normal;
  bit [WIDTH-1:0] duty_stm, phase_stm;
  bit dout_valid_stm;
  bit [15:0] stm_idx, stm_start_idx, stm_finish_idx;
  bit use_stm_start_idx, use_stm_finish_idx;
  bit [WIDTH-1:0] duty, phase;
  bit dout_valid;

  bit [WIDTH-1:0] duty_buf_normal[DEPTH];
  bit [WIDTH-1:0] phase_buf_normal[DEPTH];
  bit [WIDTH-1:0] duty_buf_stm[DEPTH];
  bit [WIDTH-1:0] phase_buf_stm[DEPTH];

  mux #(
      .WIDTH(WIDTH)
  ) mux (
      .CLK_L(CLK_20P48M),
      .OP_MODE(op_mode),
      .DUTY_NORMAL(duty_normal),
      .PHASE_NORMAL(phase_normal),
      .DOUT_VALID_NORMAL(dout_valid_normal),
      .DUTY_STM(duty_stm),
      .PHASE_STM(phase_stm),
      .DOUT_VALID_STM(dout_valid_stm),
      .STM_IDX(stm_idx),
      .USE_STM_START_IDX(use_stm_start_idx),
      .USE_STM_FINISH_IDX(use_stm_finish_idx),
      .STM_START_IDX(stm_start_idx),
      .STM_FINISH_IDX(stm_finish_idx),
      .DUTY(duty),
      .PHASE(phase),
      .DOUT_VALID(dout_valid)
  );

  task automatic set_normal();
    for (int i = 0; i < DEPTH; i++) begin
      @(posedge CLK_20P48M);
      dout_valid_normal = 1'b1;
      duty_normal = sim_helper_random.range(8000, 0);
      phase_normal = sim_helper_random.range(8000, 0);
      duty_buf_normal[i] = duty_normal;
      phase_buf_normal[i] = phase_normal;
    end
    @(posedge CLK_20P48M);
    dout_valid_normal = 1'b0;
  endtask

  task automatic set_stm();
    for (int i = 0; i < DEPTH; i++) begin
      @(posedge CLK_20P48M);
      dout_valid_stm = 1'b1;
      duty_stm = sim_helper_random.range(8000, 0);
      phase_stm = sim_helper_random.range(8000, 0);
      duty_buf_stm[i] = duty_stm;
      phase_buf_stm[i] = phase_stm;
    end
    @(posedge CLK_20P48M);
    dout_valid_stm = 1'b0;
  endtask

  task automatic check_normal();
    while (1) begin
      @(posedge CLK_20P48M);
      if (dout_valid) begin
        break;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      if (duty_buf_normal[i] != duty) begin
        $display("failed at duty[%d], %d!=%d", i, duty_buf_normal[i], duty);
        $finish();
      end
      if (phase_buf_normal[i] != phase) begin
        $display("failed at phase[%d], %d!=%d", i, phase_buf_normal[i], phase);
        $finish();
      end
      @(posedge CLK_20P48M);
    end
  endtask

  task automatic check_stm();
    while (1) begin
      @(posedge CLK_20P48M);
      if (dout_valid) begin
        break;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      if (duty_buf_stm[i] != duty) begin
        $display("failed at duty[%d], %d!=%d", i, duty_buf_stm[i], duty);
        $finish();
      end
      if (phase_buf_stm[i] != phase) begin
        $display("failed at phase[%d], %d!=%d", i, phase_buf_stm[i], phase);
        $finish();
      end
      @(posedge CLK_20P48M);
    end
  endtask

  initial begin
    sim_helper_random.init();

    @(posedge locked);

    $display("check normal");
    op_mode = 0;
    fork
      set_normal();
      set_stm();
      check_normal();
    join

    $display("check normal to stm");
    use_stm_start_idx = 0;
    use_stm_finish_idx = 0;
    op_mode = 1;
    fork
      set_normal();
      set_stm();
      check_stm();
    join

    $display("check stm to normal");
    op_mode = 0;
    fork
      set_normal();
      set_stm();
      check_normal();
    join

    $display("check stm idx...");
    use_stm_start_idx = 1;
    use_stm_finish_idx = 1;
    stm_start_idx = 1;
    stm_finish_idx = 2;
    stm_idx = 0;
    op_mode = 1;
    $display("\tstill normal");
    fork
      set_normal();
      set_stm();
      check_normal();
    join
    stm_idx = 1;
    $display("\tstm");
    fork
      set_normal();
      set_stm();
      check_stm();
    join
    op_mode = 0;
    $display("\tstill stm");
    fork
      set_normal();
      set_stm();
      check_stm();
    join
    stm_idx = 2;
    $display("\tnormal");
    fork
      set_normal();
      set_stm();
      check_normal();
    join

    $display("OK! sim_mux");
    $finish();
  end

endmodule
