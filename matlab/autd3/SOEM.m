%{
%File: SOEM.m
%Project: autd3-matlab
%Created Date: 07/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 18/04/2023
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef SOEM < handle

    properties
        ptr
        soem_
    end

    methods

        function obj = SOEM()
            obj.ptr = libpointer('voidPtr', 0);
            obj.soem_ = libpointer('voidPtr', 0);
            pp = libpointer('voidPtrPtr', obj.soem_);
            calllib('autd3capi_link_soem', 'AUTDLinkSOEM', pp);
        end

        function res = ifname(obj, name)
            calllib('autd3capi_link_soem', 'AUTDLinkSOEMIfname', obj.soem_, name);
            res = obj;
        end

        function res = buf_size(obj, size)
            calllib('autd3capi_link_soem', 'AUTDLinkSOEMBufSize', obj.soem_, size);
            res = obj;
        end

        function res = send_cycle(obj, cycle)
            calllib('autd3capi_link_soem', 'AUTDLinkSOEMSendCycle', obj.soem_, cycle);
            res = obj;
        end

        function res = sync0_cycle(obj, cycle)
            calllib('autd3capi_link_soem', 'AUTDLinkSOEMSync0Cycle', obj.soem_, cycle);
            res = obj;
        end

        function res = sync_mode(obj, mode)
            is_freerun = obj.sync_mode_ == SyncMode.FreeRun;
            calllib('autd3capi_link_soem', 'AUTDLinkSOEMFreerun', obj.soem_, is_freerun);
            res = obj;
        end

        function res = timer_strategy(obj, strategy)
            calllib('autd3capi_link_soem', 'AUTDLinkSOEMTimerStrategy', obj.soem_, uint8(strategy));
            res = obj;
        end

        function res = state_check_interval(obj, interval)
            calllib('autd3capi_link_soem', 'AUTDLinkSOEMStateCheckInterval', obj.soem_, interval);
            res = obj;
        end

        function res = timeout(obj, t)
            calllib('autd3capi_link_soem', 'AUTDLinkSOEMTimeout', obj.soem_, t);
            res = obj;
        end
        
        function res = build(obj)
            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi_link_soem', 'AUTDLinkSOEMBuild', pp, obj.soem_);
            calllib('autd3capi_link_soem', 'AUTDLinkSOEMDelete', obj.soem_);
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
