/*
 * File: sim_pwm_preconditioner.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

module sim_pwm_preconditioner ();

  bit CLK_20P48M;
  bit locked;
  sim_helper_clk sim_helper_clk (
      .CLK_163P84M(),
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME()
  );

  sim_helper_random sim_helper_random ();

  localparam int WIDTH = 13;
  localparam int DEPTH = 249;
  localparam int CYCLE = 4096;

  bit [WIDTH-1:0] cycle [DEPTH];
  bit [WIDTH-1:0] duty;
  bit [WIDTH-1:0] phase;

  bit [WIDTH-1:0] rise  [DEPTH];
  bit [WIDTH-1:0] fall  [DEPTH];
  bit din_valid, dout_valid;

  bit [WIDTH-1:0] duty_buf [DEPTH];
  bit [WIDTH-1:0] phase_buf[DEPTH];

  pwm_preconditioner #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) pwm_preconditioner (
      .CLK(CLK_20P48M),
      .DIN_VALID(din_valid),
      .CYCLE(cycle),
      .DUTY(duty),
      .PHASE(phase),
      .RISE(rise),
      .FALL(fall),
      .DOUT_VALID(dout_valid)
  );

  task automatic set(int idx, bit [WIDTH-1:0] c, bit [WIDTH-1:0] d, bit [WIDTH-1:0] p);
    for (int i = 0; i < DEPTH; i++) begin
      @(posedge CLK_20P48M);
      din_valid = 1'b1;
      if (i == idx) begin
        duty  = d;
        phase = p;
      end else begin
        duty  = 0;
        phase = 0;
      end
      duty_buf[i]  = duty;
      phase_buf[i] = phase;
    end
    @(posedge CLK_20P48M);
    din_valid = 1'b0;
  endtask

  task automatic set_random();
    for (int i = 0; i < DEPTH; i++) begin
      @(posedge CLK_20P48M);
      din_valid = 1'b1;
      duty = sim_helper_random.range(8000, 0);
      phase = sim_helper_random.range(8000, 0);
      duty_buf[i] = duty;
      phase_buf[i] = phase;
    end
    @(posedge CLK_20P48M);
    din_valid = 1'b0;
  endtask

  task automatic check();
    while (1) begin
      @(posedge CLK_20P48M);
      if (dout_valid) begin
        break;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      if ((rise[i] != ((cycle[i]-phase[i]-duty[i]/2+cycle[i])%cycle[i]))
        & (fall[i] == ((cycle[i]-phase[i]+(duty[i]+1)/2)%cycle[i]))) begin
        $error("Failed at idx=%d, d=%d, p=%d, R=%d, F=%d", i, duty[i], phase[i], rise[i], fall[i]);
        $finish();
      end
    end
  endtask

  initial begin
    cycle = '{DEPTH{0}};

    @(posedge locked);

    fork
      set(0, CYCLE, CYCLE / 2, CYCLE / 2);  // normal, D=CYCLE/2
      check();
    join

    fork
      set(0, CYCLE, CYCLE, CYCLE / 2);  // normal, D=CYCLE
      check();
    join

    fork
      set(0, CYCLE, 0, CYCLE / 2);  // normal, D=0
      check();
    join

    fork
      set(0, CYCLE, CYCLE / 2, CYCLE / 2 - CYCLE / 4);  // normal, D=CYCLE/2, left edge
      check();
    join

    fork
      set(0, CYCLE, CYCLE / 2, CYCLE / 2 + CYCLE / 4);  // normal, D=CYCLE/2, right edge
      check();
    join

    fork
      set(0, CYCLE, CYCLE / 2, 0);  // over, D=CYCLE/2
      check();
    join

    fork
      set(0, CYCLE, CYCLE / 2, CYCLE);  // over, D=CYCLE/2
      check();
    join

    fork
      set(0, CYCLE, 0, CYCLE);  // over, D=0
      check();
    join

    // at random
    sim_helper_random.init();
    for (int i = 0; i < 5000; i++) begin
      $display("check start @%d", i);
      fork
        set_random();
        check();
      join
      $display("check finish @%d", i);
    end

    $display("OK!");
    $finish();
  end

endmodule
