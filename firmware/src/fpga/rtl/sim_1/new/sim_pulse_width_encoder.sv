/*
 * File: sim_pulse_width_encoder.sv
 * Project: pulse_width_encoder
 * Created Date: 17/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */


module sim_pulse_width_encoder ();
  `define M_PI 3.14159265358979323846

  logic CLK_20P48M;
  logic locked;
  sim_helper_clk sim_helper_clk (
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME()
  );

  sim_helper_random sim_helper_random ();
  sim_helper_bram sim_helper_bram ();

  localparam int DEPTH = 249;

localparam int MAX = 255*255;

  logic din_valid, dout_valid;
  logic [15:0] intensity_in;
  logic [8:0] pulse_width_out;
  logic [7:0] phase;
  logic [7:0] phase_out;

  logic [15:0] intensity_buf[DEPTH];
  logic [7:0] phase_buf[DEPTH];

  pulse_width_encoder #(
      .DEPTH(DEPTH)
  ) pulse_width_encoder (
      .CLK(CLK_20P48M),
      .DIN_VALID(din_valid),
      .INTENSITY_IN(intensity_in),
      .PHASE_IN(phase),
      .PULSE_WIDTH_OUT(pulse_width_out),
      .PHASE_OUT(phase_out),
      .DOUT_VALID(dout_valid)
  );

  task automatic set(logic [15:0] intensity[DEPTH], logic [7:0] p[DEPTH]);
    for (int i = 0; i < DEPTH; i++) begin
      intensity_buf[i] = intensity[i];
      phase_buf[i] = p[i];
    end
    for (int i = 0; i < DEPTH; i++) begin
      @(posedge CLK_20P48M);
      din_valid <= 1'b1;
      intensity_in <= intensity_buf[i];
      phase <= phase_buf[i];
    end
    @(posedge CLK_20P48M);
    din_valid <= 1'b0;
  endtask

  task automatic set_random();
    for (int i = 0; i < DEPTH; i++) begin
      intensity_buf[i] = sim_helper_random.range(MAX, 0);
      phase_buf[i] = sim_helper_random.range(8'hFF, 0);
    end
    for (int i = 0; i < DEPTH; i++) begin
      @(posedge CLK_20P48M);
      din_valid <= 1'b1;
      intensity_in <= intensity_buf[i];
      phase <= phase_buf[i];
    end
    @(posedge CLK_20P48M);
    din_valid <= 1'b0;
  endtask

  task automatic check();
    real intensity_real;
    while (1) begin
      @(posedge CLK_20P48M);
      if (dout_valid) begin
        break;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      intensity_real = $itor(intensity_buf[i]) / 255.0 / 255.0;
      if (pulse_width_out !== int'(($asin(intensity_real) * 2.0 / `M_PI * 256.0))) begin
        $error("Failed at %d: i=%d, d=%d, d_m=%d", i, intensity_buf[i],
               $rtoi(($asin(intensity_real) * 2.0 / `M_PI * 256.0)), pulse_width_out);
        $finish();
      end
      if (phase_out !== phase_buf[i]) begin
        $error("Failed at %d: p=%d, p_m=%d", i, phase_buf[i], phase_out);
        $finish();
      end

      @(posedge CLK_20P48M);
    end
  endtask

  logic [15:0] intensity_tmp[DEPTH];
  logic [7:0] phase_tmp[DEPTH];
  initial begin
    din_valid = 0;
    sim_helper_random.init();

    @(posedge locked);

    for (int j = 0; j < 1000; j++) begin
      $display("check %d", j);
      fork
        set_random();
        check();
      join
    end

    for (int i = 0; i <= MAX;) begin
      $display("check %d/%d", i, MAX);
      for (int j = i; j < i + DEPTH; j++) begin
        intensity_tmp[j-i] = j <= MAX ? j : 0;
        phase_tmp[j-i] = sim_helper_random.range(8'hFF, 0);
      end
      fork
        set(intensity_tmp, phase_tmp);
        check();
      join
      i += DEPTH;
    end

    $display("OK! sim_pulse_width_encoder");
    $finish();
  end

endmodule
