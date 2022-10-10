%{
%File: Square.m
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

classdef Square < Modulation

    methods

        function obj = Square(varargin)
            obj = obj@Modulation();
            f = varargin{1};

            if nargin < 2
                low = 0.0;
            else
                low = varargin{2};
            end

            if nargin < 3
                high = 1.0;
            else
                high = varargin{3};
            end

            if nargin < 4
                duty = 0.5;
            else
                duty = varargin{4};
            end

            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi', 'AUTDModulationSquare', pp, cast(f, 'int32'), low, high, duty);
        end

    end

end
