/*
 * File: sim_controller.sv
 * Project: controller
 * Created Date: 22/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

module sim_controller ();

  logic CLK_20P48M;
  logic locked;
  sim_helper_clk sim_helper_clk (
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME()
  );

  localparam int DEPTH = 249;

  sim_helper_bram sim_helper_bram ();
  sim_helper_random sim_helper_random ();

  logic thermo;
  logic force_fan;
  logic [63:0] ecat_sync_time;
  logic sync_set;
  logic op_mode;
  logic stm_gain_mode;
  logic [15:0] cycle_m;
  logic [31:0] freq_div_m;
  logic [15:0] delay_m[DEPTH];
  logic [15:0] step_intensity_s, step_phase_s;
  logic [15:0] cycle_stm;
  logic [31:0] freq_div_stm;
  logic [31:0] sound_speed;
  logic [15:0] stm_start_idx;
  logic use_stm_start_idx;
  logic [15:0] stm_finish_idx;
  logic use_stm_finish_idx;

  controller #(
      .DEPTH(DEPTH)
  ) controller (
      .CLK(CLK_20P48M),
      .THERMO(thermo),
      .FORCE_FAN(force_fan),
      .CPU_BUS(sim_helper_bram.cpu_bus.ctl_port),
      .ECAT_SYNC_TIME(ecat_sync_time),
      .SYNC_SET(sync_set),
      .OP_MODE(op_mode),
      .STM_GAIN_MODE(stm_gain_mode),
      .CYCLE_M(cycle_m),
      .FREQ_DIV_M(freq_div_m),
      .DELAY_M(delay_m),
      .STEP_INTENSITY_S(step_intensity_s),
      .STEP_PHASE_S(step_phase_s),
      .CYCLE_STM(cycle_stm),
      .FREQ_DIV_STM(freq_div_stm),
      .SOUND_SPEED(sound_speed),
      .STM_START_IDX(stm_start_idx),
      .USE_STM_START_IDX(use_stm_start_idx),
      .STM_FINISH_IDX(stm_finish_idx),
      .USE_STM_FINISH_IDX(use_stm_finish_idx)
  );

  initial begin
    logic [15:0] ctrl_reg;
    logic [63:0] ecat_sync_time_buf;
    logic [15:0] cycle_m_buf;
    logic [31:0] freq_div_m_buf;
    logic [15:0] delay_buf[DEPTH];
    logic [15:0] step_intensity_s_buf, step_phase_s_buf;
    logic [15:0] cycle_stm_buf;
    logic [31:0] freq_div_stm_buf;
    logic [31:0] sound_speed_buf;
    logic [15:0] stm_start_idx_buf;
    logic [15:0] stm_finish_idx_buf;
    @(posedge locked);

    sim_helper_random.init();

    ecat_sync_time_buf[31:0]  = sim_helper_random.range(32'hFFFFFFFF, 0);
    ecat_sync_time_buf[63:32] = sim_helper_random.range(32'hFFFFFFFF, 0);
    sim_helper_bram.write_ecat_sync_time(ecat_sync_time_buf);

    cycle_m_buf = sim_helper_random.range(16'hFFFF, 0);
    sim_helper_bram.write_mod_cycle(cycle_m_buf);

    freq_div_m_buf = sim_helper_random.range(32'hFFFFFFFF, 0);
    sim_helper_bram.write_mod_freq_div(freq_div_m_buf);

    for (int i = 0; i < DEPTH; i++) begin
      delay_buf[i] = sim_helper_random.range(16'hFFFF, 0);
    end
    sim_helper_bram.write_delay(delay_buf);

    step_intensity_s_buf = sim_helper_random.range(16'hFFFF, 0);
    step_phase_s_buf = sim_helper_random.range(16'hFFFF, 0);
    sim_helper_bram.write_silent_step(step_intensity_s_buf, step_phase_s_buf);

    cycle_stm_buf = sim_helper_random.range(16'hFFFF, 0);
    sim_helper_bram.write_stm_cycle(cycle_stm_buf);

    freq_div_stm_buf = sim_helper_random.range(32'hFFFFFFFF, 0);
    sim_helper_bram.write_stm_freq_div(freq_div_stm_buf);

    sound_speed_buf = sim_helper_random.range(32'hFFFFFFFF, 0);
    sim_helper_bram.write_sound_speed(sound_speed_buf);

    stm_start_idx_buf = sim_helper_random.range(16'hFFFF, 0);
    sim_helper_bram.write_stm_start_idx(stm_start_idx_buf);

    stm_finish_idx_buf = sim_helper_random.range(16'hFFFF, 0);
    sim_helper_bram.write_stm_finish_idx(stm_finish_idx_buf);

    for (int i = 0; i < 256; i++) begin
      @(posedge CLK_20P48M);
    end

    if (cycle_m_buf !== cycle_m) begin
      $error("Failed at cycle_m");
      $finish();
    end
    if (freq_div_m_buf !== freq_div_m) begin
      $error("Failed at freq_div_m");
      $finish();
    end
    for (int i = 0; i < DEPTH; i++) begin
      if (delay_buf[i] !== delay_m[i]) begin
        $error("Failed at delay[%d]", i);
        $finish();
      end
    end
    if (step_intensity_s_buf !== step_intensity_s) begin
      $error("Failed at step_intensity_s");
      $finish();
    end
    if (step_phase_s_buf !== step_phase_s) begin
      $error("Failed at step_phase_s");
      $finish();
    end
    if (cycle_stm_buf !== cycle_stm) begin
      $error("Failed at cycle_stm");
      $finish();
    end
    if (freq_div_stm_buf !== freq_div_stm) begin
      $error("Failed at freq_div_stm");
      $finish();
    end
    if (sound_speed_buf !== sound_speed) begin
      $error("Failed at sound_speed");
      $finish();
    end
    if (stm_start_idx_buf !== stm_start_idx) begin
      $error("Failed at stm_start_idx");
      $finish();
    end
    if (stm_finish_idx_buf !== stm_finish_idx) begin
      $error("Failed at stm_finish_idx");
      $finish();
    end

    sim_helper_bram.set_ctl_reg(1, 1);
    @(posedge sync_set);

    if (ecat_sync_time_buf !== ecat_sync_time) begin
      $error("Failed at ecat_sync_time");
      $finish();
    end

    $display("OK! sim_controller");
    $finish();
  end

endmodule
