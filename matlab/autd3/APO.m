%{
%File: APO.m
%Project: autd3
%Created Date: 08/08/2022
%Author: Shun Suzuki
%-----
%Last Modified: 08/08/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}


classdef APO < Holo

    methods

        function obj = APO(varargin)
            obj = obj@Holo();
            backend = varargin{1};

            if nargin < 2
                eps = 1e-8;
            else
                eps = varargin{2};
            end

            if nargin < 3
                lambda = 1;
            else
                lambda = varargin{3};
            end

            if nargin < 4
                k_max = 200;
            else
                k_max = varargin{4};
            end

            if nargin < 5
                line_search_max = 100;
            else
                line_search_max = varargin{5};
            end

            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi_gain_holo', 'AUTDGainHoloAPO', pp, backend.ptr, eps, lambda, k_max, line_search_max);
        end

    end

end
