/*
 * File: controller.sv
 * Project: controller
 * Created Date: 01/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Hapis Lab. All rights reserved.
 *
 */

`timescale 1ns / 1ps
module controller #(
    parameter int WIDTH = 13,
    parameter int DEPTH = 249
) (
    input var CLK,
    input var THERMO,
    output var FORCE_FAN,
    cpu_bus_if.ctl_port CPU_BUS,
    output var [63:0] ECAT_SYNC_TIME,
    output var SYNC_SET,
    output var OP_MODE,
    output var STM_GAIN_MODE,
    output var [15:0] CYCLE_M,
    output var [31:0] FREQ_DIV_M,
    output var [15:0] DELAY_M[DEPTH],
    output var signed [WIDTH:0] FILTER_DUTY[DEPTH],
    output var signed [WIDTH:0] FILTER_PHASE[DEPTH],
    output var [WIDTH-1:0] STEP_S,
    output var [15:0] CYCLE_STM,
    output var [31:0] FREQ_DIV_STM,
    output var [31:0] SOUND_SPEED,
    output var [15:0] STM_START_IDX,
    output var USE_STM_START_IDX,
    output var [15:0] STM_FINISH_IDX,
    output var USE_STM_FINISH_IDX,
    output var [WIDTH-1:0] CYCLE[DEPTH],
    output var LEGACY_MODE
);

  `include "params.vh"

  bit bus_clk;
  bit ctl_ena;
  bit wea;
  bit [8:0] ctl_addr;
  bit [7:0] dly_addr;
  bit [7:0] flt_duty_addr;
  bit [7:0] flt_phase_addr;
  bit [15:0] cpu_data_in;
  bit [15:0] cpu_data_out;
  bit [8:0] addr;
  bit we;
  bit [15:0] din;
  bit [15:0] dout;

  bit [7:0] dly_cnt = 0;
  bit [7:0] dly_set = DEPTH - 2;
  bit [15:0] dly_dout;

  bit [7:0] flt_cnt = 0;
  bit [7:0] flt_set = DEPTH - 2;
  bit [15:0] flt_duty_dout;
  bit [15:0] flt_phase_dout;

  bit [15:0] ctl_reg;

  bit [63:0] ecat_sync_time;
  bit sync_set;
  bit [7:0] set_cnt;

  bit [15:0] cycle_m;
  bit [31:0] freq_div_m;
  bit [15:0] delay_m[DEPTH];
  bit signed [WIDTH:0] filter_duty[DEPTH];
  bit signed [WIDTH:0] filter_phase[DEPTH];
  bit [WIDTH-1:0] step_s;
  bit [15:0] cycle_stm;
  bit [31:0] freq_div_stm;
  bit [31:0] sound_speed;
  bit [15:0] stm_start_idx;
  bit [15:0] stm_finish_idx;
  bit [WIDTH-1:0] cycle[DEPTH];

  bit [2:0] ctl_segment;

  assign ctl_segment = CPU_BUS.BRAM_ADDR[10:8];
  assign bus_clk = CPU_BUS.BUS_CLK;
  assign ctl_ena = CPU_BUS.CTL_EN & (ctl_segment == BRAM_SELECT_CONTROLLER_MAIN | ctl_segment == BRAM_SELECT_CONTROLLER_CYCLE);
  assign wea = CPU_BUS.WE;
  assign ctl_addr = CPU_BUS.BRAM_ADDR[8:0];
  assign dly_ena = CPU_BUS.CTL_EN & (ctl_segment == BRAM_SELECT_CONTROLLER_DELAY);
  assign dly_addr = CPU_BUS.BRAM_ADDR[7:0];
  assign flt_duty_ena = CPU_BUS.CTL_EN & (ctl_segment == BRAM_SELECT_CONTROLLER_FILTER_DUTY);
  assign flt_duty_addr = CPU_BUS.BRAM_ADDR[7:0];
  assign flt_phase_ena = CPU_BUS.CTL_EN & (ctl_segment == BRAM_SELECT_CONTROLLER_FILTER_PHASE);
  assign flt_phase_addr = CPU_BUS.BRAM_ADDR[7:0];
  assign cpu_data_in = CPU_BUS.DATA_IN;
  assign CPU_BUS.DATA_OUT = cpu_data_out;

  assign LEGACY_MODE = ctl_reg[CTL_FLAG_LEGACY_MODE_BIT];
  assign FORCE_FAN = ctl_reg[CTL_FLAG_FORCE_FAN_BIT];
  assign OP_MODE = ctl_reg[CTL_FLAG_OP_MODE_BIT];
  assign STM_GAIN_MODE = ctl_reg[CTL_FLAG_STM_GAIN_MODE_BIT];
  assign USE_STM_START_IDX = ctl_reg[CTL_FLAG_USE_STM_START_IDX_BIT];
  assign USE_STM_FINISH_IDX = ctl_reg[CTL_FLAG_USE_STM_FINISH_IDX_BIT];

  assign ECAT_SYNC_TIME = ecat_sync_time;
  assign SYNC_SET = sync_set;
  assign CYCLE_M = cycle_m;
  assign FREQ_DIV_M = freq_div_m;
  assign STEP_S = step_s;
  assign CYCLE_STM = cycle_stm;
  assign FREQ_DIV_STM = freq_div_stm;
  assign SOUND_SPEED = sound_speed;
  assign STM_START_IDX = stm_start_idx;
  assign STM_FINISH_IDX = stm_finish_idx;

  for (genvar i = 0; i < DEPTH; i++) begin : gen_cycle_delay
    assign CYCLE[i]   = cycle[i];
    assign DELAY_M[i] = delay_m[i];
    assign FILTER_DUTY[i]  = filter_duty[i];
    assign FILTER_PHASE[i]  = filter_phase[i];
  end

  BRAM_CONTROLLER ctl_bram (
      .clka (bus_clk),
      .ena  (ctl_ena),
      .wea  (wea),
      .addra(ctl_addr),
      .dina (cpu_data_in),
      .douta(cpu_data_out),
      .clkb (CLK),
      .web  (we),
      .addrb(addr),
      .dinb (din),
      .doutb(dout)
  );

  BRAM_DELAY dly_bram (
      .clka (bus_clk),
      .ena  (dly_ena),
      .wea  (wea),
      .addra(dly_addr),
      .dina (cpu_data_in),
      .douta(),
      .clkb (CLK),
      .web  (1'b0),
      .addrb(dly_cnt),
      .dinb (),
      .doutb(dly_dout)
  );

  BRAM_FILTER flt_bram_duty (
      .clka (bus_clk),
      .ena  (flt_duty_ena),
      .wea  (wea),
      .addra(flt_duty_addr),
      .dina (cpu_data_in),
      .douta(),
      .clkb (CLK),
      .web  (1'b0),
      .addrb(flt_cnt),
      .dinb (),
      .doutb(flt_duty_dout)
  );

  BRAM_FILTER flt_bram_phase (
      .clka (bus_clk),
      .ena  (flt_phase_ena),
      .wea  (wea),
      .addra(flt_phase_addr),
      .dina (cpu_data_in),
      .douta(),
      .clkb (CLK),
      .web  (1'b0),
      .addrb(flt_cnt),
      .dinb (),
      .doutb(flt_phase_dout)
  );

  typedef enum bit [4:0] {
    REQ_WR_VER_MINOR,
    REQ_WR_VER,
    WAIT_WR_VER_0_REQ_RD_CTL_FLAG,
    WR_VER_MINOR_WAIT_RD_CTL_FLAG_0,
    WR_VER_WAIT_RD_CTL_FLAG_1,

    RD_CTL_FLAG_REQ_RD_MOD_FREQ_DIV_0,
    WR_FPGA_INFO_REQ_RD_MOD_FREQ_DIV_1,
    RD_MOD_CYCLE_REQ_RD_SILENT_CYCLE,
    RD_MOD_FREQ_DIV_0_REQ_RD_SILENT_STEP,
    RD_MOD_FREQ_DIV_1_REQ_RD_STM_CYCLE,
    RD_SILENT_CYCLE_REQ_RD_STM_FREQ_DIV_0,
    RD_SILENT_STEP_REQ_RD_STM_FREQ_DIV_1,
    RD_STM_CYCLE_REQ_RD_SOUND_SPEED_0,
    RD_STM_FREQ_DIV_0_REQ_RD_SOUND_SPEED_1,
    RD_STM_FREQ_DIV_1_REQ_RD_STM_START_IDX,
    RD_SOUND_SPEED_0_REQ_RD_STM_FINISH_IDX,
    RD_SOUND_SPEED_1_REQ_RD_CTL_FLAG,
    RD_STM_START_IDX_REQ_WR_FPGA_INFO,
    RD_STM_FINISH_IDX_REQ_RD_MOD_CYCLE,

    REQ_RD_EC_SYNC_TIME_0,
    REQ_RD_EC_SYNC_TIME_1,
    REQ_RD_EC_SYNC_TIME_2,
    REQ_RD_EC_SYNC_TIME_3_RD_EC_SYNC_TIME_0,
    REQ_RD_CYCLE_0_RD_EC_SYNC_TIME_1,
    REQ_RD_CYCLE_1_RD_EC_SYNC_TIME_2,
    REQ_RD_CYCLE_2_RD_EC_SYNC_TIME_3,
    RD_CYCLE,
    WAIT_CLR_SYNC_BIT_0,
    WAIT_CLR_SYNC_BIT_1,
    CLR_SYNC_BIT
  } state_t;

  state_t state = REQ_WR_VER_MINOR;

  always_ff @(posedge CLK) begin
    case (state)
      ////////////////////////// initial //////////////////////////
      REQ_WR_VER_MINOR: begin
        we <= 1'b1;

        din <= {8'd0, VERSION_NUM_MINOR};
        addr <= ADDR_VERSION_NUM_MINOR;

        state <= REQ_WR_VER;
      end
      REQ_WR_VER: begin
        din   <= {ENABLED_FEATURES_BITS, VERSION_NUM};
        addr  <= ADDR_VERSION_NUM_MAJOR;

        state <= WAIT_WR_VER_0_REQ_RD_CTL_FLAG;
      end
      WAIT_WR_VER_0_REQ_RD_CTL_FLAG: begin
        we <= 1'b0;
        addr <= ADDR_CTL_FLAG;

        state <= WR_VER_MINOR_WAIT_RD_CTL_FLAG_0;
      end
      WR_VER_MINOR_WAIT_RD_CTL_FLAG_0: begin
        state <= WR_VER_WAIT_RD_CTL_FLAG_1;
      end
      WR_VER_WAIT_RD_CTL_FLAG_1: begin
        state <= RD_CTL_FLAG_REQ_RD_MOD_FREQ_DIV_0;
      end
      ////////////////////////// initial //////////////////////////

      //////////////////////////// run ////////////////////////////
      RD_CTL_FLAG_REQ_RD_MOD_FREQ_DIV_0: begin
        ctl_reg <= dout;
        if (ctl_reg[CTL_FLAG_SYNC_BIT]) begin
          we <= 1'b1;
          addr <= ADDR_CTL_FLAG;
          din <= ctl_reg & ~(1 << CTL_FLAG_SYNC_BIT);

          state <= REQ_RD_EC_SYNC_TIME_0;
        end else begin
          addr  <= ADDR_MOD_FREQ_DIV_0;

          state <= WR_FPGA_INFO_REQ_RD_MOD_FREQ_DIV_1;
        end
      end
      WR_FPGA_INFO_REQ_RD_MOD_FREQ_DIV_1: begin
        addr  <= ADDR_MOD_FREQ_DIV_1;

        state <= RD_MOD_CYCLE_REQ_RD_SILENT_CYCLE;
      end
      RD_MOD_CYCLE_REQ_RD_SILENT_CYCLE: begin
        cycle_m <= dout;

        state   <= RD_MOD_FREQ_DIV_0_REQ_RD_SILENT_STEP;
      end
      RD_MOD_FREQ_DIV_0_REQ_RD_SILENT_STEP: begin
        addr <= ADDR_SILENT_STEP;

        freq_div_m[15:0] <= dout;

        state <= RD_MOD_FREQ_DIV_1_REQ_RD_STM_CYCLE;
      end
      RD_MOD_FREQ_DIV_1_REQ_RD_STM_CYCLE: begin
        addr <= ADDR_STM_CYCLE;

        freq_div_m[31:16] <= dout;

        state <= RD_SILENT_CYCLE_REQ_RD_STM_FREQ_DIV_0;
      end
      RD_SILENT_CYCLE_REQ_RD_STM_FREQ_DIV_0: begin
        addr  <= ADDR_STM_FREQ_DIV_0;

        state <= RD_SILENT_STEP_REQ_RD_STM_FREQ_DIV_1;
      end
      RD_SILENT_STEP_REQ_RD_STM_FREQ_DIV_1: begin
        addr   <= ADDR_STM_FREQ_DIV_1;

        step_s <= dout[WIDTH-1:0];

        state  <= RD_STM_CYCLE_REQ_RD_SOUND_SPEED_0;
      end
      RD_STM_CYCLE_REQ_RD_SOUND_SPEED_0: begin
        addr <= ADDR_SOUND_SPEED_0;

        cycle_stm <= dout;

        state <= RD_STM_FREQ_DIV_0_REQ_RD_SOUND_SPEED_1;
      end
      RD_STM_FREQ_DIV_0_REQ_RD_SOUND_SPEED_1: begin
        addr <= ADDR_SOUND_SPEED_1;

        freq_div_stm[15:0] <= dout;

        state <= RD_STM_FREQ_DIV_1_REQ_RD_STM_START_IDX;
      end
      RD_STM_FREQ_DIV_1_REQ_RD_STM_START_IDX: begin
        addr <= ADDR_STM_START_IDX;

        freq_div_stm[31:16] <= dout;

        state <= RD_SOUND_SPEED_0_REQ_RD_STM_FINISH_IDX;
      end
      RD_SOUND_SPEED_0_REQ_RD_STM_FINISH_IDX: begin
        addr <= ADDR_STM_FINISH_IDX;

        sound_speed[15:0] <= dout;

        state <= RD_SOUND_SPEED_1_REQ_RD_CTL_FLAG;
      end
      RD_SOUND_SPEED_1_REQ_RD_CTL_FLAG: begin
        addr <= ADDR_CTL_FLAG;

        sound_speed[31:16] <= dout;

        state <= RD_STM_START_IDX_REQ_WR_FPGA_INFO;
      end
      RD_STM_START_IDX_REQ_WR_FPGA_INFO: begin
        we <= 1'b1;
        addr <= ADDR_FPGA_INFO;
        din <= {15'h00, THERMO};
        stm_start_idx <= dout;

        state <= RD_STM_FINISH_IDX_REQ_RD_MOD_CYCLE;
      end
      RD_STM_FINISH_IDX_REQ_RD_MOD_CYCLE: begin
        we <= 1'b0;
        addr <= ADDR_MOD_CYCLE;

        stm_finish_idx <= dout;

        state <= RD_CTL_FLAG_REQ_RD_MOD_FREQ_DIV_0;
      end
      //////////////////////////// run ////////////////////////////

      //////////////////////// synchronize ////////////////////////
      REQ_RD_EC_SYNC_TIME_0: begin
        we <= 1'b0;

        addr <= ADDR_EC_SYNC_TIME_0;

        state <= REQ_RD_EC_SYNC_TIME_1;
      end
      REQ_RD_EC_SYNC_TIME_1: begin
        addr  <= ADDR_EC_SYNC_TIME_1;

        state <= REQ_RD_EC_SYNC_TIME_2;
      end
      REQ_RD_EC_SYNC_TIME_2: begin
        addr  <= ADDR_EC_SYNC_TIME_2;

        state <= REQ_RD_EC_SYNC_TIME_3_RD_EC_SYNC_TIME_0;
      end
      REQ_RD_EC_SYNC_TIME_3_RD_EC_SYNC_TIME_0: begin
        addr <= ADDR_EC_SYNC_TIME_3;

        ecat_sync_time[15:0] <= dout;

        state <= REQ_RD_CYCLE_0_RD_EC_SYNC_TIME_1;
      end
      REQ_RD_CYCLE_0_RD_EC_SYNC_TIME_1: begin
        addr <= ADDR_CYCLE_BASE;

        ecat_sync_time[31:16] <= dout;

        state <= REQ_RD_CYCLE_1_RD_EC_SYNC_TIME_2;
      end
      REQ_RD_CYCLE_1_RD_EC_SYNC_TIME_2: begin
        addr <= addr + 1;

        ecat_sync_time[47:32] <= dout;

        state <= REQ_RD_CYCLE_2_RD_EC_SYNC_TIME_3;
      end
      REQ_RD_CYCLE_2_RD_EC_SYNC_TIME_3: begin
        addr <= addr + 1;

        ecat_sync_time[63:48] <= dout;

        set_cnt <= 0;

        state <= RD_CYCLE;
      end
      RD_CYCLE: begin
        cycle[set_cnt] <= dout[WIDTH-1:0];
        if (set_cnt == DEPTH - 1) begin
          addr <= ADDR_CTL_FLAG;

          sync_set <= 1'b1;

          state <= WAIT_CLR_SYNC_BIT_0;
        end else begin
          addr <= addr + 1;
          set_cnt <= set_cnt + 1;

          state <= RD_CYCLE;
        end
      end
      WAIT_CLR_SYNC_BIT_0: begin
        sync_set <= 1'b0;

        state <= WAIT_CLR_SYNC_BIT_1;
      end
      WAIT_CLR_SYNC_BIT_1: begin
        state <= CLR_SYNC_BIT;
      end
      CLR_SYNC_BIT: begin
        ctl_reg <= dout;

        state   <= RD_CTL_FLAG_REQ_RD_MOD_FREQ_DIV_0;
      end
      //////////////////////// synchronize ////////////////////////

      default: begin
      end
    endcase
  end

  always_ff @(posedge CLK) begin
    dly_cnt <= (dly_cnt == DEPTH - 1) ? 0 : dly_cnt + 1;
    dly_set <= (dly_set == DEPTH - 1) ? 0 : dly_set + 1;
    delay_m[dly_set] <= dly_dout;
  end

  always_ff @(posedge CLK) begin
    flt_cnt <= (flt_cnt == DEPTH - 1) ? 0 : flt_cnt + 1;
    flt_set <= (flt_set == DEPTH - 1) ? 0 : flt_set + 1;
    filter_duty[flt_set] <= flt_duty_dout;
    filter_phase[flt_set] <= flt_phase_dout;
  end

endmodule
