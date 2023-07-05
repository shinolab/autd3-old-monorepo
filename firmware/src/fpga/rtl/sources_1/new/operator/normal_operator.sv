/*
 * File: normal_operator.sv
 * Project: operator
 * Created Date: 01/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */


`timescale 1ns / 1ps
module normal_operator #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK_L,
    cpu_bus_if.normal_port CPU_BUS,
    input var LEGACY_MODE,
    input var TRIG_40KHZ,
    output var [WIDTH-1:0] DUTY,
    output var [WIDTH-1:0] PHASE,
    output var DOUT_VALID
);

  bit bus_clk;
  bit ena;
  bit we;
  bit [8:0] addr;
  bit [15:0] data_in;

  bit dout_valid;

  bit [8:0] read_addr = 0;
  bit [31:0] dout;

  bit [WIDTH-1:0] duty;
  bit [WIDTH-1:0] phase;

  assign bus_clk = CPU_BUS.BUS_CLK;
  assign ena = CPU_BUS.NORMAL_EN;
  assign we = CPU_BUS.WE;
  assign addr = CPU_BUS.BRAM_ADDR[8:0];
  assign data_in = CPU_BUS.DATA_IN;

  assign DUTY = duty;
  assign PHASE = phase;
  assign DOUT_VALID = dout_valid;

  BRAM_NORMAL normal_bram (
      .clka (bus_clk),
      .ena  (ena),
      .wea  (we),
      .addra(addr),
      .dina (data_in),
      .douta(),
      .clkb (CLK_L),
      .web  ('0),
      .addrb(read_addr[7:0]),
      .dinb ('0),
      .doutb(dout)
  );

  typedef enum bit {
    WAITING,
    RUN
  } state_t;

  state_t state = WAITING;

  always_ff @(posedge CLK_L) begin
    case (state)
      WAITING: begin
        dout_valid <= 0;
        if (TRIG_40KHZ) begin
          read_addr <= 0;
          state <= RUN;
        end
      end
      RUN: begin
        read_addr <= read_addr + 1;

        if (read_addr > 1) begin
          dout_valid <= 1;

          if (LEGACY_MODE) begin
            phase <= {1'b0, dout[7:0], 4'h00};
            duty  <= {2'b00, dout[15:8], 3'h7} + 1;
          end else begin
            phase <= dout[WIDTH-1:0];
            duty  <= dout[WIDTH-1+16:16];
          end
        end

        if (read_addr == DEPTH + 1) begin
          state <= WAITING;
        end
      end
      default: begin
      end
    endcase
  end

endmodule
