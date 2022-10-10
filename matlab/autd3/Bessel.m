%{
%File: Bessel.m
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

classdef Bessel < Gain

    methods

        function obj = Bessel(varargin)
            obj = obj@Gain();
            f = varargin{1};
            d = varargin{2};
            theta = varargin{3};

            if nargin < 4
                amp = 1.0;
            else
                amp = varargin{4};
            end

            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi', 'AUTDGainBesselBeam', pp, f(1), f(2), f(3), d(1), d(2), d(3), theta, amp);
        end

    end

end
