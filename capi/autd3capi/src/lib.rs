/*
 * File: lib.rs
 * Project: src
 * Created Date: 11/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/09/2023
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
    common::*, ControllerPtr, DatagramPtr, DatagramSpecialPtr, LinkPtr, TransMode, AUTD3_ERR,
    AUTD3_FALSE, AUTD3_TRUE,
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
pub unsafe extern "C" fn AUTDCreateControllerBuilder() -> ControllerBuilderPtr {
    ControllerBuilderPtr(
        Box::into_raw(Box::new(ControllerBuilder::<DynamicTransducer>::default())) as _,
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
) {
    let firm_info = &cast_mut!(p_info_list.0, Vec<FirmwareInfo>)[idx as usize];
    let info_str = std::ffi::CString::new(firm_info.to_string()).unwrap();
    libc::strcpy(info, info_str.as_ptr());
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
pub unsafe extern "C" fn AUTDSynchronize() -> DatagramPtr {
    DatagramPtr::new(Synchronize::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDClear() -> DatagramPtr {
    DatagramPtr::new(Clear::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDUpdateFlags() -> DatagramPtr {
    DatagramPtr::new(UpdateFlags::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDStop() -> DatagramSpecialPtr {
    DatagramSpecialPtr::new(Stop::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDConfigureModDelay() -> DatagramPtr {
    DatagramPtr::new(ConfigureModDelay::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDConfigureAmpFilter() -> DatagramPtr {
    DatagramPtr::new(ConfigureAmpFilter::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDConfigurePhaseFilter() -> DatagramPtr {
    DatagramPtr::new(ConfigurePhaseFilter::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDCreateSilencer(step: u16) -> DatagramPtr {
    DatagramPtr::new(Silencer::new(step))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDCreateAmplitudes(amp: float) -> DatagramPtr {
    DatagramPtr::new(Amplitudes::uniform(amp))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSend(
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
        let header = Box::from_raw(d1.0 as *mut Box<dyn DynamicDatagram>);
        let body = Box::from_raw(d2.0 as *mut Box<dyn DynamicDatagram>);
        try_or_return!(
            cast_mut!(cnt.0, Cnt).send((mode, header, body, timeout)),
            err,
            AUTD3_ERR
        )
    } else if !d1.0.is_null() {
        let header = Box::from_raw(d1.0 as *mut Box<dyn DynamicDatagram>);
        try_or_return!(
            cast_mut!(cnt.0, Cnt).send((mode, header, timeout)),
            err,
            AUTD3_ERR
        )
    } else if !d2.0.is_null() {
        let body = Box::from_raw(d2.0 as *mut Box<dyn DynamicDatagram>);
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
            std::mem::size_of::<autd3::driver::defined::Drive>(),
            std::mem::size_of::<Drive>()
        );
        assert_eq!(
            memoffset::offset_of!(autd3::driver::defined::Drive, phase),
            memoffset::offset_of!(Drive, phase)
        );
        assert_eq!(
            memoffset::offset_of!(autd3::driver::defined::Drive, amp),
            memoffset::offset_of!(Drive, amp)
        );
    }

    #[test]
    fn basic() {
        unsafe {
            let cnt = create_controller();

            let mut err = vec![c_char::default(); 256];

            let firm_p = AUTDGetFirmwareInfoListPointer(cnt, err.as_mut_ptr());
            assert_ne!(firm_p.0, NULL);
            (0..2).for_each(|i| {
                let mut info = vec![c_char::default(); 256];
                AUTDGetFirmwareInfo(firm_p, i as _, info.as_mut_ptr());
                dbg!(CStr::from_ptr(info.as_ptr()).to_str().unwrap());
            });
            AUTDFreeFirmwareInfoListPointer(firm_p);

            let mut fpga_info = vec![0xFFu8; 2];
            assert!(AUTDGetFPGAInfo(
                cnt,
                fpga_info.as_mut_ptr() as _,
                err.as_mut_ptr()
            ));
            assert_eq!(fpga_info[0], 0x00);
            assert_eq!(fpga_info[1], 0x00);

            let s = AUTDSynchronize();
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    s,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr()
                ),
                AUTD3_TRUE
            );

            let s = AUTDClear();
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    s,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr()
                ),
                AUTD3_TRUE
            );

            let s = AUTDUpdateFlags();
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    s,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr()
                ),
                AUTD3_TRUE
            );

            let s = AUTDStop();
            assert_eq!(
                AUTDSendSpecial(cnt, TransMode::Legacy, s, -1, err.as_mut_ptr()),
                AUTD3_TRUE
            );

            let s = AUTDConfigureModDelay();
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    s,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr()
                ),
                AUTD3_TRUE
            );

            let s = AUTDCreateSilencer(10);
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    s,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            let s = AUTDCreateAmplitudes(1.);
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    DatagramPtr(std::ptr::null()),
                    s,
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_ERR
            );

            let s = AUTDCreateAmplitudes(1.);
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Advanced,
                    DatagramPtr(std::ptr::null()),
                    s,
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_ERR
            );

            let s = AUTDCreateAmplitudes(1.);
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::AdvancedPhase,
                    DatagramPtr(std::ptr::null()),
                    s,
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            assert!(AUTDClose(cnt, err.as_mut_ptr()));

            AUTDFreeController(cnt);
        }
    }
}
