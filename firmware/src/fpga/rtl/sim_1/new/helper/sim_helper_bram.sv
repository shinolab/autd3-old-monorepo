/*
 * File: sim_helper_bram.sv
 * Project: helper
 * Created Date: 25/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module sim_helper_bram #(
    parameter int DEPTH = 249
) ();

  `include "params.vh"

  // CPU
  logic [15:0] bram_addr;
  logic [16:0] CPU_ADDR;
  assign CPU_ADDR = {bram_addr, 1'b1};
  logic [15:0] CPU_DATA;
  logic CPU_CKIO;
  logic CPU_CS1_N;
  logic CPU_WE0_N;
  logic [15:0] CPU_DATA_READ;
  logic [15:0] bus_data_reg = 16'bzzzzzzzzzzzzzzzz;
  assign CPU_DATA = bus_data_reg;

  cpu_bus_if cpu_bus ();
  assign cpu_bus.BUS_CLK = CPU_CKIO;
  assign cpu_bus.EN = ~CPU_CS1_N;
  assign cpu_bus.WE = ~CPU_WE0_N;
  assign cpu_bus.BRAM_SELECT = CPU_ADDR[16:15];
  assign cpu_bus.BRAM_ADDR = CPU_ADDR[14:1];
  assign cpu_bus.DATA_IN = CPU_DATA;

  task automatic bram_write(input logic [1:0] select, input logic [13:0] addr,
                            input logic [15:0] data_in);
    @(posedge CPU_CKIO);
    bram_addr <= {select, addr};
    CPU_CS1_N <= 0;
    bus_data_reg <= data_in;
    @(posedge CPU_CKIO);
    @(negedge CPU_CKIO);

    CPU_WE0_N <= 0;
    repeat (2) @(posedge CPU_CKIO);

    @(negedge CPU_CKIO);
    CPU_WE0_N <= 1;
  endtask

  task automatic write_stm_gain_intensity_phase(int idx, input logic [7:0] intensity[DEPTH],
                                                input logic [7:0] phase[DEPTH]);
    logic [15:0] offset;
    logic [15:0] i;
    offset = idx[21:6];
    i = idx[5:0] << 8;
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_STM_MEM_PAGE, offset);
    for (int j = 0; j < DEPTH; j++) begin
      bram_write(BRAM_SELECT_STM, i + j, {intensity[j][7:0], phase[j][7:0]});
    end
  endtask

  task automatic write_stm_focus(int idx, input logic [17:0] x, input logic [17:0] y,
                                 input logic [17:0] z, input logic [7:0] intensity_shift);
    logic [15:0] offset = idx[15:11];
    logic [15:0] i = idx[10:0] << 3;
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_STM_MEM_PAGE, offset);
    bram_write(BRAM_SELECT_STM, i, x[15:0]);
    bram_write(BRAM_SELECT_STM, i + 1, {y[13:0], x[17:16]});
    bram_write(BRAM_SELECT_STM, i + 2, {z[11:0], y[17:14]});
    bram_write(BRAM_SELECT_STM, i + 3, {2'd0, intensity_shift, z[17:12]});
  endtask

  task automatic set_mod_bram_offset(input logic offset);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_MOD_MEM_PAGE, offset);
  endtask

  task automatic write_mod(input logic [7:0] mod_data[65536], int cnt);
    int addr;
    set_mod_bram_offset(0);
    for (int i = 0; i < cnt; i += 2) begin
      addr = i >> 1;
      bram_write(BRAM_SELECT_MOD, addr[13:0], {mod_data[i+1], mod_data[i]});
      if (addr === 32'h3fff) begin
        set_mod_bram_offset(1);
      end
    end
  endtask

  task automatic write_intensity_phase(int idx, logic [7:0] intensity, logic [7:0] phase);
    bram_write(BRAM_SELECT_NORMAL, idx, {intensity, phase});
  endtask

  task automatic set_ctl_reg(logic force_fan, logic sync);
    automatic
    logic [15:0]
    ctl_reg = (sync << CTL_FLAG_SYNC_BIT) | (force_fan << CTL_FLAG_FORCE_FAN_BIT);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_CTL_FLAG, ctl_reg);
  endtask

  task automatic write_ecat_sync_time(logic [63:0] ecat_sync_time);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_EC_SYNC_TIME_0, ecat_sync_time[15:0]);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_EC_SYNC_TIME_1, ecat_sync_time[31:16]);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_EC_SYNC_TIME_2, ecat_sync_time[47:32]);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_EC_SYNC_TIME_3, ecat_sync_time[63:48]);
  endtask

  task automatic write_mod_cycle(logic [15:0] mod_cycle);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_MOD_CYCLE, mod_cycle);
  endtask

  task automatic write_mod_freq_div(logic [31:0] mod_freq_div);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_MOD_FREQ_DIV_0, mod_freq_div[15:0]);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_MOD_FREQ_DIV_1, mod_freq_div[31:16]);
  endtask

  task automatic write_silent_step(logic [15:0] silent_step_intensity,
                                   logic [15:0] silent_step_phase);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_SILENT_STEP_INTENSITY, silent_step_intensity);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_SILENT_STEP_PHASE, silent_step_phase);
  endtask

  task automatic write_stm_cycle(logic [15:0] stm_cycle);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_STM_CYCLE, stm_cycle);
  endtask

  task automatic write_stm_freq_div(logic [31:0] stm_freq_div);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_STM_FREQ_DIV_0, stm_freq_div[15:0]);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_STM_FREQ_DIV_1, stm_freq_div[31:16]);
  endtask

  task automatic write_sound_speed(logic [31:0] sound_speed);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_SOUND_SPEED_0, sound_speed[15:0]);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_SOUND_SPEED_1, sound_speed[31:16]);
  endtask

  task automatic write_stm_start_idx(logic [15:0] stm_start_idx);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_STM_START_IDX, stm_start_idx);
  endtask

  task automatic write_stm_finish_idx(logic [15:0] stm_finish_idx);
    bram_write(BRAM_SELECT_CONTROLLER, ADDR_STM_FINISH_IDX, stm_finish_idx);
  endtask

  task automatic write_delay(logic [15:0] delay[DEPTH]);
    for (int i = 0; i < DEPTH; i++) begin
      bram_write(BRAM_SELECT_CONTROLLER, ADDR_DELAY_BASE + i, delay[i]);
    end
  endtask

  initial begin
    CPU_WE0_N = 1;
    bram_addr = 0;
    CPU_CKIO  = 0;
  end

  //  always #6.65 CPU_CKIO = ~CPU_CKIO;
  always #0.1 CPU_CKIO = ~CPU_CKIO;  // to speed up simulation

endmodule
