/*
 * File: sim_silencer.sv
 * Project: silent
 * Created Date: 22/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/08/2023
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

  localparam int MaxCycle = 8000;
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
      @(posedge CLK_20P48M);
    end
  endtask

  task automatic check();
    while (1) begin
      @(posedge CLK_20P48M);
      if (dout_valid) begin
        break;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s !== phase_buf[i]) begin
        $display("ASSERTION FAILED: PHASE(%d) != PHASE_S(%d) in %d-th transducer", phase_buf[i],
                 phase_s, i);
        $finish;
      end
      if (duty_s !== duty_buf[i]) begin
        $display("ASSERTION FAILED: DUTY(%d) != DUTY_S(%d) in %d-th transducer", duty_buf[i],
                 duty_s, i);
        $finish;
      end
      @(posedge CLK_20P48M);
    end
  endtask

  initial begin  
    sim_helper_random.init();

    @(posedge locked);

    // from 0 to random
    step = 100;
    n_repeat = int'(MaxCycle / step / 2);
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
      check();
    join

    // from random to random with random step
    repeat (100) begin
      step = sim_helper_random.range(MaxCycle, 2);
      n_repeat = int'(MaxCycle / step / 2);
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
      check();
    join

    $display("Ok! sim_silencer");
    $finish;
  end

endmodule
