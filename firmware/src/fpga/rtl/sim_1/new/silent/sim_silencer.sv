/*
 * File: sim_silencer.sv
 * Project: silent
 * Created Date: 22/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
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

  logic [8:0] step;
  logic [8:0] pulse_width;
  logic [7:0] phase;
  logic [8:0] pulse_width_s;
  logic [7:0] phase_s;
  logic din_valid, dout_valid;

  logic [8:0] pulse_width_buf[DEPTH];
  logic [7:0] phase_buf[DEPTH];
  logic [8:0] pulse_width_s_buf[DEPTH];
  logic [7:0] phase_s_buf[DEPTH];

  silencer #(
      .DEPTH(DEPTH)
  ) silencer (
      .CLK(CLK_20P48M),
      .DIN_VALID(din_valid),
      .STEP(step),
      .PULSE_WIDTH_IN(pulse_width),
      .PHASE_IN(phase),
      .PULSE_WIDTH_OUT(pulse_width_s),
      .PHASE_OUT(phase_s),
      .DOUT_VALID(dout_valid)
  );

  int n_repeat;

  task automatic set();
    for (int i = 0; i < DEPTH; i++) begin
      @(posedge CLK_20P48M);
      din_valid <= 1'b1;
      pulse_width <= pulse_width_buf[i];
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
      pulse_width_s_buf[i] = pulse_width_s;
      phase_s_buf[i] = phase_s;
      @(posedge CLK_20P48M);
    end
  endtask

  task automatic check();
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== phase_buf[i]) begin
        $display("ERR: PHASE(%d) !== PHASE_S(%d) in %d-th transducer, step = %d", phase_buf[i],
                 phase_s_buf[i], i, step);
        $finish;
      end
      if (pulse_width_s_buf[i] !== pulse_width_buf[i]) begin
        $display("ERR: PULSE_WIDTH(%d) !== PULSE_WIDTH_S(%d) in %d-th transducer, step = %d",
                 pulse_width_buf[i], pulse_width_s_buf[i], i, step);
        $finish;
      end
    end
  endtask

  initial begin
    din_valid = 0;
    step = 0;
    phase = 0;
    pulse_width = 0;
    sim_helper_random.init();

    @(posedge locked);

    //////////////// Manual check ////////////////
    step = 1;

    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 1;
      pulse_width_buf[i] = 1;
    end
    fork
      set();
      wait_calc();
    join
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== 1) begin
        $display("ASSERTION FAILED");
        $finish;
      end
      if (pulse_width_s_buf[i] !== 1) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 255;
      pulse_width_buf[i] = 256;
    end
    fork
      set();
      wait_calc();
    join
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== 0) begin
        $display("ASSERTION FAILED");
        $finish;
      end
      if (pulse_width_s_buf[i] !== 2) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end

    fork
      set();
      wait_calc();
    join
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== 255) begin
        $display("ASSERTION FAILED");
        $finish;
      end
      if (pulse_width_s_buf[i] !== 3) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end

    step = 16'd256;
    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 0;
      pulse_width_buf[i] = 0;
    end

    fork
      set();
      wait_calc();
    join
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== 0) begin
        $display("ASSERTION FAILED");
        $finish;
      end
      if (pulse_width_s_buf[i] !== 0) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end
    //////////////// Manual check ////////////////

    //////////////// Manual check ////////////////
    step = 10;

    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 1;
      pulse_width_buf[i] = 1;
    end
    fork
      set();
      wait_calc();
    join
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== 1) begin
        $display("ASSERTION FAILED");
        $finish;
      end
      if (pulse_width_s_buf[i] !== 1) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 50;
      pulse_width_buf[i] = 50;
    end
    fork
      set();
      wait_calc();
    join
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== 11) begin
        $display("ASSERTION FAILED");
        $finish;
      end
      if (pulse_width_s_buf[i] !== 11) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 255;
      pulse_width_buf[i] = 256;
    end
    fork
      set();
      wait_calc();
    join
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== 1) begin
        $display("ASSERTION FAILED");
        $finish;
      end
      if (pulse_width_s_buf[i] !== 21) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end

    fork
      set();
      wait_calc();
    join
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== 255) begin
        $display("ASSERTION FAILED");
        $finish;
      end
      if (pulse_width_s_buf[i] !== 31) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end

    step = 16'd256;
    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 0;
      pulse_width_buf[i] = 0;
    end

    fork
      set();
      wait_calc();
    join
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== 0) begin
        $display("ASSERTION FAILED");
        $finish;
      end
      if (pulse_width_s_buf[i] !== 0) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end
    //////////////// Manual check ////////////////

    // from random to random with random step
    repeat (100) begin
      step = sim_helper_random.range(256, 1);
      n_repeat = int'(256 / step) + 1;
      for (int i = 0; i < DEPTH; i++) begin
        pulse_width_buf[i] = sim_helper_random.range(256, 0);
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
    step = 16'd256;
    n_repeat = 1;

    for (int i = 0; i < DEPTH; i++) begin
      pulse_width_buf[i] = sim_helper_random.range(256, 0);
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
