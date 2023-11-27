/*
 * File: sim_pwm_generator.sv
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

module sim_pwm_generator ();

  logic CLK_20P48M;
  logic locked;
  logic [63:0] SYS_TIME;
  sim_helper_clk sim_helper_clk (
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME(SYS_TIME)
  );

  logic [8:0] time_cnt;
  assign time_cnt = SYS_TIME[8:0];

  logic [8:0] rise, fall;

  logic pwm_out;

  pwm_generator pwm_generator (
      .CLK(CLK_20P48M),
      .TIME_CNT(time_cnt),
      .RISE(rise),
      .FALL(fall),
      .PWM_OUT(pwm_out)
  );

  task automatic set(logic [8:0] r, logic [8:0] f);
    while (time_cnt !== 512 - 1) @(posedge CLK_20P48M);
    rise = r;
    fall = f;
    @(posedge CLK_20P48M);
    $display("check start\t@t=%d", SYS_TIME);
    while (1) begin
      automatic int t = time_cnt;
      @(posedge CLK_20P48M);
      if (pwm_out !== (((r <= f) & ((r <= t) & (t < f))) | ((f < r) & ((r <= t) | (t < f))))) begin
        $error("Failed at v=%u, t=%d, R=%d, F=%d", pwm_out, time_cnt, rise, fall);
        $finish();
      end
      if (time_cnt === 512 - 1) begin
        break;
      end
    end
    $display("check done\t@t=%d", SYS_TIME);
  endtask

  initial begin
    time_cnt = 0;
    rise = 0;
    fall = 0;
    @(posedge locked);

    set(512 / 2 - 512 / 4, 512 / 2 + 512 / 4);  // normal, D=512/2
    set(0, 512);  // normal, D=512
    set(512 / 2, 512 / 2);  // normal, D=0
    set(0, 512 / 2);  // normal, D=512/2, left edge
    set(512 - 512 / 2, 512);  // normal, D=512/2, right edge

    set(512 - 512 / 4, 512 / 4);  // over, D=512/2
    set(512, 0);  // over, D=0
    set(512, 512 / 2);  // over, D=512/2, right edge
    set(512 - 512 / 2, 0);  // over, D=512/2, left edge

    set(0, 0);

    $display("OK!");
    $finish();
  end

endmodule
