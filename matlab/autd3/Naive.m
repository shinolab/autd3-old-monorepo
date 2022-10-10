%{
%File: EVD.m
%Project: autd3
%Created Date: 10/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 10/06/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef Naive < Holo

    methods

        function obj = Naive(backend)
            obj = obj@Holo();
            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi_gain_holo', 'AUTDGainHoloNaive', pp, backend);
        end

    end

end
