%{
%File: GainSTM.m
%Project: autd3
%Created Date: 10/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 08/03/2023
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

        function obj = GainSTM(varargin)
            obj = obj@STM();

            if nargin < 1
                mode = 1;
            else
                mode = varargin{1};
            end

            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi', 'AUTDGainSTM', pp, uint16(mode));
        end

        function add(obj, gain)
            calllib('autd3capi', 'AUTDGainSTMAdd', obj.ptr, gain.ptr);
        end

    end

end
