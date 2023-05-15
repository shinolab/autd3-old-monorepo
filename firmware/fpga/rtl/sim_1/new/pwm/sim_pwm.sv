/*
 * File: sim_pwm.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */


module sim_pwm ();

  bit CLK_163P84M;
  bit CLK_20P48M;
  bit [63:0] SYS_TIME;
  bit locked;
  sim_helper_clk sim_helper_clk (
      .CLK_163P84M(CLK_163P84M),
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME(SYS_TIME)
  );

  sim_helper_random sim_helper_random ();

  localparam int WIDTH = 13;
  localparam int DEPTH = 249;
  localparam int CYCLE = 4096;

  bit [WIDTH-1:0] cycle[DEPTH];
  bit [WIDTH-1:0] duty[DEPTH];
  bit [WIDTH-1:0] phase[DEPTH];
  bit pwm_out[DEPTH];
  bit din_valid, dout_valid;

  bit [WIDTH-1:0] time_cnt[DEPTH];

  pwm #(
      .WIDTH(WIDTH),
      .TRANS_NUM(DEPTH)
  ) pwm (
      .CLK(CLK_163P84M),
      .CLK_L(CLK_20P48M),
      .SYS_TIME(SYS_TIME),
      .DIN_VALID(din_valid),
      .CYCLE(cycle),
      .DUTY(duty),
      .PHASE(phase),
      .PWM_OUT(pwm_out),
      .TIME_CNT(time_cnt),
      .DOUT_VALID(dout_valid)
  );

  task automatic wait_calc();
    @(posedge CLK_20P48M);
    din_valid = 1;

    @(posedge CLK_20P48M);
    din_valid = 0;

    while (1) begin
      @(posedge CLK_20P48M);
      if (dout_valid) begin
        break;
      end
    end

    @(posedge CLK_20P48M);
  endtask

  task automatic set(int idx, bit [WIDTH-1:0] c, bit [WIDTH-1:0] d, bit [WIDTH-1:0] p);
    @(posedge CLK_20P48M);
    cycle[idx] = c;
    duty[idx]  = d;
    phase[idx] = p;

    wait_calc();

    while (time_cnt[idx] != cycle[idx] - 1) @(posedge CLK_163P84M);
    @(posedge CLK_163P84M);
  endtask

  task automatic set_and_check(int idx, bit [WIDTH-1:0] c, bit [WIDTH-1:0] d,
                               bit [WIDTH-1:0] p);
    set(idx, c, d, p);

    $display("check start\tidx=%d, duty=%d, phase=%d \t@t=%d", idx[$clog2(DEPTH)-1:0], d, p,
             SYS_TIME);
    while (1) begin
      automatic int r = (cycle[idx] - phase[idx] - duty[idx] / 2 + cycle[idx]) % cycle[idx];
      automatic int f = (cycle[idx] - phase[idx] + (duty[idx] + 1) / 2) % cycle[idx];
      automatic int t = time_cnt[idx];
      @(posedge CLK_163P84M);
      if (pwm_out[idx] != (((r <= f) & ((r <= t) & (t < f)))
          | ((f < r) & ((r <= t) | (t < f))))) begin
        $error("\tFailed at v=%u, t=%d, T=%d, duty=%d, phase=%d, R=%d, F=%d", pwm_out[idx], t,
               cycle[idx], duty[idx], phase[idx], r, f);
        $finish();
      end
      if (t == cycle[idx] - 1) begin
        break;
      end
    end
  endtask

  task automatic set_and_check_random();
    automatic int idx = sim_helper_random.range(DEPTH - 1, 0);
    automatic int c = sim_helper_random.range(8000, 2000);
    automatic int d = sim_helper_random.range(cycle[idx], 0);
    automatic int p = sim_helper_random.range(cycle[idx], 0);
    set_and_check(idx, c, d, p);
  endtask

  initial begin
    sim_helper_random.init();
    cycle = '{DEPTH{0}};
    duty  = '{DEPTH{0}};
    phase = '{DEPTH{0}};
    @(posedge locked);

    set_and_check(0, CYCLE, CYCLE, CYCLE / 2);
    set_and_check(0, CYCLE, 1000, CYCLE / 2);
    set_and_check(0, CYCLE, 0, 0);

    for (int i = 0; i < 100; i++) begin
      set_and_check_random();
    end

    $display("OK! sim_pwm");
    $finish();
  end

endmodule
