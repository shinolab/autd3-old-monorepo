%{
%File: CustomModulation.m
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

classdef CustomModulation < Modulation

    methods

        function obj = CustomModulation(buf, freq_div)
            obj = obj@Modulation();

            pp = libpointer('voidPtrPtr', obj.ptr);
            pbuf = libpointer('uint8Ptr', buf);
            calllib('autd3capi', 'AUTDModulationCustom', pp, pbuf, length(buf), freq_div);
        end

    end

end
