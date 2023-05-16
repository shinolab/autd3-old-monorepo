/*
 * File: sim_modulation_sampler.sv
 * Project: modulation
 * Created Date: 25/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

module sim_modulation_sampler ();

  localparam int DivLatency = 66;

  bit CLK_20P48M;
  bit [63:0] SYS_TIME;
  bit locked;
  sim_helper_clk sim_helper_clk (
      .CLK_163P84M(),
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME(SYS_TIME)
  );

  sim_helper_random sim_helper_random ();

  localparam int WIDTH = 13;
  localparam int DEPTH = 249;

  bit [15:0] cycle_m;
  bit [31:0] freq_div_m;
  bit [15:0] idx, idx_old;

  modulation_sampler modulation_sampler (
      .CLK(CLK_20P48M),
      .SYS_TIME(SYS_TIME),
      .CYCLE_M(cycle_m),
      .FREQ_DIV_M(freq_div_m),
      .IDX(idx)
  );

  initial begin
    cycle_m = 16'hFFFF;
    freq_div_m = 4096;
    @(posedge locked);

    idx_old = idx;
    for (int i = 0; i < cycle_m; i++) begin
      while (1) begin
        @(posedge CLK_20P48M);
        if (idx_old != idx) begin
          break;
        end
      end
      idx_old = idx;
      $display("check %d", i);
      if (((SYS_TIME - 8 * DivLatency * 2 - 2 - 1) / freq_div_m) % (cycle_m + 1) != idx) begin
        $display("Index failed! %d != %d",
                 ((SYS_TIME - 8 * DivLatency * 2 - 2 - 1) / freq_div_m) % (cycle_m + 1), idx);
        $finish();
      end
    end

    $display("OK!");
    $finish();
  end

endmodule
