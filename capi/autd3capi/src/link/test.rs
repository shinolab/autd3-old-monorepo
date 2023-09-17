/*
 * File: test.rs
 * Project: link
 * Created Date: 17/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    common::{autd3::link::Test, *},
    take_link, ControllerPtr, LinkPtr,
};
use std::time::Duration;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTest() -> LinkPtr {
    LinkPtr::new(Test::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestWithTimeout(test: LinkPtr, timeout_ns: u64) -> LinkPtr {
    LinkPtr::new(take_link!(test, Test).with_timeout(Duration::from_nanos(timeout_ns)))
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TestLinkPtr(pub ConstPtr);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetLink(cnt: ControllerPtr) -> TestLinkPtr {
    TestLinkPtr(cast!(cnt.0, Cnt).link() as *const _ as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestCpuIdx(test: TestLinkPtr, idx: u32) -> u32 {
    cast!(test.0, Box<Test>).emulators()[idx as usize].idx() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestCpuNumTransducers(test: TestLinkPtr, idx: u32) -> u32 {
    cast!(test.0, Box<Test>).emulators()[idx as usize].num_transducers() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestCpuAck(test: TestLinkPtr, idx: u32) -> u8 {
    cast!(test.0, Box<Test>).emulators()[idx as usize].ack()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestCpuRxData(test: TestLinkPtr, idx: u32) -> u8 {
    cast!(test.0, Box<Test>).emulators()[idx as usize].rx_data()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestCpuFpgaFlags(test: TestLinkPtr, idx: u32) -> u8 {
    cast!(test.0, Box<Test>).emulators()[idx as usize]
        .fpga_flags()
        .bits()
}

#[cfg(test)]
mod tests {
    use autd3capi_def::TransMode;
    use driver::fpga::FPGAControlFlags;

    use crate::{
        geometry::{
            device::{AUTDDeviceSetForceFan, AUTDGetDevice},
            AUTDGetGeometry,
        },
        *,
    };

    use super::*;

    unsafe fn create_controller() -> ControllerPtr {
        let builder = AUTDCreateControllerBuilder();
        let builder = AUTDAddDevice(builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let builder = AUTDAddDevice(builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

        let test = AUTDLinkTest();
        let test = AUTDLinkTestWithTimeout(test, 0);

        let mut err = vec![c_char::default(); 256];
        let cnt = AUTDControllerOpenWith(builder, test, err.as_mut_ptr());
        assert_ne!(cnt.0, NULL);
        cnt
    }

    #[test]
    fn test_link_debug() {
        unsafe {
            let link = AUTDLinkTest();
            let _ = AUTDLinkTestWithTimeout(link, 10);
        }
    }

    #[test]
    fn test_link_cpu_idx() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            assert_eq!(AUTDLinkTestCpuIdx(link, 0), 0);
            assert_eq!(AUTDLinkTestCpuIdx(link, 1), 1);
        }
    }

    #[test]
    fn test_link_cpu_num_transducers() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            assert_eq!(AUTDLinkTestCpuNumTransducers(link, 0), 249);
            assert_eq!(AUTDLinkTestCpuNumTransducers(link, 1), 249);
        }
    }

    #[test]
    fn test_link_cpu_ack() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            assert_eq!(AUTDLinkTestCpuAck(link, 0), 3);
            assert_eq!(AUTDLinkTestCpuAck(link, 1), 3);

            let update = AUTDUpdateFlags();
            let mut err = vec![c_char::default(); 256];
            let _ = AUTDSend(
                cnt,
                TransMode::Legacy,
                update,
                DatagramPtr(std::ptr::null()),
                -1,
                err.as_mut_ptr(),
            );

            assert_eq!(AUTDLinkTestCpuAck(link, 0), 4);
            assert_eq!(AUTDLinkTestCpuAck(link, 1), 4);
        }
    }

    #[test]
    fn test_link_cpu_rx_data() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            assert_eq!(AUTDLinkTestCpuRxData(link, 0), 0);
            assert_eq!(AUTDLinkTestCpuRxData(link, 1), 0);
        }
    }

    #[test]
    fn test_link_cpu_fpga_flags() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            assert_eq!(
                AUTDLinkTestCpuFpgaFlags(link, 0),
                FPGAControlFlags::NONE.bits()
            );
            assert_eq!(
                AUTDLinkTestCpuFpgaFlags(link, 1),
                FPGAControlFlags::NONE.bits()
            );

            AUTDDeviceSetForceFan(AUTDGetDevice(AUTDGetGeometry(cnt), 0), true);
            AUTDDeviceSetForceFan(AUTDGetDevice(AUTDGetGeometry(cnt), 1), true);

            let update = AUTDUpdateFlags();
            let mut err = vec![c_char::default(); 256];
            let _ = AUTDSend(
                cnt,
                TransMode::Legacy,
                update,
                DatagramPtr(std::ptr::null()),
                -1,
                err.as_mut_ptr(),
            );

            assert_eq!(
                AUTDLinkTestCpuFpgaFlags(link, 0),
                FPGAControlFlags::FORCE_FAN.bits()
            );
            assert_eq!(
                AUTDLinkTestCpuFpgaFlags(link, 1),
                FPGAControlFlags::FORCE_FAN.bits()
            );
        }
    }
}
