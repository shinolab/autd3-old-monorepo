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
        let gain = GainWrap::new(Null::new());
        *out = Box::into_raw(gain) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainGrouped(out: *mut ConstPtr) {
    unsafe {
        let gain = GainWrap::new(GroupedGainWrap::new());
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
        let g = *Box::from_raw(gain as *mut Box<G> as *mut Box<GainWrap>);
        ((grouped_gain as *mut Box<G> as *mut Box<GainWrap>)
            .as_mut()
            .unwrap()
            .gain_mut() as *mut _ as *mut Box<GroupedGainWrap>)
            .as_mut()
            .unwrap()
            .add(device_id as _, *g);
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
        let gain = GainWrap::new(Focus::with_amp(Vector3::new(x, y, z), amp));
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
        let gain = GainWrap::new(Bessel::with_amp(
            Vector3::new(x, y, z),
            Vector3::new(nx, ny, nz),
            theta_z,
            amp,
        ));
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
        let gain = GainWrap::new(Plane::with_amp(Vector3::new(nx, ny, nz), amp));
        *out = Box::into_raw(gain) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainTransducerTest(out: *mut ConstPtr) {
    unsafe {
        let gain = GainWrap::new(TransducerTest::new());
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
        ((trans_test as *mut Box<G> as *mut Box<GainWrap>)
            .as_mut()
            .unwrap()
            .gain_mut() as *mut _ as *mut Box<TransducerTest>)
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
        let gain = GainWrap::new(CustomGain::new(amp, phase, size));
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
        let m: Box<Box<M>> = Box::new(Box::new(ModulationWrap {
            modulation: Box::new(Static::with_amp(amp)),
        }));
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
        let m: Box<Box<M>> = Box::new(Box::new(ModulationWrap {
            modulation: Box::new(Sine::with_params(freq as _, amp, offset)),
        }));
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
        let m: Box<Box<M>> = Box::new(Box::new(ModulationWrap {
            modulation: Box::new(SinePressure::with_params(freq as _, amp, offset)),
        }));
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
        let m: Box<Box<M>> = Box::new(Box::new(ModulationWrap {
            modulation: Box::new(SineLegacy::with_params(freq, amp, offset)),
        }));
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
        let m: Box<Box<M>> = Box::new(Box::new(ModulationWrap {
            modulation: Box::new(Square::with_params(freq as _, low, high, duty)),
        }));
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
        let m: Box<Box<M>> = Box::new(Box::new(ModulationWrap {
            modulation: Box::new(CustomModulation::new(amp, size, freq_div)),
        }));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationSamplingFrequencyDivision(m: ConstPtr) -> u32 {
    unsafe {
        (m as *const Box<M>)
            .as_ref()
            .unwrap()
            .modulation()
            .sampling_frequency_division() as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationSetSamplingFrequencyDivision(m: ConstPtr, freq_div: u32) {
    unsafe {
        (m as *mut Box<M>)
            .as_mut()
            .unwrap()
            .modulation_mut()
            .set_sampling_frequency_division(freq_div)
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationSamplingFrequency(m: ConstPtr) -> float {
    unsafe {
        (m as *const Box<M>)
            .as_ref()
            .unwrap()
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
pub unsafe extern "C" fn AUTDFocusSTM(out: *mut ConstPtr) {
    unsafe {
        let stm: Box<Box<SF>> = FocusSTMWrap::new();
        *out = Box::into_raw(stm) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTMAdd(stm: ConstPtr, x: float, y: float, z: float, shift: u8) {
    unsafe {
        (stm as *mut Box<SF>)
            .as_mut()
            .unwrap()
            .stm_mut()
            .add_with_shift(Vector3::new(x, y, z), shift)
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTMSetFrequency(stm: ConstPtr, freq: float) -> float {
    unsafe {
        (stm as *mut Box<SF>)
            .as_mut()
            .unwrap()
            .stm_mut()
            .set_freq(freq)
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTMGetStartIdx(stm: ConstPtr) -> i32 {
    unsafe {
        match (stm as *mut Box<SF>).as_ref().unwrap().stm().start_idx() {
            Some(idx) => idx as _,
            None => -1,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTMGetFinishIdx(stm: ConstPtr) -> i32 {
    unsafe {
        match (stm as *mut Box<SF>).as_ref().unwrap().stm().finish_idx() {
            Some(idx) => idx as _,
            None => -1,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTMSetStartIdx(stm: ConstPtr, idx: i32) {
    unsafe {
        if idx < 0 {
            (stm as *mut Box<SF>)
                .as_mut()
                .unwrap()
                .stm_mut()
                .set_start_idx(None)
        } else {
            (stm as *mut Box<SF>)
                .as_mut()
                .unwrap()
                .stm_mut()
                .set_start_idx(Some(idx as _))
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTMSetFinishIdx(stm: ConstPtr, idx: i32) {
    unsafe {
        if idx < 0 {
            (stm as *mut Box<SF>)
                .as_mut()
                .unwrap()
                .stm_mut()
                .set_start_idx(None)
        } else {
            (stm as *mut Box<SF>)
                .as_mut()
                .unwrap()
                .stm_mut()
                .set_finish_idx(Some(idx as _))
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTMFrequency(stm: ConstPtr) -> float {
    unsafe { (stm as *mut Box<SF>).as_ref().unwrap().stm().freq() }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTMSamplingFrequency(stm: ConstPtr) -> float {
    unsafe {
        (stm as *mut Box<SF>)
            .as_ref()
            .unwrap()
            .stm()
            .sampling_freq()
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTMSamplingFrequencyDivision(stm: ConstPtr) -> u32 {
    unsafe {
        (stm as *mut Box<SF>)
            .as_ref()
            .unwrap()
            .stm()
            .sampling_freq_div()
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFocusSTMSetSamplingFrequencyDivision(stm: ConstPtr, freq_div: u32) {
    unsafe {
        (stm as *mut Box<SF>)
            .as_mut()
            .unwrap()
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
pub unsafe extern "C" fn AUTDGainSTM(out: *mut ConstPtr) {
    unsafe {
        let stm: Box<Box<SG>> = GainSTMWrap::new();
        *out = Box::into_raw(stm) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMAdd(stm: ConstPtr, gain: ConstPtr) {
    unsafe {
        let g = *Box::from_raw(gain as *mut Box<G> as *mut Box<GainWrap>);
        (stm as *mut Box<SG>).as_mut().unwrap().add(g)
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMSetFrequency(stm: ConstPtr, freq: float) -> float {
    unsafe {
        (stm as *mut Box<SG>)
            .as_mut()
            .unwrap()
            .stm_mut()
            .set_freq(freq)
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMGetStartIdx(stm: ConstPtr) -> i32 {
    unsafe {
        match (stm as *mut Box<SG>).as_ref().unwrap().stm().start_idx() {
            Some(idx) => idx as _,
            None => -1,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMGetFinishIdx(stm: ConstPtr) -> i32 {
    unsafe {
        match (stm as *mut Box<SG>).as_ref().unwrap().stm().finish_idx() {
            Some(idx) => idx as _,
            None => -1,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMSetStartIdx(stm: ConstPtr, idx: i32) {
    unsafe {
        if idx < 0 {
            (stm as *mut Box<SG>)
                .as_mut()
                .unwrap()
                .stm_mut()
                .set_start_idx(None)
        } else {
            (stm as *mut Box<SG>)
                .as_mut()
                .unwrap()
                .stm_mut()
                .set_start_idx(Some(idx as _))
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMSetFinishIdx(stm: ConstPtr, idx: i32) {
    unsafe {
        if idx < 0 {
            (stm as *mut Box<SG>)
                .as_mut()
                .unwrap()
                .stm_mut()
                .set_start_idx(None)
        } else {
            (stm as *mut Box<SG>)
                .as_mut()
                .unwrap()
                .stm_mut()
                .set_finish_idx(Some(idx as _))
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMFrequency(stm: ConstPtr) -> float {
    unsafe { (stm as *mut Box<SG>).as_ref().unwrap().stm().freq() }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMSamplingFrequency(stm: ConstPtr) -> float {
    unsafe {
        (stm as *mut Box<SG>)
            .as_ref()
            .unwrap()
            .stm()
            .sampling_freq()
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMSamplingFrequencyDivision(stm: ConstPtr) -> u32 {
    unsafe {
        (stm as *mut Box<SG>)
            .as_ref()
            .unwrap()
            .stm()
            .sampling_freq_div()
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainSTMSetSamplingFrequencyDivision(stm: ConstPtr, freq_div: u32) {
    unsafe {
        (stm as *mut Box<SG>)
            .as_mut()
            .unwrap()
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
pub unsafe extern "C" fn AUTDSynchronize(out: *mut ConstPtr) {
    unsafe {
        let m: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(Synchronize::new()));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDClear(out: *mut ConstPtr) {
    unsafe {
        let m: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(Clear::new()));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDUpdateFlags(out: *mut ConstPtr) {
    unsafe {
        let m: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(UpdateFlag::new()));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDStop(out: *mut ConstPtr) {
    unsafe {
        let m: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(Stop::new()));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModDelayConfig(out: *mut ConstPtr) {
    unsafe {
        let m: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(ModDelay::new()));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteSpecialData(out: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(out as *mut Box<dyn DynamicDatagram>);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDCreateSilencer(out: *mut ConstPtr, step: u16) {
    unsafe {
        let m: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(SilencerConfig::new(step)));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteSilencer(out: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(out as *mut Box<dyn DynamicDatagram>);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDCreateAmplitudes(out: *mut ConstPtr, amp: float) {
    unsafe {
        let m: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(Amplitudes::uniform(amp)));
        *out = Box::into_raw(m) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteAmplitudes(out: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(out as *mut Box<dyn DynamicDatagram>);
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
                let header = (header as *mut Box<dyn DynamicDatagram>).as_mut().unwrap();
                let body = (body as *mut Box<dyn DynamicDatagram>).as_mut().unwrap();
                try_or_return!(
                    (cnt as *mut Cnt)
                        .as_mut()
                        .unwrap()
                        .send_with_timeout((mode, header, body), timeout),
                    err
                );
            } else if !header.is_null() {
                let header = (header as *mut Box<dyn DynamicDatagram>).as_mut().unwrap();
                try_or_return!(
                    (cnt as *mut Cnt)
                        .as_mut()
                        .unwrap()
                        .send_with_timeout((mode, header), timeout),
                    err
                );
            } else if !body.is_null() {
                let body = (body as *mut Box<dyn DynamicDatagram>).as_mut().unwrap();
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
            let special = (special as *mut Box<dyn DynamicDatagram>).as_mut().unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::ffi::CStr;

    unsafe fn make_debug_link() -> *const c_void {
        let mut builder: *const c_void = std::ptr::null();
        AUTDLinkDebug(&mut builder as *mut _);
        AUTDLinkDebugLogLevel(builder, 4);
        AUTDLinkDebugTimeout(builder, 0);
        let mut link: *const c_void = std::ptr::null();
        AUTDLinkDebugBuild(&mut link as *mut _, builder as _);
        link
    }

    #[test]
    fn basic() {
        unsafe {
            let mut geo_builder: *const c_void = std::ptr::null();
            AUTDCreateGeometryBuilder(&mut geo_builder as _);
            AUTDAddDevice(geo_builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            AUTDAddDeviceQuaternion(geo_builder, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
            let mut geometry: *const c_void = std::ptr::null();
            let mut err = vec![c_char::default(); 256];
            AUTDBuildGeometry(&mut geometry as _, geo_builder, err.as_mut_ptr());

            let link = make_debug_link();

            let mut cnt: *const c_void = std::ptr::null();
            if AUTDOpenController(&mut cnt as _, geometry, link, err.as_mut_ptr()) == -1 {
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
            if AUTDSetTransFrequency(cnt, 0, f, err.as_mut_ptr()) == -1 {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            }
            dbg!(AUTDGetTransFrequency(cnt, 0));

            let f = 4096;
            if AUTDSetTransCycle(cnt, 0, f, err.as_mut_ptr()) == -1 {
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
            if AUTDGetFPGAInfo(cnt, fpga_info.as_mut_ptr() as _, err.as_mut_ptr()) == -1 {
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

            let mut firm_p: *const c_void = std::ptr::null();
            if AUTDGetFirmwareInfoListPointer(cnt, &mut firm_p as _, err.as_mut_ptr()) == -1 {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            }
            let mut info = vec![c_char::default(); 256];
            let mut is_valid = false;
            let mut is_supported = false;
            AUTDGetFirmwareInfo(
                firm_p,
                0,
                info.as_mut_ptr(),
                &mut is_valid as _,
                &mut is_supported as _,
            );
            dbg!(CStr::from_ptr(info.as_ptr()).to_str().unwrap());
            dbg!(is_valid);
            dbg!(is_supported);
            AUTDFreeFirmwareInfoListPointer(firm_p);

            {
                let mut g: *const c_void = std::ptr::null();
                AUTDGainNull(&mut g as _);
                if AUTDSend(cnt, 0, std::ptr::null(), g, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteGain(g);
            }

            {
                let mut g: *const c_void = std::ptr::null();
                AUTDGainGrouped(&mut g as _);

                let mut g0: *const c_void = std::ptr::null();
                AUTDGainNull(&mut g0 as _);
                AUTDGainGroupedAdd(g, 0, g0);

                let mut g1: *const c_void = std::ptr::null();
                AUTDGainNull(&mut g1 as _);
                AUTDGainGroupedAdd(g, 1, g1);

                if AUTDSend(cnt, 0, std::ptr::null(), g, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteGain(g);
            }

            {
                let mut g: *const c_void = std::ptr::null();
                AUTDGainFocus(&mut g as _, 0., 0., 0., 1.);
                if AUTDSend(cnt, 0, std::ptr::null(), g, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteGain(g);
            }

            {
                let mut g: *const c_void = std::ptr::null();
                AUTDGainBesselBeam(&mut g as _, 0., 0., 0., 0., 0., 1., 1., 1.);
                if AUTDSend(cnt, 0, std::ptr::null(), g, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteGain(g);
            }

            {
                let mut g: *const c_void = std::ptr::null();
                AUTDGainPlaneWave(&mut g as _, 0., 0., 1., 1.);
                if AUTDSend(cnt, 0, std::ptr::null(), g, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteGain(g);
            }

            {
                let mut g: *const c_void = std::ptr::null();
                AUTDGainTransducerTest(&mut g as _);
                AUTDGainTransducerTestSet(g, 0, 1., 1.);
                if AUTDSend(cnt, 0, std::ptr::null(), g, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteGain(g);
            }

            {
                let mut g: *const c_void = std::ptr::null();
                let amp = vec![1.0; num_transducers as _];
                let phase = vec![0.0; num_transducers as _];
                AUTDGainCustom(
                    &mut g as _,
                    amp.as_ptr(),
                    phase.as_ptr(),
                    num_transducers as _,
                );
                if AUTDSend(cnt, 0, std::ptr::null(), g, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteGain(g);
            }

            {
                let mut m: *const c_void = std::ptr::null();
                AUTDModulationStatic(&mut m as _, 1.);
                if AUTDSend(cnt, 0, m, std::ptr::null(), -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteModulation(m);
            }

            {
                let mut m: *const c_void = std::ptr::null();
                AUTDModulationSine(&mut m as _, 150, 1., 0.5);
                if AUTDSend(cnt, 0, m, std::ptr::null(), -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteModulation(m);
            }

            {
                let mut m: *const c_void = std::ptr::null();
                AUTDModulationSineSquared(&mut m as _, 150, 1., 0.5);
                if AUTDSend(cnt, 0, m, std::ptr::null(), -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteModulation(m);
            }

            {
                let mut m: *const c_void = std::ptr::null();
                AUTDModulationSineLegacy(&mut m as _, 150., 1., 0.5);
                if AUTDSend(cnt, 0, m, std::ptr::null(), -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteModulation(m);
            }

            {
                let mut m: *const c_void = std::ptr::null();
                AUTDModulationSquare(&mut m as _, 150, 0., 1., 0.5);
                if AUTDSend(cnt, 0, m, std::ptr::null(), -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteModulation(m);
            }

            {
                let mut m: *const c_void = std::ptr::null();
                let amp = vec![1.0; 10];
                AUTDModulationCustom(&mut m as _, amp.as_ptr(), amp.len() as _, 5000);
                if AUTDSend(cnt, 0, m, std::ptr::null(), -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
                AUTDDeleteModulation(m);
            }

            {
                let mut m: *const c_void = std::ptr::null();
                AUTDModulationStatic(&mut m as _, 1.);
                let div = 1000;
                AUTDModulationSetSamplingFrequencyDivision(m, div);
                assert_eq!(div, AUTDModulationSamplingFrequencyDivision(m));
                dbg!(AUTDModulationSamplingFrequency(m));
                AUTDDeleteModulation(m);
            }

            {
                let mut stm: *const c_void = std::ptr::null();
                AUTDFocusSTM(&mut stm as _);
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

                if AUTDSend(cnt, 0, std::ptr::null(), stm, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteFocusSTM(stm);
            }

            {
                let mut stm: *const c_void = std::ptr::null();
                AUTDGainSTM(&mut stm as _);
                let mut g0: *const c_void = std::ptr::null();
                AUTDGainNull(&mut g0 as _);
                let mut g1: *const c_void = std::ptr::null();
                AUTDGainNull(&mut g1 as _);
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

                if AUTDSend(cnt, 0, std::ptr::null(), stm, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteGainSTM(stm);
            }

            {
                let mut s: *const c_void = std::ptr::null();
                AUTDSynchronize(&mut s as _);

                if AUTDSendSpecial(cnt, 0, s, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteSpecialData(s);
            }

            {
                let mut s: *const c_void = std::ptr::null();
                AUTDClear(&mut s as _);

                if AUTDSendSpecial(cnt, 0, s, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteSpecialData(s);
            }

            {
                let mut s: *const c_void = std::ptr::null();
                AUTDUpdateFlags(&mut s as _);

                if AUTDSendSpecial(cnt, 0, s, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteSpecialData(s);
            }

            {
                let mut s: *const c_void = std::ptr::null();
                AUTDStop(&mut s as _);

                if AUTDSendSpecial(cnt, 0, s, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteSpecialData(s);
            }

            {
                let mut s: *const c_void = std::ptr::null();
                AUTDModDelayConfig(&mut s as _);

                if AUTDSendSpecial(cnt, 0, s, -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteSpecialData(s);
            }

            {
                let mut s: *const c_void = std::ptr::null();
                AUTDCreateSilencer(&mut s as _, 10);

                if AUTDSend(cnt, 0, s, std::ptr::null(), -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteSilencer(s);
            }

            {
                let mut s: *const c_void = std::ptr::null();
                AUTDCreateAmplitudes(&mut s as _, 1.);

                if AUTDSend(cnt, 0, s, std::ptr::null(), -1, err.as_mut_ptr()) == -1 {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteAmplitudes(s);
            }

            if AUTDClose(cnt, err.as_mut_ptr()) == -1 {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            }

            AUTDFreeController(cnt);
        }
    }
}
