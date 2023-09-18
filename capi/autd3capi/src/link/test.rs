/*
 * File: test.rs
 * Project: link
 * Created Date: 17/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/09/2023
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
    cast!(test.0, Box<Test>)[idx as usize].idx() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestCpuNumTransducers(test: TestLinkPtr, idx: u32) -> u32 {
    cast!(test.0, Box<Test>)[idx as usize].num_transducers() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestCpuAck(test: TestLinkPtr, idx: u32) -> u8 {
    cast!(test.0, Box<Test>)[idx as usize].ack()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestCpuRxData(test: TestLinkPtr, idx: u32) -> u8 {
    cast!(test.0, Box<Test>)[idx as usize].rx_data()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestCpuFpgaFlags(test: TestLinkPtr, idx: u32) -> u8 {
    cast!(test.0, Box<Test>)[idx as usize].fpga_flags().bits()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkTestFpgaAssertThermalSensor(test: TestLinkPtr, idx: u32) {
    cast_mut!(test.0, Box<Test>)[idx as usize]
        .fpga_mut()
        .assert_thermal_sensor()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestFpgaDeassertThermalSensor(test: TestLinkPtr, idx: u32) {
    cast_mut!(test.0, Box<Test>)[idx as usize]
        .fpga_mut()
        .deassert_thermal_sensor()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestFpgaIsLegacyMode(test: TestLinkPtr, idx: u32) -> bool {
    cast!(test.0, Box<Test>)[idx as usize]
        .fpga()
        .is_legacy_mode()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestFpgaIsForceFan(test: TestLinkPtr, idx: u32) -> bool {
    cast!(test.0, Box<Test>)[idx as usize].fpga().is_force_fan()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestFpgaIsStmMode(test: TestLinkPtr, idx: u32) -> bool {
    cast!(test.0, Box<Test>)[idx as usize].fpga().is_stm_mode()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestFpgaIsStmGainMode(test: TestLinkPtr, idx: u32) -> bool {
    cast!(test.0, Box<Test>)[idx as usize]
        .fpga()
        .is_stm_gain_mode()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestFpgaSilencerStep(test: TestLinkPtr, idx: u32) -> u16 {
    cast!(test.0, Box<Test>)[idx as usize]
        .fpga()
        .silencer_step()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkTestFpgaCycles(test: TestLinkPtr, idx: u32, cycles: *mut u16) {
    std::ptr::copy_nonoverlapping(
        cast!(test.0, Box<Test>)[idx as usize]
            .fpga()
            .cycles()
            .as_ptr(),
        cycles,
        cast!(test.0, Box<Test>)[idx as usize].fpga().cycles().len(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkTestFpgaModDelays(test: TestLinkPtr, idx: u32, delay: *mut u16) {
    std::ptr::copy_nonoverlapping(
        cast!(test.0, Box<Test>)[idx as usize]
            .fpga()
            .mod_delays()
            .as_ptr(),
        delay,
        cast!(test.0, Box<Test>)[idx as usize]
            .fpga()
            .mod_delays()
            .len(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkTestFpgaDutyFilters(
    test: TestLinkPtr,
    idx: u32,
    filters: *mut i16,
) {
    std::ptr::copy_nonoverlapping(
        cast!(test.0, Box<Test>)[idx as usize]
            .fpga()
            .duty_filters()
            .as_ptr(),
        filters,
        cast!(test.0, Box<Test>)[idx as usize]
            .fpga()
            .duty_filters()
            .len(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkTestFpgaPhaseFilters(
    test: TestLinkPtr,
    idx: u32,
    filters: *mut i16,
) {
    std::ptr::copy_nonoverlapping(
        cast!(test.0, Box<Test>)[idx as usize]
            .fpga()
            .phase_filters()
            .as_ptr(),
        filters,
        cast!(test.0, Box<Test>)[idx as usize]
            .fpga()
            .phase_filters()
            .len(),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestFpgaStmFrequencyDivision(test: TestLinkPtr, idx: u32) -> u32 {
    cast!(test.0, Box<Test>)[idx as usize]
        .fpga()
        .stm_frequency_division()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestFpgaStmCycle(test: TestLinkPtr, idx: u32) -> u32 {
    cast!(test.0, Box<Test>)[idx as usize].fpga().stm_cycle() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestFpgaSoundSpeed(test: TestLinkPtr, idx: u32) -> u32 {
    cast!(test.0, Box<Test>)[idx as usize].fpga().sound_speed()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestFpgaStmStartIdx(test: TestLinkPtr, idx: u32) -> i32 {
    cast!(test.0, Box<Test>)[idx as usize]
        .fpga()
        .stm_start_idx()
        .map_or(-1, |v| v as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestFpgaStmFinishIdx(test: TestLinkPtr, idx: u32) -> i32 {
    cast!(test.0, Box<Test>)[idx as usize]
        .fpga()
        .stm_finish_idx()
        .map_or(-1, |v| v as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestFpgaModulationFrequencyDivision(
    test: TestLinkPtr,
    idx: u32,
) -> u32 {
    cast!(test.0, Box<Test>)[idx as usize]
        .fpga()
        .modulation_frequency_division()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTestFpgaModulationCycle(test: TestLinkPtr, idx: u32) -> u32 {
    cast!(test.0, Box<Test>)[idx as usize]
        .fpga()
        .modulation_cycle() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkTestFpgaModulation(test: TestLinkPtr, idx: u32, data: *mut u8) {
    std::ptr::copy_nonoverlapping(
        cast!(test.0, Box<Test>)[idx as usize]
            .fpga()
            .modulation()
            .as_ptr(),
        data,
        cast!(test.0, Box<Test>)[idx as usize]
            .fpga()
            .modulation()
            .len(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkTestFpgaDutiesAndPhases(
    test: TestLinkPtr,
    idx: u32,
    stm_idx: u32,
    duties: *mut u16,
    phases: *mut u16,
) {
    let dp = cast!(test.0, Box<Test>)[idx as usize]
        .fpga()
        .duties_and_phases(stm_idx as _);
    let d = dp.iter().map(|v| v.0).collect::<Vec<_>>();
    let p = dp.iter().map(|v| v.1).collect::<Vec<_>>();
    std::ptr::copy_nonoverlapping(d.as_ptr(), duties, d.len());
    std::ptr::copy_nonoverlapping(p.as_ptr(), phases, p.len());
}

#[cfg(test)]
mod tests {
    use autd3capi_def::TransMode;
    use driver::fpga::FPGAControlFlags;

    use crate::{
        gain::{null::AUTDGainNull, AUTDGainIntoDatagram},
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

    #[test]
    fn test_fpga_thermal_sensor() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            AUTDLinkTestFpgaAssertThermalSensor(link, 0);
        }
    }

    #[test]
    fn test_fpga_is_legacy_mode() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            assert!(!AUTDLinkTestFpgaIsLegacyMode(link, 0));
            assert!(!AUTDLinkTestFpgaIsLegacyMode(link, 1));

            let gain = AUTDGainNull();
            let gain = AUTDGainIntoDatagram(gain);
            let mut err = vec![c_char::default(); 256];
            let _ = AUTDSend(
                cnt,
                TransMode::Legacy,
                gain,
                DatagramPtr(std::ptr::null()),
                -1,
                err.as_mut_ptr(),
            );

            assert!(AUTDLinkTestFpgaIsLegacyMode(link, 0));
            assert!(AUTDLinkTestFpgaIsLegacyMode(link, 1));
        }
    }

    #[test]
    fn test_fpga_is_force_fan() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            assert!(!AUTDLinkTestFpgaIsForceFan(link, 0));
            assert!(!AUTDLinkTestFpgaIsForceFan(link, 1));

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

            assert!(AUTDLinkTestFpgaIsForceFan(link, 0));
            assert!(AUTDLinkTestFpgaIsForceFan(link, 1));
        }
    }

    #[test]
    fn test_fpga_is_stm_mode() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            assert!(!AUTDLinkTestFpgaIsStmMode(link, 0));
            assert!(!AUTDLinkTestFpgaIsStmMode(link, 1));
        }
    }

    #[test]
    fn test_fpga_is_stm_gain_mode() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            assert!(!AUTDLinkTestFpgaIsStmGainMode(link, 0));
            assert!(!AUTDLinkTestFpgaIsStmGainMode(link, 1));
        }
    }

    #[test]
    fn test_fpga_silencer_step() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            assert_eq!(AUTDLinkTestFpgaSilencerStep(link, 0), 10);
            assert_eq!(AUTDLinkTestFpgaSilencerStep(link, 1), 10);
        }
    }

    #[test]
    fn test_fpga_cycles() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            (0..2).for_each(|i| {
                let n = AUTDLinkTestCpuNumTransducers(link, i);

                let mut cycles = vec![0; n as usize];
                AUTDLinkTestFpgaCycles(link, i, cycles.as_mut_ptr());

                cycles.iter().for_each(|&v| assert_eq!(v, 4096));
            })
        }
    }

    #[test]
    fn test_fpga_mod_delays() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            (0..2).for_each(|i| {
                let n = AUTDLinkTestCpuNumTransducers(link, i);

                let mut delays = vec![0; n as usize];
                AUTDLinkTestFpgaModDelays(link, i, delays.as_mut_ptr());

                delays.iter().for_each(|&v| assert_eq!(v, 0));
            })
        }
    }

    #[test]
    fn test_fpga_duty_filter() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            (0..2).for_each(|i| {
                let n = AUTDLinkTestCpuNumTransducers(link, i);

                let mut filters = vec![0; n as usize];
                AUTDLinkTestFpgaDutyFilters(link, i, filters.as_mut_ptr());

                filters.iter().for_each(|&v| assert_eq!(v, 0));
            })
        }
    }

    #[test]
    fn test_fpga_phase_filter() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            (0..2).for_each(|i| {
                let n = AUTDLinkTestCpuNumTransducers(link, i);

                let mut filters = vec![0; n as usize];
                AUTDLinkTestFpgaPhaseFilters(link, i, filters.as_mut_ptr());

                filters.iter().for_each(|&v| assert_eq!(v, 0));
            })
        }
    }

    #[test]
    fn test_fpga_stm_freq_div() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            (0..2).for_each(|i| {
                assert_eq!(AUTDLinkTestFpgaStmFrequencyDivision(link, i), 0);
            })
        }
    }

    #[test]
    fn test_fpga_stm_cycle() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            (0..2).for_each(|i| {
                assert_eq!(AUTDLinkTestFpgaStmCycle(link, i), 1);
            })
        }
    }

    #[test]
    fn test_fpga_stm_sound_speed() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            (0..2).for_each(|i| {
                assert_eq!(AUTDLinkTestFpgaSoundSpeed(link, i), 0);
            })
        }
    }

    #[test]
    fn test_fpga_stm_start_idx() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            (0..2).for_each(|i| {
                assert_eq!(AUTDLinkTestFpgaStmStartIdx(link, i), -1);
            })
        }
    }

    #[test]
    fn test_fpga_stm_finish_idx() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            (0..2).for_each(|i| {
                assert_eq!(AUTDLinkTestFpgaStmFinishIdx(link, i), -1);
            })
        }
    }

    #[test]
    fn test_fpga_modulation_freq_div() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            (0..2).for_each(|i| {
                assert_eq!(AUTDLinkTestFpgaModulationFrequencyDivision(link, i), 40960);
            })
        }
    }

    #[test]
    fn test_fpga_modulation_cycle() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            (0..2).for_each(|i| {
                assert_eq!(AUTDLinkTestFpgaModulationCycle(link, i), 2);
            })
        }
    }

    #[test]
    fn test_fpga_modulation() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            (0..2).for_each(|i| {
                let n = AUTDLinkTestFpgaModulationCycle(link, i);
                let mut data = vec![0; n as usize];
                AUTDLinkTestFpgaModulation(link, i, data.as_mut_ptr());
                data.iter().for_each(|&v| assert_eq!(v, 0));
            })
        }
    }

    #[test]
    fn test_fpga_duties_and_phases() {
        unsafe {
            let cnt = create_controller();
            let link = AUTDGetLink(cnt);

            (0..2).for_each(|i| {
                let n = AUTDLinkTestCpuNumTransducers(link, i);

                let mut duties = vec![0; n as usize];
                let mut phases = vec![0; n as usize];
                AUTDLinkTestFpgaDutiesAndPhases(
                    link,
                    i,
                    0,
                    duties.as_mut_ptr(),
                    phases.as_mut_ptr(),
                );
                duties.iter().for_each(|&v| assert_eq!(v, 0));
                phases.iter().for_each(|&v| assert_eq!(v, 0));
            })
        }
    }
}
