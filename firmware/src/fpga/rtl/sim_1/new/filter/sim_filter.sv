/*
 * File: sim_filter.sv
 * Project: filter
 * Created Date: 28/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

`timescale 1ns / 1ps
module sim_filter ();

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

  bit signed [WIDTH:0] filter_duty[DEPTH], filter_phase[DEPTH];
  bit [WIDTH-1:0] cycle[DEPTH];
  bit [WIDTH-1:0] duty;
  bit [WIDTH-1:0] phase;
  bit [WIDTH-1:0] duty_f;
  bit [WIDTH-1:0] phase_f;
  bit din_valid, dout_valid;

  bit [WIDTH-1:0] duty_buf [DEPTH];
  bit [WIDTH-1:0] phase_buf[DEPTH];

  filter #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) filter (
      .CLK(CLK_20P48M),
      .DIN_VALID(din_valid),
      .FILTER_DUTY(filter_duty),
      .FILTER_PHASE(filter_phase),
      .CYCLE(cycle),
      .DUTY(duty),
      .PHASE(phase),
      .DUTY_F(duty_f),
      .PHASE_F(phase_f),
      .DOUT_VALID(dout_valid)
  );

  localparam int MaxCycle = 8000;

  int phase_expect;
  int duty_expect;

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
      phase_expect = phase_buf[i];
      phase_expect += filter_phase[i];
      phase_expect += cycle[i];
      phase_expect %= cycle[i];
      duty_expect = duty_buf[i];
      duty_expect += filter_duty[i];
      duty_expect = duty_expect < 0 ? 0 : duty_expect > cycle[i] ? cycle[i] : duty_expect;
      if (phase_f !== phase_expect) begin
        $display("ASSERTION FAILED: PHASE(%d) != PHASE_F(%d) in %d-th transducer", phase_expect, phase_f, i);
        $finish;
      end
      if (duty_f !== duty_expect) begin
        $display("ASSERTION FAILED: DUTY(%d) != DUTY_F(%d) in %d-th transducer", duty_expect, duty_f, i);
        $finish;
      end
      @(posedge CLK_20P48M);
    end
  endtask

  initial begin
    sim_helper_random.init();

    @(posedge locked);

    for (int i = 0; i < DEPTH; i++) begin
      cycle[i] = 4096;
    end
    // duty overrange
    duty_buf[0] = 2048;
    filter_duty[0] = 4096;
    // duty underrange
    duty_buf[1] = 2048;
    filter_duty[1] = -4096;
    // duty cycle
    duty_buf[2] = 2048;
    filter_duty[2] = 2048;
    // duty 0
    duty_buf[3] = 2048;
    filter_duty[3] = -2048;
    // phase overrange
    phase_buf[0] = 2048;
    filter_phase[0] = 4096;
    // phase underrange
    phase_buf[1] = 2048;
    filter_phase[1] = -4096;
    // phase cycle
    phase_buf[2] = 2048;
    filter_phase[2] = 2048;
    // phase 0
    phase_buf[3] = 2048;
    filter_phase[3] = -2048;
    
    fork
      set();
      check();
    join

    // random
    repeat (100) begin
      for (int i = 0; i < DEPTH; i++) begin
        cycle[i] = sim_helper_random.range(MaxCycle, 2000);
      end
  
      for (int i = 0; i < DEPTH; i++) begin
        filter_duty[i] = sim_helper_random.range(cycle[i], -cycle[i]);
        filter_phase[i] = sim_helper_random.range(cycle[i], -cycle[i]);
      end
  
      for (int i = 0; i < DEPTH; i++) begin
        duty_buf[i]  = sim_helper_random.range(cycle[i] / 2, 0);
        phase_buf[i] = sim_helper_random.range(cycle[i] - 1, 0);
      end
   
      fork
        set();
        check();
      join
    end

    // disable
    for (int i = 0; i < DEPTH; i++) begin
      cycle[i] = sim_helper_random.range(MaxCycle, 2000);
    end

    for (int i = 0; i < DEPTH; i++) begin
      filter_duty[i] = 0;
      filter_phase[i] = 0;
    end

    for (int i = 0; i < DEPTH; i++) begin
      duty_buf[i]  = sim_helper_random.range(cycle[i] / 2, 0);
      phase_buf[i] = sim_helper_random.range(cycle[i] - 1, 0);
    end
  
    fork
      set();
      check();
    join

    $display("Ok! sim_filter");
    $finish;
  end

endmodule
