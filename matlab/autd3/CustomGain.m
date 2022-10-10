%{
%File: CustomGain.m
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

classdef CustomGain < Gain

    methods

        function obj = CustomGain(amps, phases)
            obj = obj@Gain();
            pp = libpointer('voidPtrPtr', obj.ptr);
            pamps = libpointer('doublePtr', amps);
            pphases = libpointer('doublePtr', phases);
            calllib('autd3capi', 'AUTDGainCustom', pp, pamps, pphases, length(amps));
        end

    end

end
