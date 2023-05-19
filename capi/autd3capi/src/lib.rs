#![allow(clippy::missing_safety_doc)]

mod custom;

use custom::{CustomGain, CustomModulation};

use std::{
    ffi::{c_char, c_void},
    sync::{Arc, Mutex},
    time::Duration,
};

use autd3capi_common::*;

#[no_mangle]
pub unsafe extern "C" fn AUTDCreateGeometryBuilder(out: *mut ConstPtr) {
    unsafe {
        *out = Box::into_raw(Box::new(GeometryBuilder::<DynamicTransducer>::new())) as *mut c_void;
    }
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
        (builder as *mut GeometryBuilder<DynamicTransducer>)
            .as_mut()
            .unwrap()
            .add_device(AUTD3::new(
                Vector3::new(x, y, z),
                Vector3::new(rz1, ry, rz2),
            ));
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
        (builder as *mut GeometryBuilder<DynamicTransducer>)
            .as_mut()
            .unwrap()
            .add_device(AUTD3::new_with_quaternion(
                Vector3::new(x, y, z),
                UnitQuaternion::from_quaternion(Quaternion::new(qw, qx, qy, qz)),
            ));
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDBuildGeometry(
    out: *mut ConstPtr,
    builder: *const c_void,
    err: *mut c_char,
) -> i32 {
    unsafe {
        let geometry = try_or_return!(
            Box::from_raw(builder as *mut GeometryBuilder<DynamicTransducer>).build(),
            err
        );
        *out = Box::into_raw(Box::new(geometry)) as *mut c_void;
    }
    OK
}

#[no_mangle]
pub unsafe extern "C" fn AUTDOpenController(
    out: *mut ConstPtr,
    geometry: ConstPtr,
    link: ConstPtr,
    err: *mut c_char,
) -> i32 {
    unsafe {
        let link: Box<Box<L>> = Box::from_raw(link as *mut _);
        let link = DynamicLink::new(*link);
        let geometry: Box<Geometry<DynamicTransducer>> = Box::from_raw(geometry as *mut _);
        let cnt = try_or_return!(Controller::open(*geometry, link), err);
        *out = Box::into_raw(Box::new(cnt)) as _;
    }
    OK
}

#[no_mangle]
pub unsafe extern "C" fn AUTDClose(cnt: ConstPtr, err: *mut c_char) -> i32 {
    unsafe {
        try_or_return!((cnt as *mut Cnt).as_mut().unwrap().close(), err);
    }
    OK
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFreeController(cnt: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(cnt as *mut Cnt);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetReadsFPGAInfo(cnt: ConstPtr, value: bool) {
    unsafe { (cnt as *mut Cnt).as_mut().unwrap().reads_fpga_info(value) }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetForceFan(cnt: ConstPtr, value: bool) {
    unsafe { (cnt as *mut Cnt).as_mut().unwrap().force_fan(value) }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGetSoundSpeed(cnt: ConstPtr) -> float {
    unsafe { (cnt as *mut Cnt).as_mut().unwrap().geometry().sound_speed }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetSoundSpeed(cnt: ConstPtr, value: float) {
    unsafe {
        (cnt as *mut Cnt)
            .as_mut()
            .unwrap()
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
        (cnt as *mut Cnt)
            .as_mut()
            .unwrap()
            .geometry_mut()
            .set_sound_speed_from_temp_with(temp, k, r, m);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGetTransFrequency(cnt: ConstPtr, idx: i32) -> float {
    unsafe { (cnt as *mut Cnt).as_mut().unwrap().geometry()[idx as _].frequency() }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetTransFrequency(
    cnt: ConstPtr,
    idx: i32,
    value: float,
    err: *mut c_char,
) -> i32 {
    unsafe {
        try_or_return!(
            (cnt as *mut Cnt).as_mut().unwrap().geometry_mut()[idx as _].set_frequency(value),
            err
        )
    }
    OK
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGetTransCycle(cnt: ConstPtr, idx: i32) -> u16 {
    unsafe { (cnt as *mut Cnt).as_mut().unwrap().geometry()[idx as _].cycle() }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetTransCycle(
    cnt: ConstPtr,
    idx: i32,
    value: u16,
    err: *mut c_char,
) -> i32 {
    unsafe {
        try_or_return!(
            (cnt as *mut Cnt).as_mut().unwrap().geometry_mut()[idx as _].set_cycle(value),
            err
        )
    }
    OK
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGetWavelength(cnt: ConstPtr, idx: i32) -> float {
    unsafe {
        let geometry = (cnt as *mut Cnt).as_mut().unwrap().geometry();
        geometry[idx as _].wavelength(geometry.sound_speed)
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGetAttenuation(cnt: ConstPtr) -> float {
    unsafe { (cnt as *mut Cnt).as_mut().unwrap().geometry().attenuation }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetAttenuation(cnt: ConstPtr, value: float) {
    unsafe {
        (cnt as *mut Cnt)
            .as_mut()
            .unwrap()
            .geometry_mut()
            .attenuation = value;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGetFPGAInfo(cnt: ConstPtr, out: *mut u8, err: *mut c_char) -> i32 {
    unsafe {
        let fpga_info = try_or_return!((cnt as *mut Cnt).as_mut().unwrap().fpga_info(), err);
        std::ptr::copy_nonoverlapping(fpga_info.as_ptr(), out, fpga_info.len());
    }
    OK
}

#[no_mangle]
pub unsafe extern "C" fn AUTDNumTransducers(cnt: ConstPtr) -> i32 {
    unsafe {
        (cnt as *mut Cnt)
            .as_mut()
            .unwrap()
            .geometry()
            .num_transducers() as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDNumDevices(cnt: ConstPtr) -> i32 {
    unsafe { (cnt as *mut Cnt).as_mut().unwrap().geometry().num_devices() as _ }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryCenter(
    cnt: ConstPtr,
    x: *mut float,
    y: *mut float,
    z: *mut float,
) {
    unsafe {
        let center = (cnt as *mut Cnt).as_mut().unwrap().geometry().center();
        *x = center.x;
        *y = center.y;
        *z = center.z;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryCenterOf(
    cnt: ConstPtr,
    dev_idx: i32,
    x: *mut float,
    y: *mut float,
    z: *mut float,
) {
    unsafe {
        let center = (cnt as *const Cnt)
            .as_ref()
            .unwrap()
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
    tr_idx: i32,
    x: *mut float,
    y: *mut float,
    z: *mut float,
) {
    unsafe {
        let pos = (cnt as *const Cnt).as_ref().unwrap().geometry()[tr_idx as _].position();
        *x = pos.x;
        *y = pos.y;
        *z = pos.z;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransXDirection(
    cnt: ConstPtr,
    tr_idx: i32,
    x: *mut float,
    y: *mut float,
    z: *mut float,
) {
    unsafe {
        let dir = (cnt as *const Cnt).as_ref().unwrap().geometry()[tr_idx as _].x_direction();
        *x = dir.x;
        *y = dir.y;
        *z = dir.z;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransYDirection(
    cnt: ConstPtr,
    tr_idx: i32,
    x: *mut float,
    y: *mut float,
    z: *mut float,
) {
    unsafe {
        let dir = (cnt as *const Cnt).as_ref().unwrap().geometry()[tr_idx as _].y_direction();
        *x = dir.x;
        *y = dir.y;
        *z = dir.z;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransZDirection(
    cnt: ConstPtr,
    tr_idx: i32,
    x: *mut float,
    y: *mut float,
    z: *mut float,
) {
    unsafe {
        let dir = (cnt as *const Cnt).as_ref().unwrap().geometry()[tr_idx as _].z_direction();
        *x = dir.x;
        *y = dir.y;
        *z = dir.z;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGetTransModDelay(cnt: ConstPtr, tr_idx: i32) -> u16 {
    unsafe { (cnt as *mut Cnt).as_mut().unwrap().geometry()[tr_idx as _].mod_delay() }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetTransModDelay(cnt: ConstPtr, tr_idx: i32, delay: u16) {
    unsafe { (cnt as *mut Cnt).as_mut().unwrap().geometry_mut()[tr_idx as _].set_mod_delay(delay) }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGetFirmwareInfoListPointer(
    cnt: ConstPtr,
    out: *mut ConstPtr,
    err: *mut c_char,
) -> i32 {
    unsafe {
        let firmware_infos =
            try_or_return!((cnt as *mut Cnt).as_mut().unwrap().firmware_infos(), err);
        let len = firmware_infos.len() as _;
        *out = Box::into_raw(Box::new(firmware_infos)) as _;
        len
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGetFirmwareInfo(
    p_info_list: ConstPtr,
    idx: i32,
    info: *mut c_char,
    is_valid: *mut bool,
    is_supported: *mut bool,
) {
    unsafe {
        let firm_info = &(p_info_list as *mut Vec<FirmwareInfo>).as_ref().unwrap()[idx as usize];
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
pub unsafe extern "C" fn AUTDGainNull(out: *mut ConstPtr) {
    unsafe {
        let gain: Box<Box<G>> = Box::new(Box::new(Null::new()));
        *out = Box::into_raw(gain) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainGrouped(out: *mut ConstPtr) {
    unsafe {
        let gain: Box<Box<G>> = Box::new(Box::new(Grouped::new()));
        *out = Box::into_raw(gain) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainGroupedAdd(
    grouped_gain: ConstPtr,
    device_id: i32,
    gain: ConstPtr,
) {
    unsafe {
        let g = Box::from_raw(gain as *mut Box<G> as *mut Box<dyn Gain<DynamicTransducer>>);
        (grouped_gain as *mut Box<G> as *mut Box<Grouped<DynamicTransducer>>)
            .as_mut()
            .unwrap()
            .add_boxed(device_id as _, *g);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainFocus(
    out: *mut ConstPtr,
    x: float,
    y: float,
    z: float,
    amp: float,
) {
    unsafe {
        let gain: Box<Box<G>> = Box::new(Box::new(Focus::with_amp(Vector3::new(x, y, z), amp)));
        *out = Box::into_raw(gain) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainBesselBeam(
    out: *mut ConstPtr,
    x: float,
    y: float,
    z: float,
    nx: float,
    ny: float,
    nz: float,
    theta_z: float,
    amp: float,
) {
    unsafe {
        let gain: Box<Box<G>> = Box::new(Box::new(Bessel::with_amp(
            Vector3::new(x, y, z),
            Vector3::new(nx, ny, nz),
            theta_z,
            amp,
        )));
        *out = Box::into_raw(gain) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainPlaneWave(
    out: *mut ConstPtr,
    nx: float,
    ny: float,
    nz: float,
    amp: float,
) {
    unsafe {
        let gain: Box<Box<G>> = Box::new(Box::new(Plane::with_amp(Vector3::new(nx, ny, nz), amp)));
        *out = Box::into_raw(gain) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainTransducerTest(out: *mut ConstPtr) {
    unsafe {
        let gain: Box<Box<G>> = Box::new(Box::new(TransducerTest::new()));
        *out = Box::into_raw(gain) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainTransducerTestSet(
    trans_test: ConstPtr,
    id: i32,
    phase: float,
    amp: float,
) {
    unsafe {
        (trans_test as *mut Box<G> as *mut Box<TransducerTest>)
            .as_mut()
            .unwrap()
            .set(id as _, phase, amp)
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainCustom(
    out: *mut ConstPtr,
    amp: *const float,
    phase: *const float,
    size: u64,
) {
    unsafe {
        let gain: Box<Box<G>> = Box::new(Box::new(CustomGain::new(amp, phase, size)));
        *out = Box::into_raw(gain) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteGain(gain: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(gain as *mut Box<G>);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationStatic(out: *mut ConstPtr, amp: float) {
    unsafe {
        let m: Box<Box<M>> = Box::new(Box::new(Static::with_amp(amp)));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationSine(
    out: *mut ConstPtr,
    freq: i32,
    amp: float,
    offset: float,
) {
    unsafe {
        let m: Box<Box<M>> = Box::new(Box::new(Sine::with_params(freq as _, amp, offset)));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationSineSquared(
    out: *mut ConstPtr,
    freq: i32,
    amp: float,
    offset: float,
) {
    unsafe {
        let m: Box<Box<M>> = Box::new(Box::new(SinePressure::with_params(freq as _, amp, offset)));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationSineLegacy(
    out: *mut ConstPtr,
    freq: float,
    amp: float,
    offset: float,
) {
    unsafe {
        let m: Box<Box<M>> = Box::new(Box::new(SineLegacy::with_params(freq, amp, offset)));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationSquare(
    out: *mut ConstPtr,
    freq: i32,
    low: float,
    high: float,
    duty: float,
) {
    unsafe {
        let m: Box<Box<M>> = Box::new(Box::new(Square::with_params(freq as _, low, high, duty)));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCustom(
    out: *mut ConstPtr,
    amp: *const float,
    size: u64,
    freq_div: u32,
) {
    unsafe {
        let m: Box<Box<M>> = Box::new(Box::new(CustomModulation::new(amp, size, freq_div)));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationSamplingFrequencyDivision(m: ConstPtr) -> u32 {
    unsafe {
        (m as *const Box<M>)
            .as_ref()
            .unwrap()
            .sampling_frequency_division() as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationSetSamplingFrequencyDivision(m: ConstPtr, freq_div: u32) {
    unsafe {
        (m as *mut Box<M>)
            .as_mut()
            .unwrap()
            .set_sampling_frequency_division(freq_div)
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationSamplingFrequency(m: ConstPtr) -> float {
    unsafe { (m as *const Box<M>).as_ref().unwrap().sampling_freq() }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteModulation(m: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(m as *mut Box<M>);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTM(out: *mut ConstPtr) {
    unsafe {
        let m: Box<Box<S>> = Box::new(Box::new(FocusSTM::new()));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTMAdd(
    out: *mut ConstPtr,
    x: float,
    y: float,
    z: float,
    shift: u8,
) {
    unsafe {
        (out as *mut Box<S> as *mut Box<FocusSTM>)
            .as_mut()
            .unwrap()
            .add_with_shift(Vector3::new(x, y, z), shift)
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTM(out: *mut ConstPtr) {
    unsafe {
        let m: Box<Box<S>> = Box::new(Box::new(GainSTM::<DynamicTransducer>::new()));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMAdd(grouped_gain: ConstPtr, gain: ConstPtr) {
    unsafe {
        let g = Box::from_raw(gain as *mut Box<G> as *mut Box<dyn Gain<DynamicTransducer>>);
        (grouped_gain as *mut Box<G> as *mut Box<GainSTM<DynamicTransducer>>)
            .as_mut()
            .unwrap()
            .add_boxed(*g);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSTMSetFrequency(stm: ConstPtr, freq: float) -> float {
    unsafe { (stm as *mut Box<S>).as_mut().unwrap().set_freq(freq) }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSTMGetStartIdx(stm: ConstPtr) -> i32 {
    unsafe {
        match (stm as *const Box<S>).as_ref().unwrap().start_idx() {
            Some(idx) => idx as _,
            None => -1,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSTMGetFinishIdx(stm: ConstPtr) -> i32 {
    unsafe {
        match (stm as *const Box<S>).as_ref().unwrap().finish_idx() {
            Some(idx) => idx as _,
            None => -1,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSTMSetStartIdx(stm: ConstPtr, idx: i32) {
    unsafe {
        if idx < 0 {
            (stm as *mut Box<S>).as_mut().unwrap().set_start_idx(None)
        } else {
            (stm as *mut Box<S>)
                .as_mut()
                .unwrap()
                .set_start_idx(Some(idx as _))
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSTMSetFinishIdx(stm: ConstPtr, idx: i32) {
    unsafe {
        if idx < 0 {
            (stm as *mut Box<S>).as_mut().unwrap().set_start_idx(None)
        } else {
            (stm as *mut Box<S>)
                .as_mut()
                .unwrap()
                .set_finish_idx(Some(idx as _))
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSTMFrequency(stm: ConstPtr) -> float {
    unsafe { (stm as *const Box<S>).as_ref().unwrap().freq() }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSTMSamplingFrequency(stm: ConstPtr) -> float {
    unsafe { (stm as *const Box<S>).as_ref().unwrap().sampling_freq() }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSTMSamplingFrequencyDivision(stm: ConstPtr) -> u32 {
    unsafe { (stm as *const Box<S>).as_ref().unwrap().sampling_freq_div() }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSTMSetSamplingFrequencyDivision(stm: ConstPtr, freq_div: u32) {
    unsafe {
        (stm as *mut Box<S>)
            .as_mut()
            .unwrap()
            .set_sampling_freq_div(freq_div)
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteSTM(stm: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(stm as *mut Box<S>);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSynchronize(out: *mut ConstPtr) {
    unsafe {
        let m: Box<Box<dyn DynamicSendable>> = Box::new(Box::new(Synchronize::new()));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDClear(out: *mut ConstPtr) {
    unsafe {
        let m: Box<Box<dyn DynamicSendable>> = Box::new(Box::new(Clear::new()));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDUpdateFlags(out: *mut ConstPtr) {
    unsafe {
        let m: Box<Box<dyn DynamicSendable>> = Box::new(Box::new(UpdateFlag::new()));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDStop(out: *mut ConstPtr) {
    unsafe {
        let m: Box<Box<dyn DynamicSendable>> = Box::new(Box::new(Stop::new()));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModDelayConfig(out: *mut ConstPtr) {
    unsafe {
        let m: Box<Box<dyn DynamicSendable>> = Box::new(Box::new(ModDelay::new()));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteSpecialData(out: *mut ConstPtr) {
    unsafe {
        let _ = Box::from_raw(*out as *mut Box<dyn DynamicSendable>);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDCreateSilencer(out: *mut ConstPtr, step: u16) {
    unsafe {
        let m: Box<Box<dyn DynamicSendable>> = Box::new(Box::new(SilencerConfig::new(step)));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteSilencer(out: *mut ConstPtr) {
    unsafe {
        let _ = Box::from_raw(*out as *mut Box<dyn DynamicSendable>);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDCreateAmplitudes(out: *mut ConstPtr, amp: float) {
    unsafe {
        let m: Box<Box<dyn DynamicSendable>> = Box::new(Box::new(Amplitudes::uniform(amp)));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteAmplitudes(out: *mut ConstPtr) {
    unsafe {
        let _ = Box::from_raw(*out as *mut Box<dyn DynamicSendable>);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSend(
    cnt: ConstPtr,
    mode: u8,
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
    if let Some(mode) = to_mode(mode) {
        unsafe {
            if !header.is_null() && !body.is_null() {
                let header = (header as *mut Box<dyn DynamicSendable>).as_mut().unwrap();
                let body = (body as *mut Box<dyn DynamicSendable>).as_mut().unwrap();
                try_or_return!(
                    (cnt as *mut Cnt)
                        .as_mut()
                        .unwrap()
                        .send_with_timeout((mode, header, body), timeout),
                    err
                );
            } else if !header.is_null() {
                let header = (header as *mut Box<dyn DynamicSendable>).as_mut().unwrap();
                try_or_return!(
                    (cnt as *mut Cnt)
                        .as_mut()
                        .unwrap()
                        .send_with_timeout((mode, header), timeout),
                    err
                );
            } else if !body.is_null() {
                let body = (body as *mut Box<dyn DynamicSendable>).as_mut().unwrap();
                try_or_return!(
                    (cnt as *mut Cnt)
                        .as_mut()
                        .unwrap()
                        .send_with_timeout((mode, body), timeout),
                    err
                );
            } else {
                return FALSE;
            }
        }
        TRUE
    } else {
        FALSE
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSendSpecial(
    cnt: ConstPtr,
    mode: u8,
    special: ConstPtr,
    timeout_ns: i64,
    err: *mut c_char,
) -> i32 {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    if let Some(mode) = to_mode(mode) {
        unsafe {
            let special = (special as *mut Box<dyn DynamicSendable>).as_mut().unwrap();
            try_or_return!(
                (cnt as *mut Cnt)
                    .as_mut()
                    .unwrap()
                    .send_with_timeout((mode, special), timeout),
                err
            );
        }
        TRUE
    } else {
        FALSE
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkDebug(out: *mut ConstPtr) {
    unsafe {
        *out = Box::into_raw(Box::new(Debug::builder())) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkDebugLogLevel(builder: ConstPtr, level: u16) {
    unsafe {
        if let Some(level) = to_level(level) {
            (builder as *mut DebugBuilder)
                .as_mut()
                .unwrap()
                .level(level);
        }
    }
}
struct Callback(ConstPtr);
unsafe impl Send for Callback {}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkDebugLogFunc(
    builder: ConstPtr,
    level: u16,
    out_func: ConstPtr,
    flush_func: ConstPtr,
) {
    unsafe {
        if out_func.is_null() || flush_func.is_null() {
            return;
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

        let level = to_level(level).unwrap_or(Level::Debug);
        let logger = get_logger_with_custom_func(level, out_func, flush_func);

        (builder as *mut DebugBuilder)
            .as_mut()
            .unwrap()
            .logger(logger);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkDebugTimeout(builder: ConstPtr, timeout_ns: u64) {
    unsafe {
        (builder as *mut DebugBuilder)
            .as_mut()
            .unwrap()
            .timeout(Duration::from_nanos(timeout_ns));
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkDebugBuild(out: *mut ConstPtr, builder: *mut c_void) {
    unsafe {
        let builder = Box::from_raw(builder as *mut DebugBuilder);
        let link: Box<Box<L>> = Box::new(Box::new(builder.build()));
        *out = Box::into_raw(link) as _;
    }
}
