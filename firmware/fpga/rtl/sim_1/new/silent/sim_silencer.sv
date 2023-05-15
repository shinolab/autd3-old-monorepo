/*
 * File: sim_silencer.sv
 * Project: silent
 * Created Date: 22/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/05/2023
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
  bit [WIDTH-1:0] duty[DEPTH];
  bit [WIDTH-1:0] phase[DEPTH];
  bit [WIDTH-1:0] duty_s[DEPTH];
  bit [WIDTH-1:0] phase_s[DEPTH];
  bit din_valid, dout_valid;

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

  task automatic wait_calc();
    @(negedge dout_valid);
    @(posedge dout_valid);
  endtask

  localparam int MaxCycle = 8000;
  int n_repeat;

  initial begin
    cycle = '{DEPTH{4096}};
    step = 100;

    n_repeat = int'(MaxCycle / step / 2);

    // from 0 to random
    sim_helper_random.init();
    for (int i = 0; i < DEPTH; i++) begin
      cycle[i] = sim_helper_random.range(MaxCycle, 2000);
      duty[i]  = sim_helper_random.range(cycle[i], 0);
      phase[i] = sim_helper_random.range(cycle[i] - 1, 0);
    end
    din_valid = 1;

    n_repeat  = int'(MaxCycle / step / 2);
    repeat (n_repeat) wait_calc();
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s[i] !== phase[i]) begin
        $display("ASSERTION FAILED: PHASE(%d) != PHASE_S(%d) in %d-th transducer", phase[i],
                 phase_s[i], i);
        $finish;
      end
    end
    repeat (n_repeat) wait_calc();
    for (int i = 0; i < DEPTH; i++) begin
      if (duty_s[i] !== duty[i]) begin
        $display("ASSERTION FAILED: DUTY(%d) != DUTY_S(%d) in %d-th transducer", duty[i],
                 duty_s[i], i);
        $finish;
      end
    end

    // from random to random
    for (int i = 0; i < DEPTH; i++) begin
      duty[i]  = sim_helper_random.range(cycle[i], 0);
      phase[i] = sim_helper_random.range(cycle[i] - 1, 0);
    end

    repeat (n_repeat) wait_calc();
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_s[i] !== phase[i]) begin
        $display("ASSERTION FAILED: PHASE(%d) != PHASE_S(%d) in %d-th transducer", phase[i],
                 phase_s[i], i);
        $finish;
      end
    end
    repeat (n_repeat) wait_calc();
    for (int i = 0; i < DEPTH; i++) begin
      if (duty_s[i] !== duty[i]) begin
        $display("ASSERTION FAILED: DUTY(%d) != DUTY_S(%d) in %d-th transducer", duty[i],
                 duty_s[i], i);
        $finish;
      end
    end

    $display("Ok!");
    $finish;
  end

endmodule
