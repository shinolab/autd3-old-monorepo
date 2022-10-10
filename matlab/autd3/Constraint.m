%{
%File: Constraint.m
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

classdef Constraint < int32

    enumeration
        DontCare (0)
        Normalize (1)
        Uniform(2)
        Clamp(3)
    end

end
