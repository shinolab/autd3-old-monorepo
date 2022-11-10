%{
%File: SpecialData.m
%Project: autd3
%Created Date: 10/11/2022
%Author: Shun Suzuki
%-----
%Last Modified: 10/11/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef SpecialData < handle

    properties
        ptr
    end

    methods

        function obj = SpecialData()
            obj.ptr = libpointer('voidPtr', 0);
        end

        function delete(obj)
            if obj.ptr.Value ~= 0
                calllib('autd3capi', 'AUTDDeleteSpecialData', obj.ptr);
                obj.ptr = libpointer('voidPtr', 0);
            end
        end

    end

end
