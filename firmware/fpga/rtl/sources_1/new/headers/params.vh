/*
 * File: params.vh
 * Project: headers
 * Created Date: 22/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */

localparam bit [7:0] VERSION_NUM = 8'h88;
localparam bit [7:0] VERSION_NUM_MINOR = 8'h01;

localparam string ENABLE_STM = "TRUE";
localparam string ENABLE_MODULATOR = "TRUE";
localparam string ENABLE_SILENCER = "TRUE";
localparam string ENABLE_MODULATOR_DELAY = "TRUE";

localparam bit [1:0] BRAM_SELECT_CONTROLLER = 2'h0;
localparam bit [1:0] BRAM_SELECT_MOD = 2'h1;
localparam bit [1:0] BRAM_SELECT_NORMAL = 2'h2;
localparam bit [1:0] BRAM_SELECT_STM = 2'h3;

localparam bit [13:0] ADDR_CTL_FLAG = 14'h000;
localparam bit [13:0] ADDR_FPGA_INFO = 14'h001;
localparam bit [13:0] ADDR_EC_SYNC_TIME_0 = 14'h011;
localparam bit [13:0] ADDR_EC_SYNC_TIME_1 = ADDR_EC_SYNC_TIME_0 + 1;
localparam bit [13:0] ADDR_EC_SYNC_TIME_2 = ADDR_EC_SYNC_TIME_0 + 2;
localparam bit [13:0] ADDR_EC_SYNC_TIME_3 = ADDR_EC_SYNC_TIME_0 + 3;
localparam bit [13:0] ADDR_MOD_MEM_SEGMENT = 14'h020;
localparam bit [13:0] ADDR_MOD_CYCLE = 14'h021;
localparam bit [13:0] ADDR_MOD_FREQ_DIV_0 = 14'h022;
localparam bit [13:0] ADDR_MOD_FREQ_DIV_1 = 14'h023;
localparam bit [13:0] ADDR_VERSION_NUM_MAJOR = 14'h03F;  // For backward compatibility
localparam bit [13:0] ADDR_VERSION_NUM_MINOR = 14'h03E;
localparam bit [13:0] ADDR_SILENT_STEP = 14'h041;
localparam bit [13:0] ADDR_STM_MEM_SEGMENT = 14'h050;
localparam bit [13:0] ADDR_STM_CYCLE = 14'h051;
localparam bit [13:0] ADDR_STM_FREQ_DIV_0 = 14'h052;
localparam bit [13:0] ADDR_STM_FREQ_DIV_1 = 14'h053;
localparam bit [13:0] ADDR_SOUND_SPEED_0 = 14'h054;
localparam bit [13:0] ADDR_SOUND_SPEED_1 = 14'h055;
localparam bit [13:0] ADDR_STM_START_IDX = 14'h056;
localparam bit [13:0] ADDR_STM_FINISH_IDX = 14'h057;
localparam bit [13:0] ADDR_CYCLE_BASE = 14'h100;
localparam bit [13:0] ADDR_DELAY_BASE = 14'h200;

localparam int CTL_FLAG_LEGACY_MODE_BIT = 0;
localparam int CTL_FLAG_USE_STM_FINISH_IDX_BIT = 2;
localparam int CTL_FLAG_USE_STM_START_IDX_BIT = 3;
localparam int CTL_FLAG_FORCE_FAN_BIT = 4;
localparam int CTL_FLAG_STM_GAIN_MODE_BIT = 6;
localparam int CTL_FLAG_SYNC_BIT = 8;
localparam int CTL_FLAG_OP_MODE_BIT = 9;

localparam bit [7:0] ENABLED_STM_BIT = ENABLE_STM == "TRUE" ? 8'h01 : 8'h00;
localparam bit [7:0] ENABLED_MODULATOR_BIT = ENABLE_MODULATOR == "TRUE" ? 8'h02 : 8'h00;
localparam bit [7:0] ENABLED_SILENCER_BIT = ENABLE_SILENCER == "TRUE" ? 8'h04 : 8'h00;
localparam bit [7:0] ENABLED_MODULATOR_DELAY_BIT = ENABLE_MODULATOR_DELAY == "TRUE" ? 8'h08 : 8'h00;
localparam bit [7:0] ENABLED_FEATURES_BITS = ENABLED_MODULATOR_DELAY_BIT | ENABLED_STM_BIT | ENABLED_MODULATOR_BIT | ENABLED_SILENCER_BIT;
