/*
 * File: normal.sv
 * Project: operator
 * Created Date: 01/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */


`timescale 1ns / 1ps
module normal_operator#(
           parameter int WIDTH = 13,
           parameter int DEPTH = 249
       )(
           input var CLK,
           input var RST,
           cpu_bus_if.normal_port CPU_BUS,
           input var [WIDTH-1:0] CYCLE[0:DEPTH-1],
           input var [1:0] LOAD_MODE,
           output var [WIDTH-1:0] DUTY[0:DEPTH-1],
           output var [WIDTH-1:0] PHASE[0:DEPTH-1]
       );

`include "params.vh"

bit bus_clk;
bit ena;
bit we;
bit [8:0] addr;
bit [15:0] data_in;

bit [7:0] read_addr = 0;
bit [7:0] set_addr = (DEPTH - 2) % DEPTH;
bit [31:0] dout;

bit [WIDTH-1:0] duty_buf[0:DEPTH-1];
bit [WIDTH-1:0] phase_buf[0:DEPTH-1];
bit [WIDTH-1:0] duty[0:DEPTH-1];
bit [WIDTH-1:0] phase[0:DEPTH-1];

assign bus_clk = CPU_BUS.BUS_CLK;
assign ena = CPU_BUS.NORMAL_EN;
assign we = CPU_BUS.WE;
assign addr = CPU_BUS.BRAM_ADDR[8:0];
assign data_in = CPU_BUS.DATA_IN;

for (genvar i = 0; i < DEPTH; i++) begin
    assign DUTY[i] = duty[i];
    assign PHASE[i] = phase[i];
end

BRAM_NORMAL normal_bram(
                .clka(bus_clk),
                .ena(ena),
                .wea(we),
                .addra(addr),
                .dina(data_in),
                .douta(),
                .clkb(CLK),
                .web('0),
                .addrb(read_addr),
                .dinb('0),
                .doutb(dout)
            );

always_ff @(posedge CLK) begin
    if (read_addr == DEPTH - 1) begin
        read_addr <= 0;
    end
    else begin
        read_addr <= read_addr + 1;
    end

    if (set_addr == DEPTH -1) begin
        set_addr <= 0;
    end
    else begin
        set_addr <= set_addr + 1;
    end

    case(LOAD_MODE)
        LOAD_LEGACY: begin
            phase_buf[set_addr] <= {dout[7:0], 5'h00};
            duty_buf[set_addr] <= {dout[15:8], 5'h1F} + 1;
        end
        LOAD_RAW: begin
            phase_buf[set_addr] <= dout[WIDTH-1:0];
            duty_buf[set_addr] <= dout[WIDTH-1+16:16];
        end
        LOAD_DUTY_SHIFT_RAW_PHASE: begin
            phase_buf[set_addr] <= dout[WIDTH-1:0];
            duty_buf[set_addr] <= CYCLE[set_addr][WIDTH-1:1] >> dout[19:16];
        end
    endcase
end

always_ff @(posedge CLK) begin
    if (RST) begin
        duty <= '{DEPTH{0}};
        phase <= '{DEPTH{0}};
    end
    else begin
        if (set_addr == 0) begin
            duty <= duty_buf;
            phase <= phase_buf;
        end
    end
end

endmodule
