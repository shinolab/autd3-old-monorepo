/*
 * File: cpu_bus_if.sv
 * Project: bus
 * Created Date: 25/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

interface cpu_bus_if ();

  `include "params.vh"

  logic BUS_CLK;
  logic EN;
  logic RD;
  logic WE;
  logic RDWR;
  logic [1:0] BRAM_SELECT;
  logic [13:0] BRAM_ADDR;
  logic [15:0] CPU_DATA;
  logic [15:0] DATA_IN;
  logic [15:0] DATA_OUT;

  logic [2:0] ctl_we_edge = 3'b000;

  assign CPU_DATA = (EN & RD & RDWR) ? DATA_OUT : 16'bzzzzzzzzzzzzzzzz;
  assign DATA_IN  = CPU_DATA;

  //////////////////////////// Controller ////////////////////////////
  logic CTL_EN;
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
  logic NORMAL_EN;
  assign NORMAL_EN = (BRAM_SELECT == BRAM_SELECT_NORMAL) & EN;

  modport normal_port(input BUS_CLK, input NORMAL_EN, input WE, input BRAM_ADDR, input DATA_IN);
  ////////////////////////// Normal Operator //////////////////////////

  ///////////////////////// STM Operator /////////////////////////
  logic STM_EN;
  assign STM_EN = (BRAM_SELECT == BRAM_SELECT_STM) & EN;
  logic [4:0] STM_MEM_PAGE;

  modport stm_port(
      input BUS_CLK,
      input STM_EN,
      input WE,
      input BRAM_ADDR,
      input STM_MEM_PAGE,
      input DATA_IN
  );
  ///////////////////////// STM Operator /////////////////////////

  ///////////////////////////// Modulator /////////////////////////////
  logic MOD_EN;
  assign MOD_EN = (BRAM_SELECT == BRAM_SELECT_MOD) & EN;
  logic MOD_MEM_PAGE;

  modport mod_port(
      input BUS_CLK,
      input MOD_EN,
      input WE,
      input BRAM_ADDR,
      input MOD_MEM_PAGE,
      input DATA_IN
  );
  ///////////////////////////// Modulator /////////////////////////////

  always_ff @(posedge BUS_CLK) begin
    ctl_we_edge <= {ctl_we_edge[1:0], (WE & CTL_EN)};
    if (ctl_we_edge == 3'b011) begin
      case (BRAM_ADDR)
        ADDR_MOD_MEM_PAGE: MOD_MEM_PAGE <= DATA_IN[0];
        ADDR_STM_MEM_PAGE: begin
          STM_MEM_PAGE <= DATA_IN[4:0];
        end
        default: begin
        end
      endcase
    end
  end

endinterface
