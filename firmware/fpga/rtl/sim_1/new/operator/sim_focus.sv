/*
 * File: sim_gain.sv
 * Project: stm
 * Created Date: 13/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
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

  bit [WIDTH-1:0] cycle[0:DEPTH-1];
  bit [31:0] sound_speed;
  bit [15:0] cycle_s;
  bit [31:0] freq_div_s;

  bit start, done;
  bit [15:0] idx;

  bit signed [17:0] focus_x[0:65535];
  bit signed [17:0] focus_y[0:65535];
  bit signed [17:0] focus_z[0:65535];
  bit [3:0] duty_shift[0:65535];

  bit [WIDTH-1:0] duty[0:DEPTH-1];
  bit [WIDTH-1:0] phase[0:DEPTH-1];

  stm_operator #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) stm_operator (
      .CLK(CLK_20P48M),
      .SYS_TIME(SYS_TIME),
      .ULTRASOUND_CYCLE(cycle),
      .CYCLE(cycle_s),
      .FREQ_DIV(freq_div_s),
      .SOUND_SPEED(sound_speed),
      .STM_GAIN_MODE(1'b0),
      .CPU_BUS(sim_helper_bram.cpu_bus.stm_port),
      .DUTY(duty),
      .PHASE(phase),
      .START(start),
      .DONE(done),
      .IDX(idx)
  );

  bit [15:0] idx_buf;
  initial begin
    sim_helper_random.init();
    for (int i = 0; i < DEPTH; i++) begin
      cycle[i] = sim_helper_random.range(8000, 2000);
    end
    // cycle = '{DEPTH{4096}};
    sound_speed = 340 * 1024;
    cycle_s = 65536 - 1;
    freq_div_s = 8 * (1 + 18 + 66 + 66 + DEPTH + 3);
    @(posedge locked);

    for (int i = 0; i < cycle_s + 1; i++) begin
      focus_x[i] = sim_helper_random.range(131071, -131072 + 6908);
      focus_y[i] = sim_helper_random.range(131071, -131072 + 5283);
      focus_z[i] = sim_helper_random.range(131071, -131072);
      duty_shift[i] = sim_helper_random.range(15, 0);
      sim_helper_bram.write_stm_focus(i, focus_x[i], focus_y[i], focus_z[i], duty_shift[i]);
    end

    for (int j = 0; j < cycle_s + 1; j++) begin
      automatic int i = 0;
      @(posedge start);
      idx_buf = idx;
      $display("check %d @%d", idx_buf, SYS_TIME);
      @(posedge done);
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
          p = lambda % cycle[i];
          if (duty[i] != (cycle[i] >> duty_shift[idx_buf])) begin
            $error("Failed at d_out=%d, d_in=%d @%d", duty[i], cycle[i] >> duty_shift[idx_buf], i);
            $finish();
          end
          if (phase[i] != p) begin
            $error("Failed at p_out=%d, p_in=%d (r2=%d, r=%d, lambda=%d, cycle=%d) @%d", phase[i],
                   p, x * x + y * y + z * z, r, lambda, cycle[i], i);
            $error("x=%d, y=%d, z=%d", x, y, z);
            $finish();
          end
          i += 1;
        end
      end
    end

    $display("OK!");
    $finish();
  end

endmodule
