/*
 * File: sim_operator_stm_focus.sv
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

module sim_operator_stm_focus ();

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

  bit [WIDTH-1:0] cycle[DEPTH];
  bit [31:0] sound_speed;
  bit [15:0] cycle_stm;
  bit [31:0] freq_div_stm;
  bit [15:0] idx;
  bit dout_valid;

  bit signed [17:0] focus_x[65536];
  bit signed [17:0] focus_y[65536];
  bit signed [17:0] focus_z[65536];
  bit [3:0] duty_shift[65536];

  bit [WIDTH-1:0] duty;
  bit [WIDTH-1:0] phase;

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

  stm_focus_operator #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) stm_gain_operator (
      .CLK_L(CLK_20P48M),
      .IDX(idx),
      .TRIG_40KHZ(TRIG_40KHZ),
      .STM_BUS(stm_bus_if.focus_port),
      .SOUND_SPEED(sound_speed),
      .CYCLE(cycle),
      .DUTY(duty),
      .PHASE(phase),
      .DOUT_VALID(dout_valid)
  );

  bit [15:0] idx_buf;
  always @(posedge CLK_20P48M) begin
    if (TRIG_40KHZ) idx_buf = idx;
  end

  initial begin
    automatic int id = 0;
    stm_bus_if.STM_GAIN_MODE = 1'b0;
    sim_helper_random.init();

    for (int i = 0; i < DEPTH; i++) begin
      cycle[i] = sim_helper_random.range(8000, 2000);
    end

    sound_speed = 340 * 1024;
    cycle_stm = 65536 - 1;
    freq_div_stm = 4096;

    @(posedge locked);

    for (int i = 0; i < cycle_stm + 1; i++) begin
      $display("write %d/%d", i + 1, cycle_stm + 1);
      focus_x[i] = sim_helper_random.range(131071, -131072 + 6908);
      focus_y[i] = sim_helper_random.range(131071, -131072 + 5283);
      focus_z[i] = sim_helper_random.range(131071, -131072);
      duty_shift[i] = sim_helper_random.range(15, 0);
      sim_helper_bram.write_stm_focus(i, focus_x[i], focus_y[i], focus_z[i], duty_shift[i]);
    end

    for (int j = 0; j < cycle_stm + 1; j++) begin
      while (1) begin
        @(posedge CLK_20P48M);
        if (dout_valid) begin
          break;
        end
      end
      $display("check %d @%d", idx_buf, SYS_TIME);
      id = 0;
      for (int iy = 0; iy < 14; iy++) begin
        automatic bit signed [63:0] y = focus_y[idx_buf] - $rtoi(10.16 * iy / 0.025);  // [0.025mm]
        for (int ix = 0; ix < 18; ix++) begin
          automatic bit signed [63:0] x, z;
          automatic bit [63:0] r, lambda;
          automatic int p;
          if ((iy == 1) && (ix == 1 || ix == 2 || ix == 16)) begin
            continue;
          end
          x = focus_x[idx_buf] - $rtoi(10.16 * ix / 0.025);  // [0.025mm]
          z = focus_z[idx_buf];  // [0.025mm]
          r = $rtoi($sqrt(x * x + y * y + z * z));  // [0.025mm]
          lambda = (r << 22) / sound_speed;
          p = lambda % cycle[id];
          if (duty != (cycle[id] >> (1 + duty_shift[idx_buf]))) begin
            $error("Failed at d_out=%d, d_in=%d @%d", duty, cycle[id] >> (1 + duty_shift[idx_buf]),
                   id);
            $finish();
          end
          if (phase != p) begin
            $error("Failed at p_out=%d, p_in=%d (r2=%d, r=%d, lambda=%d, cycle=%d) @%d", phase, p,
                   x * x + y * y + z * z, r, lambda, cycle[id], id);
            $error("x=%d, y=%d, z=%d", x, y, z);
            $finish();
          end
          @(posedge CLK_20P48M);
          id = id + 1;
        end
      end
    end

    $display("OK!");
    $finish();
  end

endmodule
