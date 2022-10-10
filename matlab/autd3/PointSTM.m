%{
%File: PointSTM.m
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

classdef PointSTM < STM

    methods

        function obj = PointSTM()
            obj = obj@STM();
            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi', 'AUTDPointSTM', pp);
        end

        function add(varargin)
            obj = varargin{1};
            f = varargin{2};

            if nargin < 3
                shift = 0;
            else
                shift = varargin{3};
            end

            calllib('autd3capi', 'AUTDPointSTMAdd', obj.ptr, f(1), f(2), f(3), shift);
        end

    end

end
