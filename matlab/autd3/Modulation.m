%{
%File: Modulation.m
%Project: autd3-matlab
%Created Date: 07/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 11/06/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef Modulation < Header

    properties
        sampling_frequency_division
    end

    methods

        function obj = Modulation()
            obj = obj@Header();
        end

        function set.sampling_frequency_division(obj, value)
            obj.sampling_frequency_division = value;
            calllib('autd3capi', 'AUTDModulationSetSamplingFrequencyDivision', obj.ptr, value);
        end

        function div = get.sampling_frequency_division(obj)
            div = calllib('autd3capi', 'AUTDModulationSamplingFrequencyDivision', obj.ptr);
        end

        function freq = sampling_frequency(obj)
            freq = calllib('autd3capi', 'AUTDModulationSamplingFrequency', obj.ptr);
        end

        function delete(obj)

            if obj.ptr.Value ~= 0
                calllib('autd3capi', 'AUTDDeleteModulation', obj.ptr);
                obj.ptr = libpointer('voidPtr', 0);
            end

        end

    end

end
