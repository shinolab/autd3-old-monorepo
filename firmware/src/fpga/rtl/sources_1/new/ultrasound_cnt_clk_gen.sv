/*
 * File: ultrasound_cnt_clk_gen.sv
 * Project: new
 * Created Date: 12/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

module ultrasound_cnt_clk_gen (
    input var  clk_in1,
    input var  reset,
    output var clk_out,
    output var locked
);

  logic CLKOUT;
  logic CLKFB, CLKFB_BUF;
  BUFG clkf_buf (
      .O(CLKFB_BUF),
      .I(CLKFB)
  );
  BUFG clkout_buf (
      .O(clk_out),
      .I(CLKOUT)
  );

  MMCME2_ADV #(
      .BANDWIDTH           ("OPTIMIZED"),
      .CLKOUT4_CASCADE     ("FALSE"),
      .COMPENSATION        ("ZHOLD"),
      .STARTUP_WAIT        ("FALSE"),
      .DIVCLK_DIVIDE       (1),
      .CLKFBOUT_MULT_F     (39.000),
      .CLKFBOUT_PHASE      (0.000),
      .CLKFBOUT_USE_FINE_PS("FALSE"),
      .CLKOUT0_DIVIDE_F    (48.750),
      .CLKOUT0_PHASE       (0.000),
      .CLKOUT0_DUTY_CYCLE  (0.500),
      .CLKOUT0_USE_FINE_PS ("FALSE"),
      .CLKIN1_PERIOD       (39.063)
  ) MMCME2_ADV_inst (
      .CLKOUT0(CLKOUT),
      .CLKOUT0B(),
      .CLKOUT1(),
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

endmodule
