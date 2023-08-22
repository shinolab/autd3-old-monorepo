/*
 * File: lib.rs
 * Project: src
 * Created Date: 11/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

pub mod gain;
pub mod modulation;

use autd3_core::stm::STMProps;
use autd3capi_def::{
    common::{
        autd3::{
            link::{log::LogImpl, Log},
        },
        *,
    },
    take_link, ControllerPtr, DatagramBodyPtr, DatagramHeaderPtr, DatagramSpecialPtr, GainPtr,
    GainSTMMode, GeometryPtr, Level, LinkPtr, STMPropsPtr, TransMode, AUTD3_ERR,
    AUTD3_FALSE, AUTD3_TRUE,
};
use std::{
    ffi::c_char,
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ControllerBuilderPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Drive {
    pub phase: float,
    pub amp: float,
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDCreateControllerBuilder() -> ControllerBuilderPtr {
    ControllerBuilderPtr(
        Box::into_raw(Box::new(ControllerBuilder::<DynamicTransducer>::new())) as _,
    )
}

#[no_mangle]
pub unsafe extern "C" fn AUTDAddDevice(
    builder: ControllerBuilderPtr,
    x: float,
    y: float,
    z: float,
    rz1: float,
    ry: float,
    rz2: float,
) -> ControllerBuilderPtr {
    ControllerBuilderPtr(Box::into_raw(Box::new(
        Box::from_raw(builder.0 as *mut ControllerBuilder<DynamicTransducer>).add_device(
            AUTD3::new(Vector3::new(x, y, z), Vector3::new(rz1, ry, rz2)),
        ),
    )) as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDAddDeviceQuaternion(
    builder: ControllerBuilderPtr,
    x: float,
    y: float,
    z: float,
    qw: float,
    qx: float,
    qy: float,
    qz: float,
) -> ControllerBuilderPtr {
    ControllerBuilderPtr(Box::into_raw(Box::new(
        Box::from_raw(builder.0 as *mut ControllerBuilder<DynamicTransducer>).add_device(
            AUTD3::with_quaternion(
                Vector3::new(x, y, z),
                UnitQuaternion::from_quaternion(Quaternion::new(qw, qx, qy, qz)),
            ),
        ),
    )) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerOpenWith(
    builder: ControllerBuilderPtr,
    link: LinkPtr,
    err: *mut c_char,
) -> ControllerPtr {
    let link: Box<Box<L>> = Box::from_raw(link.0 as *mut Box<L>);
    let cnt = try_or_return!(
        Box::from_raw(builder.0 as *mut ControllerBuilder<DynamicTransducer>).open_with(*link),
        err,
        ControllerPtr(NULL)
    );
    ControllerPtr(Box::into_raw(Box::new(cnt)) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDClose(cnt: ControllerPtr, err: *mut c_char) -> bool {
    try_or_return!(cast_mut!(cnt.0, Cnt).close(), err, false);
    true
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFreeController(cnt: ControllerPtr) {
    let mut cnt = Box::from_raw(cnt.0 as *mut Cnt);
    let _ = cnt.close();
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetReadsFPGAInfo(cnt: ControllerPtr, value: bool) {
    cast_mut!(cnt.0, Cnt).reads_fpga_info(value)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetForceFan(cnt: ControllerPtr, value: bool) {
    cast_mut!(cnt.0, Cnt).force_fan(value)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetGeometry(cnt: ControllerPtr) -> GeometryPtr {
    GeometryPtr(cast!(cnt.0, Geo) as *const _ as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetSoundSpeed(geo: GeometryPtr) -> float {
    cast!(geo.0, Geo).sound_speed
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetSoundSpeed(geo: GeometryPtr, value: float) {
    cast_mut!(geo.0, Geo).sound_speed = value;
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetSoundSpeedFromTemp(
    geo: GeometryPtr,
    temp: float,
    k: float,
    r: float,
    m: float,
) {
    cast_mut!(geo.0, Geo).set_sound_speed_from_temp_with(temp, k, r, m);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransFrequency(geo: GeometryPtr, idx: u32) -> float {
    cast!(geo.0, Geo)[idx as usize].frequency()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSetTransFrequency(
    geo: GeometryPtr,
    idx: u32,
    value: float,
    err: *mut c_char,
) -> bool {
    try_or_return!(
        cast_mut!(geo.0, Geo)[idx as usize].set_frequency(value),
        err,
        false
    );
    true
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransCycle(geo: GeometryPtr, idx: u32) -> u16 {
    cast!(geo.0, Geo)[idx as usize].cycle()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSetTransCycle(
    geo: GeometryPtr,
    idx: u32,
    value: u16,
    err: *mut c_char,
) -> bool {
    try_or_return!(
        cast_mut!(geo.0, Geo)[idx as usize].set_cycle(value),
        err,
        false
    );
    true
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetWavelength(
    geo: GeometryPtr,
    idx: u32,
    sound_speed: float,
) -> float {
    let geometry = cast!(geo.0, Geo);
    geometry[idx as usize].wavelength(sound_speed)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetAttenuation(geo: GeometryPtr) -> float {
    cast!(geo.0, Geo).attenuation
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetAttenuation(geo: GeometryPtr, value: float) {
    cast_mut!(geo.0, Geo).attenuation = value;
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDNumTransducers(geo: GeometryPtr) -> u32 {
    cast!(geo.0, Geo).num_transducers() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDNumDevices(geo: GeometryPtr) -> u32 {
    cast!(geo.0, Geo).num_devices() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryCenter(geo: GeometryPtr, center: *mut float) {
    let c = cast!(geo.0, Geo).center();
    center.add(0).write(c.x);
    center.add(1).write(c.y);
    center.add(2).write(c.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryCenterOf(geo: GeometryPtr, dev_idx: u32, center: *mut float) {
    let c = cast!(geo.0, Geo).center_of(dev_idx as usize);
    center.add(0).write(c.x);
    center.add(1).write(c.y);
    center.add(2).write(c.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransPosition(geo: GeometryPtr, tr_idx: u32, pos: *mut float) {
    let p = cast!(geo.0, Geo)[tr_idx as usize].position();
    pos.add(0).write(p.x);
    pos.add(1).write(p.y);
    pos.add(2).write(p.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransRotation(geo: GeometryPtr, tr_idx: u32, rot: *mut float) {
    let r = cast!(geo.0, Geo)[tr_idx as usize].rotation();
    rot.add(0).write(r.w);
    rot.add(1).write(r.i);
    rot.add(2).write(r.j);
    rot.add(3).write(r.k);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransXDirection(geo: GeometryPtr, tr_idx: u32, dir: *mut float) {
    let d = cast!(geo.0, Geo)[tr_idx as usize].x_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransYDirection(geo: GeometryPtr, tr_idx: u32, dir: *mut float) {
    let d = cast!(geo.0, Geo)[tr_idx as usize].y_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransZDirection(geo: GeometryPtr, tr_idx: u32, dir: *mut float) {
    let d = cast!(geo.0, Geo)[tr_idx as usize].z_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransModDelay(geo: GeometryPtr, tr_idx: u32) -> u16 {
    cast!(geo.0, Geo)[tr_idx as usize].mod_delay()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetTransModDelay(geo: GeometryPtr, tr_idx: u32, delay: u16) {
    cast_mut!(geo.0, Geo)[tr_idx as usize].set_mod_delay(delay)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetFPGAInfo(
    cnt: ControllerPtr,
    out: *mut u8,
    err: *mut c_char,
) -> bool {
    let fpga_info = try_or_return!(cast_mut!(cnt.0, Cnt).fpga_info(), err, false);
    std::ptr::copy_nonoverlapping(fpga_info.as_ptr() as _, out, fpga_info.len());
    true
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FirmwareInfoListPtr(pub ConstPtr);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetFirmwareInfoListPointer(
    cnt: ControllerPtr,
    err: *mut c_char,
) -> FirmwareInfoListPtr {
    let firmware_infos = try_or_return!(
        cast_mut!(cnt.0, Cnt).firmware_infos(),
        err,
        FirmwareInfoListPtr(NULL)
    );
    FirmwareInfoListPtr(Box::into_raw(Box::new(firmware_infos)) as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGetFirmwareInfo(
    p_info_list: FirmwareInfoListPtr,
    idx: u32,
    info: *mut c_char,
    props: *mut bool,
) {
    let firm_info = &cast_mut!(p_info_list.0, Vec<FirmwareInfo>)[idx as usize];
    let info_str = std::ffi::CString::new(firm_info.to_string()).unwrap();
    libc::strcpy(info, info_str.as_ptr());
    props.add(0).write(firm_info.is_valid());
    props.add(1).write(firm_info.is_supported());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFreeFirmwareInfoListPointer(p_info_list: FirmwareInfoListPtr) {
    let _ = Box::from_raw(p_info_list.0 as *mut Vec<FirmwareInfo>);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGetLatestFirmware(latest: *mut c_char) {
    let info_str = std::ffi::CString::new(FirmwareInfo::latest_version()).unwrap();
    libc::strcpy(latest, info_str.as_ptr());
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMProps(freq: float) -> STMPropsPtr {
    STMPropsPtr::new(STMProps::new(freq))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsWithSamplingFreq(freq: float) -> STMPropsPtr {
    STMPropsPtr::new(STMProps::with_sampling_frequency(freq))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsWithSamplingFreqDiv(div: u32) -> STMPropsPtr {
    STMPropsPtr::new(STMProps::with_sampling_frequency_division(div))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsWithStartIdx(props: STMPropsPtr, idx: i32) -> STMPropsPtr {
    let props = Box::from_raw(props.0 as *mut STMProps);
    STMPropsPtr::new(if idx < 0 {
        props.with_start_idx(None)
    } else {
        props.with_start_idx(Some(idx as u16))
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsWithFinishIdx(props: STMPropsPtr, idx: i32) -> STMPropsPtr {
    let props = Box::from_raw(props.0 as *mut STMProps);
    STMPropsPtr::new(if idx < 0 {
        props.with_finish_idx(None)
    } else {
        props.with_finish_idx(Some(idx as u16))
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsFrequency(props: STMPropsPtr, size: u64) -> float {
    Box::from_raw(props.0 as *mut STMProps).freq(size as usize)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsSamplingFrequency(props: STMPropsPtr, size: u64) -> float {
    Box::from_raw(props.0 as *mut STMProps).sampling_frequency(size as usize)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsSamplingFrequencyDivision(
    props: STMPropsPtr,
    size: u64,
) -> u32 {
    Box::from_raw(props.0 as *mut STMProps).sampling_frequency_division(size as usize)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsStartIdx(props: STMPropsPtr) -> i32 {
    if let Some(idx) = cast!(props.0, STMProps).start_idx() {
        idx as i32
    } else {
        -1
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsFinishIdx(props: STMPropsPtr) -> i32 {
    if let Some(idx) = cast!(props.0, STMProps).finish_idx() {
        idx as i32
    } else {
        -1
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDFocusSTM(
    props: STMPropsPtr,
    points: *const float,
    shift: *const u8,
    size: u64,
) -> DatagramBodyPtr {
    DatagramBodyPtr::new(
        FocusSTM::with_props(*Box::from_raw(props.0 as *mut STMProps)).add_foci_from_iter(
            (0..size as usize).map(|i| {
                let p = Vector3::new(
                    points.add(i * 3).read(),
                    points.add(i * 3 + 1).read(),
                    points.add(i * 3 + 2).read(),
                );
                let shift = *shift.add(i);
                (p, shift)
            }),
        ),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSTMWithMode(
    props: STMPropsPtr,
    mode: GainSTMMode,
) -> DatagramBodyPtr {
    DatagramBodyPtr::new(
        GainSTM::<DynamicTransducer, Box<dyn Gain<DynamicTransducer>>>::with_props_mode(
            *Box::from_raw(props.0 as *mut STMProps),
            mode.into(),
        ),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSTM(props: STMPropsPtr) -> DatagramBodyPtr {
    AUTDGainSTMWithMode(props, GainSTMMode::PhaseDutyFull)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSTMAddGain(
    stm: DatagramBodyPtr,
    gain: GainPtr,
) -> DatagramBodyPtr {
    DatagramBodyPtr::new(
        Box::from_raw(stm.0 as *mut Box<GainSTM<DynamicTransducer, _>>)
            .add_gain(*Box::from_raw(gain.0 as *mut Box<G>)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSynchronize() -> DatagramSpecialPtr {
    DatagramSpecialPtr::new(Synchronize::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDClear() -> DatagramSpecialPtr {
    DatagramSpecialPtr::new(Clear::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDUpdateFlags() -> DatagramSpecialPtr {
    DatagramSpecialPtr::new(UpdateFlags::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDStop() -> DatagramSpecialPtr {
    DatagramSpecialPtr::new(Stop::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModDelayConfig() -> DatagramSpecialPtr {
    DatagramSpecialPtr::new(ModDelay::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDCreateSilencer(step: u16) -> DatagramHeaderPtr {
    DatagramHeaderPtr::new(SilencerConfig::new(step))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDCreateAmplitudes(amp: float) -> DatagramBodyPtr {
    DatagramBodyPtr::new(Amplitudes::uniform(amp))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSend(
    cnt: ControllerPtr,
    mode: TransMode,
    header: DatagramHeaderPtr,
    body: DatagramBodyPtr,
    timeout_ns: i64,
    err: *mut c_char,
) -> i32 {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    let mode = mode.into();
    let res = if !header.0.is_null() && !body.0.is_null() {
        let header = Box::from_raw(header.0 as *mut Box<dyn DynamicDatagram>);
        let body = Box::from_raw(body.0 as *mut Box<dyn DynamicDatagram>);
        try_or_return!(
            cast_mut!(cnt.0, Cnt).send((mode, header, body, timeout)),
            err,
            AUTD3_ERR
        )
    } else if !header.0.is_null() {
        let header = Box::from_raw(header.0 as *mut Box<dyn DynamicDatagram>);
        try_or_return!(
            cast_mut!(cnt.0, Cnt).send((mode, header, timeout)),
            err,
            AUTD3_ERR
        )
    } else if !body.0.is_null() {
        let body = Box::from_raw(body.0 as *mut Box<dyn DynamicDatagram>);
        try_or_return!(
            cast_mut!(cnt.0, Cnt).send((mode, body, timeout)),
            err,
            AUTD3_ERR
        )
    } else {
        return AUTD3_FALSE;
    };
    if res {
        AUTD3_TRUE
    } else {
        AUTD3_FALSE
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSendSpecial(
    cnt: ControllerPtr,
    mode: TransMode,
    special: DatagramSpecialPtr,
    timeout_ns: i64,
    err: *mut c_char,
) -> i32 {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    let mode = mode.into();
    let special = Box::from_raw(special.0 as *mut Box<dyn DynamicDatagram>);
    if try_or_return!(
        cast_mut!(cnt.0, Cnt).send((mode, special, timeout)),
        err,
        AUTD3_ERR
    ) {
        AUTD3_TRUE
    } else {
        AUTD3_FALSE
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkDebug() -> LinkPtr {
    LinkPtr::new(Debug::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkDebugWithLogLevel(debug: LinkPtr, level: Level) -> LinkPtr {
    LinkPtr::new(take_link!(debug, Debug).with_log_level(level.into()))
}

struct CallbackPtr(ConstPtr);
unsafe impl Send for CallbackPtr {}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkDebugWithLogFunc(
    debug: LinkPtr,
    out_func: ConstPtr,
    flush_func: ConstPtr,
) -> LinkPtr {
    if out_func.is_null() || flush_func.is_null() {
        return debug;
    }

    let out_f = Arc::new(Mutex::new(CallbackPtr(out_func)));
    let out_func = move |msg: &str| -> spdlog::Result<()> {
        let msg = std::ffi::CString::new(msg).unwrap();
        let out_f =
            std::mem::transmute::<_, unsafe extern "C" fn(*const c_char)>(out_f.lock().unwrap().0);
        out_f(msg.as_ptr());
        Ok(())
    };
    let flush_f = Arc::new(Mutex::new(CallbackPtr(flush_func)));
    let flush_func = move || -> spdlog::Result<()> {
        let flush_f = std::mem::transmute::<_, unsafe extern "C" fn()>(flush_f.lock().unwrap().0);
        flush_f();
        Ok(())
    };

    LinkPtr::new(
        take_link!(debug, Debug).with_logger(get_logger_with_custom_func(out_func, flush_func)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkDebugWithTimeout(debug: LinkPtr, timeout_ns: u64) -> LinkPtr {
    LinkPtr::new(take_link!(debug, Debug).with_timeout(Duration::from_nanos(timeout_ns)))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkLog(link: LinkPtr) -> LinkPtr {
    let link: Box<Box<L>> = Box::from_raw(link.0 as *mut Box<L>);
    LinkPtr::new(link.with_log())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkLogWithLogLevel(log: LinkPtr, level: Level) -> LinkPtr {
    LinkPtr::new(take_link!(log, LogImpl<DynamicTransducer, Box<L>>).with_log_level(level.into()))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkLogWithLogFunc(
    log: LinkPtr,
    out_func: ConstPtr,
    flush_func: ConstPtr,
) -> LinkPtr {
    if out_func.is_null() || flush_func.is_null() {
        return log;
    }

    let out_f = Arc::new(Mutex::new(CallbackPtr(out_func)));
    let out_func = move |msg: &str| -> spdlog::Result<()> {
        let msg = std::ffi::CString::new(msg).unwrap();
        let out_f =
            std::mem::transmute::<_, unsafe extern "C" fn(*const c_char)>(out_f.lock().unwrap().0);
        out_f(msg.as_ptr());
        Ok(())
    };
    let flush_f = Arc::new(Mutex::new(CallbackPtr(flush_func)));
    let flush_func = move || -> spdlog::Result<()> {
        let flush_f = std::mem::transmute::<_, unsafe extern "C" fn()>(flush_f.lock().unwrap().0);
        flush_f();
        Ok(())
    };

    LinkPtr::new(
        take_link!(log, LogImpl<DynamicTransducer, Box<L>>)
            .with_logger(get_logger_with_custom_func(out_func, flush_func)),
    )
}

#[cfg(test)]
mod tests {
    use autd3capi_def::DatagramHeaderPtr;

    use super::*;

    use crate::gain::null::*;

    use std::ffi::CStr;

    pub unsafe fn make_debug_link() -> LinkPtr {
        let debug = AUTDLinkDebug();
        let debug = AUTDLinkDebugWithLogLevel(debug, Level::Off);
        AUTDLinkDebugWithTimeout(debug, 0)
    }

    pub unsafe fn create_controller() -> ControllerPtr {
        let builder = AUTDCreateControllerBuilder();
        let builder = AUTDAddDevice(builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let builder = AUTDAddDeviceQuaternion(builder, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

        let link = make_debug_link();
        let mut err = vec![c_char::default(); 256];
        let cnt = AUTDControllerOpenWith(builder, link, err.as_mut_ptr());
        assert_ne!(cnt.0, NULL);
        cnt
    }

    #[test]
    fn drive_test() {
        assert_eq!(
            std::mem::size_of::<autd3_core::Drive>(),
            std::mem::size_of::<Drive>()
        );
        assert_eq!(
            memoffset::offset_of!(autd3_core::Drive, phase),
            memoffset::offset_of!(Drive, phase)
        );
        assert_eq!(
            memoffset::offset_of!(autd3_core::Drive, amp),
            memoffset::offset_of!(Drive, amp)
        );
    }

    #[test]
    fn basic() {
        unsafe {
            let builder = AUTDCreateControllerBuilder();
            let builder = AUTDAddDevice(builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            let builder = AUTDAddDeviceQuaternion(builder, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

            let link = make_debug_link();
            let mut err = vec![c_char::default(); 256];
            let cnt = AUTDControllerOpenWith(builder, link, err.as_mut_ptr());
            if cnt.0 == NULL {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            }

            AUTDSetReadsFPGAInfo(cnt, true);
            AUTDSetForceFan(cnt, true);

            let geo = AUTDGetGeometry(cnt);

            let c = 300e3;
            AUTDSetSoundSpeed(geo, c);
            assert_eq!(c, AUTDGetSoundSpeed(geo));

            AUTDSetSoundSpeedFromTemp(geo, 15.0, 1.4, 8.314_463, 28.9647e-3);
            dbg!(AUTDGetSoundSpeed(geo));

            let f = 70e3;
            if !AUTDSetTransFrequency(geo, 0, f, err.as_mut_ptr()) {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            }
            dbg!(AUTDGetTransFrequency(geo, 0));

            let f = 4096;
            if !AUTDSetTransCycle(geo, 0, f, err.as_mut_ptr()) {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            }
            dbg!(AUTDGetTransCycle(geo, 0));

            dbg!(AUTDGetWavelength(geo, 0, c));

            let atten = 0.1;
            AUTDSetAttenuation(geo, atten);
            dbg!(AUTDGetAttenuation(geo));

            let num_transducers = AUTDNumTransducers(geo);
            dbg!(num_transducers);
            let num_devices = AUTDNumDevices(geo) as usize;
            dbg!(num_devices);

            let mut fpga_info = vec![0xFFu8; num_devices];
            if !AUTDGetFPGAInfo(cnt, fpga_info.as_mut_ptr() as _, err.as_mut_ptr()) {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            }
            dbg!(fpga_info);

            let mut v = [0., 0., 0.];
            AUTDGeometryCenter(geo, v.as_mut_ptr());
            dbg!(&v);
            AUTDGeometryCenterOf(geo, 0, v.as_mut_ptr());
            dbg!(&v);

            AUTDTransPosition(geo, 0, v.as_mut_ptr());
            dbg!(&v);
            AUTDTransXDirection(geo, 0, v.as_mut_ptr());
            dbg!(&v);
            AUTDTransYDirection(geo, 0, v.as_mut_ptr());
            dbg!(&v);
            AUTDTransZDirection(geo, 0, v.as_mut_ptr());
            dbg!(&v);

            let delay = 0xFFFF;
            AUTDSetTransModDelay(geo, 0, delay);
            assert_eq!(delay, AUTDGetTransModDelay(geo, 0));

            let firm_p = AUTDGetFirmwareInfoListPointer(cnt, err.as_mut_ptr());
            if firm_p.0 == NULL {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            }
            (0..num_devices).for_each(|i| {
                let mut info = vec![c_char::default(); 256];
                let mut props = [false, false];
                AUTDGetFirmwareInfo(firm_p, i as _, info.as_mut_ptr(), props.as_mut_ptr());
                dbg!(CStr::from_ptr(info.as_ptr()).to_str().unwrap());
                dbg!(props);
            });
            AUTDFreeFirmwareInfoListPointer(firm_p);

            {
                let props = AUTDSTMProps(1.);
                assert_eq!(1., AUTDSTMPropsFrequency(props, 0));

                let props = AUTDSTMPropsWithSamplingFreq(1.);
                assert_eq!(1., AUTDSTMPropsSamplingFrequency(props, 0));

                let props = AUTDSTMPropsWithSamplingFreqDiv(512);
                assert_eq!(512, AUTDSTMPropsSamplingFrequencyDivision(props, 0));
            }

            {
                let props = AUTDSTMProps(1.);
                let props = AUTDSTMPropsWithStartIdx(props, 0);
                assert_eq!(0, AUTDSTMPropsStartIdx(props));
                let props = AUTDSTMPropsWithFinishIdx(props, 1);
                assert_eq!(1, AUTDSTMPropsFinishIdx(props));

                let len = 2;
                let points = vec![0.; len * 3];
                let shifts = vec![0; len];

                let stm = AUTDFocusSTM(props, points.as_ptr(), shifts.as_ptr(), len as _);

                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    stm,
                    -1,
                    err.as_mut_ptr(),
                ) == AUTD3_ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            {
                let props = AUTDSTMProps(1.);

                let g0 = AUTDGainNull();
                let g1 = AUTDGainNull();

                let stm = AUTDGainSTM(props);
                let stm = AUTDGainSTMAddGain(stm, g0);
                let stm = AUTDGainSTMAddGain(stm, g1);

                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    stm,
                    -1,
                    err.as_mut_ptr(),
                ) == AUTD3_ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            {
                let s = AUTDSynchronize();

                if AUTDSendSpecial(cnt, TransMode::Legacy, s, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            {
                let s = AUTDClear();
                if AUTDSendSpecial(cnt, TransMode::Legacy, s, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            {
                let s = AUTDUpdateFlags();
                if AUTDSendSpecial(cnt, TransMode::Legacy, s, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            {
                let s = AUTDStop();
                if AUTDSendSpecial(cnt, TransMode::Legacy, s, -1, err.as_mut_ptr()) == AUTD3_ERR {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            {
                let s = AUTDModDelayConfig();
                if AUTDSendSpecial(cnt, TransMode::Legacy, s, -1, err.as_mut_ptr()) == AUTD3_ERR {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            {
                let s = AUTDCreateSilencer(10);
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    s,
                    DatagramBodyPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr(),
                ) == AUTD3_ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            {
                let s = AUTDCreateAmplitudes(1.);
                if AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    s,
                    -1,
                    err.as_mut_ptr(),
                ) == AUTD3_ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            if !AUTDClose(cnt, err.as_mut_ptr()) {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            }

            AUTDFreeController(cnt);
        }
    }
}
