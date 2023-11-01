/*
 * File: pwm.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

module pwm #(
    parameter int WIDTH = 9,
    parameter int DEPTH = 249
) (
    input var CLK,
    input var [WIDTH-1:0] TIME_CNT,
    input var UPDATE,
    input var DIN_VALID,
    input var [WIDTH-1:0] DUTY,
    input var [WIDTH-1:0] PHASE,
    output var PWM_OUT[DEPTH],
    output var DOUT_VALID
);

  bit [WIDTH-1:0] R[DEPTH];
  bit [WIDTH-1:0] F[DEPTH];

  pwm_preconditioner #(
      .WIDTH(WIDTH),
      .DEPTH(DEPTH)
  ) pwm_preconditioner (
      .CLK(CLK),
      .DIN_VALID(DIN_VALID),
      .DUTY(DUTY),
      .PHASE(PHASE),
      .RISE(R),
      .FALL(F),
      .DOUT_VALID(DOUT_VALID)
  );

  for (genvar i = 0; i < DEPTH; i++) begin : gen_pwm
    bit [WIDTH-1:0] R_buf, F_buf;
    pwm_buffer #(
        .WIDTH(WIDTH)
    ) pwm_buffer (
        .CLK(CLK),
        .UPDATE(UPDATE),
        .RISE_IN(R[i]),
        .FALL_IN(F[i]),
        .RISE_OUT(R_buf),
        .FALL_OUT(F_buf)
    );
    pwm_generator #(
        .WIDTH(WIDTH)
    ) pwm_generator (
        .CLK(CLK),
        .TIME_CNT(TIME_CNT),
        .RISE(R_buf),
        .FALL(F_buf),
        .PWM_OUT(PWM_OUT[i])
    );
  end

endmodule
