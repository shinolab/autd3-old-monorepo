/*
 * File: normal_operator.sv
 * Project: operator
 * Created Date: 01/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */


`timescale 1ns / 1ps
module normal_operator #(
    parameter int DEPTH = 249
) (
    input var CLK,
    cpu_bus_if.normal_port CPU_BUS,
    input var UPDATE,
    output var [7:0] INTENSITY,
    output var [7:0] PHASE,
    output var DOUT_VALID
);

  logic bus_clk;
  logic ena;
  logic we;
  logic [7:0] addr;
  logic [15:0] data_in;

  logic dout_valid;

  logic [7:0] read_addr = 0;
  logic [15:0] dout;

  logic [7:0] intensity;
  logic [7:0] phase;

  assign bus_clk = CPU_BUS.BUS_CLK;
  assign ena = CPU_BUS.NORMAL_EN;
  assign we = CPU_BUS.WE;
  assign addr = CPU_BUS.BRAM_ADDR[7:0];
  assign data_in = CPU_BUS.DATA_IN;

  assign INTENSITY = intensity;
  assign PHASE = phase;
  assign DOUT_VALID = dout_valid;

  BRAM_NORMAL normal_bram (
      .clka (bus_clk),
      .ena  (ena),
      .wea  (we),
      .addra(addr),
      .dina (data_in),
      .douta(),
      .clkb (CLK),
      .web  ('0),
      .addrb(read_addr),
      .dinb ('0),
      .doutb(dout)
  );

  typedef enum logic {
    WAITING,
    RUN
  } state_t;

  state_t state = WAITING;

  always_ff @(posedge CLK) begin
    case (state)
      WAITING: begin
        dout_valid <= 0;
        if (UPDATE) begin
          read_addr <= 0;
          state <= RUN;
        end
      end
      RUN: begin
        read_addr <= read_addr + 1;

        if (read_addr > 1) begin
          dout_valid <= 1;

          phase <= dout[7:0];
          intensity <= dout[15:8];
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
