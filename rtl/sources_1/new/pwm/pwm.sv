/*
 * File: pwm.sv
 * Project: pwm
 * Created Date: 15/03/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */

module pwm#(
           parameter int WIDTH = 13,
           parameter int TRANS_NUM = 249
       )(
           input var CLK,
           input var CLK_L,
           input var [63:0] SYS_TIME,
           input var [WIDTH-1:0] CYCLE[0:TRANS_NUM-1],
           input var [WIDTH-1:0] DUTY[0:TRANS_NUM-1],
           input var [WIDTH-1:0] PHASE[0:TRANS_NUM-1],
           output var PWM_OUT[0:TRANS_NUM-1],
           output var DONE,
           output var [WIDTH-1:0] TIME_CNT[0:TRANS_NUM-1]
       );

bit [WIDTH-1:0] R[0:TRANS_NUM-1];
bit [WIDTH-1:0] F[0:TRANS_NUM-1];

time_cnt_generator#(
                      .WIDTH(WIDTH),
                      .DEPTH(TRANS_NUM)
                  ) time_cnt_generator(
                      .CLK(CLK),
                      .SYS_TIME(SYS_TIME),
                      .CYCLE(CYCLE),
                      .TIME_CNT(TIME_CNT)
                  );

pwm_preconditioner#(
                      .WIDTH(WIDTH),
                      .DEPTH(TRANS_NUM)
                  ) pwm_preconditioner(
                      .CLK(CLK_L),
                      .CYCLE(CYCLE),
                      .DUTY(DUTY),
                      .PHASE(PHASE),
                      .RISE(R),
                      .FALL(F),
                      .DONE(DONE)
                  );

for (genvar i = 0; i < TRANS_NUM; i++) begin
    bit [WIDTH-1:0] R_buf, F_buf;
    pwm_buffer#(
                  .WIDTH(WIDTH)
              ) pwm_buffer(
                  .CLK(CLK),
                  .CYCLE(CYCLE[i]),
                  .TIME_CNT(TIME_CNT[i]),
                  .RISE_IN(R[i]),
                  .FALL_IN(F[i]),
                  .RISE_OUT(R_buf),
                  .FALL_OUT(F_buf)
              );
    pwm_generator#(
                     .WIDTH(WIDTH)
                 ) pwm_generator(
                     .CLK(CLK),
                     .TIME_CNT(TIME_CNT[i]),
                     .RISE(R_buf),
                     .FALL(F_buf),
                     .PWM_OUT(PWM_OUT[i])
                 );
end

endmodule
