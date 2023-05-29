/*
 * File: lib.rs
 * Project: src
 * Created Date: 11/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

mod custom;

use custom::{CustomGain, CustomModulation};

use std::{
    ffi::c_char,
    sync::{Arc, Mutex},
    time::Duration,
};

use autd3capi_common::*;
use autd3capi_def::{GainSTMMode, Level, TransMode, ERR, FALSE, TRUE};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDCreateGeometryBuilder() -> ConstPtr {
    Box::into_raw(Box::new(GeometryBuilder::<DynamicTransducer>::new())) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDAddDevice(
    builder: ConstPtr,
    x: float,
    y: float,
    z: float,
    rz1: float,
    ry: float,
    rz2: float,
) {
    unsafe {
        cast_without_ownership_mut!(builder, GeometryBuilder<DynamicTransducer>).add_device(
            AUTD3::new(Vector3::new(x, y, z), Vector3::new(rz1, ry, rz2)),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDAddDeviceQuaternion(
    builder: ConstPtr,
    x: float,
    y: float,
    z: float,
    qw: float,
    qx: float,
    qy: float,
    qz: float,
) {
    unsafe {
        cast_without_ownership_mut!(builder, GeometryBuilder<DynamicTransducer>).add_device(
            AUTD3::new_with_quaternion(
                Vector3::new(x, y, z),
                UnitQuaternion::from_quaternion(Quaternion::new(qw, qx, qy, qz)),
            ),
        );
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDBuildGeometry(builder: ConstPtr, err: *mut c_char) -> ConstPtr {
    unsafe {
        let geometry = try_or_return!(
            Box::from_raw(builder as *mut GeometryBuilder<DynamicTransducer>).build(),
            err,
            NULL
        );
        Box::into_raw(Box::new(geometry)) as _
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDOpenController(
    geometry: ConstPtr,
    link: ConstPtr,
    err: *mut c_char,
) -> ConstPtr {
    unsafe {
        let link: Box<Box<L>> = Box::from_raw(link as *mut _);
        let link = DynamicLink::new(*link);
        let geometry: Box<Geometry<DynamicTransducer>> = Box::from_raw(geometry as *mut _);
        let cnt = try_or_return!(Controller::open(*geometry, link), err, NULL);
        Box::into_raw(Box::new(cnt)) as _
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDClose(cnt: ConstPtr, err: *mut c_char) -> bool {
    unsafe {
        try_or_return!(cast_without_ownership_mut!(cnt, Cnt).close(), err, false);
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFreeController(cnt: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(cnt as *mut Cnt);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetReadsFPGAInfo(cnt: ConstPtr, value: bool) {
    unsafe { cast_without_ownership_mut!(cnt, Cnt).reads_fpga_info(value) }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetForceFan(cnt: ConstPtr, value: bool) {
    unsafe { cast_without_ownership_mut!(cnt, Cnt).force_fan(value) }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetSoundSpeed(cnt: ConstPtr) -> float {
    unsafe { cast_without_ownership_mut!(cnt, Cnt).geometry().sound_speed }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetSoundSpeed(cnt: ConstPtr, value: float) {
    unsafe {
        cast_without_ownership_mut!(cnt, Cnt)
            .geometry_mut()
            .sound_speed = value;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetSoundSpeedFromTemp(
    cnt: ConstPtr,
    temp: float,
    k: float,
    r: float,
    m: float,
) {
    unsafe {
        cast_without_ownership_mut!(cnt, Cnt)
            .geometry_mut()
            .set_sound_speed_from_temp_with(temp, k, r, m);
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransFrequency(cnt: ConstPtr, idx: u32) -> float {
    unsafe { cast_without_ownership!(cnt, Cnt).geometry()[idx as _].frequency() }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSetTransFrequency(
    cnt: ConstPtr,
    idx: u32,
    value: float,
    err: *mut c_char,
) -> bool {
    unsafe {
        try_or_return!(
            cast_without_ownership_mut!(cnt, Cnt).geometry_mut()[idx as _].set_frequency(value),
            err,
            false
        )
    }
    true
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransCycle(cnt: ConstPtr, idx: u32) -> u16 {
    unsafe { cast_without_ownership!(cnt, Cnt).geometry()[idx as _].cycle() }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSetTransCycle(
    cnt: ConstPtr,
    idx: u32,
    value: u16,
    err: *mut c_char,
) -> bool {
    unsafe {
        try_or_return!(
            cast_without_ownership_mut!(cnt, Cnt).geometry_mut()[idx as _].set_cycle(value),
            err,
            false
        )
    }
    true
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetWavelength(cnt: ConstPtr, idx: u32) -> float {
    unsafe {
        let geometry = cast_without_ownership!(cnt, Cnt).geometry();
        geometry[idx as _].wavelength(geometry.sound_speed)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetAttenuation(cnt: ConstPtr) -> float {
    unsafe { cast_without_ownership!(cnt, Cnt).geometry().attenuation }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetAttenuation(cnt: ConstPtr, value: float) {
    unsafe {
        cast_without_ownership_mut!(cnt, Cnt)
            .geometry_mut()
            .attenuation = value;
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetFPGAInfo(cnt: ConstPtr, out: *const u8, err: *mut c_char) -> bool {
    unsafe {
        let fpga_info = try_or_return!(
            cast_without_ownership_mut!(cnt, Cnt).fpga_info(),
            err,
            false
        );
        std::ptr::copy_nonoverlapping(fpga_info.as_ptr() as _, out as *mut _, fpga_info.len());
    }
    true
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDNumTransducers(cnt: ConstPtr) -> u32 {
    unsafe {
        cast_without_ownership!(cnt, Cnt)
            .geometry()
            .num_transducers() as _
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDNumDevices(cnt: ConstPtr) -> u32 {
    unsafe { cast_without_ownership!(cnt, Cnt).geometry().num_devices() as _ }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryCenter(
    cnt: ConstPtr,
    x: *mut float,
    y: *mut float,
    z: *mut float,
) {
    unsafe {
        let center = cast_without_ownership!(cnt, Cnt).geometry().center();
        *x = center.x;
        *y = center.y;
        *z = center.z;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryCenterOf(
    cnt: ConstPtr,
    dev_idx: u32,
    x: *mut float,
    y: *mut float,
    z: *mut float,
) {
    unsafe {
        let center = cast_without_ownership!(cnt, Cnt)
            .geometry()
            .center_of(dev_idx as _);
        *x = center.x;
        *y = center.y;
        *z = center.z;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransPosition(
    cnt: ConstPtr,
    tr_idx: u32,
    x: *mut float,
    y: *mut float,
    z: *mut float,
) {
    unsafe {
        let pos = cast_without_ownership!(cnt, Cnt).geometry()[tr_idx as _].position();
        *x = pos.x;
        *y = pos.y;
        *z = pos.z;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransXDirection(
    cnt: ConstPtr,
    tr_idx: u32,
    x: *mut float,
    y: *mut float,
    z: *mut float,
) {
    unsafe {
        let dir = cast_without_ownership!(cnt, Cnt).geometry()[tr_idx as _].x_direction();
        *x = dir.x;
        *y = dir.y;
        *z = dir.z;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransYDirection(
    cnt: ConstPtr,
    tr_idx: u32,
    x: *mut float,
    y: *mut float,
    z: *mut float,
) {
    unsafe {
        let dir = cast_without_ownership!(cnt, Cnt).geometry()[tr_idx as _].y_direction();
        *x = dir.x;
        *y = dir.y;
        *z = dir.z;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransZDirection(
    cnt: ConstPtr,
    tr_idx: u32,
    x: *mut float,
    y: *mut float,
    z: *mut float,
) {
    unsafe {
        let dir = cast_without_ownership!(cnt, Cnt).geometry()[tr_idx as _].z_direction();
        *x = dir.x;
        *y = dir.y;
        *z = dir.z;
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransModDelay(cnt: ConstPtr, tr_idx: u32) -> u16 {
    unsafe { cast_without_ownership!(cnt, Cnt).geometry()[tr_idx as _].mod_delay() }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetTransModDelay(cnt: ConstPtr, tr_idx: u32, delay: u16) {
    unsafe {
        cast_without_ownership_mut!(cnt, Cnt).geometry_mut()[tr_idx as _].set_mod_delay(delay)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetFirmwareInfoListPointer(
    cnt: ConstPtr,
    err: *mut c_char,
) -> ConstPtr {
    unsafe {
        let firmware_infos = try_or_return!(
            cast_without_ownership_mut!(cnt, Cnt).firmware_infos(),
            err,
            NULL
        );
        Box::into_raw(Box::new(firmware_infos)) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGetFirmwareInfo(
    p_info_list: ConstPtr,
    idx: u32,
    info: *mut c_char,
    is_valid: *mut bool,
    is_supported: *mut bool,
) {
    unsafe {
        let firm_info = &cast_without_ownership_mut!(p_info_list, Vec<FirmwareInfo>)[idx as usize];
        let info_str = std::ffi::CString::new(firm_info.to_string()).unwrap();
        libc::strcpy(info, info_str.as_ptr());
        *is_valid = firm_info.is_valid();
        *is_supported = firm_info.is_supported();
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFreeFirmwareInfoListPointer(p_info_list: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(p_info_list as *mut Vec<FirmwareInfo>);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGetLatestFirmware(latest: *mut c_char) {
    unsafe {
        let info_str = std::ffi::CString::new(FirmwareInfo::latest_version()).unwrap();
        libc::strcpy(latest, info_str.as_ptr());
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainNull() -> ConstPtr {
    Box::into_raw(GainWrap::new(Null::new())) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainGrouped() -> ConstPtr {
    Box::into_raw(GainWrap::new(Grouped::<'static, DynamicTransducer>::new())) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainGroupedAdd(
    grouped_gain: ConstPtr,
    device_id: u32,
    gain: ConstPtr,
) {
    unsafe {
        let g = Box::from_raw(gain as *mut Box<G> as *mut Box<GainWrap>).gain;
        ((grouped_gain as *mut Box<G> as *mut Box<GainWrap>)
            .as_mut()
            .unwrap()
            .gain_mut() as *mut _ as *mut Grouped<'static, DynamicTransducer>)
            .as_mut()
            .unwrap()
            .add_boxed(device_id as _, g);
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainFocus(x: float, y: float, z: float, amp: float) -> ConstPtr {
    Box::into_raw(GainWrap::new(Focus::with_amp(Vector3::new(x, y, z), amp))) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainBesselBeam(
    x: float,
    y: float,
    z: float,
    nx: float,
    ny: float,
    nz: float,
    theta_z: float,
    amp: float,
) -> ConstPtr {
    Box::into_raw(GainWrap::new(Bessel::with_amp(
        Vector3::new(x, y, z),
        Vector3::new(nx, ny, nz),
        theta_z,
        amp,
    ))) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainPlaneWave(
    nx: float,
    ny: float,
    nz: float,
    amp: float,
) -> ConstPtr {
    Box::into_raw(GainWrap::new(Plane::with_amp(
        Vector3::new(nx, ny, nz),
        amp,
    ))) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainTransducerTest() -> ConstPtr {
    Box::into_raw(GainWrap::new(TransducerTest::new())) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainTransducerTestSet(
    trans_test: ConstPtr,
    id: u32,
    phase: float,
    amp: float,
) {
    unsafe {
        ((trans_test as *mut Box<G> as *mut Box<GainWrap>)
            .as_mut()
            .unwrap()
            .gain_mut() as *mut _ as *mut TransducerTest)
            .as_mut()
            .unwrap()
            .set(id as _, phase, amp)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainCustom(
    amp: *const float,
    phase: *const float,
    size: u64,
) -> ConstPtr {
    Box::into_raw(GainWrap::new(CustomGain::new(amp, phase, size))) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteGain(gain: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(gain as *mut Box<G>);
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStatic(amp: float) -> ConstPtr {
    Box::into_raw(ModulationWrap::new(Static::with_amp(amp))) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSine(freq: u32, amp: float, offset: float) -> ConstPtr {
    Box::into_raw(ModulationWrap::new(Sine::with_params(
        freq as _, amp, offset,
    ))) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineSquared(
    freq: u32,
    amp: float,
    offset: float,
) -> ConstPtr {
    Box::into_raw(ModulationWrap::new(SinePressure::with_params(
        freq as _, amp, offset,
    ))) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineLegacy(
    freq: float,
    amp: float,
    offset: float,
) -> ConstPtr {
    Box::into_raw(ModulationWrap::new(SineLegacy::with_params(
        freq, amp, offset,
    ))) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquare(
    freq: u32,
    low: float,
    high: float,
    duty: float,
) -> ConstPtr {
    Box::into_raw(ModulationWrap::new(Square::with_params(
        freq as _, low, high, duty,
    ))) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCustom(
    amp: *const float,
    size: u64,
    freq_div: u32,
) -> ConstPtr {
    Box::into_raw(ModulationWrap::new(CustomModulation::new(
        amp, size, freq_div,
    ))) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSamplingFrequencyDivision(m: ConstPtr) -> u32 {
    unsafe {
        cast_without_ownership!(m, Box<M>)
            .modulation()
            .sampling_frequency_division() as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationSetSamplingFrequencyDivision(m: ConstPtr, freq_div: u32) {
    unsafe {
        cast_without_ownership_mut!(m, Box<M>)
            .modulation_mut()
            .set_sampling_frequency_division(freq_div)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSamplingFrequency(m: ConstPtr) -> float {
    unsafe {
        cast_without_ownership!(m, Box<M>)
            .modulation()
            .sampling_freq()
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteModulation(m: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(m as *mut Box<M>);
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDFocusSTM() -> ConstPtr {
    Box::into_raw(FocusSTMWrap::new()) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTMAdd(stm: ConstPtr, x: float, y: float, z: float, shift: u8) {
    unsafe {
        cast_without_ownership_mut!(stm, Box<SF>)
            .stm_mut()
            .add_with_shift(Vector3::new(x, y, z), shift)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDFocusSTMSetFrequency(stm: ConstPtr, freq: float) -> float {
    unsafe {
        cast_without_ownership_mut!(stm, Box<SF>)
            .stm_mut()
            .set_freq(freq)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDFocusSTMGetStartIdx(stm: ConstPtr) -> i32 {
    unsafe {
        match cast_without_ownership!(stm, Box<SF>).stm().start_idx() {
            Some(idx) => idx as _,
            None => -1,
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDFocusSTMGetFinishIdx(stm: ConstPtr) -> i32 {
    unsafe {
        match cast_without_ownership!(stm, Box<SF>).stm().finish_idx() {
            Some(idx) => idx as _,
            None => -1,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTMSetStartIdx(stm: ConstPtr, idx: i32) {
    unsafe {
        if idx < 0 {
            cast_without_ownership_mut!(stm, Box<SF>)
                .stm_mut()
                .set_start_idx(None)
        } else {
            cast_without_ownership_mut!(stm, Box<SF>)
                .stm_mut()
                .set_start_idx(Some(idx as _))
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTMSetFinishIdx(stm: ConstPtr, idx: i32) {
    unsafe {
        if idx < 0 {
            cast_without_ownership_mut!(stm, Box<SF>)
                .stm_mut()
                .set_start_idx(None)
        } else {
            cast_without_ownership_mut!(stm, Box<SF>)
                .stm_mut()
                .set_finish_idx(Some(idx as _))
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDFocusSTMFrequency(stm: ConstPtr) -> float {
    unsafe { cast_without_ownership!(stm, Box<SF>).stm().freq() }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDFocusSTMSamplingFrequency(stm: ConstPtr) -> float {
    unsafe { cast_without_ownership!(stm, Box<SF>).stm().sampling_freq() }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDFocusSTMSamplingFrequencyDivision(stm: ConstPtr) -> u32 {
    unsafe {
        cast_without_ownership!(stm, Box<SF>)
            .stm()
            .sampling_freq_div()
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTMSetSamplingFrequencyDivision(stm: ConstPtr, freq_div: u32) {
    unsafe {
        cast_without_ownership_mut!(stm, Box<SF>)
            .stm_mut()
            .set_sampling_freq_div(freq_div)
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteFocusSTM(stm: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(stm as *mut Box<SF>);
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSTM() -> ConstPtr {
    Box::into_raw(GainSTMWrap::new()) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMAdd(stm: ConstPtr, gain: ConstPtr) {
    unsafe {
        let g = *Box::from_raw(gain as *mut Box<G> as *mut Box<GainWrap>);
        (stm as *mut Box<SG>).as_mut().unwrap().add(g)
    }
}
#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMSetMode(stm: ConstPtr, mode: GainSTMMode) {
    unsafe {
        cast_without_ownership_mut!(stm, Box<SG>)
            .stm_mut()
            .set_mode(mode.into())
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSTMSetFrequency(stm: ConstPtr, freq: float) -> float {
    unsafe {
        cast_without_ownership_mut!(stm, Box<SG>)
            .stm_mut()
            .set_freq(freq)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSTMGetStartIdx(stm: ConstPtr) -> i32 {
    unsafe {
        match cast_without_ownership!(stm, Box<SG>).stm().start_idx() {
            Some(idx) => idx as _,
            None => -1,
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSTMGetFinishIdx(stm: ConstPtr) -> i32 {
    unsafe {
        match cast_without_ownership!(stm, Box<SG>).stm().finish_idx() {
            Some(idx) => idx as _,
            None => -1,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMSetStartIdx(stm: ConstPtr, idx: i32) {
    unsafe {
        if idx < 0 {
            cast_without_ownership_mut!(stm, Box<SG>)
                .stm_mut()
                .set_start_idx(None)
        } else {
            cast_without_ownership_mut!(stm, Box<SG>)
                .stm_mut()
                .set_start_idx(Some(idx as _))
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMSetFinishIdx(stm: ConstPtr, idx: i32) {
    unsafe {
        if idx < 0 {
            cast_without_ownership_mut!(stm, Box<SG>)
                .stm_mut()
                .set_start_idx(None)
        } else {
            cast_without_ownership_mut!(stm, Box<SG>)
                .stm_mut()
                .set_finish_idx(Some(idx as _))
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSTMFrequency(stm: ConstPtr) -> float {
    unsafe { cast_without_ownership!(stm, Box<SG>).stm().freq() }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSTMSamplingFrequency(stm: ConstPtr) -> float {
    unsafe { cast_without_ownership!(stm, Box<SG>).stm().sampling_freq() }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSTMSamplingFrequencyDivision(stm: ConstPtr) -> u32 {
    unsafe {
        cast_without_ownership!(stm, Box<SG>)
            .stm()
            .sampling_freq_div()
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMSetSamplingFrequencyDivision(stm: ConstPtr, freq_div: u32) {
    unsafe {
        cast_without_ownership_mut!(stm, Box<SG>)
            .stm_mut()
            .set_sampling_freq_div(freq_div)
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteGainSTM(stm: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(stm as *mut Box<SG>);
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSynchronize() -> ConstPtr {
    let m: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(Synchronize::new()));
    Box::into_raw(m) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDClear() -> ConstPtr {
    let m: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(Clear::new()));
    Box::into_raw(m) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDUpdateFlags() -> ConstPtr {
    let m: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(UpdateFlag::new()));
    Box::into_raw(m) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDStop() -> ConstPtr {
    let m: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(Stop::new()));
    Box::into_raw(m) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModDelayConfig() -> ConstPtr {
    let m: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(ModDelay::new()));
    Box::into_raw(m) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteSpecialData(special: ConstPtr) {
    let _ = Box::from_raw(special as *mut Box<dyn DynamicDatagram>);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDCreateSilencer(step: u16) -> ConstPtr {
    let m: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(SilencerConfig::new(step)));
    Box::into_raw(m) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteSilencer(silencer: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(silencer as *mut Box<dyn DynamicDatagram>);
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDCreateAmplitudes(amp: float) -> ConstPtr {
    let m: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(Amplitudes::uniform(amp)));
    Box::into_raw(m) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteAmplitudes(amplitudes: ConstPtr) {
    let _ = Box::from_raw(amplitudes as *mut Box<dyn DynamicDatagram>);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSend(
    cnt: ConstPtr,
    mode: TransMode,
    header: ConstPtr,
    body: ConstPtr,
    timeout_ns: i64,
    err: *mut c_char,
) -> i32 {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    let mode = mode.into();
    unsafe {
        let res = if !header.is_null() && !body.is_null() {
            let header = (header as *mut Box<dyn DynamicDatagram>).as_mut().unwrap();
            let body = (body as *mut Box<dyn DynamicDatagram>).as_mut().unwrap();
            try_or_return!(
                cast_without_ownership_mut!(cnt, Cnt)
                    .send_with_timeout((mode, header, body), timeout),
                err,
                ERR
            )
        } else if !header.is_null() {
            let header = (header as *mut Box<dyn DynamicDatagram>).as_mut().unwrap();
            try_or_return!(
                cast_without_ownership_mut!(cnt, Cnt).send_with_timeout((mode, header), timeout),
                err,
                ERR
            )
        } else if !body.is_null() {
            let body = (body as *mut Box<dyn DynamicDatagram>).as_mut().unwrap();
            try_or_return!(
                cast_without_ownership_mut!(cnt, Cnt).send_with_timeout((mode, body), timeout),
                err,
                ERR
            )
        } else {
            return FALSE;
        };
        if res {
            TRUE
        } else {
            FALSE
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSendSpecial(
    cnt: ConstPtr,
    mode: TransMode,
    special: ConstPtr,
    timeout_ns: i64,
    err: *mut c_char,
) -> i32 {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    let mode = mode.into();
    unsafe {
        let special = (special as *mut Box<dyn DynamicDatagram>).as_mut().unwrap();
        if try_or_return!(
            cast_without_ownership_mut!(cnt, Cnt).send_with_timeout((mode, special), timeout),
            err,
            ERR
        ) {
            TRUE
        } else {
            FALSE
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkDebug() -> ConstPtr {
    Box::into_raw(Box::new(Debug::builder())) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkDebugLogLevel(builder: ConstPtr, level: Level) -> ConstPtr {
    Box::into_raw(Box::new(
        Box::from_raw(builder as *mut DebugBuilder).level(level.into()),
    )) as _
}

struct Callback(ConstPtr);
unsafe impl Send for Callback {}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkDebugLogFunc(
    builder: ConstPtr,
    level: Level,
    out_func: ConstPtr,
    flush_func: ConstPtr,
) -> ConstPtr {
    unsafe {
        if out_func.is_null() || flush_func.is_null() {
            return builder;
        }

        let out_f = Arc::new(Mutex::new(Callback(out_func)));
        let out_func = move |msg: &str| -> spdlog::Result<()> {
            let msg = std::ffi::CString::new(msg).unwrap();
            let out_f = std::mem::transmute::<_, unsafe extern "C" fn(*const c_char)>(
                out_f.lock().unwrap().0,
            );
            out_f(msg.as_ptr());
            Ok(())
        };
        let flush_f = Arc::new(Mutex::new(Callback(flush_func)));
        let flush_func = move || -> spdlog::Result<()> {
            let flush_f =
                std::mem::transmute::<_, unsafe extern "C" fn()>(flush_f.lock().unwrap().0);
            flush_f();
            Ok(())
        };

        let logger = get_logger_with_custom_func(level.into(), out_func, flush_func);

        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut DebugBuilder).logger(logger),
        )) as _
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkDebugTimeout(builder: ConstPtr, timeout_ns: u64) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut DebugBuilder).timeout(Duration::from_nanos(timeout_ns)),
        )) as _
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkDebugBuild(builder: ConstPtr) -> ConstPtr {
    unsafe {
        let builder = Box::from_raw(builder as *mut DebugBuilder);
        let link: Box<Box<L>> = Box::new(Box::new(builder.build()));
        Box::into_raw(link) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::ffi::CStr;

    unsafe fn make_debug_link() -> *const c_void {
        let builder = AUTDLinkDebug();
        let builder = AUTDLinkDebugLogLevel(builder, Level::Off);
        let builder = AUTDLinkDebugTimeout(builder, 0);
        AUTDLinkDebugBuild(builder as _)
    }

    #[test]
    fn basic() {
        unsafe {
            let geo_builder = AUTDCreateGeometryBuilder();
            AUTDAddDevice(geo_builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            AUTDAddDeviceQuaternion(geo_builder, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
            let mut err = vec![c_char::default(); 256];
            let geometry = AUTDBuildGeometry(geo_builder, err.as_mut_ptr());

            let link = make_debug_link();

            let cnt = AUTDOpenController(geometry, link, err.as_mut_ptr());
            if cnt == NULL {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            }

            AUTDSetReadsFPGAInfo(cnt, true);
            AUTDSetForceFan(cnt, true);

            let c = 300e3;
            AUTDSetSoundSpeed(cnt, c);
            assert_eq!(c, AUTDGetSoundSpeed(cnt));

            AUTDSetSoundSpeedFromTemp(cnt, 15.0, 1.4, 8.314_463, 28.9647e-3);
            dbg!(AUTDGetSoundSpeed(cnt));

            let f = 70e3;
            if !AUTDSetTransFrequency(cnt, 0, f, err.as_mut_ptr()) {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            }
            dbg!(AUTDGetTransFrequency(cnt, 0));

            let f = 4096;
            if !AUTDSetTransCycle(cnt, 0, f, err.as_mut_ptr()) {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            }
            dbg!(AUTDGetTransCycle(cnt, 0));

            dbg!(AUTDGetWavelength(cnt, 0));

            let atten = 0.1;
            AUTDSetAttenuation(cnt, atten);
            dbg!(AUTDGetAttenuation(cnt));

            let num_transducers = AUTDNumTransducers(cnt);
            dbg!(num_transducers);
            let num_devices = AUTDNumDevices(cnt) as usize;
            dbg!(num_devices);

            let mut fpga_info = vec![0xFFu8; num_devices];
            if !AUTDGetFPGAInfo(cnt, fpga_info.as_mut_ptr() as _, err.as_mut_ptr()) {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            }
            dbg!(fpga_info);

            let mut x = 0.0;
            let mut y = 0.0;
            let mut z = 0.0;
            AUTDGeometryCenter(cnt, &mut x as _, &mut y as _, &mut z as _);
            dbg!(Vector3::new(x, y, z));
            AUTDGeometryCenterOf(cnt, 0, &mut x as _, &mut y as _, &mut z as _);
            dbg!(Vector3::new(x, y, z));

            AUTDTransPosition(cnt, 0, &mut x as _, &mut y as _, &mut z as _);
            dbg!(Vector3::new(x, y, z));
            AUTDTransXDirection(cnt, 0, &mut x as _, &mut y as _, &mut z as _);
            dbg!(Vector3::new(x, y, z));
            AUTDTransYDirection(cnt, 0, &mut x as _, &mut y as _, &mut z as _);
            dbg!(Vector3::new(x, y, z));
            AUTDTransZDirection(cnt, 0, &mut x as _, &mut y as _, &mut z as _);
            dbg!(Vector3::new(x, y, z));

            let delay = 0xFFFF;
            AUTDSetTransModDelay(cnt, 0, delay);
            assert_eq!(delay, AUTDGetTransModDelay(cnt, 0));

            let firm_p = AUTDGetFirmwareInfoListPointer(cnt, err.as_mut_ptr());
            if firm_p == NULL {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            }
            for i in 0..num_devices {
                let mut info = vec![c_char::default(); 256];
                let mut is_valid = false;
                let mut is_supported = false;
                AUTDGetFirmwareInfo(
                    firm_p,
                    i as _,
                    info.as_mut_ptr(),
                    &mut is_valid as _,
                    &mut is_supported as _,
                );
                dbg!(CStr::from_ptr(info.as_ptr()).to_str().unwrap());
                dbg!(is_valid);
                dbg!(is_supported);
            }
            AUTDFreeFirmwareInfoListPointer(firm_p);

            {
                let g = AUTDGainNull();
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    std::ptr::null(),
                    g,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteGain(g);
            }

            {
                let g = AUTDGainGrouped();

                let g0 = AUTDGainNull();
                AUTDGainGroupedAdd(g, 0, g0);

                let g1 = AUTDGainNull();
                AUTDGainGroupedAdd(g, 1, g1);

                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    std::ptr::null(),
                    g,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteGain(g);
            }

            {
                let g = AUTDGainFocus(0., 0., 0., 1.);
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    std::ptr::null(),
                    g,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteGain(g);
            }

            {
                let g = AUTDGainBesselBeam(0., 0., 0., 0., 0., 1., 1., 1.);
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    std::ptr::null(),
                    g,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteGain(g);
            }

            {
                let g = AUTDGainPlaneWave(0., 0., 1., 1.);
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    std::ptr::null(),
                    g,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteGain(g);
            }

            {
                let g = AUTDGainTransducerTest();
                AUTDGainTransducerTestSet(g, 0, 1., 1.);
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    std::ptr::null(),
                    g,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteGain(g);
            }

            {
                let amp = vec![1.0; num_transducers as _];
                let phase = vec![0.0; num_transducers as _];
                let g = AUTDGainCustom(amp.as_ptr(), phase.as_ptr(), num_transducers as _);
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    std::ptr::null(),
                    g,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteGain(g);
            }

            {
                let m = AUTDModulationStatic(1.);
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    m,
                    std::ptr::null(),
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteModulation(m);
            }

            {
                let m = AUTDModulationSine(150, 1., 0.5);
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    m,
                    std::ptr::null(),
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteModulation(m);
            }

            {
                let m = AUTDModulationSineSquared(150, 1., 0.5);
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    m,
                    std::ptr::null(),
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteModulation(m);
            }

            {
                let m = AUTDModulationSineLegacy(150., 1., 0.5);
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    m,
                    std::ptr::null(),
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteModulation(m);
            }

            {
                let m = AUTDModulationSquare(150, 0., 1., 0.5);
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    m,
                    std::ptr::null(),
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteModulation(m);
            }

            {
                let amp = vec![1.0; 10];
                let m = AUTDModulationCustom(amp.as_ptr(), amp.len() as _, 5000);
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    m,
                    std::ptr::null(),
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteModulation(m);
            }

            {
                let m = AUTDModulationStatic(1.);
                let div = 1000;
                AUTDModulationSetSamplingFrequencyDivision(m, div);
                assert_eq!(div, AUTDModulationSamplingFrequencyDivision(m));
                dbg!(AUTDModulationSamplingFrequency(m));
                AUTDDeleteModulation(m);
            }

            {
                let stm = AUTDFocusSTM();
                AUTDFocusSTMAdd(stm, 0., 0., 0., 0);
                AUTDFocusSTMAdd(stm, 0., 0., 0., 0);

                let freq = AUTDFocusSTMSetFrequency(stm, 1.);
                assert_eq!(freq, AUTDFocusSTMFrequency(stm));

                let div = 1000;
                AUTDFocusSTMSetSamplingFrequencyDivision(stm, div);
                assert_eq!(div, AUTDFocusSTMSamplingFrequencyDivision(stm));
                dbg!(AUTDFocusSTMSamplingFrequency(stm));

                let start_idx = 1;
                let finish_idx = 0;
                AUTDFocusSTMSetStartIdx(stm, start_idx);
                AUTDFocusSTMSetFinishIdx(stm, finish_idx);
                assert_eq!(start_idx, AUTDFocusSTMGetStartIdx(stm));
                assert_eq!(finish_idx, AUTDFocusSTMGetFinishIdx(stm));

                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    std::ptr::null(),
                    stm,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteFocusSTM(stm);
            }

            {
                let stm = AUTDGainSTM();
                let g0 = AUTDGainNull();
                let g1 = AUTDGainNull();
                AUTDGainSTMAdd(stm, g0);
                AUTDGainSTMAdd(stm, g1);

                let freq = AUTDGainSTMSetFrequency(stm, 1.);
                assert_eq!(freq, AUTDGainSTMFrequency(stm));

                let div = 1000;
                AUTDGainSTMSetSamplingFrequencyDivision(stm, div);
                assert_eq!(div, AUTDGainSTMSamplingFrequencyDivision(stm));
                dbg!(AUTDGainSTMSamplingFrequency(stm));

                let start_idx = 1;
                let finish_idx = 0;
                AUTDGainSTMSetStartIdx(stm, start_idx);
                AUTDGainSTMSetFinishIdx(stm, finish_idx);
                assert_eq!(start_idx, AUTDGainSTMGetStartIdx(stm));
                assert_eq!(finish_idx, AUTDGainSTMGetFinishIdx(stm));

                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    std::ptr::null(),
                    stm,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteGainSTM(stm);
            }

            {
                let s = AUTDSynchronize();

                if AUTDSendSpecial(cnt, TransMode::Legacy, s, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteSpecialData(s);
            }

            {
                let s = AUTDClear();
                if AUTDSendSpecial(cnt, TransMode::Legacy, s, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteSpecialData(s);
            }

            {
                let s = AUTDUpdateFlags();
                if AUTDSendSpecial(cnt, TransMode::Legacy, s, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteSpecialData(s);
            }

            {
                let s = AUTDStop();
                if AUTDSendSpecial(cnt, TransMode::Legacy, s, -1, err.as_mut_ptr()) == ERR {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteSpecialData(s);
            }

            {
                let s = AUTDModDelayConfig();
                if AUTDSendSpecial(cnt, TransMode::Legacy, s, -1, err.as_mut_ptr()) == ERR {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteSpecialData(s);
            }

            {
                let s = AUTDCreateSilencer(10);
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    s,
                    std::ptr::null(),
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteSilencer(s);
            }

            {
                let s = AUTDCreateAmplitudes(1.);
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    s,
                    std::ptr::null(),
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteAmplitudes(s);
            }

            if !AUTDClose(cnt, err.as_mut_ptr()) {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            }

            AUTDFreeController(cnt);
        }
    }
}
