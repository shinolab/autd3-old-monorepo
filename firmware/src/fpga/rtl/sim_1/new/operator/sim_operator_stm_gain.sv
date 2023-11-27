/*
 * File: sim_operator_stm_gain.sv
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

module sim_operator_stm_gain ();

  logic CLK_20P48M;
  logic locked;
  logic [63:0] SYS_TIME;
  sim_helper_clk sim_helper_clk (
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME(SYS_TIME)
  );

  localparam int DEPTH = 249;

  sim_helper_bram sim_helper_bram ();
  sim_helper_random sim_helper_random ();

  logic [15:0] cycle_stm;
  logic [31:0] freq_div_stm;

  logic [7:0] intensity;
  logic [7:0] phase;
  logic [15:0] idx;
  logic dout_valid;

  logic [7:0] intensity_buf[2048][DEPTH];
  logic [7:0] phase_buf[2048][DEPTH];

  time_cnt_generator #(
      .DEPTH(DEPTH)
  ) time_cnt_generator (
      .CLK(CLK_20P48M),
      .SYS_TIME(SYS_TIME),
      .SKIP_ONE_ASSERT(1'b0),
      .TIME_CNT(),
      .UPDATE(UPDATE)
  );

  stm_operator stm_operator (
      .CLK(CLK_20P48M),
      .SYS_TIME(SYS_TIME),
      .UPDATE(UPDATE),
      .CYCLE_STM(cycle_stm),
      .FREQ_DIV_STM(freq_div_stm),
      .SOUND_SPEED(),
      .STM_GAIN_MODE(1'b1),
      .CPU_BUS(sim_helper_bram.cpu_bus.stm_port),
      .INTENSITY(intensity),
      .PHASE(phase),
      .DOUT_VALID(dout_valid),
      .IDX(idx)
  );

  logic [15:0] idx_buf;
  always @(posedge CLK_20P48M) begin
    if (UPDATE) idx_buf = idx;
  end

  initial begin
    cycle_stm = 2048 - 1;
    freq_div_stm = 512;

    sim_helper_random.init();

    @(posedge locked);

    for (int i = 0; i < cycle_stm + 1; i++) begin
      $display("write %d/%d", i + 1, cycle_stm + 1);
      for (int j = 0; j < DEPTH; j++) begin
        intensity_buf[i][j] = sim_helper_random.range(8'hFF, 0);
        phase_buf[i][j] = sim_helper_random.range(8'hFF, 0);
      end
      sim_helper_bram.write_stm_gain_intensity_phase(i, intensity_buf[i], phase_buf[i]);
    end

    while (1) begin
      @(posedge CLK_20P48M);
      if (~dout_valid) begin
        break;
      end
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
        if (intensity_buf[idx_buf][i] !== intensity) begin
          $display("failed at intensity[%d], %d!=%d", i, intensity_buf[idx_buf][i] + 1, intensity);
          $finish();
        end
        if (phase_buf[idx_buf][i] !== phase) begin
          $display("failed at phase[%d], %d!=%d", i, phase_buf[idx_buf][i], phase);
          $finish();
        end
        @(posedge CLK_20P48M);
      end
    end

    $display("OK!");
    $finish();
  end

endmodule
