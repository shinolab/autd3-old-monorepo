%{
%File: SOEM.m
%Project: autd3-matlab
%Created Date: 07/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 26/10/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef SOEM < handle

    properties
        ptr
        ifname_
        sync0_cycle_
        send_cycle_
        freerun_
        high_precision_
    end

    methods

        function obj = SOEM()
            obj.ptr = libpointer('voidPtr', 0);
            obj.ifname_ = libpointer('int8Ptr', 0);
            obj.sync0_cycle_ = 2;
            obj.send_cycle_ = 2;
            obj.freerun_ = false;
            obj.high_precision_ = false;
        end

        function ifname(obj, name)
            obj.ifname_ = libpointer('int8Ptr', name);
        end

        function send_cycle(obj, cycle)
            obj.send_cycle = cycle;
        end

        function sync0_cycle(obj, cycle)
            obj.sync0_cycle_ = cycle;
        end

        function freerun(obj, flag)
            obj.freerun_ = flag;
        end

        function high_precision(obj, flag)
            obj.high_precision_ = flag;
        end

        function res = build(obj)
            pp = libpointer('voidPtrPtr', obj.ptr);
            on_lost = libpointer('voidPtr', 0);
            calllib('autd3capi_link_soem', 'AUTDLinkSOEM', pp, obj.ifname_, obj.sync0_cycle_, obj.send_cycle_, obj.freerun_, on_lost, obj.high_precision_);
            res = obj;
        end

    end

    methods (Static)

        function adapters = enumerate_adapters()
            p = libpointer('voidPtr', 0);
            pp = libpointer('voidPtrPtr', p);
            n = calllib('autd3capi_link_soem', 'AUTDGetAdapterPointer', pp);
            adapters = strings(n, 2);

            for i = 1:n
                desc_p = libpointer('int8Ptr', zeros(128, 1, 'int8'));
                name_p = libpointer('int8Ptr', zeros(128, 1, 'int8'));
                calllib('autd3capi_link_soem', 'AUTDGetAdapter', p, i - 1, desc_p, name_p);
                desc = erase(convertCharsToStrings(char(desc_p.value)), char(0));
                name = erase(convertCharsToStrings(char(name_p.value)), char(0));
                adapters(i, :) = [desc, name];
            end

            calllib('autd3capi_link_soem', 'AUTDFreeAdapterPointer', p);
        end

    end

end
