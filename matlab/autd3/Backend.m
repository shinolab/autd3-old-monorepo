%{
%File: Backend.m
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

classdef Backend < handle

    properties
        ptr
    end

    methods

        function obj = Backend()
            obj.ptr = libpointer('voidPtr', 0);
        end

        function delete(obj)

            if obj.ptr.Value ~= 0
                calllib('autd3capi_gain_holo', 'AUTDDeleteBackend', obj.ptr);
                obj.ptr = libpointer('voidPtr', 0);
            end

        end

    end

end
