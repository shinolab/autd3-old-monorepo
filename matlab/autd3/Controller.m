%{
%File: Controller.m
%Project: autd3-matlab
%Created Date: 07/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 28/11/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef Controller < handle

    properties
        ptr
        reads_fpga_info = false
        force_fan = false
        attenuation = 0.0
        ack_check_timeout = 0
        send_interval = 500000
        sound_speed = 340e3
    end

    methods

        function obj = Controller(varargin)
            if nargin < 1
                driver_version = 0;
            else
                driver_version = varargin{1};
            end

            obj.ptr = libpointer('voidPtr', 0);
            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi', 'AUTDCreateController', pp, driver_version);
        end

        function to_legacy(obj)
            calllib('autd3capi', 'AUTDSetMode', obj.ptr, 0);
        end

        function to_normal(obj)
            calllib('autd3capi', 'AUTDSetMode', obj.ptr, 1);
        end

        function to_normal_phase(obj)
            calllib('autd3capi', 'AUTDSetMode', obj.ptr, 2);
        end

        function add_device(obj, pos, rot)
            calllib('autd3capi', 'AUTDAddDevice', obj.ptr, pos(1), pos(2), pos(3), rot(1), rot(2), rot(3));
        end

        function add_device_quaternion(obj, pos, rot)
            calllib('autd3capi', 'AUTDAddDeviceQuaternion', obj.ptr, pos(1), pos(2), pos(3), rot(1), rot(2), rot(3), rot(4));
        end

        function res = open(obj, link)
            res = calllib('autd3capi', 'AUTDOpenController', obj.ptr, link.ptr);
        end

        function res = close(obj)
            res = calllib('autd3capi', 'AUTDClose', obj.ptr);
        end

        function res = is_open(obj)
            res = calllib('autd3capi', 'AUTDIsOpen', obj.ptr);
        end

        function set.force_fan(obj, value)
            obj.force_fan = value;
            calllib('autd3capi', 'AUTDSetForceFan', obj.ptr, value);
        end

        function value = get.force_fan(obj)
            value = calllib('autd3capi', 'AUTDGetForceFan', obj.ptr);
        end

        function set.reads_fpga_info(obj, value)
            obj.reads_fpga_info = value;
            calllib('autd3capi', 'AUTDSetReadsFPGAInfo', obj.ptr, value);
        end

        function value = get.reads_fpga_info(obj)
            value = calllib('autd3capi', 'AUTDGetReadsFPGAInfo', obj.ptr);
        end

        function set.ack_check_timeout(obj, value)
            obj.ack_check_timeout = value;
            calllib('autd3capi', 'AUTDSetAckCheckTimeout', obj.ptr, value);
        end

        function value = get.ack_check_timeout(obj)
            value = calllib('autd3capi', 'AUTDGetAckCheckTimeout', obj.ptr);
        end

        function set.send_interval(obj, value)
            obj.send_interval = value;
            calllib('autd3capi', 'AUTDeSetSendInterval', obj.ptr, value);
        end

        function value = get.send_interval(obj)
            value = calllib('autd3capi', 'AUTDGetSendInterval', obj.ptr);
        end

        function set.sound_speed(obj, value)
            obj.sound_speed = value;
            calllib('autd3capi', 'AUTDSetSoundSpeed', obj.ptr, value);
        end

        function value = get.sound_speed(obj)
            value = calllib('autd3capi', 'AUTDGetSoundSpeed', obj.ptr);
        end

        function set.attenuation(obj, value)
            obj.attenuation = value;
            calllib('autd3capi', 'AUTDSetAttenuation', obj.ptr, value);
        end

        function value = get.attenuation(obj)
            value = calllib('autd3capi', 'AUTDGetAttenuation', obj.ptr);
        end

        function freq = get_trans_frequency(obj, trans_idx)
            freq = calllib('autd3capi', 'AUTDGetTransFrequency', obj.ptr, trans_idx);
        end

        function set_trans_frequency(obj, trans_idx, freq)
            calllib('autd3capi', 'AUTDSetTransFrequency', obj.ptr, trans_idx, freq);
        end

        function cycle = get_trans_cycle(obj, trans_idx)
            cycle = calllib('autd3capi', 'AUTDGetTransCycle', obj.ptr, trans_idx);
        end

        function set_trans_cycle(obj, trans_idx, cycle)
            calllib('autd3capi', 'AUTDSetTransCycle', obj.ptr, trans_idx, cycle);
        end

        function wavelength = wavelength(obj, trans_idx)
            wavelength = calllib('autd3capi', 'AUTDGetWavelength', obj.ptr, trans_idx);
        end

        function set_mod_delay(obj, trans_idx, delay)
            calllib('autd3capi', 'AUTDSetModDelay', obj.ptr, trans_idx, delay);
        end

        function pos = trans_position(obj, trans_idx)
            px = libpointer('doublePtr', 0);
            py = libpointer('doublePtr', 0);
            pz = libpointer('doublePtr', 0);
            calllib('autd3capi', 'AUTDTransPosition', obj.ptr, trans_idx, px, py, pz);
            pos = [px.Value; py.Value; pz.Value];
        end

        function dir = trans_x_direction(obj, trans_idx)
            px = libpointer('doublePtr', 0);
            py = libpointer('doublePtr', 0);
            pz = libpointer('doublePtr', 0);
            calllib('autd3capi', 'AUTDTransXDirection', obj.ptr, trans_idx, px, py, pz);
            dir = [px.Value; py.Value; pz.Value];
        end

        function dir = trans_y_direction(obj, trans_idx)
            px = libpointer('doublePtr', 0);
            py = libpointer('doublePtr', 0);
            pz = libpointer('doublePtr', 0);
            calllib('autd3capi', 'AUTDTransYDirection', obj.ptr, trans_idx, px, py, pz);
            dir = [px.Value; py.Value; pz.Value];
        end

        function dir = trans_z_direction(obj, trans_idx)
            px = libpointer('doublePtr', 0);
            py = libpointer('doublePtr', 0);
            pz = libpointer('doublePtr', 0);
            calllib('autd3capi', 'AUTDTransZDirection', obj.ptr, trans_idx, px, py, pz);
            dir = [px.Value; py.Value; pz.Value];
        end

        function res = num_transducers(obj)
            res = calllib('autd3capi', 'AUTDNumTransducers', obj.ptr);
        end

        function res = send(varargin)
            obj = varargin{1};

            if nargin >= 4
                res = false;
                return;
            end

            if nargin == 3

                if isa(varargin{2}, 'Header') && isa(varargin{3}, 'Body')
                    res = calllib('autd3capi', 'AUTDSend', obj.ptr, varargin{2}.ptr, varargin{3}.ptr);
                    return;
                end

                if isa(varargin{3}, 'Header') && isa(varargin{2}, 'Body')
                    res = calllib('autd3capi', 'AUTDSend', obj.ptr, varargin{3}.ptr, varargin{2}.ptr);
                    return;
                end

            end

            if nargin == 2

                if isa(varargin{2}, 'SpecialData')
                    res = calllib('autd3capi', 'AUTDSendSpecial', obj.ptr, varargin{2}.ptr);
                    return;
                end

                if isa(varargin{2}, 'Header')
                    np = libpointer('voidPtr', []);
                    res = calllib('autd3capi', 'AUTDSend', obj.ptr, varargin{2}.ptr, np);
                    return;
                end

                if isa(varargin{2}, 'Body')
                    np = libpointer('voidPtr', []);
                    res = calllib('autd3capi', 'AUTDSend', obj.ptr, np, varargin{2}.ptr);
                    return;
                end

            end

            res = false;
        end

        function list = firmware_info_list(obj)
            p = libpointer('voidPtr', 0);
            pp = libpointer('voidPtrPtr', p);
            n = calllib('autd3capi', 'AUTDGetFirmwareInfoListPointer', obj.ptr, pp);
            list = strings(n, 1);

            for i = 1:n
                info_p = libpointer('int8Ptr', zeros(128, 1, 'int8'));
                calllib('autd3capi', 'AUTDGetFirmwareInfo', p, i - 1, info_p);
                info = erase(convertCharsToStrings(char(info_p.value)), char(0));
                list(i) = info;
            end

            calllib('autd3capi', 'AUTDFreeFirmwareInfoListPointer', p);
        end

        function list = fpga_info(obj)
            n = obj.num_devices();
            info_p = libpointer('uint8Ptr', zeros(n, 1, 'uint8'));
            calllib('autd3capi', 'AUTDGetFPGAInfo', obj.ptr, info_p);
        end

        function delete(obj)

            if obj.ptr.Value ~= 0
                calllib('autd3capi', 'AUTDFreeController', obj.ptr);
                obj.ptr = libpointer('voidPtr', 0);
            end

        end

    end

end
