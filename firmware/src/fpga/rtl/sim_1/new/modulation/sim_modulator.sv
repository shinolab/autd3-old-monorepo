/*
 * File: sim_modulator.sv
 * Project: modulation
 * Created Date: 25/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

module sim_modulator ();
  `define M_PI 3.14159265358979323846

  logic [63:0] SYS_TIME;
  logic CLK_20P48M;
  logic locked;
  sim_helper_clk sim_helper_clk (
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME(SYS_TIME)
  );

  sim_helper_random sim_helper_random ();
  sim_helper_bram sim_helper_bram ();

  localparam int DEPTH = 249;

  logic din_valid, dout_valid;
  logic [15:0] idx;
  logic [15:0] cycle_m;
  logic [31:0] freq_div_m;
  logic [15:0] delay_m[DEPTH];
  logic [7:0] intensity;
  logic [15:0] intensity_out;
  logic [7:0] phase;
  logic [7:0] phase_out;

  logic [7:0] mod[65536];
  logic [7:0] intensity_buf[DEPTH];
  logic [7:0] phase_buf[DEPTH];
  logic [15:0] idx_buf;

  modulator #(
      .DEPTH(DEPTH)
  ) modulator (
      .CLK(CLK_20P48M),
      .SYS_TIME(SYS_TIME),
      .CYCLE_M(cycle_m),
      .FREQ_DIV_M(freq_div_m),
      .CPU_BUS(sim_helper_bram.cpu_bus.mod_port),
      .DIN_VALID(din_valid),
      .INTENSITY_IN(intensity),
      .PHASE_IN(phase),
      .DELAY_M(delay_m),
      .INTENSITY_OUT(intensity_out),
      .PHASE_OUT(phase_out),
      .DOUT_VALID(dout_valid),
      .IDX(idx)
  );
  always @(posedge din_valid) idx_buf = idx;

  task automatic set();
    for (int i = 0; i < DEPTH; i++) begin
      intensity_buf[i] = sim_helper_random.range(8'hFF, 0);
      phase_buf[i] = sim_helper_random.range(8'hFF, 0);
    end
    for (int i = 0; i < DEPTH; i++) begin
      @(posedge CLK_20P48M);
      din_valid <= 1'b1;
      intensity <= intensity_buf[i];
      phase <= phase_buf[i];
    end
    @(posedge CLK_20P48M);
    din_valid <= 1'b0;
  endtask

  task automatic check();
    while (1) begin
      @(posedge CLK_20P48M);
      if (dout_valid) begin
        break;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      if (intensity_out !== int'(intensity_buf[i]) * mod[(idx_buf-delay_m[i]+cycle_m+1)%(cycle_m+1)]) begin
        $error("Failed at %d: d=%d, m=%d, d_m=%d !== %d", i, intensity_buf[i],
               mod[(idx_buf-delay_m[i]+cycle_m+1)%(cycle_m+1)], intensity_out,
               int'(intensity_buf[i]) * mod[(idx_buf-delay_m[i]+cycle_m+1)%(cycle_m+1)]);
        $finish();
      end
      if (phase_out !== phase_buf[i]) begin
        $error("Failed at %d: p=%d, p_m=%d", i, phase_buf[i], phase_out);
        $finish();
      end

      @(posedge CLK_20P48M);
    end
  endtask

  initial begin
    din_valid = 0;
    cycle_m = 16'hFFFF;
    freq_div_m = 512;
    sim_helper_random.init();

    for (int i = 0; i < DEPTH; i++) begin
      delay_m[i] = sim_helper_random.range(cycle_m, 0);
    end

    @(posedge locked);

    #15000;

    for (int i = 0; i < cycle_m + 1; i++) begin
      mod[i] = sim_helper_random.range(8'hFF, 0);
    end
    sim_helper_bram.write_mod(mod, cycle_m + 1);

    for (int j = 0; j < 5000; j++) begin
      $display("check %d", j);
      fork
        set();
        check();
      join
    end

    $display("OK! sim_modulator");
    $finish();
  end

endmodule
