%{
%File: simulator.m
%Project: autd3-matlab
%Created Date: 11/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 10/10/2022
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

    cnt = Controller();
    cnt.add_device([0 0 0], [0 0 0]);

    sim = Simulator();
    link = sim.build();

    if ~cnt.open(link)
        disp(Controller.last_error());
        throw(MException('MATLAB:RuntimeError', 'Cannot open link'));
    end

    runner(cnt);
catch Error

    for e = Error
        disp(e);
    end

end

deinit_autd(use_link_soem, use_backend_cuda);
