%{
%File: Simulator.m
%Project: autd3
%Created Date: 10/10/2022
%Author: Shun Suzuki
%-----
%Last Modified: 18/04/2023
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef Simulator < handle

    properties
        ptr
        timeout_
    end

    methods

        function obj = Simulator()
            obj.ptr = libpointer('voidPtr', 0);
            obj.timeout_ = 20 * 1000 * 1000;
        end

        function res = timeout(obj, t)
            obj.timeout_ = t;
            res = obj;
        end

        function res = build(obj)
            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi_link_simulator', 'AUTDLinkSimulator', pp, obj.timeout_);
            res = obj;
        end

    end

end
