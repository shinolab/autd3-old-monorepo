%{
%File: simulator.m
%Project: autd3-matlab
%Created Date: 11/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 02/02/2023
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

addpath('autd3')

Error = [];

use_link_soem = false;
use_backend_cuda = false;

try
    init_autd(use_link_soem, use_backend_cuda);

    builder = GeometryBuilder();
    builder.add_device([0 0 0], [0 0 0]);
    geometry = builder.build();

    sim = Simulator();
    link = sim.build();

    cnt = Controller(geometry, link);

    runner(cnt);
catch Error

    for e = Error
        disp(e);
    end

end

deinit_autd(use_link_soem, use_backend_cuda);
