%{
%File: Transducer.m
%Project: autd3
%Created Date: 02/02/2023
%Author: Shun Suzuki
%-----
%Last Modified: 02/02/2023
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2023 Shun Suzuki. All rights reserved.
%
%}

classdef Transducer < handle

    properties
        ptr
        id
    end

    methods

        function obj = Transducer(id, ptr)
            obj.id = id
            obj.ptr = ptr
        end

        function value = get.id(obj)
            value = obj.id;
        end

        function value = get.position(obj)
            px = libpointer('doublePtr', 0);
            py = libpointer('doublePtr', 0);
            pz = libpointer('doublePtr', 0);
            calllib('autd3capi', 'AUTDTransPosition', obj.ptr, trans_idx, px, py, pz);
            value = [px.Value; py.Value; pz.Value];
        end

        function value = get.frequency(obj)
            value = calllib('autd3capi', 'AUTDGetTransFrequency', obj.ptr, obj.id);
        end

        function set.frequency(obj, value)
            calllib('autd3capi', 'AUTDSetTransFrequency', obj.ptr, obj.id, value);
        end

        function value = get.cycle(obj)
            value = calllib('autd3capi', 'AUTDGetTransCycle', obj.ptr, obj.id);
        end

        function set.cycle(obj, value)
            calllib('autd3capi', 'AUTDSetTransCycle', obj.ptr, obj.id, value);
        end

        function value = get.mod_delay(obj)
            value = calllib('autd3capi', 'AUTDGetTransModDelay', obj.ptr, obj.id);
        end

        function set.mod_delay(obj, value)
            calllib('autd3capi', 'AUTDSetTransModDelay', obj.ptr, obj.id, value);
        end

        function value = get.wavelength(obj)
            value = calllib('autd3capi', 'AUTDGetWavelength', obj.ptr, obj.id);
        end

        function value = trans_x_direction(obj)
            px = libpointer('doublePtr', 0);
            py = libpointer('doublePtr', 0);
            pz = libpointer('doublePtr', 0);
            calllib('autd3capi', 'AUTDTransXDirection', obj.ptr, obj.id, px, py, pz);
            value = [px.Value; py.Value; pz.Value];
        end

        function value = trans_y_direction(obj)
            px = libpointer('doublePtr', 0);
            py = libpointer('doublePtr', 0);
            pz = libpointer('doublePtr', 0);
            calllib('autd3capi', 'AUTDTransYDirection', obj.ptr, obj.id, px, py, pz);
            value = [px.Value; py.Value; pz.Value];
        end

        function value = trans_z_direction(obj)
            px = libpointer('doublePtr', 0);
            py = libpointer('doublePtr', 0);
            pz = libpointer('doublePtr', 0);
            calllib('autd3capi', 'AUTDTransZDirection', obj.ptr, obj.id, px, py, pz);
            value = [px.Value; py.Value; pz.Value];
        end
    end

end
