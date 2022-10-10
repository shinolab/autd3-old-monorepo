%{
%File: LM.m
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

classdef LM < Holo

    methods

        function obj = LM(varargin)
            obj = obj@Holo();
            backend = varargin{1};

            if nargin < 2
                eps_1 = 1e-8;
            else
                eps_1 = varargin{2};
            end

            if nargin < 3
                eps_2 = 1e-8;
            else
                eps_2 = varargin{3};
            end

            if nargin < 4
                tau = 1e-3;
            else
                tau = varargin{4};
            end

            if nargin < 5
                k_max = 5;
            else
                k_max = varargin{5};
            end

            if nargin < 6
                initial = libpointer('voidPtr', []);
                initial_size = 0;
            else
                initial = libpointer('voidPtr', varargin{6});
                initial_size = length(varargin{6});
            end

            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi_gain_holo', 'AUTDGainHoloLM', pp, backend.ptr, eps_1, eps_2, tau, k_max, initial, initial_size);
        end

    end

end
