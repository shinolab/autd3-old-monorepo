/*
 * File: lib.rs
 * Project: src
 * Created Date: 11/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

pub mod gain;
pub mod geometry;
pub mod link;
pub mod modulation;
pub mod stm;

use autd3capi_def::{
    common::*, ControllerPtr, DatagramPtr, DatagramSpecialPtr, GroupKVMapPtr, LinkPtr, TransMode,
    AUTD3_ERR, AUTD3_FALSE, AUTD3_TRUE,
};
use std::{ffi::c_char, time::Duration};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ControllerBuilderPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Drive {
    pub phase: float,
    pub amp: float,
}

struct CallbackPtr(ConstPtr);
unsafe impl Send for CallbackPtr {}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDControllerBuilder() -> ControllerBuilderPtr {
    ControllerBuilderPtr(
        Box::into_raw(Box::new(ControllerBuilder::<DynamicTransducer>::default())) as _,
    )
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerBuilderAddDevice(
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
pub unsafe extern "C" fn AUTDControllerBuilderAddDeviceQuaternion(
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
pub unsafe extern "C" fn AUTDControllerClose(cnt: ControllerPtr, err: *mut c_char) -> bool {
    try_or_return!(cast_mut!(cnt.0, Cnt).close(), err, false);
    true
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerDelete(cnt: ControllerPtr) {
    let mut cnt = Box::from_raw(cnt.0 as *mut Cnt);
    let _ = cnt.close();
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFPGAInfo(
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
pub unsafe extern "C" fn AUTDControllerFirmwareInfoListPointer(
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
pub unsafe extern "C" fn AUTDControllerFirmwareInfoGet(
    p_info_list: FirmwareInfoListPtr,
    idx: u32,
    info: *mut c_char,
) {
    let firm_info = &cast_mut!(p_info_list.0, Vec<FirmwareInfo>)[idx as usize];
    let info_str = std::ffi::CString::new(firm_info.to_string()).unwrap();
    libc::strcpy(info, info_str.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerFirmwareInfoListPointerDelete(
    p_info_list: FirmwareInfoListPtr,
) {
    let _ = Box::from_raw(p_info_list.0 as *mut Vec<FirmwareInfo>);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFirmwareLatest(latest: *mut c_char) {
    let info_str = std::ffi::CString::new(FirmwareInfo::latest_version()).unwrap();
    libc::strcpy(latest, info_str.as_ptr());
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSynchronize() -> DatagramPtr {
    DatagramPtr::new(Synchronize::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramClear() -> DatagramPtr {
    DatagramPtr::new(Clear::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramUpdateFlags() -> DatagramPtr {
    DatagramPtr::new(UpdateFlags::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramStop() -> DatagramSpecialPtr {
    DatagramSpecialPtr::new(Stop::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramConfigureModDelay() -> DatagramPtr {
    DatagramPtr::new(ConfigureModDelay::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramConfigureAmpFilter() -> DatagramPtr {
    DatagramPtr::new(ConfigureAmpFilter::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramConfigurePhaseFilter() -> DatagramPtr {
    DatagramPtr::new(ConfigurePhaseFilter::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencer(step: u16) -> DatagramPtr {
    DatagramPtr::new(Silencer::new(step))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramAmplitudes(amp: float) -> DatagramPtr {
    DatagramPtr::new(Amplitudes::uniform(amp))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerSend(
    cnt: ControllerPtr,
    mode: TransMode,
    d1: DatagramPtr,
    d2: DatagramPtr,
    timeout_ns: i64,
    err: *mut c_char,
) -> i32 {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    let mode = mode.into();
    let res = if !d1.0.is_null() && !d2.0.is_null() {
        let d1 = Box::from_raw(d1.0 as *mut Box<dyn DynamicDatagram>);
        let d2 = Box::from_raw(d2.0 as *mut Box<dyn DynamicDatagram>);
        try_or_return!(
            cast_mut!(cnt.0, Cnt).send((mode, d1, d2, timeout)),
            err,
            AUTD3_ERR
        )
    } else if !d1.0.is_null() {
        let d = Box::from_raw(d1.0 as *mut Box<dyn DynamicDatagram>);
        try_or_return!(
            cast_mut!(cnt.0, Cnt).send((mode, d, timeout)),
            err,
            AUTD3_ERR
        )
    } else if !d2.0.is_null() {
        let d = Box::from_raw(d2.0 as *mut Box<dyn DynamicDatagram>);
        try_or_return!(
            cast_mut!(cnt.0, Cnt).send((mode, d, timeout)),
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
pub unsafe extern "C" fn AUTDControllerSendSpecial(
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

type K = i32;
type V = (
    Box<dyn driver::operation::Operation<DynamicTransducer>>,
    Box<dyn driver::operation::Operation<DynamicTransducer>>,
    Option<Duration>,
);
type M = std::collections::HashMap<K, V>;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroupCreateKVMap() -> GroupKVMapPtr {
    GroupKVMapPtr(Box::into_raw(Box::<M>::default()) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroupKVMapSet(
    map: GroupKVMapPtr,
    key: i32,
    d1: DatagramPtr,
    d2: DatagramPtr,
    mode: TransMode,
    timeout_ns: i64,
    err: *mut c_char,
) -> GroupKVMapPtr {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    let mode = mode.into();
    let mut map = Box::from_raw(map.0 as *mut M);
    if !d1.0.is_null() && !d2.0.is_null() {
        let d1 = Box::from_raw(d1.0 as *mut Box<dyn DynamicDatagram>);
        let d2 = Box::from_raw(d2.0 as *mut Box<dyn DynamicDatagram>);
        let (op1, op2) = try_or_return!(
            (mode, d1, d2, timeout).operation(),
            err,
            GroupKVMapPtr(std::ptr::null())
        );
        map.insert(key, (op1, op2, timeout));
    } else if !d1.0.is_null() {
        let d = Box::from_raw(d1.0 as *mut Box<dyn DynamicDatagram>);
        let (op1, op2) = try_or_return!(
            (mode, d, timeout).operation(),
            err,
            GroupKVMapPtr(std::ptr::null())
        );
        map.insert(key, (op1, op2, timeout));
    } else if !d2.0.is_null() {
        let d = Box::from_raw(d2.0 as *mut Box<dyn DynamicDatagram>);
        let (op1, op2) = try_or_return!(
            (mode, d, timeout).operation(),
            err,
            GroupKVMapPtr(std::ptr::null())
        );
        map.insert(key, (op1, op2, timeout));
    }
    GroupKVMapPtr(Box::into_raw(map) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroupKVMapSetSpecial(
    map: GroupKVMapPtr,
    key: i32,
    special: DatagramSpecialPtr,
    mode: TransMode,
    timeout_ns: i64,
    err: *mut c_char,
) -> GroupKVMapPtr {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    let mode = mode.into();
    let mut map = Box::from_raw(map.0 as *mut M);

    let d = Box::from_raw(special.0 as *mut Box<dyn DynamicDatagram>);
    let (op1, op2) = try_or_return!(
        (mode, d, timeout).operation(),
        err,
        GroupKVMapPtr(std::ptr::null())
    );
    map.insert(key, (op1, op2, timeout));

    GroupKVMapPtr(Box::into_raw(map) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroup(
    cnt: ControllerPtr,
    map: *const i32,
    kv_map: GroupKVMapPtr,
    err: *mut c_char,
) -> i32 {
    let kv_map = Box::from_raw(kv_map.0 as *mut M);
    try_or_return!(
        try_or_return!(
            kv_map.into_iter().try_fold(
                cast_mut!(cnt.0, Cnt).group(|dev| {
                    let k = map.add(dev.idx()).read();
                    if k < 0 {
                        None
                    } else {
                        Some(k)
                    }
                }),
                |acc, (k, (op1, op2, timeout))| { acc.set_boxed_op(k, op1, op2, timeout) }
            ),
            err,
            AUTD3_ERR
        )
        .send(),
        err,
        AUTD3_ERR
    );

    AUTD3_TRUE
}

struct SoftwareSTMCallbackPtr(ConstPtr);
unsafe impl Send for SoftwareSTMCallbackPtr {}

struct SoftwareSTMContextPtr(ConstPtr);
unsafe impl Send for SoftwareSTMContextPtr {}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerSoftwareSTM(
    cnt: ControllerPtr,
    callback: ConstPtr,
    context: ConstPtr,
    timer_strategy: TimerStrategy,
    interval_ns: u64,
    err: *mut c_char,
) -> i32 {
    let callback = std::sync::Arc::new(std::sync::Mutex::new(SoftwareSTMCallbackPtr(callback)));
    let context = std::sync::Arc::new(std::sync::Mutex::new(SoftwareSTMContextPtr(context)));
    try_or_return!(
        cast_mut!(cnt.0, Cnt)
            .software_stm(move |_cnt: &mut Cnt, i: usize, elapsed: Duration| -> bool {
                let f = std::mem::transmute::<
                    _,
                    unsafe extern "C" fn(SoftwareSTMContextPtr, u64, u64) -> bool,
                >(callback.lock().unwrap().0);
                f(
                    SoftwareSTMContextPtr(context.lock().unwrap().0),
                    i as u64,
                    elapsed.as_nanos() as u64,
                )
            })
            .with_timer_strategy(timer_strategy)
            .start(Duration::from_nanos(interval_ns))
            .map(|_| AUTD3_TRUE),
        err,
        AUTD3_ERR
    )
}

#[cfg(test)]
mod tests {
    use autd3capi_def::{DatagramPtr, Level};

    use super::*;

    use crate::link::debug::*;

    use std::ffi::CStr;

    pub unsafe fn make_debug_link() -> LinkPtr {
        let debug = AUTDLinkDebug();
        let debug = AUTDLinkDebugWithLogLevel(debug, Level::Off);
        AUTDLinkDebugWithTimeout(debug, 0)
    }

    pub unsafe fn create_controller() -> ControllerPtr {
        let builder = AUTDControllerBuilder();
        let builder = AUTDControllerBuilderAddDevice(builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let builder =
            AUTDControllerBuilderAddDeviceQuaternion(builder, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

        let link = make_debug_link();
        let mut err = vec![c_char::default(); 256];
        let cnt = AUTDControllerOpenWith(builder, link, err.as_mut_ptr());
        assert_ne!(cnt.0, NULL);
        cnt
    }

    #[test]
    fn drive_test() {
        assert_eq!(
            std::mem::size_of::<autd3_driver::defined::Drive>(),
            std::mem::size_of::<Drive>()
        );
        assert_eq!(
            memoffset::offset_of!(autd3_driver::defined::Drive, phase),
            memoffset::offset_of!(Drive, phase)
        );
        assert_eq!(
            memoffset::offset_of!(autd3_driver::defined::Drive, amp),
            memoffset::offset_of!(Drive, amp)
        );
    }

    #[test]
    fn basic() {
        unsafe {
            let cnt = create_controller();

            let mut err = vec![c_char::default(); 256];

            let firm_p = AUTDControllerFirmwareInfoListPointer(cnt, err.as_mut_ptr());
            assert_ne!(firm_p.0, NULL);
            (0..2).for_each(|i| {
                let mut info = vec![c_char::default(); 256];
                AUTDControllerFirmwareInfoGet(firm_p, i as _, info.as_mut_ptr());
                dbg!(CStr::from_ptr(info.as_ptr()).to_str().unwrap());
            });
            AUTDControllerFirmwareInfoListPointerDelete(firm_p);

            let mut fpga_info = vec![0xFFu8; 2];
            assert!(AUTDControllerFPGAInfo(
                cnt,
                fpga_info.as_mut_ptr() as _,
                err.as_mut_ptr()
            ));
            assert_eq!(fpga_info[0], 0x00);
            assert_eq!(fpga_info[1], 0x00);

            let s = AUTDDatagramSynchronize();
            assert_eq!(
                AUTDControllerSend(
                    cnt,
                    TransMode::Legacy,
                    s,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr()
                ),
                AUTD3_TRUE
            );

            let s = AUTDDatagramClear();
            assert_eq!(
                AUTDControllerSend(
                    cnt,
                    TransMode::Legacy,
                    s,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr()
                ),
                AUTD3_TRUE
            );

            let s = AUTDDatagramUpdateFlags();
            assert_eq!(
                AUTDControllerSend(
                    cnt,
                    TransMode::Legacy,
                    s,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr()
                ),
                AUTD3_TRUE
            );

            let s = AUTDDatagramStop();
            assert_eq!(
                AUTDControllerSendSpecial(cnt, TransMode::Legacy, s, -1, err.as_mut_ptr()),
                AUTD3_TRUE
            );

            let s = AUTDDatagramConfigureModDelay();
            assert_eq!(
                AUTDControllerSend(
                    cnt,
                    TransMode::Legacy,
                    s,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr()
                ),
                AUTD3_TRUE
            );

            let s = AUTDDatagramSilencer(10);
            assert_eq!(
                AUTDControllerSend(
                    cnt,
                    TransMode::Legacy,
                    s,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            let s = AUTDDatagramAmplitudes(1.);
            assert_eq!(
                AUTDControllerSend(
                    cnt,
                    TransMode::Legacy,
                    DatagramPtr(std::ptr::null()),
                    s,
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_ERR
            );

            let s = AUTDDatagramAmplitudes(1.);
            assert_eq!(
                AUTDControllerSend(
                    cnt,
                    TransMode::Advanced,
                    DatagramPtr(std::ptr::null()),
                    s,
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_ERR
            );

            let s = AUTDDatagramAmplitudes(1.);
            assert_eq!(
                AUTDControllerSend(
                    cnt,
                    TransMode::AdvancedPhase,
                    DatagramPtr(std::ptr::null()),
                    s,
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            assert!(AUTDControllerClose(cnt, err.as_mut_ptr()));

            AUTDControllerDelete(cnt);
        }
    }
}
