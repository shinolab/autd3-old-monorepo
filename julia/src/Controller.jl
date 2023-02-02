# File: Controller.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 02/02/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

using StaticArrays

mutable struct Transducer
    _ptr::Ptr{Cvoid}
    _id::Int32
    id
    position
    get_frequency
    set_frequency
    get_cycle
    set_cycle
    get_mod_delay
    set_mod_delay
    wavelength
    x_direction
    y_direction
    z_direction
    function Transducer(id::Int32, ptr::Ptr{Cvoid})
        tr = new(ptr, id)
        tr.id = () -> tr._id
        tr.position = function ()
            x = Ref{Float64}(0)
            y = Ref{Float64}(0)
            z = Ref{Float64}(0)
            autd3capi.autd_trans_position(tr._ptr, tr._id, x, y, z)
            SVector(x[], y[], z[])
        end
        tr.get_frequency = () -> autd3capi.autd_get_trans_frequency(tr._ptr, tr._id)
        tr.set_frequency = (value::Float64) -> autd3capi.autd_set_trans_frequency(tr._ptr, tr._id, value)
        tr.get_cycle = () -> autd3capi.autd_get_trans_cycle(tr._ptr, tr._id)
        tr.set_cycle = (value) -> autd3capi.autd_set_trans_cycle(tr._ptr, tr._id, Uint16(value))
        tr.get_mod_delay = () -> autd3capi.autd_get_trans_mod_delay(tr._ptr, tr._id)
        tr.set_mod_delay = (value) -> autd3capi.autd_set_trans_mod_delay(tr._ptr, tr._id, Uint16(value))
        tr.wavelength = () -> autd3capi.autd_get_wavelength(tr._ptr, tr._id)
        tr.x_direction = function ()
            x = Ref{Float64}(0)
            y = Ref{Float64}(0)
            z = Ref{Float64}(0)
            autd3capi.autd_trans_x_direction(tr._ptr, tr._id, x, y, z)
            SVector(x[], y[], z[])
        end
        tr.y_direction = function ()
            x = Ref{Float64}(0)
            y = Ref{Float64}(0)
            z = Ref{Float64}(0)
            autd3capi.autd_trans_y_direction(tr._ptr, tr._id, x, y, z)
            SVector(x[], y[], z[])
        end
        tr.z_direction = function ()
            x = Ref{Float64}(0)
            y = Ref{Float64}(0)
            z = Ref{Float64}(0)
            autd3capi.autd_trans_z_direction(tr._ptr, tr._id, x, y, z)
            SVector(x[], y[], z[])
        end
        tr
    end
end

mutable struct Geometry
    _ptr::Ptr{Cvoid}
    get_sound_speed
    set_sound_speed
    get_attenuation
    set_attenuation
    num_transducers
    num_devices
    center
    center_of
    iter
    function Geometry(ptr::Ptr{Cvoid})
        geometry = new(ptr)
        geometry.get_sound_speed = () -> autd3capi.autd_get_sound_speed(geometry._ptr)
        geometry.set_sound_speed = (value::Float64) -> autd3capi.autd_set_sound_speed(geometry._ptr, value)
        geometry.get_attenuation = () -> autd3capi.autd_get_attenuation(geometry._ptr)
        geometry.set_attenuation = (value::Float64) -> autd3capi.autd_set_attenuation(geometry._ptr, value)
        geometry.num_transducers = () -> autd3capi.autd_num_transducers(geometry._ptr)
        geometry.num_devices = () -> autd3capi.autd_num_devices(geometry._ptr)
        geometry.center = function ()
            x = Ref{Float64}(0)
            y = Ref{Float64}(0)
            z = Ref{Float64}(0)
            autd3capi.autd_geometry_center(geometry._ptr, x, y, z)
            SVector(x[], y[], z[])
        end
        geometry.center_of = function (dev_idx)
            x = Ref{Float64}(0)
            y = Ref{Float64}(0)
            z = Ref{Float64}(0)
            autd3capi.autd_geometry_center_of(geometry._ptr, Int32(dev_idx), x, y, z)
            SVector(x[], y[], z[])
        end
        finalizer(geometry -> autd3capi.autd_free_geometry(geometry._ptr), geometry)
        geometry
    end
