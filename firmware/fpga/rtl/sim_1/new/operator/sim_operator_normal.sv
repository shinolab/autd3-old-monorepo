/*
 * File: sim_operator_normal.sv
 * Project: operator
 * Created Date: 12/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

module sim_operator_normal ();

  bit [63:0] SYS_TIME;
  bit CLK_20P48M;
  bit locked;
  bit TRIG_40KHZ;
  sim_helper_clk sim_helper_clk (
      .CLK_163P84M(),
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME(SYS_TIME)
  );

  localparam int WIDTH = 13;
  localparam int DEPTH = 249;
  localparam bit [WIDTH-1:0] MAX = (1 << WIDTH) - 1;

  sim_helper_bram sim_helper_bram ();
  sim_helper_random sim_helper_random ();

  bit [WIDTH-1:0] cycle[DEPTH];
  bit legacy_mode;
  bit [WIDTH-1:0] duty;
  bit [WIDTH-1:0] phase;
  bit dout_valid;

  bit [WIDTH-1:0] duty_buf[DEPTH];
  bit [WIDTH-1:0] phase_buf[DEPTH];

  timer_40kHz timer_40kHz (
      .CLK_L(CLK_20P48M),
      .SYS_TIME(SYS_TIME),
      .TRIG_40KHZ(TRIG_40KHZ)
  );

  normal_operator #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) normal_operator (
      .CLK_L(CLK_20P48M),
      .CPU_BUS(sim_helper_bram.cpu_bus.normal_port),
      .TRIG_40KHZ(TRIG_40KHZ),
      .LEGACY_MODE(legacy_mode),
      .DUTY(duty),
      .PHASE(phase),
      .DOUT_VALID(dout_valid)
  );

  initial begin
    legacy_mode = 0;

    sim_helper_random.init();
    for (int i = 0; i < DEPTH; i++) begin
      if (legacy_mode) begin
        cycle[i] = 4096;
      end else begin
        cycle[i] = sim_helper_random.range(8000, 2000);
      end
    end

    @(posedge locked);

    for (int i = 0; i < DEPTH; i++) begin
      duty_buf[i]  = sim_helper_random.range(cycle[i], 0);
      phase_buf[i] = sim_helper_random.range(cycle[i], 0);
      sim_helper_bram.write_duty_phase(i, duty_buf[i], phase_buf[i]);
    end

    while (1) begin
      @(posedge CLK_20P48M);
      if (dout_valid) begin
        break;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      if (duty_buf[i] != duty) begin
        $display("failed at duty[%d], %d!=%d", i, duty_buf[i], duty);
        $finish();
      end
      if (phase_buf[i] != phase) begin
        $display("failed at phase[%d], %d!=%d", i, phase_buf[i], phase);
        $finish();
      end

      @(posedge CLK_20P48M);
    end

    legacy_mode = 1;

    for (int i = 0; i < DEPTH; i++) begin
      duty_buf[i]  = sim_helper_random.range(8'hFF, 0);
      phase_buf[i] = sim_helper_random.range(8'hFF, 0);
      sim_helper_bram.write_duty_phase(i, 0, {duty_buf[i][7:0], phase_buf[i][7:0]});
    end

    while (1) begin
      @(posedge CLK_20P48M);
      if (dout_valid) begin
        break;
      end
    end

    for (int i = 0; i < DEPTH; i++) begin
      if (({duty_buf[i], 3'h7} + 1) != duty) begin
        $display("failed at duty[%d], %d!=%d", i, duty_buf[i], duty);
        $finish();
      end
      if ({phase_buf[i], 4'h00} != phase) begin
        $display("failed at phase[%d], %d!=%d", i, {phase_buf[i], 5'h00}, phase);
        $finish();
      end
      @(posedge CLK_20P48M);
    end

    $display("OK! sim_operator_normal");
    $finish();
  end

endmodule
