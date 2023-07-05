/*
 * File: sim_controller.sv
 * Project: controller
 * Created Date: 22/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

module sim_controller ();

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
  localparam bit [WIDTH-1:0] MAX = (1 << WIDTH) - 1;

  sim_helper_bram sim_helper_bram ();
  sim_helper_random sim_helper_random ();

  bit thermo;
  bit force_fan;
  bit use_stm_start_idx;
  bit [63:0] ecat_sync_time;
  bit sync_set;
  bit [15:0] cycle_m;
  bit [31:0] freq_div_m;
  bit [15:0] delay_m[DEPTH];
  bit [WIDTH-1:0] step_s;
  bit [15:0] cycle_stm;
  bit [31:0] freq_div_stm;
  bit [31:0] sound_speed;
  bit [15:0] stm_start_idx;
  bit [WIDTH-1:0] cycle[DEPTH];

  controller #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) controller (
      .CLK(CLK_20P48M),
      .THERMO(thermo),
      .FORCE_FAN(force_fan),
      .CPU_BUS(sim_helper_bram.cpu_bus.ctl_port),
      .ECAT_SYNC_TIME(ecat_sync_time),
      .SYNC_SET(sync_set),
      .CYCLE_M(cycle_m),
      .FREQ_DIV_M(freq_div_m),
      .DELAY_M(delay_m),
      .STEP_S(step_s),
      .CYCLE_STM(cycle_stm),
      .FREQ_DIV_STM(freq_div_stm),
      .SOUND_SPEED(sound_speed),
      .STM_START_IDX(stm_start_idx),
      .USE_STM_START_IDX(use_stm_start_idx),
      .CYCLE(cycle)
  );

  initial begin
    bit [15:0] ctrl_reg;
    bit [63:0] ecat_sync_time_buf;
    bit [15:0] cycle_m_buf;
    bit [31:0] freq_div_m_buf;
    bit [WIDTH-1:0] step_s_buf;
    bit [15:0] cycle_stm_buf;
    bit [31:0] freq_div_stm_buf;
    bit [31:0] sound_speed_buf;
    bit [15:0] stm_start_idx_buf;
    bit [WIDTH-1:0] cycle_buf[DEPTH];
    bit [15:0] delay_buf[DEPTH];
    @(posedge locked);

    sim_helper_random.init();

    ecat_sync_time_buf[31:0]  = sim_helper_random.range(32'hFFFFFFFF, 0);
    ecat_sync_time_buf[63:32] = sim_helper_random.range(32'hFFFFFFFF, 0);
    sim_helper_bram.write_ecat_sync_time(ecat_sync_time_buf);

    cycle_m_buf = sim_helper_random.range(16'hFFFF, 0);
    sim_helper_bram.write_mod_cycle(cycle_m_buf);

    freq_div_m_buf = sim_helper_random.range(32'hFFFFFFFF, 0);
    sim_helper_bram.write_mod_freq_div(freq_div_m_buf);

    step_s_buf = sim_helper_random.range(MAX, 0);
    sim_helper_bram.write_silent_step(step_s_buf);

    cycle_stm_buf = sim_helper_random.range(16'hFFFF, 0);
    sim_helper_bram.write_stm_cycle(cycle_stm_buf);

    freq_div_stm_buf = sim_helper_random.range(32'hFFFFFFFF, 0);
    sim_helper_bram.write_stm_freq_div(freq_div_stm_buf);

    sound_speed_buf = sim_helper_random.range(32'hFFFFFFFF, 0);
    sim_helper_bram.write_sound_speed(sound_speed_buf);

    stm_start_idx_buf = sim_helper_random.range(16'hFFFF, 0);
    sim_helper_bram.write_stm_start_idx(stm_start_idx_buf);

    for (int i = 0; i < DEPTH; i++) begin
      cycle_buf[i] = sim_helper_random.range(MAX, 0);
    end
    sim_helper_bram.write_cycle(cycle_buf);

    for (int i = 0; i < DEPTH; i++) begin
      delay_buf[i] = sim_helper_random.range(16'hFFFF, 0);
    end
    sim_helper_bram.write_delay(delay_buf);

    for (int i = 0; i < 100; i++) begin
      @(posedge CLK_20P48M);
    end

    if (cycle_m_buf != cycle_m) begin
      $error("Failed at cycle_m");
      $finish();
    end
    if (freq_div_m_buf != freq_div_m) begin
      $error("Failed at freq_div_m");
      $finish();
    end
    if (step_s_buf != step_s) begin
      $error("Failed at step_s");
      $finish();
    end
    if (cycle_stm_buf != cycle_stm) begin
      $error("Failed at cycle_stm");
      $finish();
    end
    if (freq_div_stm_buf != freq_div_stm) begin
      $error("Failed at freq_div_stm");
      $finish();
    end
    if (sound_speed_buf != sound_speed) begin
      $error("Failed at sound_speed");
      $finish();
    end
    if (stm_start_idx_buf != stm_start_idx) begin
      $error("Failed at stm_start_idx");
      $finish();
    end

    sim_helper_bram.set_ctl_reg(1, 1);
    @(posedge sync_set);

    if (ecat_sync_time_buf != ecat_sync_time) begin
      $error("Failed at ecat_sync_time");
      $finish();
    end
    for (int i = 0; i < DEPTH; i++) begin
      if (cycle_buf[i] != cycle[i]) begin
        $error("Failed at cycle[%d]", i);
        $finish();
      end
    end
    for (int i = 0; i < DEPTH; i++) begin
      if (delay_buf[i] != delay_m[i]) begin
        $error("Failed at delay[%d]", i);
        $finish();
      end
    end

    $display("OK! sim_controller");
    $finish();
  end

endmodule