end

Base.iterate(g::Geometry, state=0) = state >= g.num_transducers() ? nothing : (Transducer(Int32(state), g._ptr), state + 1)

mutable struct GeometryBuilder
    _ptr::Ptr{Cvoid}
    add_device
    add_device_quaternion
    build
    function GeometryBuilder()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_create_geometry_builder(chandle)
        builder = new(chandle[])
        builder.add_device = function (pos::SVector{3,Float64}, rot::SVector{3,Float64})
            x, y, z = pos
            az1, ay, az2 = rot
            autd3capi.autd_add_device(builder._ptr, x, y, z, az1, ay, az2)
            builder
        end
        builder.add_device_quaternion = function (pos::SVector{3,Float64}, rot::SVector{4,Float64})
            x, y, z = pos
            rw, rx, ry, rz = rot
            autd3capi.autd_add_device_quaternion(builder._ptr, x, y, z, rw, rx, ry, rz,)
            builder
        end
        builder.build = function ()
            chandle = Ref(Ptr{Cvoid}(0))
            autd3capi.autd_build_geometry(chandle, builder._ptr)
            Geometry(chandle[])
        end
        builder
    end
end

mutable struct Controller
    _ptr::Ptr{Cvoid}
    _geometry::Geometry
    to_legacy
    to_normal
    to_normal_phase
    geometry
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
    firmware_info_list
    send
    function Controller(geometry, link)
        chandle = Ref(Ptr{Cvoid}(0))
        if !autd3capi.autd_open_controller(chandle, geometry._ptr, link._link._ptr)
            throw(ErrorException("Failed to open controller"))
        end
        cnt = new(chandle[], geometry)
        cnt.to_legacy = () -> autd3capi.autd_set_mode(cnt._ptr, 0)
        cnt.to_normal = () -> autd3capi.autd_set_mode(cnt._ptr, 1)
        cnt.to_normal_phase = () -> autd3capi.autd_set_mode(cnt._ptr, 2)
        cnt.geometry = () -> cnt._geometry
        cnt.close = () -> autd3capi.autd_close(cnt._ptr)
        cnt.is_open = () -> autd3capi.autd_is_open(cnt._ptr)
        cnt.get_force_fan = () -> autd3capi.autd_get_force_fan(cnt._ptr)
        cnt.set_force_fan = (flag::Bool) -> autd3capi.autd_set_force_fan(cnt._ptr, flag)
        cnt.get_reads_fpga_info = () -> autd3capi.autd_get_reads_fpga_info(cnt._ptr)
        cnt.set_reads_fpga_info = (flag::Bool) -> autd3capi.autd_set_reads_fpga_info(cnt._ptr, flag)
        cnt.get_ack_check_timeout = () -> autd3capi.autd_get_ack_check_timeout(cnt._ptr)
        cnt.set_ack_check_timeout = (value) -> autd3capi.autd_set_ack_check_timeout(cnt._ptr, UInt64(value))
        cnt.get_send_interval = () -> autd3capi.autd_get_send_interval(cnt._ptr)
        cnt.set_send_interval = (value) -> autd3capi.autd_set_send_interval(cnt._ptr, UInt64(value))
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

struct Clear
    _special_data_ptr::Ptr{Cvoid}
    function Clear()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_clear(chandle)
        new(chandle[])
    end
end

struct UpdateFlag
    _special_data_ptr::Ptr{Cvoid}
    function UpdateFlag()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_update_flags(chandle)
        new(chandle[])
    end
end

struct Stop
    _special_data_ptr::Ptr{Cvoid}
    function Stop()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_stop(chandle)
        new(chandle[])
    end
end

struct ModDelayConfig
    _special_data_ptr::Ptr{Cvoid}
    function ModDelayConfig()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_mod_delay_config(chandle)
        new(chandle[])
    end
end

struct Synchronize
    _special_data_ptr::Ptr{Cvoid}
    function Synchronize()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_synchronize(chandle)
        new(chandle[])
    end
end
