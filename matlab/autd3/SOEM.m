%{
%File: SOEM.m
%Project: autd3-matlab
%Created Date: 07/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 20/03/2023
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef SOEM < handle

    properties
        ptr
        ifname_
        buf_size_
        sync0_cycle_
        send_cycle_
        sync_mode_
        timer_strategy_
        check_interval_
        debug_level_
    end

    methods

        function obj = SOEM()
            obj.ptr = libpointer('voidPtr', 0);
            obj.ifname_ = [];
            obj.buf_size_ = 0;
            obj.sync0_cycle_ = 2;
            obj.send_cycle_ = 2;
            obj.sync_mode_ = SyncMode.FreeRun;
            obj.timer_strategy_ = TimerStrategy.Sleep;
            obj.check_interval_ = 500;
            obj.debug_level_ = 2;
        end

        function ifname(obj, name)
            obj.ifname_ = name;
        end

        function buf_size(obj, size)
            obj.buf_size_ = size;
        end

        function send_cycle(obj, cycle)
            obj.send_cycle = cycle;
        end

        function sync0_cycle(obj, cycle)
            obj.sync0_cycle_ = cycle;
        end

        function sync_mode(obj, mode)
            obj.sync_mode_ = mode;
        end

        function timer_strategy(obj, strategy)
            obj.timer_strategy_ = strategy;
        end

        function check_interval(obj, interval)
            obj.check_interval_ = interval;
        end
        
        function debug_level(obj, level)
            obj.debug_level_ = level;
        end

        function res = build(obj)
            pp = libpointer('voidPtrPtr', obj.ptr);
            on_lost = libpointer('voidPtr', 0);
            log_out = libpointer('voidPtr', 0);
            log_flush = libpointer('voidPtr', 0);
            is_freerun = obj.sync_mode_ == SyncMode.FreeRun;
            strategy = uint8(obj.timer_strategy_);
            calllib('autd3capi_link_soem', 'AUTDLinkSOEM', pp, obj.ifname_,  obj.buf_size_, obj.sync0_cycle_, obj.send_cycle_, is_freerun, on_lost, strategy, obj.check_interval_, obj.debug_level_, log_out, log_flush);
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
