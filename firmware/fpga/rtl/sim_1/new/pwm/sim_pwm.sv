/*
 * File: sim_pwm.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
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
  bit [WIDTH-1:0] duty;
  bit [WIDTH-1:0] phase;
  bit pwm_out[DEPTH];
  bit din_valid, dout_valid;

  bit [WIDTH-1:0] time_cnt [DEPTH];

  bit [WIDTH-1:0] duty_buf [DEPTH];
  bit [WIDTH-1:0] phase_buf[DEPTH];

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

  task automatic set_random();
    for (int i = 0; i < DEPTH; i++) begin
      @(posedge CLK_20P48M);
      din_valid = 1'b1;
      cycle[i] = sim_helper_random.range(8000, 2000);
      duty = sim_helper_random.range(cycle[i] / 2, 0);
      phase = sim_helper_random.range(cycle[i] - 1, 0);
      duty_buf[i] = duty;
      phase_buf[i] = phase;
    end
    @(posedge CLK_20P48M);
    din_valid = 1'b0;
  endtask

  task automatic check(int i);
    while (1) begin
      @(posedge CLK_163P84M);
      if (dout_valid) begin
        break;
      end
    end
    @(posedge CLK_163P84M);
    while (time_cnt[i] != cycle[i] - 1) @(posedge CLK_163P84M);
    @(posedge CLK_163P84M);
    while (1) begin
      automatic int r = (cycle[i] - phase_buf[i] - duty_buf[i] / 2 + cycle[i]) % cycle[i];
      automatic int f = (cycle[i] - phase_buf[i] + (duty_buf[i] + 1) / 2) % cycle[i];
      automatic int t = time_cnt[i];
      @(posedge CLK_163P84M);
      if (pwm_out[i] != (((r <= f) & ((r <= t) & (t < f)))
          | ((f < r) & ((r <= t) | (t < f))))) begin
        $error("\tFailed at v=%u, t=%d, T=%d, duty=%d, phase=%d, R=%d, F=%d", pwm_out[i], t,
               cycle[i], duty_buf[i], phase_buf[i], r, f);
        $finish();
      end
      if (t == cycle[i] - 1) begin
        break;
      end
    end
  endtask

  initial begin
    cycle = '{DEPTH{0}};

    sim_helper_random.init();

    @(posedge locked);

    for (int i = 0; i < 100; i++) begin
      $display("check: %d", i);
      fork
        set_random();
      join_none
      for (int j = 0; j < DEPTH; j++) begin
        fork
          automatic int k = j;
          check(k);
        join_none
      end
      wait fork;
    end

    $display("OK! sim_pwm");
    $finish();
  end

endmodule
