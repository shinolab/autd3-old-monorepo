/*
 * File: pwm.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

module pwm #(
    parameter int WIDTH = 13,
    parameter int TRANS_NUM = 249
) (
    input var CLK,
    input var CLK_L,
    input var [63:0] SYS_TIME,
    input var DIN_VALID,
    input var [WIDTH-1:0] CYCLE[TRANS_NUM],
    input var [WIDTH-1:0] DUTY[TRANS_NUM],
    input var [WIDTH-1:0] PHASE[TRANS_NUM],
    output var PWM_OUT[TRANS_NUM],
    output var [WIDTH-1:0] TIME_CNT[TRANS_NUM],
    output var DOUT_VALID
);

  bit [WIDTH-1:0] R[TRANS_NUM];
  bit [WIDTH-1:0] F[TRANS_NUM];

  bit [WIDTH-1:0] cycle_m1[TRANS_NUM];
  bit [WIDTH-1:0] cycle_m2[TRANS_NUM];

  cycle_buffer #(
      .WIDTH(WIDTH),
      .DEPTH(TRANS_NUM)
  ) cycle_buffer (
      .CLK(CLK),
      .CYCLE(CYCLE),
      .CYCLE_M1(cycle_m1),
      .CYCLE_M2(cycle_m2)
  );

  time_cnt_generator #(
      .WIDTH(WIDTH),
      .DEPTH(TRANS_NUM)
  ) time_cnt_generator (
      .CLK(CLK),
      .SYS_TIME(SYS_TIME),
      .CYCLE(CYCLE),
      .CYCLE_M1(cycle_m1),
      .CYCLE_M2(cycle_m2),
      .TIME_CNT(TIME_CNT)
  );

  pwm_preconditioner #(
      .WIDTH(WIDTH),
      .DEPTH(TRANS_NUM)
  ) pwm_preconditioner (
      .CLK(CLK_L),
      .DIN_VALID(DIN_VALID),
      .CYCLE(CYCLE),
      .DUTY(DUTY),
      .PHASE(PHASE),
      .RISE(R),
      .FALL(F),
      .DOUT_VALID(DOUT_VALID)
  );

  for (genvar i = 0; i < TRANS_NUM; i++) begin : gen_pwm
    bit [WIDTH-1:0] R_buf, F_buf;
    pwm_buffer #(
        .WIDTH(WIDTH)
    ) pwm_buffer (
        .CLK(CLK),
        .CYCLE_M1(cycle_m1[i]),
        .TIME_CNT(TIME_CNT[i]),
        .RISE_IN(R[i]),
        .FALL_IN(F[i]),
        .RISE_OUT(R_buf),
        .FALL_OUT(F_buf)
    );
    pwm_generator #(
        .WIDTH(WIDTH)
    ) pwm_generator (
        .CLK(CLK),
        .TIME_CNT(TIME_CNT[i]),
        .RISE(R_buf),
        .FALL(F_buf),
        .PWM_OUT(PWM_OUT[i])
    );
  end

endmodule
