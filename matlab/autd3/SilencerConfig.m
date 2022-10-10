%{
%File: SilencerConfig.m
%Project: autd3-matlab
%Created Date: 07/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 11/06/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef SilencerConfig < Header

    methods

        function obj = SilencerConfig(varargin)
            obj = obj@Header();
            pp = libpointer('voidPtrPtr', obj.ptr);

            if nargin < 1
                step = 10;
            else
                step = varargin{1};
            end

            if nargin < 2
                freq_div = 4096;
            else
                freq_div = varargin{2};
            end

            calllib('autd3capi', 'AUTDCreateSilencer', pp, step, freq_div);
        end

        function delete(obj)

            if obj.ptr.Value ~= 0
                calllib('autd3capi', 'AUTDDeleteSilencer', obj.ptr);
                obj.ptr = libpointer('voidPtr', 0);
            end

        end

    end

    methods (Static)

        function config = none()
            config = SilencerConfig(0x1fff, 4096);
        end

    end

end
