/*
 * File: params.vh
 * Project: headers
 * Created Date: 22/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */

localparam bit [7:0] VERSION_NUM = 8'h80;

localparam string ENABLE_STM = "TRUE";
localparam string ENABLE_MODULATOR = "TRUE";
localparam string ENABLE_SILENCER = "TRUE";

localparam bit [1:0] BRAM_SELECT_CONTROLLER = 2'h0;
localparam bit [1:0] BRAM_SELECT_MOD        = 2'h1;
localparam bit [1:0] BRAM_SELECT_NORMAL     = 2'h2;
localparam bit [1:0] BRAM_SELECT_STM        = 2'h3;

localparam bit [13:0] ADDR_CTL_REG             = 14'h000;
localparam bit [13:0] ADDR_FPGA_INFO           = 14'h001;
localparam bit [13:0] ADDR_EC_SYNC_CYCLE_TICKS = 14'h010;
localparam bit [13:0] ADDR_EC_SYNC_TIME_0      = ADDR_EC_SYNC_CYCLE_TICKS + 1;
localparam bit [13:0] ADDR_EC_SYNC_TIME_1      = ADDR_EC_SYNC_CYCLE_TICKS + 2;
localparam bit [13:0] ADDR_EC_SYNC_TIME_2      = ADDR_EC_SYNC_CYCLE_TICKS + 3;
localparam bit [13:0] ADDR_EC_SYNC_TIME_3      = ADDR_EC_SYNC_CYCLE_TICKS + 4;
localparam bit [13:0] ADDR_MOD_ADDR_OFFSET     = 14'h020;
localparam bit [13:0] ADDR_MOD_CYCLE           = 14'h021;
localparam bit [13:0] ADDR_MOD_FREQ_DIV_0      = 14'h022;
localparam bit [13:0] ADDR_MOD_FREQ_DIV_1      = 14'h023;
localparam bit [13:0] ADDR_VERSION_NUM         = 14'h03F; // For backward compatibility
localparam bit [13:0] ADDR_SILENT_CYCLE        = 14'h040;
localparam bit [13:0] ADDR_SILENT_STEP         = 14'h041;
localparam bit [13:0] ADDR_STM_ADDR_OFFSET     = 14'h050;
localparam bit [13:0] ADDR_STM_CYCLE           = 14'h051;
localparam bit [13:0] ADDR_STM_FREQ_DIV_0      = 14'h052;
localparam bit [13:0] ADDR_STM_FREQ_DIV_1      = 14'h053;
localparam bit [13:0] ADDR_SOUND_SPEED_0       = 14'h054;
localparam bit [13:0] ADDR_SOUND_SPEED_1       = 14'h055;
localparam bit [13:0] ADDR_CYCLE_BASE          = 14'h100;

localparam int CTL_REG_LEGACY_MODE_BIT     = 0;
localparam int CTL_REG_FORCE_FAN_BIT       = 4;
localparam int CTL_REG_OP_MODE_BIT         = 5;
localparam int CTL_REG_STM_GAIN_MODE_BIT   = 6;
localparam int CTL_REG_SYNC_BIT            = 14;
localparam int CTL_REG_WDT_RST_BIT         = 15;

localparam bit [7:0] ENABLED_STM_BIT = ENABLE_STM == "TRUE" ? 8'h01 : 8'h00;
localparam bit [7:0] ENABLED_MODULATOR_BIT = ENABLE_MODULATOR == "TRUE" ? 8'h02 : 8'h00;
localparam bit [7:0] ENABLED_SILENCER_BIT = ENABLE_SILENCER == "TRUE" ? 8'h04 : 8'h00;
localparam bit [7:0] ENABLED_FEATURES_BITS = ENABLED_STM_BIT | ENABLED_MODULATOR_BIT | ENABLED_SILENCER_BIT;
