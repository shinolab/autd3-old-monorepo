/*
 * File: fpga_info.rs
 * Project: defined
 * Created Date: 05/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::cpu::RxMessage;

/// FPGA information
#[repr(C)]
pub struct FPGAInfo {
    info: u8,
}

impl FPGAInfo {
    pub const fn new(info: u8) -> Self {
        Self { info }
    }

    /// Check if thermal sensor is asserted
    pub const fn is_thermal_assert(&self) -> bool {
        (self.info & 0x01) != 0
    }

    pub const fn info(&self) -> u8 {
        self.info
    }
}

impl From<&RxMessage> for FPGAInfo {
    fn from(msg: &RxMessage) -> Self {
        Self { info: msg.data }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn fpga_info() {
        assert_eq!(size_of::<FPGAInfo>(), 1);

        let info = FPGAInfo::new(0x00);
        assert!(!info.is_thermal_assert());
        assert_eq!(info.info(), 0x00);

        let info = FPGAInfo::new(0x01);
        assert!(info.is_thermal_assert());
        assert_eq!(info.info(), 0x01);

        let info = FPGAInfo::new(0x02);
        assert!(!info.is_thermal_assert());
        assert_eq!(info.info(), 0x02);

        let rx = RxMessage { ack: 0, data: 0x01 };
        let info = FPGAInfo::from(&rx);
        assert!(info.is_thermal_assert());
        assert_eq!(info.info(), 0x01);
    }
}
