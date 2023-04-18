%{
%File: Geometry.m
%Project: autd3
%Created Date: 02/02/2023
%Author: Shun Suzuki
%-----
%Last Modified: 18/04/2023
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2023 Shun Suzuki. All rights reserved.
%
%}

classdef Geometry < handle

    properties
        attenuation
        sound_speed
        ptr
    end

    methods

        function obj = Geometry(ptr)
            obj.ptr = ptr;
        end

        function value = get.sound_speed(obj)
            value = calllib('autd3capi', 'AUTDGetSoundSpeed', obj.ptr);
        end

        function set.sound_speed(obj, value)
            calllib('autd3capi', 'AUTDSetSoundSpeed', obj.ptr, value);
        end

        function value = get.attenuation(obj)
            value = calllib('autd3capi', 'AUTDGetAttenuation', obj.ptr);
        end

        function set.attenuation(obj, value)
            calllib('autd3capi', 'AUTDSetAttenuation', obj.ptr, value);
        end

        function value = num_transducers(obj)
            value = calllib('autd3capi', 'AUTDNumTransducers', obj.ptr);
        end

        function value = num_devices(obj)
            value = calllib('autd3capi', 'AUTDNumDevices', obj.ptr);
        end

        function value = center(obj)
            px = libpointer('doublePtr', 0);
            py = libpointer('doublePtr', 0);
            pz = libpointer('doublePtr', 0);
            calllib('autd3capi', 'AUTDGeometryCenter', obj.ptr, px, py, pz);
            value = [px.Value; py.Value; pz.Value];
        end

        function value = center_of(obj, dev_idx)
            px = libpointer('doublePtr', 0);
            py = libpointer('doublePtr', 0);
            pz = libpointer('doublePtr', 0);
            calllib('autd3capi', 'AUTDGeometryCenterOf', obj.ptr, dev_idx, px, py, pz);
            value = [px.Value; py.Value; pz.Value];
        end

        function value = transducer(obj, idx)
            value = Transducer(idx, obj.ptr);
        end

    end
end
