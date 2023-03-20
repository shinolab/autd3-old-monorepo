%{
%File: TimerStrategy.m
%Project: autd3
%Created Date: 20/03/2023
%Author: Shun Suzuki
%-----
%Last Modified: 20/03/2023
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2023 Shun Suzuki. All rights reserved.
%
%}


classdef TimerStrategy < uint8

    enumeration
        Sleep(0)
        BusyWait(1)
        NativeTimer(2)
    end

end
