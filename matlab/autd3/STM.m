%{
%File: STM.m
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

classdef STM < Body

    properties
        freqeuncy
        sampling_frequency_division
    end

    methods

        function obj = STM()
            obj = obj@Body();
        end

        function set.freqeuncy(obj, value)
            obj.freqeuncy = value;
            calllib('autd3capi', 'AUTDSTMSetFrequency', obj.ptr, value);
        end

        function freq = get.freqeuncy(obj)
            freq = calllib('autd3capi', 'AUTDSTMFrequency', obj.ptr);
        end

        function set.sampling_frequency_division(obj, value)
            obj.sampling_frequency_division = value;
            calllib('autd3capi', 'AUTDSTMSetSamplingFrequencyDivision', obj.ptr, value);
        end

        function div = get.sampling_frequency_division(obj)
            div = calllib('autd3capi', 'AUTDSTMSamplingFrequencyDivision', obj.ptr);
        end

        function freq = sampling_frequency(obj)
            freq = calllib('autd3capi', 'AUTDSTMSamplingFrequency', obj.ptr);
        end

        function delete(obj)

            if obj.ptr.Value ~= 0
                calllib('autd3capi', 'AUTDDeleteSTM', obj.ptr);
                obj.ptr = libpointer('voidPtr', 0);
            end

        end

    end

end
