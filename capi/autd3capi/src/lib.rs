/*
 * File: lib.rs
 * Project: src
 * Created Date: 11/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/11/2023
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

use async_ffi::{FfiFuture, FutureExt};

use autd3capi_def::{
    common::{driver::fpga::FPGAInfo, *},
    ControllerPtr, DatagramPtr, DatagramSpecialPtr, GroupKVMapPtr, LinkBuilderPtr,
    ResultPtrWrapper, Resulti32Wrapper, RuntimePtr,
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
    runtime: RuntimePtr,
    builder: ControllerBuilderPtr,
    link_builder: LinkBuilderPtr,
    err: *mut c_char,
) -> ControllerPtr {
    let runtime = cast!(runtime, tokio::runtime::Runtime);
    let link_builder: Box<Box<dyn DynamicLinkBuilder>> =
        Box::from_raw(link_builder.0 as *mut Box<dyn DynamicLinkBuilder>);
    let cnt_builder =
        Box::from_raw(builder.0 as *mut autd3::controller::builder::ControllerBuilder);
    let cnt = try_or_return!(
        runtime.block_on(cnt_builder.open_with(*link_builder)),
        err,
        ControllerPtr(NULL)
    );
    ControllerPtr(Box::into_raw(Box::new(cnt)) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerCloseAsync(
    cnt: ControllerPtr,
) -> FfiFuture<Resulti32Wrapper> {
    let cnt = cast_mut!(cnt, Cnt);
    async move { cnt.close().await.into() }.into_ffi()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerDelete(cnt: ControllerPtr) {
    let _ = Box::from_raw(cnt.0 as *mut Cnt);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFPGAInfoAsync(
    cnt: ControllerPtr,
) -> FfiFuture<ResultPtrWrapper> {
    let cnt = cast_mut!(cnt, Cnt);
    async move {
        cnt.fpga_info()
            .await
            .map(|info| Box::into_raw(Box::new(info)) as *const _)
            .into()
    }
    .into_ffi()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFPGAInfoAwaitResult(
    runtime: RuntimePtr,
    future: FfiFuture<ResultPtrWrapper>,
    out: *mut u8,
) -> ResultPtrWrapper {
    let runtime = cast!(runtime, tokio::runtime::Runtime);
    let res = runtime.block_on(future);
    if !res.result.is_null() {
        let info = *Box::from_raw(res.result as *mut Vec<FPGAInfo>);
        std::ptr::copy_nonoverlapping(info.as_ptr() as _, out, info.len());
    }
    res
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FirmwareInfoListPtr(pub ConstPtr);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFirmwareInfoListPointerAsync(
    cnt: ControllerPtr,
) -> FfiFuture<ResultPtrWrapper> {
    let cnt = cast_mut!(cnt, Cnt);
    async move {
        cnt.firmware_infos()
            .await
            .map(|firmware_infos| Box::into_raw(Box::new(firmware_infos)) as ConstPtr)
            .into()
    }
    .into_ffi()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFirmwareInfoListPointerAwaitResult(
    runtime: RuntimePtr,
    future: FfiFuture<ResultPtrWrapper>,
) -> ResultPtrWrapper {
    let runtime = cast!(runtime, tokio::runtime::Runtime);
    runtime.block_on(future)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerFirmwareInfoGet(
    p_info_list: ResultPtrWrapper,
    idx: u32,
    info: *mut c_char,
) {
    let firm_info = &(p_info_list.result as *mut Vec<FirmwareInfo>)
        .as_ref()
        .unwrap()[idx as usize];
    let info_str = std::ffi::CString::new(firm_info.to_string()).unwrap();
    libc::strcpy(info, info_str.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerFirmwareInfoListPointerDelete(
    p_info_list: ResultPtrWrapper,
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
pub unsafe extern "C" fn AUTDControllerSendAsync(
    cnt: ControllerPtr,
    d1: DatagramPtr,
    d2: DatagramPtr,
    timeout_ns: i64,
) -> FfiFuture<Resulti32Wrapper> {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    let cnt = cast_mut!(cnt, Cnt);
    if !d1.0.is_null() && !d2.0.is_null() {
        let d1 = Box::from_raw(d1.0 as *mut Box<dyn DynamicDatagram>);
        let d2 = Box::from_raw(d2.0 as *mut Box<dyn DynamicDatagram>);
        let pack = DynamicDatagramPack2 { d1, d2, timeout };
        async move { cnt.send(pack).await.into() }.into_ffi()
    } else if !d1.0.is_null() {
        let d = Box::from_raw(d1.0 as *mut Box<dyn DynamicDatagram>);
        let pack = DynamicDatagramPack { d, timeout };
        async move { cnt.send(pack).await.into() }.into_ffi()
    } else if !d2.0.is_null() {
        let d = Box::from_raw(d2.0 as *mut Box<dyn DynamicDatagram>);
        let pack = DynamicDatagramPack { d, timeout };
        async move { cnt.send(pack).await.into() }.into_ffi()
    } else {
        async move { Result::<bool, AUTDInternalError>::Ok(false).into() }.into_ffi()
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerSendSpecialAsync(
    cnt: ControllerPtr,
    special: DatagramSpecialPtr,
    timeout_ns: i64,
) -> FfiFuture<Resulti32Wrapper> {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    let cnt = cast_mut!(cnt, Cnt);
    let d = Box::from_raw(special.0 as *mut Box<dyn DynamicDatagram>);
    let pack = DynamicDatagramPack { d, timeout };
    async move { cnt.send(pack).await.into() }.into_ffi()
}

type K = i32;
type V = (
    Box<dyn driver::operation::Operation>,
    Box<dyn driver::operation::Operation>,
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
    timeout_ns: i64,
    err: *mut c_char,
) -> GroupKVMapPtr {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    let mut map = Box::from_raw(map.0 as *mut M);
    if !d1.0.is_null() && !d2.0.is_null() {
        let d1 = Box::from_raw(d1.0 as *mut Box<dyn DynamicDatagram>);
        let d2 = Box::from_raw(d2.0 as *mut Box<dyn DynamicDatagram>);
        let (op1, op2) = try_or_return!(
            DynamicDatagramPack2 { d1, d2, timeout }.operation(),
            err,
            GroupKVMapPtr(NULL)
        );
        map.insert(key, (op1, op2, timeout));
    } else if !d1.0.is_null() {
        let d = Box::from_raw(d1.0 as *mut Box<dyn DynamicDatagram>);
        let (op1, op2) = try_or_return!(
            DynamicDatagramPack { d, timeout }.operation(),
            err,
            GroupKVMapPtr(NULL)
        );
        map.insert(key, (op1, op2, timeout));
    } else if !d2.0.is_null() {
        let d = Box::from_raw(d2.0 as *mut Box<dyn DynamicDatagram>);
        let (op1, op2) = try_or_return!(
            DynamicDatagramPack { d, timeout }.operation(),
            err,
            GroupKVMapPtr(NULL)
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
    timeout_ns: i64,
    err: *mut c_char,
) -> GroupKVMapPtr {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    let mut map = Box::from_raw(map.0 as *mut M);

    let d = Box::from_raw(special.0 as *mut Box<dyn DynamicDatagram>);
    let (op1, op2) = try_or_return!(
        DynamicDatagramPack { d, timeout }.operation(),
        err,
        GroupKVMapPtr(NULL)
    );
    map.insert(key, (op1, op2, timeout));

    GroupKVMapPtr(Box::into_raw(map) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroupAsync(
    cnt: ControllerPtr,
    map: *const i32,
    kv_map: GroupKVMapPtr,
) -> FfiFuture<Resulti32Wrapper> {
    let kv_map = Box::from_raw(kv_map.0 as *mut M);
    let cnt = cast_mut!(cnt, Cnt);
    let map = cnt
        .geometry
        .iter()
        .map(|dev| map.add(dev.idx()).read())
        .collect::<Vec<_>>();
    let group = kv_map.into_iter().try_fold(
        cnt.group(move |dev| {
            let k = map[dev.idx()];
            if k < 0 {
                None
            } else {
                Some(k)
            }
        }),
        |acc, (k, (op1, op2, timeout))| acc.set_boxed_op(k, op1, op2, timeout),
    );
    match group {
        Ok(group) => async move { group.send().await.into() }.into_ffi(),
        Err(e) => async move { Result::<(), AUTDInternalError>::Err(e).into() }.into_ffi(),
    }
}

#[cfg(test)]
mod tests {
    use autd3capi_def::DatagramPtr;

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
        let mut err = vec![c_char::default(); 256];
        let cnt = AUTDControllerOpenWith(builder, link, err.as_mut_ptr());
        assert_ne!(cnt.0, NULL);
        cnt
    }

    #[test]
    fn drive_test() {
        assert_eq!(
            std::mem::size_of::<autd3capi_def::common::driver::common::Drive>(),
            std::mem::size_of::<Drive>()
        );
        assert_eq!(
            memoffset::offset_of!(autd3capi_def::common::driver::common::Drive, phase),
            memoffset::offset_of!(Drive, phase)
        );
        assert_eq!(
            memoffset::offset_of!(autd3capi_def::common::driver::common::Drive, amp),
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
                AUTDControllerSend(cnt, s, DatagramPtr(NULL()), -1, err.as_mut_ptr()),
                AUTD3_TRUE
            );

            let s = AUTDDatagramClear();
            assert_eq!(
                AUTDControllerSend(cnt, s, DatagramPtr(NULL()), -1, err.as_mut_ptr()),
                AUTD3_TRUE
            );

            let s = AUTDDatagramUpdateFlags();
            assert_eq!(
                AUTDControllerSend(cnt, s, DatagramPtr(NULL()), -1, err.as_mut_ptr()),
                AUTD3_TRUE
            );

            let s = AUTDDatagramStop();
            assert_eq!(
                AUTDControllerSendSpecial(cnt, s, -1, err.as_mut_ptr()),
                AUTD3_TRUE
            );

            let s = AUTDDatagramConfigureModDelay();
            assert_eq!(
                AUTDControllerSend(cnt, s, DatagramPtr(NULL()), -1, err.as_mut_ptr()),
                AUTD3_TRUE
            );

            let s = AUTDDatagramSilencer(10);
            assert_eq!(
                AUTDControllerSend(cnt, s, DatagramPtr(NULL()), -1, err.as_mut_ptr(),),
                AUTD3_TRUE
            );

            assert!(AUTDControllerClose(cnt, err.as_mut_ptr()));

            AUTDControllerDelete(cnt);
        }
    }
}
