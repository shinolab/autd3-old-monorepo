/*
 * File: sim_silencer.sv
 * Project: silent
 * Created Date: 22/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module sim_silencer ();

  parameter int DEPTH = 249;

  logic CLK_20P48M;
  logic locked;
  sim_helper_clk sim_helper_clk (
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME()
  );

  sim_helper_random sim_helper_random ();

  logic [15:0] step_intensity, step_phase;
  logic [15:0] intensity;
  logic [ 7:0] phase;
  logic [15:0] intensity_s;
  logic [ 7:0] phase_s;
  logic din_valid, dout_valid;

  logic [15:0] intensity_buf[DEPTH];
  logic [7:0] phase_buf[DEPTH];
  logic [15:0] intensity_s_buf[DEPTH];
  logic [7:0] phase_s_buf[DEPTH];

  silencer #(
      .DEPTH(DEPTH)
  ) silencer (
      .CLK(CLK_20P48M),
      .DIN_VALID(din_valid),
      .STEP_INTENSITY(step_intensity),
      .STEP_PHASE(step_phase),
      .INTENSITY_IN(intensity),
      .PHASE_IN(phase),
      .INTENSITY_OUT(intensity_s),
      .PHASE_OUT(phase_s),
      .DOUT_VALID(dout_valid)
  );

  int n_repeat;

  task automatic set();
    for (int i = 0; i < DEPTH; i++) begin
      @(posedge CLK_20P48M);
      din_valid <= 1'b1;
      intensity <= intensity_buf[i];
      phase <= phase_buf[i];
    end
    @(posedge CLK_20P48M);
    din_valid = 1'b0;
  endtask

  task automatic wait_calc();
    while (1) begin
      @(posedge CLK_20P48M);
      if (dout_valid) begin
        break;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      intensity_s_buf[i] = intensity_s;
      phase_s_buf[i] = phase_s;
      @(posedge CLK_20P48M);
    end
  endtask

  task automatic check_manual(logic [15:0] expect_intensity, logic [7:0] expect_phase);
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== expect_phase) begin
        $display("ERR: PHASE(%d) !== %d in %d-th transducer, step = %d", phase_buf[i],
                 expect_phase, i, step_phase);
        $finish;
      end
      if (intensity_s_buf[i] !== expect_intensity) begin
        $display("ERR: INTENSITY(%d) !== %d in %d-th transducer, step = %d", intensity_buf[i],
                 expect_intensity, i, step_intensity);
        $finish;
      end
    end
  endtask

  task automatic check();
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== phase_buf[i]) begin
        $display("ERR: PHASE(%d) !== PHASE_S(%d) in %d-th transducer, step = %d", phase_buf[i],
                 phase_s_buf[i], i, step_phase);
        $finish;
      end
      if (intensity_s_buf[i] !== intensity_buf[i]) begin
        $display("ERR: INTENSITY(%d) !== INTENSITY_S(%d) in %d-th transducer, step = %d",
                 intensity_buf[i], intensity_s_buf[i], i, step_intensity);
        $finish;
      end
    end
  endtask

  initial begin
    din_valid = 0;
    step_intensity = 0;
    step_phase = 0;
    phase = 0;
    intensity = 0;
    sim_helper_random.init();

    @(posedge locked);

    //////////////// Manual check ////////////////
    step_intensity = 1;
    step_phase = 256;

    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 1;
      intensity_buf[i] = 1;
    end
    fork
      set();
      wait_calc();
    join
    check_manual(1, 1);

    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 255;
      intensity_buf[i] = 256;
    end
    fork
      set();
      wait_calc();
    join
    check_manual(2, 0);

    fork
      set();
      wait_calc();
    join
    check_manual(3, 255);

    step_intensity = 16'd65535;
    step_phase = 16'd65535;
    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 0;
      intensity_buf[i] = 0;
    end
    fork
      set();
      wait_calc();
    join
    check_manual(0, 0);

    // Full jump
    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 128;
      intensity_buf[i] = 255 * 255;
    end
    fork
      set();
      wait_calc();
    join
    check_manual(255 * 255, 128);

    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 0;
      intensity_buf[i] = 0;
    end
    fork
      set();
      wait_calc();
    join
    check_manual(0, 0);
    //////////////// Manual check ////////////////

    // from random to random with random step
    for (int i = 0; i < 100; i++) begin
      $display("Random test %d/100", i);
      step_intensity = sim_helper_random.range(65535, 1);
      step_phase = sim_helper_random.range(65535, 1);
      n_repeat = step_intensity < step_phase ? int'(65536 / step_intensity) + 1 : int'(65536 / step_phase) + 1;
      for (int i = 0; i < DEPTH; i++) begin
        intensity_buf[i] = sim_helper_random.range(255 * 255, 0);
        phase_buf[i] = sim_helper_random.range(255, 0);
      end
      repeat (n_repeat) begin
        fork
          set();
          wait_calc();
        join
      end
      fork
        set();
        wait_calc();
        check();
      join
    end

    // disable
    step_intensity = 16'd65535;
    step_phase = 16'd65535;
    n_repeat = 1;

    for (int i = 0; i < DEPTH; i++) begin
      intensity_buf[i] = sim_helper_random.range(255 * 255, 0);
      phase_buf[i] = sim_helper_random.range(255, 0);
    end
    repeat (n_repeat) begin
      fork
        set();
        wait_calc();
      join
    end
    fork
      set();
      wait_calc();
      check();
    join

    $display("Ok! sim_silencer");
    $finish;
  end

endmodule
