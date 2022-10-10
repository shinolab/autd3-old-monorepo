%{
%File: Mode.m
%Project: autd3
%Created Date: 10/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 11/06/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef Mode < uint16

    enumeration
        PhaseDutyFull(0x0001)
        PhaseFull(0x0002)
        PhaseHalf(0x0004)
    end

end
