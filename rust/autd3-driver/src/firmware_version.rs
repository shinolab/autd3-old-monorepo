/*
 * File: firmware_version.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::fmt;

use crate::{VERSION_NUM_MAJOR, VERSION_NUM_MINOR};

const ENABLED_STM_BIT: u8 = 1 << 0;
const ENABLED_MODULATOR_BIT: u8 = 1 << 1;
const ENABLED_SILENCER_BIT: u8 = 1 << 2;
const ENABLED_MOD_DELAY_BIT: u8 = 1 << 3;
const ENABLED_EMULATOR_BIT: u8 = 1 << 7;

pub struct FirmwareInfo {
    idx: usize,
    cpu_version_number_major: u8,
    fpga_version_number_major: u8,
    cpu_version_number_minor: u8,
    fpga_version_number_minor: u8,
    fpga_function_bits: u8,
}

impl FirmwareInfo {
    pub fn new(
        idx: usize,
        cpu_version_number_major: u8,
        fpga_version_number_major: u8,
        cpu_version_number_minor: u8,
        fpga_version_number_minor: u8,
        fpga_function_bits: u8,
    ) -> Self {
        Self {
            idx,
            cpu_version_number_major,
            fpga_version_number_major,
            cpu_version_number_minor,
            fpga_version_number_minor,
            fpga_function_bits,
        }
    }

    pub fn cpu_version(&self) -> String {
        Self::firmware_version_map(self.cpu_version_number_major, self.cpu_version_number_minor)
    }

    pub fn fpga_version(&self) -> String {
        Self::firmware_version_map(
            self.fpga_version_number_major,
            self.fpga_version_number_minor,
        )
    }

    pub fn stm_enabled(&self) -> bool {
        (self.fpga_function_bits & ENABLED_STM_BIT) == ENABLED_STM_BIT
    }

    pub fn modulator_enabled(&self) -> bool {
        (self.fpga_function_bits & ENABLED_MODULATOR_BIT) == ENABLED_MODULATOR_BIT
    }

    pub fn silencer_enabled(&self) -> bool {
        (self.fpga_function_bits & ENABLED_SILENCER_BIT) == ENABLED_SILENCER_BIT
    }

    pub fn modulation_delay_enabled(&self) -> bool {
        (self.fpga_function_bits & ENABLED_MOD_DELAY_BIT) == ENABLED_MOD_DELAY_BIT
    }

    pub fn is_emulator(&self) -> bool {
        (self.fpga_function_bits & ENABLED_EMULATOR_BIT) == ENABLED_EMULATOR_BIT
    }

    fn firmware_version_map(version_number_major: u8, version_number_minor: u8) -> String {
        match version_number_major {
            0 => "older than v0.4".to_string(),
            0x01..=0x06 => format!("v0.{}", version_number_major + 3),
            0x0A..=0x15 => format!("v1.{}", version_number_major - 0x0A),
            0x80..=0x88 => format!(
                "v2.{}.{}",
                version_number_major - 0x80,
                version_number_minor
            ),
            _ => format!("unknown: {version_number_major}"),
        }
    }

    pub fn matched_version(&self) -> bool {
        self.cpu_version_number_major == self.fpga_version_number_major
            && self.cpu_version_number_minor == self.fpga_version_number_minor
    }

    pub fn is_supported(&self) -> bool {
        self.cpu_version_number_major == VERSION_NUM_MAJOR
            && self.fpga_version_number_major == VERSION_NUM_MAJOR
            && self.cpu_version_number_minor == VERSION_NUM_MINOR
            && self.fpga_version_number_minor == VERSION_NUM_MINOR
    }

    pub fn latest_version() -> String {
        Self::firmware_version_map(VERSION_NUM_MAJOR, VERSION_NUM_MINOR)
    }
}

impl fmt::Display for FirmwareInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r"{}: CPU = {}, FPGA = {} (STM = {}, Modulator = {}, Silencer = {}, ModDelay = {}){}",
            self.idx,
            self.cpu_version(),
            self.fpga_version(),
            self.stm_enabled(),
            self.modulator_enabled(),
            self.silencer_enabled(),
            self.modulation_delay_enabled(),
            if self.is_emulator() {
                " [Emulator]"
            } else {
                ""
            }
        )
    }
}
