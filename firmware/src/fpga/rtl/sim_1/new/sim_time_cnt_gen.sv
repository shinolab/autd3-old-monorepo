/*
 * File: sim_time_cnt_gen.sv
 * Project: new
 * Created Date: 17/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */


module sim_time_cnt_gen ();

  logic CLK_20P48M;
  logic [63:0] SYS_TIME;
  logic locked;
  sim_helper_clk sim_helper_clk (
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME()
  );

  localparam int DEPTH = 249;

  logic [8:0] time_cnt;
  logic skip_one_assert;

  time_cnt_generator #(
      .DEPTH(DEPTH)
  ) time_cnt_generator (
      .CLK(CLK_20P48M),
      .SYS_TIME(SYS_TIME),
      .SKIP_ONE_ASSERT(skip_one_assert),
      .TIME_CNT(time_cnt),
      .UPDATE(update)
  );

  initial begin
    SYS_TIME = 0;
    skip_one_assert = 0;
    @(posedge locked);

    for (int i = 0; i < 512 * 2; i++) begin
      @(posedge CLK_20P48M);
      SYS_TIME = SYS_TIME + 1;
    end

    @(posedge CLK_20P48M);
    SYS_TIME = 0;
    for (int i = 0; i < 510; i++) begin
      @(posedge CLK_20P48M);
      SYS_TIME = SYS_TIME + 1;
    end
    @(posedge CLK_20P48M);
    SYS_TIME = SYS_TIME + 2;
    skip_one_assert = 1;
    for (int i = 0; i < 512; i++) begin
      @(posedge CLK_20P48M);
      skip_one_assert = 0;
      SYS_TIME = SYS_TIME + 1;
    end

    @(posedge CLK_20P48M);
    SYS_TIME = 0;
    for (int i = 0; i < 511; i++) begin
      @(posedge CLK_20P48M);
      SYS_TIME = SYS_TIME + 1;
    end
    @(posedge CLK_20P48M);
    SYS_TIME = SYS_TIME + 2;
    skip_one_assert = 1;
    for (int i = 0; i < 512; i++) begin
      @(posedge CLK_20P48M);
      skip_one_assert = 0;
      SYS_TIME = SYS_TIME + 1;
    end

    $finish;
  end

endmodule
