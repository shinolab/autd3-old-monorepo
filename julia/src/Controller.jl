# File: Controller.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 15/11/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

using StaticArrays

mutable struct Controller
    _ptr::Ptr{Cvoid}
    to_legacy
    to_normal
    to_normal_phase
    add_device
    add_device_quaternion
    open
    close
    is_open
    get_force_fan
    set_force_fan
    get_reads_fpga_info
    set_reads_fpga_info
    get_ack_check_timeout
    set_ack_check_timeout
    get_send_interval
    set_send_interval
    get_sound_speed
    set_sound_speed
    get_attenuation
    set_attenuation
    get_trans_frequency
    set_trans_frequency
    get_trans_cycle
    set_trans_cycle
    get_wavelength
    trans_position
    trans_direction_x
    trans_direction_y
    trans_direction_z
    set_mod_delay
    num_devices
    firmware_info_list
    send
    function Controller()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_create_controller(chandle)
        cnt = new(chandle[])
        cnt.add_device = function (pos::SVector{3,Float64}, rot::SVector{3,Float64})
            x, y, z = pos
            az1, ay, az2 = rot
            autd3capi.autd_add_device(cnt._ptr, x, y, z, az1, ay, az2)
        end
        cnt.add_device_quaternion = function (pos::SVector{3,Float64}, rot::SVector{4,Float64})
            x, y, z = pos
            rw, rx, ry, rz = rot
            autd3capi.autd_add_device_quaternion(cnt._ptr, x, y, z, rw, rx, ry, rz,)
        end
        cnt.to_legacy = () -> autd3capi.set_mode(cnt._ptr, 0)
        cnt.to_normal = () -> autd3capi.set_mode(cnt._ptr, 1)
        cnt.to_normal_phase = () -> autd3capi.set_mode(cnt._ptr, 2)
        cnt.open = (link) -> autd3capi.autd_open_controller(cnt._ptr, link._link._ptr)
        cnt.close = () -> autd3capi.autd_close(cnt._ptr)
        cnt.num_devices = () -> autd3capi.autd_num_devices(cnt._ptr)
        cnt.is_open = () -> autd3capi.autd_is_open(cnt._ptr)
        cnt.get_force_fan = () -> autd3capi.autd_get_force_fan(cnt._ptr)
        cnt.set_force_fan = (flag::Bool) -> autd3capi.autd_set_force_fan(cnt._ptr, flag)
        cnt.get_reads_fpga_info = () -> autd3capi.autd_get_reads_fpga_info(cnt._ptr)
        cnt.set_reads_fpga_info = (flag::Bool) -> autd3capi.autd_set_reads_fpga_info(cnt._ptr, flag)
        cnt.get_ack_check_timeout = () -> autd3capi.autd_get_ack_check_timeout(cnt._ptr)
        cnt.set_ack_check_timeout = (value) -> autd3capi.autd_set_ack_check_timeout(cnt._ptr, UInt64(value))
        cnt.get_send_interval = () -> autd3capi.autd_get_send_interval(cnt._ptr)
        cnt.set_send_interval = (value) -> autd3capi.autd_set_send_interval(cnt._ptr, UInt64(value))
        cnt.get_sound_speed = () -> autd3capi.autd_get_sound_speed(cnt._ptr)
        cnt.set_sound_speed = (value::Float64) -> autd3capi.autd_set_sound_speed(cnt._ptr, value)
        cnt.get_attenuation = () -> autd3capi.autd_get_attenuation(cnt._ptr)
        cnt.set_attenuation = (value::Float64) -> autd3capi.autd_set_attenuation(cnt._ptr, value)
        cnt.get_trans_frequency = (devId, transIdx) -> autd3capi.autd_get_trans_frequency(cnt._ptr, Int32(devId), Int32(transIdx))
        cnt.set_trans_frequency = (devId, transIdx, value::Float64) -> autd3capi.autd_set_trans_frequency(cnt._ptr, Int32(devId), Int32(transIdx), value)
        cnt.get_trans_cycle = (devId, transIdx) -> autd3capi.autd_get_trans_cycle(cnt._ptr, Int32(devId), Int32(transIdx))
        cnt.set_trans_cycle = (devId, transIdx, value) -> autd3capi.autd_set_trans_frequency(cnt._ptr, Int32(devId), Int32(transIdx), Uint16(value))
        cnt.get_wavelength = function (devId, transIdx)
            autd3capi.autd_get_wavelength(cnt._ptr, Int32(devId), Int32(transIdx))
        end
        cnt.trans_position = function (devId, transIdx)
            x = Ref{Float64}(0)
            y = Ref{Float64}(0)
            z = Ref{Float64}(0)
            autd3capi.autd_trans_position(cnt._ptr, Int32(devId), Int32(transIdx), x, y, z)
            SVector(x[], y[], z[])
        end
        cnt.trans_direction_x = function (devId, transIdx)
            x = Ref{Float64}(0)
            y = Ref{Float64}(0)
            z = Ref{Float64}(0)
            autd3capi.autd_trans_x_direction(cnt._ptr, Int32(devId), Int32(transIdx), x, y, z)
            SVector(x[], y[], z[])
        end
        cnt.trans_direction_y = function (devId, transIdx)
            x = Ref{Float64}(0)
            y = Ref{Float64}(0)
            z = Ref{Float64}(0)
            autd3capi.autd_trans_y_direction(cnt._ptr, Int32(devId), Int32(transIdx), x, y, z)
            SVector(x[], y[], z[])
        end
        cnt.trans_direction_z = function (devId, transIdx)
            x = Ref{Float64}(0)
            y = Ref{Float64}(0)
            z = Ref{Float64}(0)
            autd3capi.autd_trans_z_direction(cnt._ptr, Int32(devId), Int32(transIdx), x, y, z)
            SVector(x[], y[], z[])
        end
        cnt.set_mod_delay = (devId, transIdx, value) -> autd3capi.autd_set_mod_delay(cnt._ptr, devId, transIdx, UInt16(value))
        cnt.firmware_info_list = function ()
            res = []
            phandle = Ref(Ptr{Cvoid}(0))
            size = autd3capi.autd_get_firmware_info_list_pointer(cnt._ptr, phandle)
            handle::Ptr{Cvoid} = phandle[]
            for i = 0:size-1
                info = zeros(UInt8, 256)
                autd3capi.autd_get_firmware_info(handle, i, info)
                push!(res, String(strip(String(info), '\0')))
            end
            autd3capi.autd_free_firmware_info_list_pointer(handle)
            res
        end
        cnt.send = function (a, b=Nothing)
            np = Ptr{Cvoid}(0)
            if b == Nothing
                if hasproperty(a, :_special_data_ptr)
                    autd3capi.autd_send_special(cnt._ptr, a._special_data_ptr)
                elseif hasproperty(a, :_header_ptr)
                    autd3capi.autd_send(cnt._ptr, a._header_ptr, np)
                elseif hasproperty(a, :_body_ptr)
                    autd3capi.autd_send(cnt._ptr, np, a._body_ptr)
                end
            else
                if hasproperty(a, :_header_ptr) && hasproperty(b, :_body_ptr)
                    autd3capi.autd_send(cnt._ptr, a._header_ptr, b._body_ptr)
                elseif hasproperty(b, :_header_ptr) && hasproperty(a, :_body_ptr)
                    autd3capi.autd_send(cnt._ptr, b._header_ptr, a._body_ptr)
                end
            end
        end

        finalizer(cnt -> autd3capi.autd_free_controller(cnt._ptr), cnt)
        cnt
    end
end

function get_last_error()
    p = Ptr{Cvoid}(C_NULL)
    n = autd3capi.autd_get_last_error(p)
    err = zeros(UInt8, n)
    autd3capi.autd_get_last_error(err)
    String(err[1:n-1])
end

struct Clear
    _special_data_ptr
    function Clear()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_clear(chandle)
        new(chandle[])
    end
end

struct UpdateFlag
    _special_data_ptr
    function UpdateFlag()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_update_flags(chandle)
        new(chandle[])
    end
end

struct Stop
    _special_data_ptr
    function Stop()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_stop(chandle)
        new(chandle[])
    end
end

struct ModDelayConfig
    _special_data_ptr
    function ModDelayConfig()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_mod_delay_config(chandle)
        new(chandle[])
    end
end

struct Synchronize
    _special_data_ptr
    function Synchronize()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_synchronize(chandle)
        new(chandle[])
    end
end
