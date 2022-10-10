%{
%File: Static.m
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

classdef Static < Modulation

    methods

        function obj = Static(varargin)
            obj = obj@Modulation();

            if nargin < 1
                amp = 1.0;
            else
                amp = varargin{1};
            end

            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi', 'AUTDModulationStatic', pp, amp);
        end

    end

end
