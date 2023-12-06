/*
 * File: params.vh
 * Project: headers
 * Created Date: 22/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */

localparam int NUM_TRANSDUCERS = 249;

localparam bit [7:0] VERSION_NUM = 8'h8C;
localparam bit [7:0] VERSION_NUM_MINOR = 8'h00;

localparam bit [1:0] BRAM_SELECT_CONTROLLER = 2'h0;
localparam bit [1:0] BRAM_SELECT_MOD = 2'h1;
localparam bit [1:0] BRAM_SELECT_NORMAL = 2'h2;
localparam bit [1:0] BRAM_SELECT_STM = 2'h3;

localparam bit [2:0] BRAM_SELECT_CONTROLLER_MAIN = 3'b000;
localparam bit [2:0] BRAM_SELECT_CONTROLLER_DELAY = 3'b010;

localparam bit [13:0] ADDR_CTL_FLAG = 14'h000;
localparam bit [13:0] ADDR_FPGA_INFO = 14'h001;
localparam bit [13:0] ADDR_EC_SYNC_TIME_0 = 14'h011;
localparam bit [13:0] ADDR_EC_SYNC_TIME_1 = ADDR_EC_SYNC_TIME_0 + 1;
localparam bit [13:0] ADDR_EC_SYNC_TIME_2 = ADDR_EC_SYNC_TIME_0 + 2;
localparam bit [13:0] ADDR_EC_SYNC_TIME_3 = ADDR_EC_SYNC_TIME_0 + 3;
localparam bit [13:0] ADDR_MOD_MEM_PAGE = 14'h020;
localparam bit [13:0] ADDR_MOD_CYCLE = 14'h021;
localparam bit [13:0] ADDR_MOD_FREQ_DIV_0 = 14'h022;
localparam bit [13:0] ADDR_MOD_FREQ_DIV_1 = 14'h023;
localparam bit [13:0] ADDR_VERSION_NUM_MAJOR = 14'h030;
localparam bit [13:0] ADDR_VERSION_NUM_MINOR = 14'h031;
localparam bit [13:0] ADDR_SILENT_STEP_INTENSITY = 14'h040;
localparam bit [13:0] ADDR_SILENT_STEP_PHASE = 14'h041;
localparam bit [13:0] ADDR_STM_MEM_PAGE = 14'h050;
localparam bit [13:0] ADDR_STM_CYCLE = 14'h051;
localparam bit [13:0] ADDR_STM_FREQ_DIV_0 = 14'h052;
localparam bit [13:0] ADDR_STM_FREQ_DIV_1 = 14'h053;
localparam bit [13:0] ADDR_SOUND_SPEED_0 = 14'h054;
localparam bit [13:0] ADDR_SOUND_SPEED_1 = 14'h055;
localparam bit [13:0] ADDR_STM_START_IDX = 14'h056;
localparam bit [13:0] ADDR_STM_FINISH_IDX = 14'h057;
localparam bit [13:0] ADDR_DEBUG_OUT_IDX = 14'h0F0;
localparam bit [13:0] ADDR_DELAY_BASE = 14'h200;

localparam int CTL_FLAG_FORCE_FAN_BIT = 0;
localparam int CTL_FLAG_OP_MODE_BIT = 9;
localparam int CTL_FLAG_STM_GAIN_MODE_BIT = 10;
localparam int CTL_FLAG_USE_STM_FINISH_IDX_BIT = 11;
localparam int CTL_FLAG_USE_STM_START_IDX_BIT = 12;
localparam int CTL_FLAG_FORCE_FAN_EX_BIT = 13;
localparam int CTL_FLAG_SYNC_BIT = 15;
