%{
%File: Focus.m
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

classdef Focus < Gain

    methods

        function obj = Focus(varargin)
            obj = obj@Gain();
            f = varargin{1};

            if nargin < 2
                amp = 1.0;
            else
                amp = varargin{2};
            end

            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi', 'AUTDGainFocus', pp, f(1), f(2), f(3), amp);
        end

    end

end
