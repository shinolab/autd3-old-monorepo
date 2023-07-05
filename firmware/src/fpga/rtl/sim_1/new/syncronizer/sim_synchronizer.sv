/*
 * File: sim_synchronizer.sv
 * Project: syncronizer
 * Created Date: 29/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

`timescale 1ns / 1ps
module sim_synchronizer ();

  localparam int ECAT_SYNC_BASE = 500000;  // 500 us
  localparam bit [15:0] ECAT_SYNC_CYCLE_TICKS = 1;

  bit CLK_163P84M, CLK_163P84M_p50, CLK_163P84M_m50;
  bit [63:0] SYS_TIME, SYS_TIME_p50, SYS_TIME_m50;

  bit ECAT_SYNC;

  bit set;
  bit [63:0] ecat_sync_time;  // [ns]

  synchronizer synchronizer (
      .CLK(CLK_163P84M),
      .ECAT_SYNC_TIME(ecat_sync_time),
      .SET(set),
      .ECAT_SYNC(ECAT_SYNC),
      .SYS_TIME(SYS_TIME)
  );

  synchronizer synchronizer_p50 (
      .CLK(CLK_163P84M_p50),
      .ECAT_SYNC_TIME(ecat_sync_time),
      .SET(set),
      .ECAT_SYNC(ECAT_SYNC),
      .SYS_TIME(SYS_TIME_p50)
  );

  synchronizer synchronizer_m50 (
      .CLK(CLK_163P84M_m50),
      .ECAT_SYNC_TIME(ecat_sync_time),
      .SET(set),
      .ECAT_SYNC(ECAT_SYNC),
      .SYS_TIME(SYS_TIME_m50)
  );

  task sync();
    @(posedge ECAT_SYNC);
    #1000;

    ecat_sync_time = ECAT_SYNC_BASE * 10;
    set = 1;
    @(posedge CLK_163P84M);
    @(posedge CLK_163P84M_p50);
    @(posedge CLK_163P84M_m50);
    set = 0;
  endtask

  initial begin
    CLK_163P84M = 1;
    CLK_163P84M_p50 = 1;
    CLK_163P84M_m50 = 1;
    SYS_TIME = 0;
    SYS_TIME_p50 = 0;
    SYS_TIME_m50 = 0;

    set = 0;

    #10000;

    sync();

    #1000000000;

    $finish();
  end

  // (31 + 33) / (6.103ns * 31 + 6.104ns * 33) = 163.84MHz
  always begin
    for (int i = 0; i < 31; i++) begin
      #3.051 CLK_163P84M = ~CLK_163P84M;
      #3.052 CLK_163P84M = ~CLK_163P84M;
    end
    for (int i = 0; i < 33; i++) begin
      #3.052 CLK_163P84M = ~CLK_163P84M;
      #3.052 CLK_163P84M = ~CLK_163P84M;
    end
  end

  // (31583 + 8419) / (6.103ns*31583 + 6.104ns*8419) = 163.84MHz + 50ppm
  always begin
    for (int i = 0; i < 31583; i++) begin
      #3.051 CLK_163P84M_p50 = ~CLK_163P84M_p50;
      #3.052 CLK_163P84M_p50 = ~CLK_163P84M_p50;
    end
    for (int i = 0; i < 8419; i++) begin
      #3.052 CLK_163P84M_p50 = ~CLK_163P84M_p50;
      #3.052 CLK_163P84M_p50 = ~CLK_163P84M_p50;
    end
  end

  // (7167 + 32831) / (6.103ns*7167 + 6.104ns*32831) = 163.84MHz - 50ppm
  always begin
    for (int i = 0; i < 7167; i++) begin
      #3.051 CLK_163P84M_m50 = ~CLK_163P84M_m50;
      #3.052 CLK_163P84M_m50 = ~CLK_163P84M_m50;
    end
    for (int i = 0; i < 32831; i++) begin
      #3.052 CLK_163P84M_m50 = ~CLK_163P84M_m50;
      #3.052 CLK_163P84M_m50 = ~CLK_163P84M_m50;
    end
  end

  always begin
    #800 ECAT_SYNC = 0;
    #(ECAT_SYNC_BASE * ECAT_SYNC_CYCLE_TICKS - 800) ECAT_SYNC = 1;
  end

endmodule
