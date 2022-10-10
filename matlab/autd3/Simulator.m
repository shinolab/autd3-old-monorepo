%{
%File: Simulator.m
%Project: autd3
%Created Date: 10/10/2022
%Author: Shun Suzuki
%-----
%Last Modified: 10/10/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef Simulator < handle

    properties
        ptr
        port
    end

    methods

        function obj = Simulator(port)
            arguments
                port = 50632
            end

            obj.ptr = libpointer('voidPtr', 0);
            obj.port = port;
        end

        function res = build(obj)
            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi_link_simulator', 'AUTDLinkSimulator', pp, obj.port, libpointer('int8Ptr', 0));
            res = obj;
        end

    end

end
