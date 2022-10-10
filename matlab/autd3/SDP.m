%{
%File: SDP.m
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

classdef SDP < Holo

    methods

        function obj = SDP(varargin)
            obj = obj@Holo();
            backend = varargin{1};

            if nargin < 2
                alpha = 1e-3;
            else
                alpha = varargin{2};
            end

            if nargin < 3
                lambda = 0.9;
            else
                lambda = varargin{3};
            end

            if nargin < 4
                repeat = 100;
            else
                repeat = varargin{4};
            end

            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi_gain_holo', 'AUTDGainHoloSDP', pp, backend.ptr, alpha, lambda, repeat);
        end

    end

end
