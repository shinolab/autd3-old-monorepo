%{
%File: GainSTM.m
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

classdef GainSTM < STM

    properties
        mode = Mode.PhaseDutyFull
    end

    methods

        function obj = GainSTM(cnt)
            obj = obj@STM();
            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi', 'AUTDGainSTM', pp, cnt.ptr);
        end

        function set.mode(obj, value)
            obj.mode = value;
            calllib('autd3capi', 'AUTDSetGainSTMMode', obj.ptr, uint16(value));
        end

        function mode = get.mode(obj)
            mode = Mode(calllib('autd3capi', 'AUTDGetGainSTMMode', obj.ptr));
        end

        function add(obj, gain)
            calllib('autd3capi', 'AUTDGainSTMAdd', obj.ptr, gain.ptr);
        end

    end

end
