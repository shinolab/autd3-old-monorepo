/*
 * File: pulse_width_encoder.sv
 * Project: modulation
 * Created Date: 17/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

module pulse_width_encoder #(
    parameter int DEPTH = 249
) (
    input var CLK,
    input var DIN_VALID,
    input var [15:0] INTENSITY_IN,
    input var [7:0] PHASE_IN,
    output var [8:0] PULSE_WIDTH_OUT,
    output var [7:0] PHASE_OUT,
    output var DOUT_VALID
);

  localparam int Latency = 1;

  logic [15:0] addr;
  logic [7:0] dout;
  logic full_width[3];

  BRAM_ASIN asin_bram (
      .clka (CLK),
      .ena  (1'b1),
      .addra(addr),
      .douta(dout)
  );

  logic [7:0] phase_buf[DEPTH];
  logic [$clog2(DEPTH):0] phase_set_cnt = 0;
  logic [7:0] phase_out;

  logic [8:0] pulse_width_out;
  logic dout_valid;

  logic [$clog2(DEPTH+(Latency+1))-1:0] cnt, set_cnt;

  typedef enum logic {
    WAITING,
    RUN
  } state_t;

  state_t state = WAITING;

  assign PULSE_WIDTH_OUT = pulse_width_out;
  assign PHASE_OUT = phase_out;
  assign DOUT_VALID = dout_valid;

  always_ff @(posedge CLK) begin
    case (state)
      WAITING: begin
        dout_valid <= 1'b0;
        if (DIN_VALID) begin
          cnt <= 0;
          set_cnt <= 0;

          phase_buf[0] <= PHASE_IN;
          phase_set_cnt <= 1;

          addr <= INTENSITY_IN;

          state <= RUN;
        end
      end
      RUN: begin
        if (phase_set_cnt < DEPTH) begin
          phase_buf[phase_set_cnt] <= PHASE_IN;
          phase_set_cnt <= phase_set_cnt + 1;
        end

        addr <= INTENSITY_IN;

        cnt  <= cnt + 1;

        if (cnt > Latency) begin
          dout_valid <= 1'b1;
          pulse_width_out <= {full_width[2], dout[7:0]};
          phase_out <= phase_buf[set_cnt];
          set_cnt <= set_cnt + 1;
          if (set_cnt == DEPTH - 1) begin
            state <= WAITING;
          end
        end
      end
      default: begin
      end
    endcase
  end

  always_ff @(posedge CLK) begin
    full_width[0] <= INTENSITY_IN == 16'd65025;  // 255*255
    full_width[1] <= full_width[0];
    full_width[2] <= full_width[1];
  end

endmodule
