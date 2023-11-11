/*
 * File: lib.rs
 * Project: src
 * Created Date: 11/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/11/2023
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
    common::*, ControllerPtr, DatagramPtr, DatagramSpecialPtr, LinkBuilderPtr, ResultController,
    ResultI32,
};
use std::{ffi::c_char, time::Duration};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ControllerBuilderPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Drive {
    pub phase: float,
    pub amp: u16,
}

struct CallbackPtr(ConstPtr);
unsafe impl Send for CallbackPtr {}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDControllerBuilder() -> ControllerBuilderPtr {
    ControllerBuilderPtr(Box::into_raw(Box::new(Controller::builder_with())) as _)
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
        Box::from_raw(builder.0 as *mut autd3::controller::builder::ControllerBuilder).add_device(
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
        Box::from_raw(builder.0 as *mut autd3::controller::builder::ControllerBuilder).add_device(
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
    link_builder: LinkBuilderPtr,
) -> ResultController {
    let link_builder: Box<DynamicLinkBuilder> =
        Box::from_raw(link_builder.0 as *mut DynamicLinkBuilder);
    Box::from_raw(builder.0 as *mut autd3::controller::builder::ControllerBuilder)
        .open_with(*link_builder)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerClose(cnt: ControllerPtr) -> ResultI32 {
    cast_mut!(cnt.0, Cnt).close().into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerDelete(cnt: ControllerPtr) {
    let mut cnt = Box::from_raw(cnt.0 as *mut Cnt);
    let _ = cnt.close();
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFPGAInfo(cnt: ControllerPtr, out: *mut u8) -> ResultI32 {
    cast_mut!(cnt.0, Cnt)
        .fpga_info()
        .map(|fpga_info| {
            std::ptr::copy_nonoverlapping(fpga_info.as_ptr() as _, out, fpga_info.len());
            true
        })
        .into()
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultFirmwareInfoList {
    pub result: ConstPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<Vec<FirmwareInfo>, AUTDError>> for ResultFirmwareInfoList {
    fn from(r: Result<Vec<FirmwareInfo>, AUTDError>) -> Self {
        match r {
            Ok(v) => Self {
                result: Box::into_raw(Box::new(v)) as _,
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: NULL,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFirmwareInfoListPointer(
    cnt: ControllerPtr,
) -> ResultFirmwareInfoList {
    cast_mut!(cnt.0, Cnt).firmware_infos().into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerFirmwareInfoGet(
    p_info_list: ResultFirmwareInfoList,
    idx: u32,
    info: *mut c_char,
) {
    let firm_info = &cast_mut!(p_info_list.result, Vec<FirmwareInfo>)[idx as usize];
    let info_str = std::ffi::CString::new(firm_info.to_string()).unwrap();
    libc::strcpy(info, info_str.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerFirmwareInfoListPointerDelete(
    p_info_list: ResultFirmwareInfoList,
) {
    let _ = Box::from_raw(p_info_list.result as *mut Vec<FirmwareInfo>);
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
pub unsafe extern "C" fn AUTDControllerSend(
    cnt: ControllerPtr,
    d1: DatagramPtr,
    d2: DatagramPtr,
    timeout_ns: i64,
) -> ResultI32 {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    if !d1.0.is_null() && !d2.0.is_null() {
        let d1 = Box::from_raw(d1.0 as *mut Box<dyn DynamicDatagram>);
        let d2 = Box::from_raw(d2.0 as *mut Box<dyn DynamicDatagram>);
        cast_mut!(cnt.0, Cnt)
            .send(DynamicDatagramPack2 { d1, d2, timeout })
            .into()
    } else if !d1.0.is_null() {
        let d = Box::from_raw(d1.0 as *mut Box<dyn DynamicDatagram>);
        cast_mut!(cnt.0, Cnt)
            .send(DynamicDatagramPack { d, timeout })
            .into()
    } else if !d2.0.is_null() {
        let d = Box::from_raw(d2.0 as *mut Box<dyn DynamicDatagram>);
        cast_mut!(cnt.0, Cnt)
            .send(DynamicDatagramPack { d, timeout })
            .into()
    } else {
        Result::<bool, AUTDError>::Ok(false).into()
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerSendSpecial(
    cnt: ControllerPtr,
    special: DatagramSpecialPtr,
    timeout_ns: i64,
) -> ResultI32 {
    if special.0.is_null() {
        return Result::<bool, AUTDError>::Ok(false).into();
    }
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    let d = Box::from_raw(special.0 as *mut Box<dyn DynamicDatagram>);
    cast_mut!(cnt.0, Cnt)
        .send(DynamicDatagramPack { d, timeout })
        .into()
}

type K = i32;
type V = (
    Box<dyn driver::operation::Operation>,
    Box<dyn driver::operation::Operation>,
    Option<Duration>,
);
type M = std::collections::HashMap<K, V>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultGroupKVMap {
    pub result: ConstPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroupCreateKVMap() -> ResultGroupKVMap {
    ResultGroupKVMap {
        result: Box::into_raw(Box::<M>::default()) as _,
        err_len: 0,
        err: std::ptr::null_mut(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroupKVMapSet(
    map: ResultGroupKVMap,
    key: i32,
    d1: DatagramPtr,
    d2: DatagramPtr,
    timeout_ns: i64,
) -> ResultGroupKVMap {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    let mut map = Box::from_raw(map.result as *mut M);
    if !d1.0.is_null() && !d2.0.is_null() {
        let d1 = Box::from_raw(d1.0 as *mut Box<dyn DynamicDatagram>);
        let d2 = Box::from_raw(d2.0 as *mut Box<dyn DynamicDatagram>);
        let d = DynamicDatagramPack2 { d1, d2, timeout };
        match d.operation() {
            Ok((op1, op2)) => map.insert(key, (op1, op2, timeout)),
            Err(e) => {
                let err = e.to_string();
                return ResultGroupKVMap {
                    result: std::ptr::null(),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                };
            }
        };
    } else if !d1.0.is_null() {
        let d = Box::from_raw(d1.0 as *mut Box<dyn DynamicDatagram>);
        let d = DynamicDatagramPack { d, timeout };
        match d.operation() {
            Ok((op1, op2)) => map.insert(key, (op1, op2, timeout)),
            Err(e) => {
                let err = e.to_string();
                return ResultGroupKVMap {
                    result: std::ptr::null(),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                };
            }
        };
    } else if !d2.0.is_null() {
        let d = Box::from_raw(d2.0 as *mut Box<dyn DynamicDatagram>);
        let d = DynamicDatagramPack { d, timeout };
        match d.operation() {
            Ok((op1, op2)) => map.insert(key, (op1, op2, timeout)),
            Err(e) => {
                let err = e.to_string();
                return ResultGroupKVMap {
                    result: std::ptr::null(),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                };
            }
        };
    }
    ResultGroupKVMap {
        result: Box::into_raw(map) as _,
        err_len: 0,
        err: std::ptr::null_mut(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroupKVMapSetSpecial(
    map: ResultGroupKVMap,
    key: i32,
    special: DatagramSpecialPtr,
    timeout_ns: i64,
) -> ResultGroupKVMap {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    let mut map = Box::from_raw(map.result as *mut M);

    let d = Box::from_raw(special.0 as *mut Box<dyn DynamicDatagram>);
    let d = DynamicDatagramPack { d, timeout };
    match d.operation() {
        Ok((op1, op2)) => map.insert(key, (op1, op2, timeout)),
        Err(e) => {
            let err = e.to_string();
            return ResultGroupKVMap {
                result: std::ptr::null(),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            };
        }
    };
    ResultGroupKVMap {
        result: Box::into_raw(map) as _,
        err_len: 0,
        err: std::ptr::null_mut(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroup(
    cnt: ControllerPtr,
    map: *const i32,
    kv_map: ResultGroupKVMap,
) -> ResultI32 {
    let kv_map = Box::from_raw(kv_map.result as *mut M);
    kv_map
        .into_iter()
        .try_fold(
            cast_mut!(cnt.0, Cnt).group(|dev| {
                let k = map.add(dev.idx()).read();
                if k < 0 {
                    None
                } else {
                    Some(k)
                }
            }),
            |acc, (k, (op1, op2, timeout))| acc.set_boxed_op(k, op1, op2, timeout),
        )
        .and_then(|g| g.send())
        .into()
}

#[cfg(test)]
mod tests {
    use autd3capi_def::{DatagramPtr, AUTD3_TRUE};

    use super::*;

    use crate::link::nop::*;

    pub unsafe fn make_nop_link() -> LinkBuilderPtr {
        AUTDLinkNop()
    }

    pub unsafe fn create_controller() -> ControllerPtr {
        let builder = AUTDControllerBuilder();
        let builder = AUTDControllerBuilderAddDevice(builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let builder =
            AUTDControllerBuilderAddDeviceQuaternion(builder, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

        let link = make_nop_link();
        let cnt = AUTDControllerOpenWith(builder, link);
        assert_ne!(cnt.result.0, NULL);
        cnt.result
    }

    #[test]
    fn drive_test() {
        assert_eq!(
            std::mem::size_of::<autd3_driver::common::Drive>(),
            std::mem::size_of::<Drive>()
        );
        assert_eq!(
            memoffset::offset_of!(autd3_driver::common::Drive, phase),
            memoffset::offset_of!(Drive, phase)
        );
        assert_eq!(
            memoffset::offset_of!(autd3_driver::common::Drive, amp),
            memoffset::offset_of!(Drive, amp)
        );
    }

    #[test]
    fn basic() {
        unsafe {
            let cnt = create_controller();

            let firm_p = AUTDControllerFirmwareInfoListPointer(cnt);
            assert_ne!(firm_p.result, NULL);
            (0..2).for_each(|i| {
                let mut info = vec![c_char::default(); 256];
                AUTDControllerFirmwareInfoGet(firm_p, i as _, info.as_mut_ptr());
            });
            AUTDControllerFirmwareInfoListPointerDelete(firm_p);

            let mut fpga_info = vec![0xFFu8; 2];
            let res = AUTDControllerFPGAInfo(cnt, fpga_info.as_mut_ptr() as _);
            assert_eq!(res.result, AUTD3_TRUE);
            assert_eq!(fpga_info[0], 0x00);
            assert_eq!(fpga_info[1], 0x00);

            let s = AUTDDatagramSynchronize();
            let r = AUTDControllerSend(cnt, s, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            let s = AUTDDatagramClear();
            let r = AUTDControllerSend(cnt, s, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            let s = AUTDDatagramUpdateFlags();
            let r = AUTDControllerSend(cnt, s, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            let s = AUTDDatagramStop();
            let r = AUTDControllerSendSpecial(cnt, s, -1);
            assert_eq!(r.result, AUTD3_TRUE);

            let s = AUTDDatagramConfigureModDelay();
            let r = AUTDControllerSend(cnt, s, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            let s = AUTDDatagramSilencer(10);
            let r = AUTDControllerSend(cnt, s, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            assert_eq!(AUTDControllerClose(cnt).result, AUTD3_TRUE);

            AUTDControllerDelete(cnt);
        }
    }
}
