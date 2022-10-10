%{
%File: GS.m
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

classdef GS < Holo

    methods

        function obj = GS(varargin)
            obj = obj@Holo();
            backend = varargin{1};

            if nargin < 2
                repeat = 100;
            else
                repeat = varargin{2};
            end

            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi_gain_holo', 'AUTDGainHoloGS', pp, backend.ptr, repeat);
        end

    end

end
