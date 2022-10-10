/*
 * File: sim_time_cnt_gen.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

module sim_pwm_time_cnt_gen ();

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

  localparam int WIDTH = 13;
  localparam int DEPTH = 249;

  bit [WIDTH-1:0] cycle[0:DEPTH-1];

  bit [WIDTH-1:0] time_cnt[0:DEPTH-1];

  bit [WIDTH-1:0] t_0, t_1;

  assign t_0 = time_cnt[0];
  assign t_1 = time_cnt[DEPTH-1];

  time_cnt_generator #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) time_cnt_generator (
      .CLK(CLK_163P84M),
      .SYS_TIME(SYS_TIME),
      .CYCLE(cycle),
      .TIME_CNT(time_cnt)
  );

  initial begin
    cycle = '{DEPTH{4096}};
    cycle[DEPTH-1] = 4097;
  end

endmodule
