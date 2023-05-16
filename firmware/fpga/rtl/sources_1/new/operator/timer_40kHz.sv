/*
 * File: timer_40kHz.sv
 * Project: operator
 * Created Date: 16/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

module timer_40kHz (
    input var CLK_L,
    input var [63:0] SYS_TIME,
    output var TRIG_40KHZ
);

  bit [1:0] zero_cross = 2'b11;
  bit trig_40khz;

  assign TRIG_40KHZ = trig_40khz;

  always_ff @(posedge CLK_L) begin
    zero_cross <= {zero_cross[0], SYS_TIME[11:0] < 12'h800};
    trig_40khz <= zero_cross == 2'b01;
  end

endmodule
