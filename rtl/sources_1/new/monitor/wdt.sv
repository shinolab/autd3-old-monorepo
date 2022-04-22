/*
 * File: wdt.sv
 * Project: monitor
 * Created Date: 19/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 * 
 */

module wdt#(
           parameter int CYCLE = 20480000
       ) (
           input var CLK,
           input var RST,
           output var ASSERT
       );

bit [$clog2(CYCLE)-1:0] cnt = 0;
bit a = 0;
bit [1:0] rst;

assign ASSERT = a;

always_ff @(posedge CLK) begin
    rst <= {rst[0], RST};
end

always_ff @(posedge CLK) begin
    if (^rst) begin
        cnt <= 0;
        a <= 0;
    end
    else begin
        if (cnt == CYCLE - 1) begin
            cnt <= cnt;
            a <= 1;
        end
        else begin
            cnt <= cnt + 1;
        end
    end
end

endmodule
