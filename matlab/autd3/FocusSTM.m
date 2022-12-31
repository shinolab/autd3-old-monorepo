%{
%File: FocusSTM.m
%Project: autd3
%Created Date: 10/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 29/11/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef FocusSTM < STM

    methods

        function obj = FocusSTM()
            obj = obj@STM();
            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi', 'AUTDFocusSTM', pp);
        end

        function add(varargin)
            obj = varargin{1};
            f = varargin{2};

            if nargin < 3
                shift = 0;
            else
                shift = varargin{3};
            end

            calllib('autd3capi', 'AUTDFocusSTMAdd', obj.ptr, f(1), f(2), f(3), shift);
        end

    end

end
