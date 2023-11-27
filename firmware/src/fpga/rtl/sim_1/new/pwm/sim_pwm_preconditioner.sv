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

  localparam int DEPTH = 249;

  logic [8:0] pulse_width;
  logic [7:0] phase;

  logic [8:0] rise[DEPTH];
  logic [8:0] fall[DEPTH];
  logic din_valid, dout_valid;

  logic [8:0] pulse_width_buf[DEPTH];
  logic [7:0] phase_buf[DEPTH];

  pwm_preconditioner #(
      .DEPTH(DEPTH)
  ) pwm_preconditioner (
      .CLK(CLK_20P48M),
      .DIN_VALID(din_valid),
      .PULSE_WIDTH(pulse_width),
      .PHASE(phase),
      .RISE(rise),
      .FALL(fall),
      .DOUT_VALID(dout_valid)
  );

  task automatic set(int idx, logic [8:0] d, logic [7:0] p);
    for (int i = 0; i < DEPTH; i++) begin
      if (i === idx) begin
        pulse_width_buf[i] = d;
        phase_buf[i] = p;
      end else begin
        pulse_width_buf[i] = 0;
        phase_buf[i] = 0;
      end
    end
    for (int i = 0; i < DEPTH; i++) begin
      @(posedge CLK_20P48M);
      din_valid <= 1'b1;
      pulse_width <= pulse_width_buf[i];
      phase <= phase_buf[i];
    end
    @(posedge CLK_20P48M);
    din_valid <= 1'b0;
  endtask


  task automatic check_manual(int idx, logic [8:0] rise_e, logic [8:0] fall_e);
    while (1) begin
      @(posedge CLK_20P48M);
      if (dout_valid) begin
        break;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      if (i === idx) begin
        if ((rise[i] !== rise_e) | (fall[i] !== fall_e)) begin
          $error("Failed at idx=%d, R(%d) != %d, F(%d) != %d", i, rise[i], rise_e, fall[i], fall_e);
          $finish();
        end
      end else begin
        if ((rise[i] !== 0) | (fall[i] !== 0)) begin
          $error("Failed at idx=%d, R=%d, F=%d", i, rise[i], fall[i]);
          $finish();
        end
      end
    end
  endtask

  task automatic set_random();
    for (int i = 0; i < DEPTH; i++) begin
      pulse_width_buf[i] = sim_helper_random.range(256, 0);
      phase_buf[i] = sim_helper_random.range(255, 0);
    end
    for (int i = 0; i < DEPTH; i++) begin
      @(posedge CLK_20P48M);
      din_valid <= 1'b1;
      pulse_width <= pulse_width_buf[i];
      phase <= phase_buf[i];
    end
    @(posedge CLK_20P48M);
    din_valid <= 1'b0;
  endtask

  task automatic check();
    while (1) begin
      @(posedge CLK_20P48M);
      if (dout_valid) begin
        break;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      if ((rise[i] !== ((512-phase_buf[i]*2-pulse_width_buf[i]/2+512)%512))
        | (fall[i] !== ((512-phase_buf[i]*2+(pulse_width_buf[i]+1)/2)%512))) begin
        $error("Failed at idx=%d, d=%d, p=%d, R=%d, F=%d", i, pulse_width_buf[i], phase_buf[i],
               rise[i], fall[i]);
        $finish();
      end
    end
  endtask

  initial begin
    @(posedge locked);

    fork
      set(0, 256, 128);  // normal, D=256
      check_manual(0, 128, 384);
    join

    fork
      set(0, 1, 128);  // normal, D=1
      check_manual(0, 256, 257);
    join

    fork
      set(0, 255, 128);  // normal, D=255
      check_manual(0, 129, 384);
    join

    fork
      set(0, 0, 128);  // normal, D=0
      check_manual(0, 256, 256);
    join

    fork
      set(0, 256, 64);  // normal, D=256, left edge
      check_manual(0, 256, 0);
    join

    fork
      set(0, 256, 192);  // normal, D=256, right edge
      check_manual(0, 0, 256);
    join

    fork
      set(0, 256, 255);  // over, D=256
      check_manual(0, 386, 130);
    join

    fork
      set(0, 256, 1);  // over, D=256
      check_manual(0, 382, 126);
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
