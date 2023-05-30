/*
 * File: cpu_bus_if.sv
 * Project: bus
 * Created Date: 25/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

interface cpu_bus_if ();

  `include "params.vh"

  bit BUS_CLK;
  bit EN;
  bit RD;
  bit WE;
  bit RDWR;
  bit [1:0] BRAM_SELECT;
  bit [13:0] BRAM_ADDR;
  bit [15:0] CPU_DATA;
  bit [15:0] DATA_IN;
  bit [15:0] DATA_OUT;

  bit [2:0] ctl_we_edge = 3'b000;

  assign CPU_DATA = (EN & RD & RDWR) ? DATA_OUT : 16'bzzzzzzzzzzzzzzzz;
  assign DATA_IN  = CPU_DATA;

  //////////////////////////// Controller ////////////////////////////
  bit CTL_EN;
  assign CTL_EN = (BRAM_SELECT == BRAM_SELECT_CONTROLLER) & EN;

  modport ctl_port(
      input BUS_CLK,
      input CTL_EN,
      input WE,
      input BRAM_ADDR,
      input DATA_IN,
      output DATA_OUT
  );
  //////////////////////////// Controller ////////////////////////////

  ////////////////////////// Normal Operator //////////////////////////
  bit NORMAL_EN;
  assign NORMAL_EN = (BRAM_SELECT == BRAM_SELECT_NORMAL) & EN;

  modport normal_port(input BUS_CLK, input NORMAL_EN, input WE, input BRAM_ADDR, input DATA_IN);
  ////////////////////////// Normal Operator //////////////////////////

  ///////////////////////// STM Operator /////////////////////////
  bit STM_EN;
  assign STM_EN = (BRAM_SELECT == BRAM_SELECT_STM) & EN;
  bit [4:0] STM_MEM_SEGMENT;

  modport stm_port(
      input BUS_CLK,
      input STM_EN,
      input WE,
      input BRAM_ADDR,
      input STM_MEM_SEGMENT,
      input DATA_IN
  );
  ///////////////////////// STM Operator /////////////////////////

  ///////////////////////////// Modulator /////////////////////////////
  bit MOD_EN;
  assign MOD_EN = (BRAM_SELECT == BRAM_SELECT_MOD) & EN;
  bit MOD_MEM_SEGMENT;

  modport mod_port(
      input BUS_CLK,
      input MOD_EN,
      input WE,
      input BRAM_ADDR,
      input MOD_MEM_SEGMENT,
      input DATA_IN
  );
  ///////////////////////////// Modulator /////////////////////////////

  always_ff @(posedge BUS_CLK) begin
    ctl_we_edge <= {ctl_we_edge[1:0], (WE & CTL_EN)};
    if (ctl_we_edge == 3'b011) begin
      case (BRAM_ADDR)
        ADDR_MOD_MEM_SEGMENT: MOD_MEM_SEGMENT <= DATA_IN[0];
        ADDR_STM_MEM_SEGMENT: begin
          STM_MEM_SEGMENT <= DATA_IN[4:0];
        end
        default: begin
        end
      endcase
    end
  end

endinterface
