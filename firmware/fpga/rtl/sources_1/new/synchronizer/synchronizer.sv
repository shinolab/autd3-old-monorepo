/*
 * File: synchronizer.sv
 * Project: synchronizer
 * Created Date: 24/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

`timescale 1ns / 1ps
module synchronizer (
    input var CLK,
    input var [63:0] ECAT_SYNC_TIME,
    input var SET,
    input var ECAT_SYNC,
    output var [63:0] SYS_TIME,
    output var SYNC
);

  localparam int ADDSUB_LATENCY = 6;

  localparam bit [31:0] ECAT_SYNC_BASE = 32'd500000;  // ns
  localparam bit [17:0] ECAT_SYNC_BASE_CNT = 18'd81920;

  localparam bit [31:0] FREQUENCY_TOLERANCE = 32'd320;  // 50ppm * 163.84MHz / 25.6MHz
  localparam bit [31:0] SYS_TIME_DIFF_ADJUST_CNT_MAX = 1000000 / FREQUENCY_TOLERANCE;

  bit [63:0] ecat_sync_time;
  bit [63:0] lap;
  bit [31:0] _unused_rem_lap;
  bit [80:0] sync_time_raw;
  bit [63:0] sync_time;

  assign sync_time = sync_time_raw[63:0];

  bit [2:0] sync_tri = 0;
  bit sync;
  assign SYNC = sync;

  bit [63:0] sys_time = 0;
  bit [63:0] next_sync_time = 0;
  bit signed [64:0] sync_time_diff = 0;
  bit [$clog2(ADDSUB_LATENCY+1+1)-1:0] addsub_cnt = ADDSUB_LATENCY + 1;
  bit [$clog2(ADDSUB_LATENCY+1+1)-1:0] addsub_next_cnt = ADDSUB_LATENCY + 1;
  bit set;

  bit [$clog2(SYS_TIME_DIFF_ADJUST_CNT_MAX)-1:0] sys_time_adjust_cnt = 0;

  bit [17:0] next_sync_cnt = 0;

  bit signed [64:0] a_diff, b_diff, s_diff;
  bit signed [64:0] a_next, b_next, s_next;

  div_64_32 div_64_32_lap (
      .s_axis_dividend_tdata(ecat_sync_time),
      .s_axis_dividend_tvalid(1'b1),
      .s_axis_divisor_tdata(ECAT_SYNC_BASE),
      .s_axis_divisor_tvalid(1'b1),
      .aclk(CLK),
      .m_axis_dout_tdata({lap, _unused_rem_lap}),
      .m_axis_dout_tvalid()
  );

  mult_sync_base mult_sync_base_time (
      .CLK(CLK),
      .A  (lap),
      .P  (sync_time_raw)
  );

  addsub_64_64 addsub_diff (
      .CLK(CLK),
      .A  (a_diff),
      .B  (b_diff),
      .ADD(1'b0),
      .S  (s_diff)
  );

  addsub_64_64 addsub_next (
      .CLK(CLK),
      .A  (a_next),
      .B  (b_next),
      .ADD(1'b1),
      .S  (s_next)
  );

  assign sync = sync_tri == 3'b011;
  assign SYS_TIME = sys_time;

  always_ff @(posedge CLK) begin
    if (set & sync) begin
      set <= 1'b0;
    end else if (SET) begin
      set <= 1'b1;
      ecat_sync_time <= ECAT_SYNC_TIME;
    end
  end

  always_ff @(posedge CLK) begin
    sys_time_adjust_cnt <= sys_time_adjust_cnt == SYS_TIME_DIFF_ADJUST_CNT_MAX - 1 ? 0 : sys_time_adjust_cnt + 1;
  end

  always_ff @(posedge CLK) begin
    if (sync) begin
      if (set) begin
        sys_time <= sync_time;
        a_diff <= {1'b0, sync_time};
        b_diff <= {1'b0, sync_time};
        next_sync_time <= sync_time;
        sync_time_diff <= 65'd0;
      end else begin
        a_diff   <= {1'b0, next_sync_time};
        b_diff   <= {1'b0, sys_time + 1};
        sys_time <= sys_time + 1;
      end
      addsub_cnt <= 0;
      next_sync_cnt <= ECAT_SYNC_BASE_CNT >> 1;
    end else begin
      if (addsub_cnt == ADDSUB_LATENCY + 1) begin
        if (sys_time_adjust_cnt == 0) begin
          if (sync_time_diff == 65'd0) begin
            sys_time <= sys_time + 1;
          end else if (sync_time_diff[64] == 1'b1) begin
            sys_time <= sys_time;
            sync_time_diff <= sync_time_diff + 1;
          end else begin
            sys_time <= sys_time + 2;
            sync_time_diff <= sync_time_diff - 1;
          end
        end else begin
          sys_time <= sys_time + 1;
        end
      end else if (addsub_cnt == ADDSUB_LATENCY) begin
        sync_time_diff <= s_diff;
        addsub_cnt <= addsub_cnt + 1;
        sys_time <= sys_time + 1;
      end else begin
        addsub_cnt <= addsub_cnt + 1;
        sys_time   <= sys_time + 1;
      end

      if (next_sync_cnt == ECAT_SYNC_BASE_CNT - 1) begin
        next_sync_cnt <= 0;
        a_next <= {1'b0, next_sync_time};
        b_next <= {47'd0, ECAT_SYNC_BASE_CNT};
        addsub_next_cnt <= 0;
      end else begin
        if (addsub_next_cnt == ADDSUB_LATENCY + 1) begin
          addsub_next_cnt <= addsub_next_cnt;
        end else if (addsub_next_cnt == ADDSUB_LATENCY) begin
          next_sync_time  <= s_next[63:0];
          addsub_next_cnt <= addsub_next_cnt + 1;
        end else begin
          addsub_next_cnt <= addsub_next_cnt + 1;
        end
        next_sync_cnt <= next_sync_cnt + 1;
      end
    end
  end

  always_ff @(posedge CLK) begin
    sync_tri <= {sync_tri[1:0], ECAT_SYNC};
  end

endmodule
