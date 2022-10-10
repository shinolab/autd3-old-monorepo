%{
%File: Amplitudes.m
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

classdef Amplitudes < Header

    methods

        function obj = Amplitudes(amp)
            obj = obj@Header();
            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi', 'AUTDCreateAmplitudes', pp, amp);
        end

        function delete(obj)

            if obj.ptr.Value ~= 0
                calllib('autd3capi', 'AUTDDeleteAmplitudes', obj.ptr);
                obj.ptr = libpointer('voidPtr', 0);
            end

        end

    end

end
