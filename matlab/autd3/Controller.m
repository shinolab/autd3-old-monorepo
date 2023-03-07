%{
%File: Controller.m
%Project: autd3-matlab
%Created Date: 07/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 08/03/2023
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef Controller < handle

    properties
        ptr
        geometry
        ack_check_timeout = 0
        send_interval = 500000
    end

    methods

        function obj = Controller(geometry, link)
            obj.ptr = libpointer('voidPtr', 0);
            obj.geometry = geometry;
            pp = libpointer('voidPtrPtr', obj.ptr);
            if ~calllib('autd3capi', 'AUTDOpenController', pp, geometry.ptr, link.ptr)
                throw(MException('MATLAB:RuntimeError', 'Failed to open controller'));
            end
        end

        function value = get.geometry(obj)
            value = obj.geometry;
        end

        function res = close(obj)
            res = calllib('autd3capi', 'AUTDClose', obj.ptr);
        end

        function res = is_open(obj)
            res = calllib('autd3capi', 'AUTDIsOpen', obj.ptr);
        end

        function force_fan(obj, value)
            calllib('autd3capi', 'AUTDSetForceFan', obj.ptr, value);
        end

        function reads_fpga_info(obj, value)
            calllib('autd3capi', 'AUTDSetReadsFPGAInfo', obj.ptr, value);
        end

        function res = send(varargin)
            obj = varargin{1};

            if nargin >= 4
                res = false;
                return;
            end

            if nargin == 3

                if isa(varargin{2}, 'Header') && isa(varargin{3}, 'Body')
                    res = calllib('autd3capi', 'AUTDSend', obj.ptr, varargin{2}.ptr, varargin{3}.ptr, uint64(0));
                    return;
                end

                if isa(varargin{3}, 'Header') && isa(varargin{2}, 'Body')
                    res = calllib('autd3capi', 'AUTDSend', obj.ptr, varargin{3}.ptr, varargin{2}.ptr, uint64(0));
                    return;
                end

            end

            if nargin == 2

                if isa(varargin{2}, 'SpecialData')
                    res = calllib('autd3capi', 'AUTDSendSpecial', obj.ptr, varargin{2}.ptr, uint64(0));
                    return;
                end

                if isa(varargin{2}, 'Header')
                    np = libpointer('voidPtr', []);
                    res = calllib('autd3capi', 'AUTDSend', obj.ptr, varargin{2}.ptr, np, uint64(0));
                    return;
                end

                if isa(varargin{2}, 'Body')
                    np = libpointer('voidPtr', []);
                    res = calllib('autd3capi', 'AUTDSend', obj.ptr, np, varargin{2}.ptr, uint64(0));
                    return;
                end

            end

            res = false;
        end

        function list = fpga_info(obj)
            n = obj.geometry_.num_devices;
            info_p = libpointer('uint8Ptr', zeros(n, 1, 'uint8'));
            calllib('autd3capi', 'AUTDGetFPGAInfo', obj.ptr, info_p);
            list = info_p.Value;
        end

        function delete(obj)

            if obj.ptr.Value ~= 0
                calllib('autd3capi', 'AUTDFreeController', obj.ptr);
                obj.ptr = libpointer('voidPtr', 0);
            end

        end

    end

end
