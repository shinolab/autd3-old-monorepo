/*
 * File: ultrasound_cnt_clk_gen.sv
 * Project: new
 * Created Date: 12/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

module ultrasound_cnt_clk_gen (
    input var  clk_in1,
    input var  reset,
    output var clk_out1,
    output var clk_out2,
    output var locked
);

  `include "params.vh"

  bit CLKOUT0, CLKOUT1;
  bit CLKFB, CLKFB_BUF;
  BUFG clkf_buf (
      .O(CLKFB_BUF),
      .I(CLKFB)
  );
  BUFG clkout1_buf (
      .O(clk_out1),
      .I(CLKOUT0)
  );
  BUFG clkout2_buf (
      .O(clk_out2),
      .I(CLKOUT1)
  );

  if (SUB_CLOCK_FREQ == "81.92MHZ") begin
    MMCME2_ADV #(
        .BANDWIDTH("OPTIMIZED"),
        .CLKFBOUT_MULT_F(32.000),
        .CLKIN1_PERIOD(39.063),
        .CLKOUT1_DIVIDE(10),
        .CLKOUT0_DIVIDE_F(5.0),
        .COMPENSATION("ZHOLD"),
        .DIVCLK_DIVIDE(1),
        .REF_JITTER1(0.010),
        .REF_JITTER2(0.010),
        .STARTUP_WAIT("FALSE"),
        .SS_EN("FALSE"),
        .SS_MODE("CENTER_HIGH"),
        .SS_MOD_PERIOD(10000),
        .CLKFBOUT_USE_FINE_PS("FALSE"),
        .CLKOUT0_USE_FINE_PS("FALSE"),
        .CLKOUT1_USE_FINE_PS("FALSE")
    ) MMCME2_ADV_inst (
        .CLKOUT0(CLKOUT0),
        .CLKOUT0B(),
        .CLKOUT1(CLKOUT1),
        .CLKOUT1B(),
        .CLKFBOUT(CLKFB),
        .CLKFBOUTB(),
        .CLKFBSTOPPED(),
        .CLKINSTOPPED(),
        .LOCKED(locked),
        .CLKIN1(clk_in1),
        .CLKINSEL(1'b1),
        .PWRDWN(1'b0),
        .RST(reset),
        .CLKFBIN(CLKFB_BUF),
        .CLKOUT2(),
        .CLKOUT2B(),
        .CLKOUT3(),
        .CLKOUT3B(),
        .CLKOUT4(),
        .CLKOUT5(),
        .CLKOUT6(),
        .DO(),
        .DRDY(),
        .PSDONE(),
        .CLKIN2(),
        .DADDR(),
        .DCLK(),
        .DEN(),
        .DI(),
        .DWE(),
        .PSCLK(),
        .PSEN(),
        .PSINCDEC()
    );
  end else if (SUB_CLOCK_FREQ == "40.96MHZ") begin
    MMCME2_ADV #(
        .BANDWIDTH("OPTIMIZED"),
        .CLKFBOUT_MULT_F(40.000),
        .CLKIN1_PERIOD(39.063),
        .CLKOUT1_DIVIDE(25),
        .CLKOUT0_DIVIDE_F(6.250),
        .COMPENSATION("ZHOLD"),
        .DIVCLK_DIVIDE(1),
        .REF_JITTER1(0.010),
        .REF_JITTER2(0.010),
        .STARTUP_WAIT("FALSE"),
        .SS_EN("FALSE"),
        .SS_MODE("CENTER_HIGH"),
        .SS_MOD_PERIOD(10000),
        .CLKFBOUT_USE_FINE_PS("FALSE"),
        .CLKOUT0_USE_FINE_PS("FALSE"),
        .CLKOUT1_USE_FINE_PS("FALSE")
    ) MMCME2_ADV_inst (
        .CLKOUT0(CLKOUT0),
        .CLKOUT0B(),
        .CLKOUT1(CLKOUT1),
        .CLKOUT1B(),
        .CLKFBOUT(CLKFB),
        .CLKFBOUTB(),
        .CLKFBSTOPPED(),
        .CLKINSTOPPED(),
        .LOCKED(locked),
        .CLKIN1(clk_in1),
        .CLKINSEL(1'b1),
        .PWRDWN(1'b0),
        .RST(reset),
        .CLKFBIN(CLKFB_BUF),
        .CLKOUT2(),
        .CLKOUT2B(),
        .CLKOUT3(),
        .CLKOUT3B(),
        .CLKOUT4(),
        .CLKOUT5(),
        .CLKOUT6(),
        .DO(),
        .DRDY(),
        .PSDONE(),
        .CLKIN2(),
        .DADDR(),
        .DCLK(),
        .DEN(),
        .DI(),
        .DWE(),
        .PSCLK(),
        .PSEN(),
        .PSINCDEC()
    );
  end

endmodule
