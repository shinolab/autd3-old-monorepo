%{
%File: PlaneWave.m
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

classdef PlaneWave < Gain

    methods

        function obj = PlaneWave(varargin)
            obj = obj@Gain();
            d = varargin{1};

            if nargin < 2
                amp = 1.0;
            else
                amp = varargin{2};
            end

            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi', 'AUTDGainPlaneWave', pp, d(1), d(2), d(3), amp);
        end

    end

end
