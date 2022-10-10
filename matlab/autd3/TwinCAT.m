%{
%File: TwinCAT.m
%Project: autd3
%Created Date: 10/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 08/08/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef TwinCAT < handle

    properties
        ptr
    end

    methods

        function obj = TwinCAT()
            obj.ptr = libpointer('voidPtr', 0);
        end
        
        function res = build(obj)
            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi_link_twincat', 'AUTDLinkTwinCAT', pp);
            res = obj;
        end

    end

end
