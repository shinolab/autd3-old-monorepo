/*
 * File: sim_operator_stm_focus.sv
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

module sim_operator_stm_focus ();

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

  logic [31:0] sound_speed;
  logic [15:0] cycle_stm;
  logic [31:0] freq_div_stm;
  logic [15:0] idx;
  logic dout_valid;

  logic signed [17:0] focus_x[65536];
  logic signed [17:0] focus_y[65536];
  logic signed [17:0] focus_z[65536];
  logic [7:0] intensity_buf[65536];

  logic [7:0] intensity;
  logic [7:0] phase;

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
      .SOUND_SPEED(sound_speed),
      .STM_GAIN_MODE(1'b0),
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
    automatic int id = 0;
    sim_helper_random.init();

    sound_speed = 340 * 1024;
    cycle_stm = 65536 - 1;
    freq_div_stm = 512;

    @(posedge locked);

    for (int i = 0; i < cycle_stm + 1; i++) begin
      $display("write %d/%d", i + 1, cycle_stm + 1);
      focus_x[i] = sim_helper_random.range(131071, -131072 + 6908);
      focus_y[i] = sim_helper_random.range(131071, -131072 + 5283);
      focus_z[i] = sim_helper_random.range(131071, -131072);
      intensity_buf[i] = sim_helper_random.range(8'hFF, 0);
      sim_helper_bram.write_stm_focus(i, focus_x[i], focus_y[i], focus_z[i], intensity_buf[i]);
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
      $display("check %d @%d", idx_buf, SYS_TIME);
      id = 0;
      for (int iy = 0; iy < 14; iy++) begin
        automatic
        logic signed [63:0]
        y = focus_y[idx_buf] - $rtoi(
            10.16 * iy / 0.025
        );  // [0.025mm]
        for (int ix = 0; ix < 18; ix++) begin
          automatic logic signed [63:0] x, z;
          automatic logic [63:0] r, lambda;
          automatic int p;
          if ((iy === 1) && (ix === 1 || ix === 2 || ix === 16)) begin
            continue;
          end
          x = focus_x[idx_buf] - $rtoi(10.16 * ix / 0.025);  // [0.025mm]
          z = focus_z[idx_buf];  // [0.025mm]
          r = $rtoi($sqrt(x * x + y * y + z * z));  // [0.025mm]
          lambda = (r << 18) / sound_speed;
          p = lambda % 256;
          if (intensity !== intensity_buf[idx_buf]) begin
            $error("Failed at d_out=%d, d_in=%d @%d", intensity, intensity_buf[idx_buf], id);
            $finish();
          end
          if (phase !== p) begin
            $error("Failed at p_out=%d, p_in=%d (r2=%d, r=%d, lambda=%d) @%d", phase, p,
                   x * x + y * y + z * z, r, lambda, id);
            $error("x=%d, y=%d, z=%d", x, y, z);
            $finish();
          end
          @(posedge CLK_20P48M);
          id = id + 1;
        end
      end
    end

    $display("OK! sim_operator_stm_focus");
    $finish();
  end

endmodule
