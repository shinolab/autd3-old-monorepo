/*
 * File: cvt_uid.vh
 * Project: new
 * Created Date: 15/12/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Shun Suzuki. All rights reserved.
 * 
 */

function automatic [7:0] cvt_uid (input [7:0] idx);
    if (idx < 8'd19) begin
        cvt_uid = idx;
    end
    else if (idx < 8'd32) begin
        cvt_uid = idx + 2;
    end
    else begin
        cvt_uid = idx + 3;
    end
endfunction
