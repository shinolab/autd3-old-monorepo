/*
 * File: firmware_version.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::fmt;

pub const LATEST_VERSION_NUM_MAJOR: u8 = 0x8A;
pub const LATEST_VERSION_NUM_MINOR: u8 = 0x00;

const ENABLED_STM_BIT: u8 = 1 << 0;
const ENABLED_MODULATOR_BIT: u8 = 1 << 1;
const ENABLED_SILENCER_BIT: u8 = 1 << 2;
const ENABLED_MOD_DELAY_BIT: u8 = 1 << 3;
const ENABLED_FILTER_BIT: u8 = 1 << 4;
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
    #[doc(hidden)]
    pub const fn new(
        idx: usize,
        cpu_version_number_major: u8,
        cpu_version_number_minor: u8,
        fpga_version_number_major: u8,
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

    pub const fn stm_enabled(&self) -> bool {
        (self.fpga_function_bits & ENABLED_STM_BIT) == ENABLED_STM_BIT
    }

    pub const fn modulator_enabled(&self) -> bool {
        (self.fpga_function_bits & ENABLED_MODULATOR_BIT) == ENABLED_MODULATOR_BIT
    }

    pub const fn silencer_enabled(&self) -> bool {
        (self.fpga_function_bits & ENABLED_SILENCER_BIT) == ENABLED_SILENCER_BIT
    }

    pub const fn modulation_delay_enabled(&self) -> bool {
        (self.fpga_function_bits & ENABLED_MOD_DELAY_BIT) == ENABLED_MOD_DELAY_BIT
    }

    pub const fn filter_enabled(&self) -> bool {
        (self.fpga_function_bits & ENABLED_FILTER_BIT) == ENABLED_FILTER_BIT
    }

    pub const fn is_emulator(&self) -> bool {
        (self.fpga_function_bits & ENABLED_EMULATOR_BIT) == ENABLED_EMULATOR_BIT
    }

    fn firmware_version_map(version_number_major: u8, version_number_minor: u8) -> String {
        match version_number_major {
            0 => "older than v0.4".to_string(),
            0x01..=0x06 => format!("v0.{}", version_number_major + 3),
            0x0A..=0x15 => format!("v1.{}", version_number_major - 0x0A),
            0x80..=0x89 => format!(
                "v2.{}.{}",
                version_number_major - 0x80,
                version_number_minor
            ),
            0x8A..=0x8A => format!(
                "v3.{}.{}",
                version_number_major - 0x8A,
                version_number_minor
            ),
            _ => format!("unknown ({version_number_major})"),
        }
    }

    pub fn latest_version() -> String {
        Self::firmware_version_map(LATEST_VERSION_NUM_MAJOR, LATEST_VERSION_NUM_MINOR)
    }

    pub const fn cpu_version_number_major(&self) -> u8 {
        self.cpu_version_number_major
    }

    pub const fn cpu_version_number_minor(&self) -> u8 {
        self.cpu_version_number_minor
    }

    pub const fn fpga_version_number_major(&self) -> u8 {
        self.fpga_version_number_major
    }

    pub const fn fpga_version_number_minor(&self) -> u8 {
        self.fpga_version_number_minor
    }

    pub const fn fpga_function_bits(&self) -> u8 {
        self.fpga_function_bits
    }
}

impl fmt::Display for FirmwareInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r"{}: CPU = {}, FPGA = {} {}",
            self.idx,
            self.cpu_version(),
            self.fpga_version(),
            if self.is_emulator() {
                " [Emulator]"
            } else {
                ""
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn firmware_version() {
        let info = FirmwareInfo::new(0, 0, 0, 0, 0, 0);
        assert_eq!("older than v0.4", info.cpu_version());
        assert_eq!("older than v0.4", info.fpga_version());

        let info = FirmwareInfo::new(0, 1, 0, 1, 0, 0);
        assert_eq!("v0.4", info.cpu_version());
        assert_eq!("v0.4", info.fpga_version());

        let info = FirmwareInfo::new(0, 2, 0, 2, 0, 0);
        assert_eq!("v0.5", info.cpu_version());
        assert_eq!("v0.5", info.fpga_version());

        let info = FirmwareInfo::new(0, 3, 0, 3, 0, 0);
        assert_eq!("v0.6", info.cpu_version());
        assert_eq!("v0.6", info.fpga_version());

        let info = FirmwareInfo::new(0, 4, 0, 4, 0, 0);
        assert_eq!("v0.7", info.cpu_version());
        assert_eq!("v0.7", info.fpga_version());

        let info = FirmwareInfo::new(0, 5, 0, 5, 0, 0);
        assert_eq!("v0.8", info.cpu_version());
        assert_eq!("v0.8", info.fpga_version());

        let info = FirmwareInfo::new(0, 6, 0, 6, 0, 0);
        assert_eq!("v0.9", info.cpu_version());
        assert_eq!("v0.9", info.fpga_version());

        let info = FirmwareInfo::new(0, 7, 0, 7, 0, 0);
        assert_eq!("unknown (7)", info.cpu_version());
        assert_eq!("unknown (7)", info.fpga_version());

        let info = FirmwareInfo::new(0, 8, 0, 8, 0, 0);
        assert_eq!("unknown (8)", info.cpu_version());
        assert_eq!("unknown (8)", info.fpga_version());

        let info = FirmwareInfo::new(0, 9, 0, 9, 0, 0);
        assert_eq!("unknown (9)", info.cpu_version());
        assert_eq!("unknown (9)", info.fpga_version());

        let info = FirmwareInfo::new(0, 10, 0, 10, 0, 0);
        assert_eq!("v1.0", info.cpu_version());
        assert_eq!("v1.0", info.fpga_version());

        let info = FirmwareInfo::new(0, 11, 0, 11, 0, 0);
        assert_eq!("v1.1", info.cpu_version());
        assert_eq!("v1.1", info.fpga_version());

        let info = FirmwareInfo::new(0, 12, 0, 12, 0, 0);
        assert_eq!("v1.2", info.cpu_version());
        assert_eq!("v1.2", info.fpga_version());

        let info = FirmwareInfo::new(0, 13, 0, 13, 0, 0);
        assert_eq!("v1.3", info.cpu_version());
        assert_eq!("v1.3", info.fpga_version());

        let info = FirmwareInfo::new(0, 14, 0, 14, 0, 0);
        assert_eq!("v1.4", info.cpu_version());
        assert_eq!("v1.4", info.fpga_version());

        let info = FirmwareInfo::new(0, 15, 0, 15, 0, 0);
        assert_eq!("v1.5", info.cpu_version());
        assert_eq!("v1.5", info.fpga_version());

        let info = FirmwareInfo::new(0, 16, 0, 16, 0, 0);
        assert_eq!("v1.6", info.cpu_version());
        assert_eq!("v1.6", info.fpga_version());

        let info = FirmwareInfo::new(0, 17, 0, 17, 0, 0);
        assert_eq!("v1.7", info.cpu_version());
        assert_eq!("v1.7", info.fpga_version());

        let info = FirmwareInfo::new(0, 18, 0, 18, 0, 0);
        assert_eq!("v1.8", info.cpu_version());
        assert_eq!("v1.8", info.fpga_version());

        let info = FirmwareInfo::new(0, 19, 0, 19, 0, 0);
        assert_eq!("v1.9", info.cpu_version());
        assert_eq!("v1.9", info.fpga_version());

        let info = FirmwareInfo::new(0, 20, 0, 20, 0, 0);
        assert_eq!("v1.10", info.cpu_version());
        assert_eq!("v1.10", info.fpga_version());

        let info = FirmwareInfo::new(0, 21, 0, 21, 0, 0);
        assert_eq!("v1.11", info.cpu_version());
        assert_eq!("v1.11", info.fpga_version());

        let info = FirmwareInfo::new(0, 128, 0, 128, 0, 0);
        assert_eq!("v2.0.0", info.cpu_version());
        assert_eq!("v2.0.0", info.fpga_version());

        let info = FirmwareInfo::new(0, 129, 0, 129, 0, 0);
        assert_eq!("v2.1.0", info.cpu_version());
        assert_eq!("v2.1.0", info.fpga_version());

        let info = FirmwareInfo::new(0, 130, 0, 130, 0, 0);
        assert_eq!("v2.2.0", info.cpu_version());
        assert_eq!("v2.2.0", info.fpga_version());

        let info = FirmwareInfo::new(0, 131, 0, 131, 0, 0);
        assert_eq!("v2.3.0", info.cpu_version());
        assert_eq!("v2.3.0", info.fpga_version());

        let info = FirmwareInfo::new(0, 132, 0, 132, 0, 0);
        assert_eq!("v2.4.0", info.cpu_version());
        assert_eq!("v2.4.0", info.fpga_version());

        let info = FirmwareInfo::new(0, 133, 0, 133, 0, 0);
        assert_eq!("v2.5.0", info.cpu_version());
        assert_eq!("v2.5.0", info.fpga_version());

        let info = FirmwareInfo::new(0, 134, 0, 134, 0, 0);
        assert_eq!("v2.6.0", info.cpu_version());
        assert_eq!("v2.6.0", info.fpga_version());

        let info = FirmwareInfo::new(0, 135, 0, 135, 0, 0);
        assert_eq!("v2.7.0", info.cpu_version());
        assert_eq!("v2.7.0", info.fpga_version());

        let info = FirmwareInfo::new(0, 136, 0, 136, 0, 0);
        assert_eq!("v2.8.0", info.cpu_version());
        assert_eq!("v2.8.0", info.fpga_version());

        let info = FirmwareInfo::new(0, 136, 1, 136, 1, 0);
        assert_eq!("v2.8.1", info.cpu_version());
        assert_eq!("v2.8.1", info.fpga_version());

        let info = FirmwareInfo::new(0, 137, 0, 137, 0, 0);
        assert_eq!("v2.9.0", info.cpu_version());
        assert_eq!("v2.9.0", info.fpga_version());

        let info = FirmwareInfo::new(0, 138, 0, 138, 0, 0);
        assert_eq!("v3.0.0", info.cpu_version());
        assert_eq!("v3.0.0", info.fpga_version());

        let info = FirmwareInfo::new(0, 139, 0, 139, 0, 0);
        assert_eq!("unknown (139)", info.cpu_version());
        assert_eq!("unknown (139)", info.fpga_version());
    }
}
