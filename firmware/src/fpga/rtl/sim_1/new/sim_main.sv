/*
 * File: sim_main.sv
 * Project: new
 * Created Date: 11/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module sim_main ();

  logic CLK_20P48M;
  logic [63:0] SYS_TIME;
  logic locked;
  logic CAT_SYNC0;
  sim_helper_clk sim_helper_clk (
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME(SYS_TIME)
  );

  sim_helper_bram sim_helper_bram ();
  sim_helper_random sim_helper_random ();

  localparam int DEPTH = 249;

  logic [7:0] intensity_buf[DEPTH];
  logic [7:0] phase_buf[DEPTH];
  logic pwm_out[DEPTH];

  main #(
      .DEPTH(DEPTH)
  ) main (
      .CLK(CLK_20P48M),
      .CAT_SYNC0(CAT_SYNC0),
      .CPU_BUS_CTL(sim_helper_bram.cpu_bus.ctl_port),
      .CPU_BUS_NORMAL(sim_helper_bram.cpu_bus.normal_port),
      .CPU_BUS_STM(sim_helper_bram.cpu_bus.stm_port),
      .CPU_BUS_MOD(sim_helper_bram.cpu_bus.mod_port),
      .THERMO(1'b0),
      .FORCE_FAN(),
      .PWM_OUT(pwm_out)
  );

  initial begin
    sim_helper_random.init();

    @(posedge locked);

    for (int i = 0; i < DEPTH; i++) begin
      intensity_buf[i] = sim_helper_random.range(8'hFF, 0);
      phase_buf[i] = sim_helper_random.range(8'hFF, 0);
      sim_helper_bram.write_intensity_phase(i, intensity_buf[i], phase_buf[i]);
    end

  end

  localparam int ECATSyncBase = 500000;  // 500 us
  localparam logic [15:0] ECATSyncCycleTicks = 1;

  always begin
    #800 CAT_SYNC0 = 0;
    #(ECATSyncBase * ECATSyncCycleTicks - 800) CAT_SYNC0 = 1;
  end

endmodule
