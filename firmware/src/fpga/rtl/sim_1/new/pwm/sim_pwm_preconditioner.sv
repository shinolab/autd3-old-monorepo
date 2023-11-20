/*
 * File: sim_pwm_preconditioner.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

module sim_pwm_preconditioner ();

  logic CLK_20P48M;
  logic locked;
  sim_helper_clk sim_helper_clk (
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME()
  );

  sim_helper_random sim_helper_random ();

  localparam int WIDTH = 9;
  localparam int DEPTH = 249;

  logic [WIDTH-1:0] duty;
  logic [WIDTH-1:0] phase;

  logic [WIDTH-1:0] rise  [DEPTH];
  logic [WIDTH-1:0] fall  [DEPTH];
  logic din_valid, dout_valid;

  logic [WIDTH-1:0] duty_buf [DEPTH];
  logic [WIDTH-1:0] phase_buf[DEPTH];

  pwm_preconditioner #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) pwm_preconditioner (
      .CLK(CLK_20P48M),
      .DIN_VALID(din_valid),
      .DUTY(duty),
      .PHASE(phase),
      .RISE(rise),
      .FALL(fall),
      .DOUT_VALID(dout_valid)
  );

  task automatic set(int idx, logic [WIDTH-1:0] d, logic [WIDTH-1:0] p);
    for (int i = 0; i < DEPTH; i++) begin
      @(posedge CLK_20P48M);
      din_valid = 1'b1;
      if (i === idx) begin
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
      duty = sim_helper_random.range(511, 0);
      phase = sim_helper_random.range(511, 0);
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
      if ((rise[i] !== ((512-phase_buf[i]-duty_buf[i]/2+512)%512))
        & (fall[i] === ((512-phase_buf[i]+(duty_buf[i]+1)/2)%512))) begin
        $error("Failed at idx=%d, d=%d, p=%d, R=%d, F=%d", i, duty_buf[i], phase_buf[i], rise[i],
               fall[i]);
        $finish();
      end
    end
  endtask

  initial begin
    @(posedge locked);

    fork
      set(0, 512 / 2, 512 / 2);  // normal, D=512/2
      check();
    join

    fork
      set(0, 511, 512 / 2);  // normal, D=511
      check();
    join

    fork
      set(0, 0, 512 / 2);  // normal, D=0
      check();
    join

    fork
      set(0, 512 / 2, 512 / 2 - 512 / 4);  // normal, D=512/2, left edge
      check();
    join

    fork
      set(0, 512 / 2, 512 / 2 + 512 / 4);  // normal, D=512/2, right edge
      check();
    join

    fork
      set(0, 512 / 2, 511);  // over, D=512/2
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
    end

    $display("OK! sim_pwm_preconditioner");
    $finish();
  end

endmodule
