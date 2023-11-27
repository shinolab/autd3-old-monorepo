/*
 * File: sim_operator_normal.sv
 * Project: operator
 * Created Date: 12/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

module sim_operator_normal ();

  logic [63:0] SYS_TIME;
  logic CLK_20P48M;
  logic locked;
  sim_helper_clk sim_helper_clk (
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME(SYS_TIME)
  );

  localparam int DEPTH = 249;

  sim_helper_bram sim_helper_bram ();
  sim_helper_random sim_helper_random ();

  logic [7:0] intensity;
  logic [7:0] phase;
  logic dout_valid;

  logic [7:0] intensity_buf[DEPTH];
  logic [7:0] phase_buf[DEPTH];

  time_cnt_generator #(
      .DEPTH(DEPTH)
  ) time_cnt_generator (
      .CLK(CLK_20P48M),
      .SYS_TIME(SYS_TIME),
      .SKIP_ONE_ASSERT(1'b0),
      .TIME_CNT(),
      .UPDATE(UPDATE)
  );

  normal_operator #(
      .DEPTH(DEPTH)
  ) normal_operator (
      .CLK(CLK_20P48M),
      .CPU_BUS(sim_helper_bram.cpu_bus.normal_port),
      .UPDATE(UPDATE),
      .INTENSITY(intensity),
      .PHASE(phase),
      .DOUT_VALID(dout_valid)
  );

  initial begin
    sim_helper_random.init();

    @(posedge locked);

    for (int i = 0; i < DEPTH; i++) begin
      intensity_buf[i] = sim_helper_random.range(8'hFF, 0);
      phase_buf[i] = sim_helper_random.range(8'hFF, 0);
      sim_helper_bram.write_intensity_phase(i, intensity_buf[i], phase_buf[i]);
    end

    while (1) begin
      @(posedge CLK_20P48M);
      if (~dout_valid) begin
        break;
      end
    end
    while (1) begin
      @(posedge CLK_20P48M);
      if (dout_valid) begin
        break;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      if (intensity_buf[i] !== intensity) begin
        $display("failed at intensity[%d], %d!=%d", i, intensity_buf[i], intensity);
        $finish();
      end
      if (phase_buf[i] !== phase) begin
        $display("failed at phase[%d], %d!=%d", i, phase_buf[i], phase);
        $finish();
      end
      @(posedge CLK_20P48M);
    end

    $display("OK! sim_operator_normal");
    $finish();
  end

endmodule
