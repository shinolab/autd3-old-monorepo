# This file was automatically generated from header file

module autd3capi

function get_bin_path()
if Sys.iswindows()
    return "win-x64"
elseif Sys.isapple()
    return "macos-universal"
elseif Sys.islinux()
    return "linux-x64"
end
end

function get_lib_ext()
if Sys.iswindows()
    return ".dll"
elseif Sys.isapple()
    return ".dylib"
elseif Sys.islinux()
    return ".so"
end
end

function get_lib_prefix()
if Sys.iswindows()
    return ""
else
    return "lib"
end
end

const _dll = joinpath(@__DIR__, get_bin_path(), "bin", get_lib_prefix() * "autd3capi" * get_lib_ext())

autd_set_log_level(level) = ccall((:AUTDSetLogLevel, _dll), Cvoid, (Int32, ), level);
autd_set_default_logger(out, flush) = ccall((:AUTDSetDefaultLogger, _dll), Cvoid, (Ptr{Cvoid}, Ptr{Cvoid}, ), out, flush);
autd_create_geometry_builder(out) = ccall((:AUTDCreateGeometryBuilder, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), out);
autd_add_device(geometry_builder, x, y, z, rz1, ry, rz2) = ccall((:AUTDAddDevice, _dll), Bool, (Ptr{Cvoid}, Float64, Float64, Float64, Float64, Float64, Float64, ), geometry_builder, x, y, z, rz1, ry, rz2);
autd_add_device_quaternion(geometry_builder, x, y, z, qw, qx, qy, qz) = ccall((:AUTDAddDeviceQuaternion, _dll), Bool, (Ptr{Cvoid}, Float64, Float64, Float64, Float64, Float64, Float64, Float64, ), geometry_builder, x, y, z, qw, qx, qy, qz);
autd_build_geometry(out, geometry_builder) = ccall((:AUTDBuildGeometry, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, ), out, geometry_builder);
autd_free_geometry(geometry) = ccall((:AUTDFreeGeometry, _dll), Cvoid, (Ptr{Cvoid}, ), geometry);
autd_open_controller(out, geometry, link) = ccall((:AUTDOpenController, _dll), Bool, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, Ptr{Cvoid}, ), out, geometry, link);
autd_get_geometry(geometry, cnt) = ccall((:AUTDGetGeometry, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, ), geometry, cnt);
autd_close(handle) = ccall((:AUTDClose, _dll), Bool, (Ptr{Cvoid}, ), handle);
autd_free_controller(handle) = ccall((:AUTDFreeController, _dll), Cvoid, (Ptr{Cvoid}, ), handle);
autd_is_open(handle) = ccall((:AUTDIsOpen, _dll), Bool, (Ptr{Cvoid}, ), handle);
autd_set_reads_fpga_info(handle, reads_fpga_info) = ccall((:AUTDSetReadsFPGAInfo, _dll), Cvoid, (Ptr{Cvoid}, Bool, ), handle, reads_fpga_info);
autd_set_force_fan(handle, force) = ccall((:AUTDSetForceFan, _dll), Cvoid, (Ptr{Cvoid}, Bool, ), handle, force);
autd_get_sound_speed(geometry) = ccall((:AUTDGetSoundSpeed, _dll), Float64, (Ptr{Cvoid}, ), geometry);
autd_set_sound_speed(geometry, sound_speed) = ccall((:AUTDSetSoundSpeed, _dll), Cvoid, (Ptr{Cvoid}, Float64, ), geometry, sound_speed);
autd_set_sound_speed_from_temp(geometry, temp, k, r, m) = ccall((:AUTDSetSoundSpeedFromTemp, _dll), Cvoid, (Ptr{Cvoid}, Float64, Float64, Float64, Float64, ), geometry, temp, k, r, m);
autd_get_trans_frequency(geometry, trans_idx) = ccall((:AUTDGetTransFrequency, _dll), Float64, (Ptr{Cvoid}, Int32, ), geometry, trans_idx);
autd_set_trans_frequency(geometry, trans_idx, frequency) = ccall((:AUTDSetTransFrequency, _dll), Cvoid, (Ptr{Cvoid}, Int32, Float64, ), geometry, trans_idx, frequency);
autd_get_trans_cycle(geometry, trans_idx) = ccall((:AUTDGetTransCycle, _dll), UInt16, (Ptr{Cvoid}, Int32, ), geometry, trans_idx);
autd_set_trans_cycle(geometry, trans_idx, cycle) = ccall((:AUTDSetTransCycle, _dll), Cvoid, (Ptr{Cvoid}, Int32, UInt16, ), geometry, trans_idx, cycle);
autd_get_wavelength(geometry, trans_idx) = ccall((:AUTDGetWavelength, _dll), Float64, (Ptr{Cvoid}, Int32, ), geometry, trans_idx);
autd_get_attenuation(geometry) = ccall((:AUTDGetAttenuation, _dll), Float64, (Ptr{Cvoid}, ), geometry);
autd_set_attenuation(geometry, attenuation) = ccall((:AUTDSetAttenuation, _dll), Cvoid, (Ptr{Cvoid}, Float64, ), geometry, attenuation);
autd_get_fpga_info(handle, out) = ccall((:AUTDGetFPGAInfo, _dll), Bool, (Ptr{Cvoid}, Array{UInt8,1}, ), handle, out);
autd_num_transducers(geometry) = ccall((:AUTDNumTransducers, _dll), Int32, (Ptr{Cvoid}, ), geometry);
autd_num_devices(geometry) = ccall((:AUTDNumDevices, _dll), Int32, (Ptr{Cvoid}, ), geometry);
autd_geometry_center(geometry, x, y, z) = ccall((:AUTDGeometryCenter, _dll), Cvoid, (Ptr{Cvoid}, Ref{Float64}, Ref{Float64}, Ref{Float64}, ), geometry, x, y, z);
autd_geometry_center_of(geometry, dev_idx, x, y, z) = ccall((:AUTDGeometryCenterOf, _dll), Cvoid, (Ptr{Cvoid}, Int32, Ref{Float64}, Ref{Float64}, Ref{Float64}, ), geometry, dev_idx, x, y, z);
autd_trans_position(geometry, trans_idx, x, y, z) = ccall((:AUTDTransPosition, _dll), Cvoid, (Ptr{Cvoid}, Int32, Ref{Float64}, Ref{Float64}, Ref{Float64}, ), geometry, trans_idx, x, y, z);
autd_trans_x_direction(geometry, trans_idx, x, y, z) = ccall((:AUTDTransXDirection, _dll), Cvoid, (Ptr{Cvoid}, Int32, Ref{Float64}, Ref{Float64}, Ref{Float64}, ), geometry, trans_idx, x, y, z);
autd_trans_y_direction(geometry, trans_idx, x, y, z) = ccall((:AUTDTransYDirection, _dll), Cvoid, (Ptr{Cvoid}, Int32, Ref{Float64}, Ref{Float64}, Ref{Float64}, ), geometry, trans_idx, x, y, z);
autd_trans_z_direction(geometry, trans_idx, x, y, z) = ccall((:AUTDTransZDirection, _dll), Cvoid, (Ptr{Cvoid}, Int32, Ref{Float64}, Ref{Float64}, Ref{Float64}, ), geometry, trans_idx, x, y, z);
autd_get_firmware_info_list_pointer(handle, out) = ccall((:AUTDGetFirmwareInfoListPointer, _dll), Int32, (Ptr{Cvoid}, Ref{Ptr{Cvoid}}, ), handle, out);
autd_get_firmware_info(p_firm_info_list, index, info, matches_version, is_supported) = ccall((:AUTDGetFirmwareInfo, _dll), Cvoid, (Ptr{Cvoid}, Int32, Ptr{UInt8}, Ref{Bool}, Ref{Bool}, ), p_firm_info_list, index, info, matches_version, is_supported);
autd_free_firmware_info_list_pointer(p_firm_info_list) = ccall((:AUTDFreeFirmwareInfoListPointer, _dll), Cvoid, (Ptr{Cvoid}, ), p_firm_info_list);
autd_get_latest_firmware(latest_version) = ccall((:AUTDGetLatestFirmware, _dll), Cvoid, (Ptr{UInt8}, ), latest_version);
autd_gain_null(gain) = ccall((:AUTDGainNull, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), gain);
autd_gain_grouped(gain) = ccall((:AUTDGainGrouped, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), gain);
autd_gain_grouped_add(grouped_gain, device_id, gain) = ccall((:AUTDGainGroupedAdd, _dll), Cvoid, (Ptr{Cvoid}, Int32, Ptr{Cvoid}, ), grouped_gain, device_id, gain);
autd_gain_focus(gain, x, y, z, amp) = ccall((:AUTDGainFocus, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Float64, Float64, Float64, Float64, ), gain, x, y, z, amp);
autd_gain_bessel_beam(gain, x, y, z, n_x, n_y, n_z, theta_z, amp) = ccall((:AUTDGainBesselBeam, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Float64, Float64, Float64, Float64, Float64, Float64, Float64, Float64, ), gain, x, y, z, n_x, n_y, n_z, theta_z, amp);
autd_gain_plane_wave(gain, n_x, n_y, n_z, amp) = ccall((:AUTDGainPlaneWave, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Float64, Float64, Float64, Float64, ), gain, n_x, n_y, n_z, amp);
autd_gain_transducer_test(gain) = ccall((:AUTDGainTransducerTest, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), gain);
autd_gain_transducer_test_set(gain, tr_idx, amp, phase) = ccall((:AUTDGainTransducerTestSet, _dll), Cvoid, (Ptr{Cvoid}, Int32, Float64, Float64, ), gain, tr_idx, amp, phase);
autd_gain_custom(gain, amp, phase, size) = ccall((:AUTDGainCustom, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Array{Float64,1}, Array{Float64,1}, UInt64, ), gain, amp, phase, size);
autd_delete_gain(gain) = ccall((:AUTDDeleteGain, _dll), Cvoid, (Ptr{Cvoid}, ), gain);
autd_modulation_static(mod, amp) = ccall((:AUTDModulationStatic, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Float64, ), mod, amp);
autd_modulation_sine(mod, freq, amp, offset) = ccall((:AUTDModulationSine, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Int32, Float64, Float64, ), mod, freq, amp, offset);
autd_modulation_sine_squared(mod, freq, amp, offset) = ccall((:AUTDModulationSineSquared, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Int32, Float64, Float64, ), mod, freq, amp, offset);
autd_modulation_sine_legacy(mod, freq, amp, offset) = ccall((:AUTDModulationSineLegacy, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Float64, Float64, Float64, ), mod, freq, amp, offset);
autd_modulation_square(mod, freq, low, high, duty) = ccall((:AUTDModulationSquare, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Int32, Float64, Float64, Float64, ), mod, freq, low, high, duty);
autd_modulation_lpf(mod, mod_in) = ccall((:AUTDModulationLPF, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, ), mod, mod_in);
autd_modulation_custom(mod, buffer, size, freq_div) = ccall((:AUTDModulationCustom, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Array{Float64,1}, UInt64, UInt32, ), mod, buffer, size, freq_div);
autd_modulation_sampling_frequency_division(mod) = ccall((:AUTDModulationSamplingFrequencyDivision, _dll), UInt32, (Ptr{Cvoid}, ), mod);
autd_modulation_set_sampling_frequency_division(mod, freq_div) = ccall((:AUTDModulationSetSamplingFrequencyDivision, _dll), Cvoid, (Ptr{Cvoid}, UInt32, ), mod, freq_div);
autd_modulation_sampling_frequency(mod) = ccall((:AUTDModulationSamplingFrequency, _dll), Float64, (Ptr{Cvoid}, ), mod);
autd_delete_modulation(mod) = ccall((:AUTDDeleteModulation, _dll), Cvoid, (Ptr{Cvoid}, ), mod);
autd_focus_stm(out) = ccall((:AUTDFocusSTM, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), out);
autd_gain_stm(out, mode) = ccall((:AUTDGainSTM, _dll), Cvoid, (Ref{Ptr{Cvoid}}, UInt16, ), out, mode);
autd_focus_stm_add(stm, x, y, z, shift) = ccall((:AUTDFocusSTMAdd, _dll), Cvoid, (Ptr{Cvoid}, Float64, Float64, Float64, UInt8, ), stm, x, y, z, shift);
autd_gain_stm_add(stm, gain) = ccall((:AUTDGainSTMAdd, _dll), Cvoid, (Ptr{Cvoid}, Ptr{Cvoid}, ), stm, gain);
autdstm_set_frequency(stm, freq) = ccall((:AUTDSTMSetFrequency, _dll), Float64, (Ptr{Cvoid}, Float64, ), stm, freq);
autdstm_get_start_idx(stm) = ccall((:AUTDSTMGetStartIdx, _dll), Int32, (Ptr{Cvoid}, ), stm);
autdstm_get_finish_idx(stm) = ccall((:AUTDSTMGetFinishIdx, _dll), Int32, (Ptr{Cvoid}, ), stm);
autdstm_set_start_idx(stm, start_idx) = ccall((:AUTDSTMSetStartIdx, _dll), Cvoid, (Ptr{Cvoid}, Int32, ), stm, start_idx);
autdstm_set_finish_idx(stm, finish_idx) = ccall((:AUTDSTMSetFinishIdx, _dll), Cvoid, (Ptr{Cvoid}, Int32, ), stm, finish_idx);
autdstm_frequency(stm) = ccall((:AUTDSTMFrequency, _dll), Float64, (Ptr{Cvoid}, ), stm);
autdstm_sampling_frequency(stm) = ccall((:AUTDSTMSamplingFrequency, _dll), Float64, (Ptr{Cvoid}, ), stm);
autdstm_sampling_frequency_division(stm) = ccall((:AUTDSTMSamplingFrequencyDivision, _dll), UInt32, (Ptr{Cvoid}, ), stm);
autdstm_set_sampling_frequency_division(stm, freq_div) = ccall((:AUTDSTMSetSamplingFrequencyDivision, _dll), Cvoid, (Ptr{Cvoid}, UInt32, ), stm, freq_div);
autd_delete_stm(stm) = ccall((:AUTDDeleteSTM, _dll), Cvoid, (Ptr{Cvoid}, ), stm);
autd_synchronize(out) = ccall((:AUTDSynchronize, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), out);
autd_clear(out) = ccall((:AUTDClear, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), out);
autd_update_flags(out) = ccall((:AUTDUpdateFlags, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), out);
autd_stop(out) = ccall((:AUTDStop, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), out);
autd_mod_delay_config(out) = ccall((:AUTDModDelayConfig, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), out);
autd_delete_special_data(data) = ccall((:AUTDDeleteSpecialData, _dll), Cvoid, (Ptr{Cvoid}, ), data);
autd_create_silencer(out, step, cycle) = ccall((:AUTDCreateSilencer, _dll), Cvoid, (Ref{Ptr{Cvoid}}, UInt16, UInt16, ), out, step, cycle);
autd_delete_silencer(config) = ccall((:AUTDDeleteSilencer, _dll), Cvoid, (Ptr{Cvoid}, ), config);
autd_send(handle, header, body, timeout_ns) = ccall((:AUTDSend, _dll), Bool, (Ptr{Cvoid}, Ptr{Cvoid}, Ptr{Cvoid}, UInt64, ), handle, header, body, timeout_ns);
autd_send_special(handle, special, timeout_ns) = ccall((:AUTDSendSpecial, _dll), Bool, (Ptr{Cvoid}, Ptr{Cvoid}, UInt64, ), handle, special, timeout_ns);
autd_get_trans_mod_delay(geometry, trans_idx) = ccall((:AUTDGetTransModDelay, _dll), UInt16, (Ptr{Cvoid}, Int32, ), geometry, trans_idx);
autd_set_trans_mod_delay(geometry, trans_idx, delay) = ccall((:AUTDSetTransModDelay, _dll), Cvoid, (Ptr{Cvoid}, Int32, UInt16, ), geometry, trans_idx, delay);
autd_create_amplitudes(out, amp) = ccall((:AUTDCreateAmplitudes, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Float64, ), out, amp);
autd_delete_amplitudes(amplitudes) = ccall((:AUTDDeleteAmplitudes, _dll), Cvoid, (Ptr{Cvoid}, ), amplitudes);
autd_set_mode(handle, mode) = ccall((:AUTDSetMode, _dll), Cvoid, (Ptr{Cvoid}, UInt8, ), handle, mode);
autd_software_stm(out) = ccall((:AUTDSoftwareSTM, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), out);
autd_software_stm_set_strategy(stm, strategy) = ccall((:AUTDSoftwareSTMSetStrategy, _dll), Cvoid, (Ptr{Cvoid}, UInt8, ), stm, strategy);
autd_software_stm_add(stm, gain) = ccall((:AUTDSoftwareSTMAdd, _dll), Cvoid, (Ptr{Cvoid}, Ptr{Cvoid}, ), stm, gain);
autd_software_stm_start(handle, stm, cnt) = ccall((:AUTDSoftwareSTMStart, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, Ptr{Cvoid}, ), handle, stm, cnt);
autd_software_stm_finish(handle) = ccall((:AUTDSoftwareSTMFinish, _dll), Cvoid, (Ptr{Cvoid}, ), handle);
autd_software_stm_set_frequency(stm, freq) = ccall((:AUTDSoftwareSTMSetFrequency, _dll), Float64, (Ptr{Cvoid}, Float64, ), stm, freq);
autd_software_stm_frequency(stm) = ccall((:AUTDSoftwareSTMFrequency, _dll), Float64, (Ptr{Cvoid}, ), stm);
autd_software_stm_period(stm) = ccall((:AUTDSoftwareSTMPeriod, _dll), UInt64, (Ptr{Cvoid}, ), stm);
autd_software_stm_sampling_frequency(stm) = ccall((:AUTDSoftwareSTMSamplingFrequency, _dll), Float64, (Ptr{Cvoid}, ), stm);
autd_software_stm_sampling_period(stm) = ccall((:AUTDSoftwareSTMSamplingPeriod, _dll), UInt64, (Ptr{Cvoid}, ), stm);
autd_software_stm_set_sampling_period(stm, period) = ccall((:AUTDSoftwareSTMSetSamplingPeriod, _dll), Cvoid, (Ptr{Cvoid}, UInt64, ), stm, period);
autd_delete_software_stm(stm) = ccall((:AUTDDeleteSoftwareSTM, _dll), Cvoid, (Ptr{Cvoid}, ), stm);
end
