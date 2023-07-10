/*
 * File: sim_modulation_multiplier.sv
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

module sim_modulation_multiplier ();

  bit CLK_20P48M;
  bit locked;
  sim_helper_clk sim_helper_clk (
      .CLK_163P84M(),
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME()
  );

  sim_helper_random sim_helper_random ();
  sim_helper_bram sim_helper_bram ();

  localparam int WIDTH = 13;
  localparam int DEPTH = 249;

  bit din_valid, dout_valid;
  bit [15:0] idx;
  bit [15:0] cycle_m;
  bit [15:0] delay_m[DEPTH];
  bit [WIDTH-1:0] duty;
  bit [WIDTH-1:0] duty_out;
  bit [WIDTH-1:0] phase;
  bit [WIDTH-1:0] phase_out;

  bit [7:0] mod[65536];
  bit [WIDTH-1:0] duty_buf[DEPTH];
  bit [WIDTH-1:0] phase_buf[DEPTH];
  bit [15:0] idx_buf;

  modulation_bus_if m_bus ();

  modulation_memory modulation_memory (
      .CLK(CLK_20P48M),
      .CPU_BUS(sim_helper_bram.cpu_bus.mod_port),
      .M_BUS(m_bus.memory_port)
  );

  modulation_multiplier #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) modulation_multiplier (
      .CLK(CLK_20P48M),
      .CYCLE_M(cycle_m),
      .DIN_VALID(din_valid),
      .IDX(idx),
      .M_BUS(m_bus.sampler_port),
      .DELAY_M(delay_m),
      .DUTY_IN(duty),
      .PHASE_IN(phase),
      .DUTY_OUT(duty_out),
      .PHASE_OUT(phase_out),
      .DOUT_VALID(dout_valid)
  );

  always @(posedge CLK_20P48M) idx = idx + 1'b1;

  always @(posedge din_valid) idx_buf = idx;

  task automatic set();
    for (int i = 0; i < DEPTH; i++) begin
      @(posedge CLK_20P48M);
      din_valid = 1'b1;
      duty = sim_helper_random.range(8000, 0);
      phase = sim_helper_random.range(8000, 0);
      duty_buf[i] = duty;
      phase_buf[i] = phase;
    end
    @(posedge CLK_20P48M);
    din_valid = 1'b0;
  endtask

  task automatic check();
    while (1) begin
      @(posedge CLK_20P48M);
      if (dout_valid) begin
        break;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      if (duty_out != (duty_buf[i] * mod[(idx_buf-delay_m[i]+cycle_m+1)%(cycle_m+1)] / 255)) begin
        $error("Failed at %d: d=%d, m=%d, d_m=%d", i, duty_buf[i],
               mod[(idx_buf-delay_m[i]+cycle_m+1)%(cycle_m+1)], duty_out);
        $finish();
      end
      if (phase_out != phase_buf[i]) begin
        $error("Failed at %d: p=%d, p_m=%d", i, phase_buf[i], phase_out);
        $finish();
      end

      @(posedge CLK_20P48M);
    end
  endtask

  initial begin
    din_valid = 0;
    cycle_m   = 16'hFFFF;
    sim_helper_random.init();

    for (int i = 0; i < DEPTH; i++) begin
      delay_m[i] = sim_helper_random.range(16'hFFFF, 0);
    end

    @(posedge locked);

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

    $display("OK! sim_modulation_multiplier");
    $finish();
  end

endmodule
