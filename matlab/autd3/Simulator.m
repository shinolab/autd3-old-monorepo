%{
%File: Simulator.m
%Project: autd3
%Created Date: 10/10/2022
%Author: Shun Suzuki
%-----
%Last Modified: 28/04/2023
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef Simulator < handle

    properties
        ptr
        builder_
    end

    methods

        function obj = Simulator()
            obj.ptr = libpointer('voidPtr', 0);
            obj.builder_ = libpointer('voidPtr', 0);
            pp = libpointer('voidPtrPtr', obj.builder_);
            calllib('autd3capi_link_simulator', 'AUTDLinkSimulator', pp);
        end

        function res = timeout(obj, t)
            calllib('autd3capi_link_simulator', 'AUTDLinkSimulatorTimeout', obj.builder_, t);
            res = obj;
        end
        
        function res = build(obj)
            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi_link_simulator', 'AUTDLinkSimulatorBuild', pp, obj.builder_);
            res = obj;
        end

    end

end 
