/*
 * File: sim_operator_stm_gain.sv
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

module sim_operator_stm_gain ();

  bit CLK_20P48M;
  bit locked;
  bit [63:0] SYS_TIME;
  sim_helper_clk sim_helper_clk (
      .CLK_163P84M(),
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME(SYS_TIME)
  );

  localparam int WIDTH = 13;
  localparam int DEPTH = 249;
  localparam bit [WIDTH-1:0] MAX = (1 << WIDTH) - 1;

  sim_helper_bram sim_helper_bram ();
  sim_helper_random sim_helper_random ();

  bit [15:0] cycle_stm;
  bit [31:0] freq_div_stm;
  bit legacy_mode;

  bit [WIDTH-1:0] duty;
  bit [WIDTH-1:0] phase;
  bit [15:0] idx;
  bit dout_valid;

  bit [WIDTH-1:0] duty_buf[1024][DEPTH];
  bit [WIDTH-1:0] phase_buf[1024][DEPTH];

  stm_bus_if stm_bus_if ();

  timer_40kHz timer_40kHz (
      .CLK_L(CLK_20P48M),
      .SYS_TIME(SYS_TIME),
      .TRIG_40KHZ(TRIG_40KHZ)
  );

  stm_memory stm_memory (
      .CLK_L  (CLK_20P48M),
      .CPU_BUS(sim_helper_bram.cpu_bus.stm_port),
      .STM_BUS(stm_bus_if.memory_port)
  );

  stm_sampler stm_sampler (
      .CLK_L(CLK_20P48M),
      .SYS_TIME(SYS_TIME),
      .CYCLE_STM(cycle_stm),
      .FREQ_DIV_STM(freq_div_stm),
      .IDX(idx)
  );

  stm_gain_operator #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) stm_gain_operator (
      .CLK_L(CLK_20P48M),
      .TRIG_40KHZ(TRIG_40KHZ),
      .IDX(idx),
      .STM_BUS(stm_bus_if.gain_port),
      .LEGACY_MODE(legacy_mode),
      .DUTY(duty),
      .PHASE(phase),
      .DOUT_VALID(dout_valid)
  );

  bit [15:0] idx_buf;
  always @(posedge CLK_20P48M) begin
    if (TRIG_40KHZ) idx_buf = idx;
  end

  initial begin
    legacy_mode = 0;
    stm_bus_if.STM_GAIN_MODE = 1'b1;

    cycle_stm = 1024 - 1;
    freq_div_stm = 4096;

    sim_helper_random.init();

    @(posedge locked);

    for (int i = 0; i < cycle_stm + 1; i++) begin
      $display("write %d/%d", i + 1, cycle_stm + 1);
      for (int j = 0; j < DEPTH; j++) begin
        duty_buf[i][j]  = sim_helper_random.range(MAX, 0);
        phase_buf[i][j] = sim_helper_random.range(MAX, 0);
      end
      sim_helper_bram.write_stm_gain_duty_phase(i, duty_buf[i], phase_buf[i]);
    end

    for (int j = 0; j < cycle_stm + 1; j++) begin
      while (1) begin
        @(posedge CLK_20P48M);
        if (dout_valid) begin
          break;
        end
      end
      $display("check %d/%d", j + 1, cycle_stm + 1);
      for (int i = 0; i < DEPTH; i++) begin
        if (duty_buf[idx_buf][i] != duty) begin
          $error("Failed at d_in=%d, d_out=%d", duty_buf[idx_buf][i], duty);
          $finish();
        end
        if (phase_buf[idx_buf][i] != phase) begin
          $error("Failed at p_in=%d, p_out=%d", phase_buf[idx_buf][i], phase);
          $finish();
        end
        @(posedge CLK_20P48M);
      end
    end

    legacy_mode = 1;

    cycle_stm   = 2048 - 1;

    for (int i = 0; i < cycle_stm + 1; i++) begin
      $display("write %d/%d", i + 1, cycle_stm + 1);
      for (int j = 0; j < DEPTH; j++) begin
        duty_buf[i][j]  = sim_helper_random.range(8'hFF, 0);
        phase_buf[i][j] = sim_helper_random.range(8'hFF, 0);
      end
      sim_helper_bram.write_stm_gain_duty_phase_legacy(i, duty_buf[i], phase_buf[i]);
    end

    for (int j = 0; j < cycle_stm + 1; j++) begin
      while (1) begin
        @(posedge CLK_20P48M);
        if (dout_valid) begin
          break;
        end
      end
      $display("check %d/%d", j + 1, cycle_stm + 1);
      for (int i = 0; i < DEPTH; i++) begin
        if (({duty_buf[idx_buf][i], 3'h7} + 1) != duty) begin
          $display("failed at duty[%d], %d!=%d", i, duty_buf[idx_buf][i], duty);
          $finish();
        end
        if ({phase_buf[idx_buf][i], 4'h00} != phase) begin
          $display("failed at phase[%d], %d!=%d", i, {phase_buf[idx_buf][i], 4'h00}, phase);
          $finish();
        end
        @(posedge CLK_20P48M);
      end
    end

    $display("OK!");
    $finish();
  end

endmodule
