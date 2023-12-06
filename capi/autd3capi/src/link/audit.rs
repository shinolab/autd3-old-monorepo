/*
 * File: audit.rs
 * Project: link
 * Created Date: 18/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{autd3::link::audit::*, driver::link::LinkSync, *};
use std::time::Duration;

#[repr(C)]
pub struct LinkAuditBuilderPtr(pub ConstPtr);

impl LinkAuditBuilderPtr {
    pub fn new(builder: AuditBuilder) -> Self {
        Self(Box::into_raw(Box::new(builder)) as _)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAudit() -> LinkAuditBuilderPtr {
    LinkAuditBuilderPtr::new(Audit::builder())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditWithTimeout(
    audit: LinkAuditBuilderPtr,
    timeout_ns: u64,
) -> LinkAuditBuilderPtr {
    LinkAuditBuilderPtr::new(
        Box::from_raw(audit.0 as *mut AuditBuilder).with_timeout(Duration::from_nanos(timeout_ns)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditIntoBuilder(audit: LinkAuditBuilderPtr) -> LinkBuilderPtr {
    LinkBuilderPtr::new(*Box::from_raw(audit.0 as *mut AuditBuilder))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditIsOpen(audit: LinkPtr) -> bool {
    cast!(audit.0, Box<Audit>).is_open()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditTimeoutNs(audit: LinkPtr) -> u64 {
    cast!(audit.0, Box<Audit>).timeout().as_nanos() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditLastTimeoutNs(audit: LinkPtr) -> u64 {
    cast!(audit.0, Box<Audit>).last_timeout().as_nanos() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditDown(audit: LinkPtr) {
    cast_mut!(audit.0, Box<Audit>).down()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditUp(audit: LinkPtr) {
    cast_mut!(audit.0, Box<Audit>).up()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditBreakDown(audit: LinkPtr) {
    cast_mut!(audit.0, Box<Audit>).break_down()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditRepair(audit: LinkPtr) {
    cast_mut!(audit.0, Box<Audit>).repair()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditCpuUpdate(audit: LinkPtr, idx: u32) {
    cast_mut!(audit.0, Box<Audit>)[idx as usize].update()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditCpuIdx(audit: LinkPtr, idx: u32) -> u32 {
    cast!(audit.0, Box<Audit>)[idx as usize].idx() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditCpuNumTransducers(audit: LinkPtr, idx: u32) -> u32 {
    cast!(audit.0, Box<Audit>)[idx as usize].num_transducers() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditCpuAck(audit: LinkPtr, idx: u32) -> u8 {
    cast!(audit.0, Box<Audit>)[idx as usize].ack()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditCpuRxData(audit: LinkPtr, idx: u32) -> u8 {
    cast!(audit.0, Box<Audit>)[idx as usize].rx_data()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaAssertThermalSensor(audit: LinkPtr, idx: u32) {
    cast_mut!(audit.0, Box<Audit>)[idx as usize]
        .fpga_mut()
        .assert_thermal_sensor()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaDeassertThermalSensor(audit: LinkPtr, idx: u32) {
    cast_mut!(audit.0, Box<Audit>)[idx as usize]
        .fpga_mut()
        .deassert_thermal_sensor()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaIsForceFan(audit: LinkPtr, idx: u32) -> bool {
    cast!(audit.0, Box<Audit>)[idx as usize]
        .fpga()
        .is_force_fan()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaIsStmMode(audit: LinkPtr, idx: u32) -> bool {
    cast!(audit.0, Box<Audit>)[idx as usize]
        .fpga()
        .is_stm_mode()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaIsStmGainMode(audit: LinkPtr, idx: u32) -> bool {
    cast!(audit.0, Box<Audit>)[idx as usize]
        .fpga()
        .is_stm_gain_mode()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerStepIntensity(audit: LinkPtr, idx: u32) -> u16 {
    cast!(audit.0, Box<Audit>)[idx as usize]
        .fpga()
        .silencer_step_intensity()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerStepPhase(audit: LinkPtr, idx: u32) -> u16 {
    cast!(audit.0, Box<Audit>)[idx as usize]
        .fpga()
        .silencer_step_phase()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaDebugOutputIdx(audit: LinkPtr, idx: u32) -> u8 {
    cast!(audit.0, Box<Audit>)[idx as usize]
        .fpga()
        .debug_output_idx()
        .unwrap_or(0xFF)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaModDelays(audit: LinkPtr, idx: u32, delay: *mut u16) {
    std::ptr::copy_nonoverlapping(
        cast!(audit.0, Box<Audit>)[idx as usize]
            .fpga()
            .mod_delays()
            .as_ptr(),
        delay,
        cast!(audit.0, Box<Audit>)[idx as usize]
            .fpga()
            .mod_delays()
            .len(),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaStmFrequencyDivision(audit: LinkPtr, idx: u32) -> u32 {
    cast!(audit.0, Box<Audit>)[idx as usize]
        .fpga()
        .stm_frequency_division()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaStmCycle(audit: LinkPtr, idx: u32) -> u32 {
    cast!(audit.0, Box<Audit>)[idx as usize].fpga().stm_cycle() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSoundSpeed(audit: LinkPtr, idx: u32) -> u32 {
    cast!(audit.0, Box<Audit>)[idx as usize]
        .fpga()
        .sound_speed()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaStmStartIdx(audit: LinkPtr, idx: u32) -> i32 {
    cast!(audit.0, Box<Audit>)[idx as usize]
        .fpga()
        .stm_start_idx()
        .map_or(-1, |v| v as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaStmFinishIdx(audit: LinkPtr, idx: u32) -> i32 {
    cast!(audit.0, Box<Audit>)[idx as usize]
        .fpga()
        .stm_finish_idx()
        .map_or(-1, |v| v as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaModulationFrequencyDivision(
    audit: LinkPtr,
    idx: u32,
) -> u32 {
    cast!(audit.0, Box<Audit>)[idx as usize]
        .fpga()
        .modulation_frequency_division()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaModulationCycle(audit: LinkPtr, idx: u32) -> u32 {
    cast!(audit.0, Box<Audit>)[idx as usize]
        .fpga()
        .modulation_cycle() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaModulation(audit: LinkPtr, idx: u32, data: *mut u8) {
    std::ptr::copy_nonoverlapping(
        cast!(audit.0, Box<Audit>)[idx as usize]
            .fpga()
            .modulation()
            .as_ptr(),
        data,
        cast!(audit.0, Box<Audit>)[idx as usize]
            .fpga()
            .modulation()
            .len(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaIntensitiesAndPhases(
    audit: LinkPtr,
    idx: u32,
    stm_idx: u32,
    intensities: *mut u8,
    phases: *mut u8,
) {
    let dp = cast!(audit.0, Box<Audit>)[idx as usize]
        .fpga()
        .intensities_and_phases(stm_idx as _);
    let d = dp.iter().map(|v| v.0).collect::<Vec<_>>();
    let p = dp.iter().map(|v| v.1).collect::<Vec<_>>();
    std::ptr::copy_nonoverlapping(d.as_ptr(), intensities, d.len());
    std::ptr::copy_nonoverlapping(p.as_ptr(), phases, p.len());
}
