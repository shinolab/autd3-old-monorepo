/*
 * File: sim_silencer.sv
 * Project: silent
 * Created Date: 22/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module sim_silencer ();

  parameter int WIDTH = 9;
  parameter int DEPTH = 249;

  bit CLK_20P48M;
  bit locked;
  sim_helper_clk sim_helper_clk (
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME()
  );

  sim_helper_random sim_helper_random ();

  bit [WIDTH-1:0] step;
  bit [WIDTH-1:0] duty;
  bit [WIDTH-1:0] phase;
  bit [WIDTH-1:0] duty_s;
  bit [WIDTH-1:0] phase_s;
  bit din_valid, dout_valid;

  bit [WIDTH-1:0] duty_buf[DEPTH];
  bit [WIDTH-1:0] phase_buf[DEPTH];
  bit [WIDTH-1:0] duty_s_buf[DEPTH];
  bit [WIDTH-1:0] phase_s_buf[DEPTH];

  silencer #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) silencer (
      .CLK(CLK_20P48M),
      .DIN_VALID(din_valid),
      .STEP(step),
      .DUTY(duty),
      .PHASE(phase),
      .DUTY_S(duty_s),
      .PHASE_S(phase_s),
      .DOUT_VALID(dout_valid)
  );

  int n_repeat;

  task automatic set();
    for (int i = 0; i < DEPTH; i++) begin
      @(posedge CLK_20P48M);
      din_valid = 1'b1;
      duty = duty_buf[i];
      phase = phase_buf[i];
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
      duty_s_buf[i]  = duty_s;
      phase_s_buf[i] = phase_s;
      @(posedge CLK_20P48M);
    end
  endtask

  task automatic check();
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== phase_buf[i]) begin
        $display("ASSERTION FAILED: PHASE(%d) != PHASE_S(%d) in %d-th transducer, step = %d",
                 phase_buf[i], phase_s_buf[i], i, step);
        $finish;
      end
      if (duty_s_buf[i] !== duty_buf[i]) begin
        $display("ASSERTION FAILED: DUTY(%d) != DUTY_S(%d) in %d-th transducer,  step = %d",
                 duty_buf[i], duty_s_buf[i], i, step);
        $finish;
      end
    end
  endtask

  initial begin
    sim_helper_random.init();

    @(posedge locked);

    //////////////// Manual check ////////////////
    step = 1;

    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 1;
      duty_buf[i]  = 1;
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
      if (duty_s_buf[i] !== 1) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 511;
      duty_buf[i]  = 511;
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
      if (duty_s_buf[i] !== 2) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end

    fork
      set();
      wait_calc();
    join
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== 511) begin
        $display("ASSERTION FAILED");
        $finish;
      end
      if (duty_s_buf[i] !== 3) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end

    step = 16'hFFFF;
    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 0;
      duty_buf[i]  = 0;
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
      if (duty_s_buf[i] !== 0) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end
    //////////////// Manual check ////////////////

    //////////////// Manual check ////////////////
    step = 10;

    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 1;
      duty_buf[i]  = 1;
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
      if (duty_s_buf[i] !== 1) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 50;
      duty_buf[i]  = 50;
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
      if (duty_s_buf[i] !== 11) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 511;
      duty_buf[i]  = 511;
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
      if (duty_s_buf[i] !== 21) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end

    fork
      set();
      wait_calc();
    join
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== 511) begin
        $display("ASSERTION FAILED");
        $finish;
      end
      if (duty_s_buf[i] !== 31) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end

    step = 16'hFFFF;
    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 0;
      duty_buf[i]  = 0;
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
      if (duty_s_buf[i] !== 0) begin
        $display("ASSERTION FAILED");
        $finish;
      end
    end
    //////////////// Manual check ////////////////

    // from random to random with random step
    repeat (100) begin
      step = sim_helper_random.range(512, 2);
      n_repeat = int'(512 / step) + 1;
      for (int i = 0; i < DEPTH; i++) begin
        duty_buf[i]  = sim_helper_random.range(511, 0);
        phase_buf[i] = sim_helper_random.range(511, 0);
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
    step = 16'hFFFF;
    n_repeat = 1;

    for (int i = 0; i < DEPTH; i++) begin
      duty_buf[i]  = sim_helper_random.range(511, 0);
      phase_buf[i] = sim_helper_random.range(511, 0);
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
