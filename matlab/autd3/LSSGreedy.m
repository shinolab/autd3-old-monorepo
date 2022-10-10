%{
%File: LSSGreedy.m
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

classdef LSSGreedy < Holo

    methods

        function obj = LSSGreedy(varargin)
            obj = obj@Holo();
            backend = varargin{1};

            if nargin < 2
                phase_div = 16;
            else
                phase_div = varargin{2};
            end

            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi_gain_holo', 'AUTDGainHoloLSSGreedy', pp, backend.ptr, phase_div);
        end

    end

end
