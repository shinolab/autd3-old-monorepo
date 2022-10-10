%{
%File: Gain.m
%Project: autd3-matlab
%Created Date: 07/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 07/06/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef Gain < Body

    methods

        function obj = Gain()
            obj = obj@Body();
        end

        function delete(obj)

            if obj.ptr.Value ~= 0
                calllib('autd3capi', 'AUTDDeleteGain', obj.ptr);
                obj.ptr = libpointer('voidPtr', 0);
            end

        end

    end

end
