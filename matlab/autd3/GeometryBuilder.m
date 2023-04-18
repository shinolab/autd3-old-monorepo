%{
%File: GeometryBuilder.m
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

classdef GeometryBuilder < handle

    properties
        ptr
    end

    methods

        function obj = GeometryBuilder()
            obj.ptr = libpointer('voidPtr', 0);
            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi', 'AUTDCreateGeometryBuilder', pp);
        end

        function res = add_device(obj, pos, rot)
            calllib('autd3capi', 'AUTDAddDevice', obj.ptr, pos(1), pos(2), pos(3), rot(1), rot(2), rot(3));
            res = obj;
        end

        function res = add_device_quaternion(obj, pos, rot)
            calllib('autd3capi', 'AUTDAddDeviceQuaternion', obj.ptr, pos(1), pos(2), pos(3), rot(1), rot(2), rot(3), rot(4));
            res = obj;
        end

        function res = legacy_mode(obj)
            calllib('autd3capi', 'AUTDSetMode', obj.ptr, 0);
            res = obj;
        end

        function res = advanced_mode(obj)
            calllib('autd3capi', 'AUTDSetMode', obj.ptr, 1);
            res = obj;
        end

        function res = advanced_phase_mode(obj)
            calllib('autd3capi', 'AUTDSetMode', obj.ptr, 2);
            res = obj;
        end

        function value = build(obj, idx)
            ptr = libpointer('voidPtr', 0);
            pp = libpointer('voidPtrPtr', ptr);
            calllib('autd3capi', 'AUTDBuildGeometry', pp, obj.ptr);
            value = Geometry(ptr);
        end

    end
end
