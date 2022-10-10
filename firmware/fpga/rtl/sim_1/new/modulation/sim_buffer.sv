/*
 * File: sim_buffer.sv
 * Project: modulation
 * Created Date: 25/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

module sim_modulation_buffer ();

  bit CLK_20P48M;
  bit locked;
  sim_helper_clk sim_helper_clk (
      .CLK_163P84M(),
      .CLK_20P48M(CLK_20P48M),
      .LOCKED(locked),
      .SYS_TIME()
  );

  localparam int WIDTH = 13;
  localparam int DEPTH = 249;

  bit start;
  bit done;
  bit [WIDTH-1:0] duty[0:DEPTH-1];
  bit [WIDTH-1:0] phase[0:DEPTH-1];
  bit [WIDTH-1:0] duty_out[0:DEPTH-1];
  bit [WIDTH-1:0] phase_out[0:DEPTH-1];

  modulation_buffer #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) modulation_buffer (
      .CLK(CLK_20P48M),
      .START(start),
      .DONE(done),
      .DUTY_IN(duty),
      .PHASE_IN(phase),
      .DUTY_OUT(duty_out),
      .PHASE_OUT(phase_out)
  );

  initial begin
    start = 0;
    done  = 0;
    @(posedge locked);

    phase = '{DEPTH{2500}};
    start = 1;

    #1000;
    duty = '{DEPTH{2500}};
    @(posedge CLK_20P48M);
    done = 1;

    for (int i = 0; i < DEPTH; i++) begin
      if (phase_out[i] != 0 || duty_out[i] != 0) begin
        $display("Failed!");
        $finish();
      end
    end

    @(posedge CLK_20P48M);
    for (int i = 0; i < DEPTH; i++) begin
      if (phase_out[i] != phase[i] || duty_out[i] != duty[i]) begin
        $display("Failed!");
        $finish();
      end
    end

    $display("OK!");
    $finish();
  end

endmodule
