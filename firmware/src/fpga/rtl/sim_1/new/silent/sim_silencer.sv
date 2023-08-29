/*
 * File: sim_silencer.sv
 * Project: silent
 * Created Date: 22/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module sim_silencer ();

  parameter int WIDTH = 13;
  parameter int DEPTH = 249;

  bit CLK_20P48M;
  bit locked;
  sim_helper_clk sim_helper_clk (
      .CLK_163P84M(),
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME()
  );

  sim_helper_random sim_helper_random ();

  bit [WIDTH-1:0] step;
  bit [WIDTH-1:0] cycle[DEPTH];
  bit [WIDTH-1:0] duty;
  bit [WIDTH-1:0] phase;
  bit [WIDTH-1:0] duty_s;
  bit [WIDTH-1:0] phase_s;
  bit din_valid, dout_valid;

  bit [WIDTH-1:0] duty_buf [DEPTH];
  bit [WIDTH-1:0] phase_buf[DEPTH];
  bit [WIDTH-1:0] duty_s_buf [DEPTH];
  bit [WIDTH-1:0] phase_s_buf[DEPTH];

  silencer #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) silencer (
      .CLK(CLK_20P48M),
      .DIN_VALID(din_valid),
      .STEP(step),
      .CYCLE(cycle),
      .DUTY(duty),
      .PHASE(phase),
      .DUTY_S(duty_s),
      .PHASE_S(phase_s),
      .DOUT_VALID(dout_valid)
  );

  localparam int MaxCycle = 8191;
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
      duty_s_buf[i] = duty_s;
      phase_s_buf[i] = phase_s;
      @(posedge CLK_20P48M);
    end
  endtask

  task automatic check();
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s_buf[i] !== phase_buf[i]) begin
        $display("ASSERTION FAILED: PHASE(%d) != PHASE_S(%d) in %d-th transducer, cycle = %d, step = %d", phase_buf[i],
                 phase_s_buf[i], i, cycle[i], step);
        $finish;
      end
      if (duty_s_buf[i] !== duty_buf[i]) begin
        $display("ASSERTION FAILED: DUTY(%d) != DUTY_S(%d) in %d-th transducer, cycle = %d, step = %d", duty_buf[i],
                 duty_s_buf[i], i, cycle[i], step);
        $finish;
      end
    end
  endtask

  initial begin  
    sim_helper_random.init();

    for (int i = 0; i < DEPTH; i++) begin
      cycle[i] = 2000;
    end

    @(posedge locked);

    //////////////// Manual check ////////////////
    step = 1;
    cycle[0] = 2;
    cycle[10] = 4096;
    cycle[20] = MaxCycle;
    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 1;
      duty_buf[i]  = 1;
    end
    fork
      set();
      wait_calc();
    join
    if (phase_s_buf[0] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[0] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[10] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[10] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[20] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[20] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = cycle[i] - 1;
      duty_buf[i]  = cycle[i];
    end
    fork
      set();
      wait_calc();
    join
    if (phase_s_buf[0] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[0] !== 2) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[10] !== 0) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[10] !== 2) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[20] !== 0) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[20] !== 2) begin $display("ASSERTION FAILED"); $finish; end
    fork
      set();
      wait_calc();
    join
    if (phase_s_buf[0] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[0] !== 2) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[10] !== 4095) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[10] !== 3) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[20] !== MaxCycle - 1) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[20] !== 3) begin $display("ASSERTION FAILED"); $finish; end

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
      if (phase_s_buf[i] !== 0) begin $display("ASSERTION FAILED"); $finish; end
      if (duty_s_buf[i] !== 0) begin $display("ASSERTION FAILED"); $finish; end
    end
    //////////////// Manual check ////////////////

    //////////////// Manual check ////////////////
    step = 10;
    cycle[0] = 20;
    cycle[1] = 19;
    cycle[2] = 21;
    cycle[3] = 22;
    cycle[4] = 23;
    cycle[10] = 4096;
    cycle[20] = MaxCycle;
    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 1;
      duty_buf[i]  = 1;
    end
    fork
      set();
      wait_calc();
    join
    if (phase_s_buf[0] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[0] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[1] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[1] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[2] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[2] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[3] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[3] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[4] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[4] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[10] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[10] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[20] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[20] !== 1) begin $display("ASSERTION FAILED"); $finish; end
    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = cycle[i] - 1;
      duty_buf[i]  = cycle[i];
    end
    fork
      set();
      wait_calc();
    join
    if (phase_s_buf[0] !== 19) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[0] !== 11) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[1] !== 18) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[1] !== 11) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[2] !== 20) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[2] !== 11) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[3] !== 21) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[3] !== 11) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[4] !== 22) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[4] !== 11) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[10] !== 4095) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[10] !== 11) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[20] !== MaxCycle - 1) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[20] !== 11) begin $display("ASSERTION FAILED"); $finish; end
    
    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = 10;
      duty_buf[i]  = 10;
    end
    fork
      set();
      wait_calc();
    join
    if (phase_s_buf[0] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[0] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[1] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[1] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[2] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[2] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[3] !== 11) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[3] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[4] !== 12) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[4] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[10] !== 9) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[10] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[20] !== 9) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[20] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    fork
      set();
      wait_calc();
    join
    if (phase_s_buf[0] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[0] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[1] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[1] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[2] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[2] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[3] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[3] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[4] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[4] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[10] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[10] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[20] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[20] !== 10) begin $display("ASSERTION FAILED"); $finish; end
    
    step = 5;
    for (int i = 0; i < DEPTH; i++) begin
      phase_buf[i] = cycle[i] - 1;
      duty_buf[i]  = cycle[i];
    end
    fork
      set();
      wait_calc();
    join
    if (phase_s_buf[0] !== 15) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[0] !== 15) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[1] !== 15) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[1] !== 15) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[2] !== 15) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[2] !== 15) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[3] !== 15) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[3] !== 15) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[4] !== 5) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[4] !== 15) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[10] !== 5) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[10] !== 15) begin $display("ASSERTION FAILED"); $finish; end
    if (phase_s_buf[20] !== 5) begin $display("ASSERTION FAILED"); $finish; end
    if (duty_s_buf[20] !== 15) begin $display("ASSERTION FAILED"); $finish; end

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
      if (phase_s_buf[i] !== 0) begin $display("ASSERTION FAILED"); $finish; end
      if (duty_s_buf[i] !== 0) begin $display("ASSERTION FAILED"); $finish; end
    end
    //////////////// Manual check ////////////////

    // from random to random with random step
    for (int i = 0; i < DEPTH; i++) begin
      cycle[i] = sim_helper_random.range(MaxCycle, 2000);
    end
    repeat (100) begin
      step = sim_helper_random.range(MaxCycle, 2);
      n_repeat = int'(MaxCycle / step / 2) + 1;
      for (int i = 0; i < DEPTH; i++) begin
        duty_buf[i]  = sim_helper_random.range(cycle[i] / 2, 0);
        phase_buf[i] = sim_helper_random.range(cycle[i] - 1, 0);
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
      cycle[i] = sim_helper_random.range(MaxCycle, 2000);
    end

    for (int i = 0; i < DEPTH; i++) begin
      duty_buf[i]  = sim_helper_random.range(cycle[i] / 2, 0);
      phase_buf[i] = sim_helper_random.range(cycle[i] - 1, 0);
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
