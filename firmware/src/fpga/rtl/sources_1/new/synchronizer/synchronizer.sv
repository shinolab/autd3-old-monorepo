/*
 * File: synchronizer.sv
 * Project: synchronizer
 * Created Date: 24/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
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
    output var SYNC,
    output var SKIP_ONE_ASSERT
);

  localparam int ADDSUB_LATENCY = 6;

  localparam logic [31:0] ECAT_SYNC_BASE = 32'd500000;  // ns
  localparam logic [17:0] ECAT_SYNC_BASE_CNT = 18'd10240;  // 20.48MHz * 500000ns

  logic [63:0] ecat_sync_time;
  logic [63:0] lap;
  logic [31:0] lap_rem_unused;
  logic [63:0] sync_time_raw;
  logic [13:0] sync_time_raw_unused;
  logic [63:0] sync_time;

  assign sync_time = sync_time_raw;

  logic [2:0] sync_tri = 0;
  logic sync;
  assign SYNC = sync;

  logic [63:0] sys_time = 0;
  logic [63:0] next_sync_time = 0;
  logic signed [64:0] sync_time_diff = 0;
  logic [$clog2(ADDSUB_LATENCY+1+1)-1:0] addsub_cnt = ADDSUB_LATENCY + 1;
  logic [$clog2(ADDSUB_LATENCY+1+1)-1:0] addsub_next_cnt = ADDSUB_LATENCY + 1;
  logic set;
  logic [17:0] next_sync_cnt = 0;

  logic signed [64:0] a_diff, b_diff, s_diff;
  logic signed [64:0] a_next, b_next, s_next;

  logic skip_one_assert;
  assign SKIP_ONE_ASSERT = skip_one_assert;

  div_64_32 div_64_32_lap (
      .s_axis_dividend_tdata(ecat_sync_time),
      .s_axis_dividend_tvalid(1'b1),
      .s_axis_divisor_tdata(ECAT_SYNC_BASE),
      .s_axis_divisor_tvalid(1'b1),
      .aclk(CLK),
      .m_axis_dout_tdata({lap, lap_rem_unused}),
      .m_axis_dout_tvalid()
  );

  mult_sync_base mult_sync_base_time (
      .CLK(CLK),
      .A  (lap),
      .P  ({sync_time_raw_unused, sync_time_raw})
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
      skip_one_assert <= 1'b0;
    end else begin
      if (addsub_cnt == ADDSUB_LATENCY + 1) begin
        if (sync_time_diff == 65'd0) begin
          sys_time <= sys_time + 1;
          skip_one_assert <= 1'b0;
        end else if (sync_time_diff[64] == 1'b1) begin
          sys_time <= sys_time;
          skip_one_assert <= 1'b0;
          sync_time_diff <= sync_time_diff + 1;
        end else begin
          sys_time <= sys_time + 2;
          skip_one_assert <= 1'b1;
          sync_time_diff <= sync_time_diff - 1;
        end
      end else if (addsub_cnt == ADDSUB_LATENCY) begin
        sync_time_diff <= s_diff;
        addsub_cnt <= addsub_cnt + 1;
        sys_time <= sys_time + 1;
        skip_one_assert <= 1'b0;
      end else begin
        addsub_cnt <= addsub_cnt + 1;
        sys_time <= sys_time + 1;
        skip_one_assert <= 1'b0;
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
