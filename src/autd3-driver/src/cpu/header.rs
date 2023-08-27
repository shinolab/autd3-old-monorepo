/*
 * File: header.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{cpu::CPUControlFlags, fpga::FPGAControlFlags};

pub const MSG_CLEAR: u8 = 0x00;
pub const MSG_RD_CPU_VERSION: u8 = 0x01;
pub const MSG_RD_FPGA_VERSION: u8 = 0x03;
pub const MSG_RD_FPGA_FUNCTION: u8 = 0x04;
pub const MSG_BEGIN: u8 = 0x05;
pub const MSG_END: u8 = 0xF0;
pub const MSG_RD_CPU_VERSION_MINOR: u8 = 0xF1;
pub const MSG_RD_FPGA_VERSION_MINOR: u8 = 0xF2;

pub const MOD_HEADER_INITIAL_DATA_SIZE: usize = 120;
pub const MOD_HEADER_SUBSEQUENT_DATA_SIZE: usize = 124;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GlobalHeader {
    pub msg_id: u8,
    pub fpga_flag: FPGAControlFlags,
    pub cpu_flag: CPUControlFlags,
    pub size: u8,
    pub data: [u8; 124],
}

#[repr(C)]
pub struct ModInitial {
    pub freq_div: u32,
    pub data: [u8; MOD_HEADER_INITIAL_DATA_SIZE],
}

#[repr(C)]
pub struct ModSubsequent {
    pub data: [u8; MOD_HEADER_SUBSEQUENT_DATA_SIZE],
}

#[repr(C)]
pub struct SilencerHeader {
    _cycle: u16,
    pub step: u16,
    _unused: [u8; 120],
}

impl GlobalHeader {
    pub const fn new() -> Self {
        Self {
            msg_id: 0,
            fpga_flag: FPGAControlFlags::NONE,
            cpu_flag: CPUControlFlags::NONE,
            size: 0,
            data: [0x00; 124],
        }
    }

    pub const fn mod_initial(&self) -> &ModInitial {
        unsafe { std::mem::transmute(&self.data) }
    }

    pub fn mod_initial_mut(&mut self) -> &mut ModInitial {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub const fn mod_subsequent(&self) -> &ModSubsequent {
        unsafe { std::mem::transmute(&self.data) }
    }

    pub fn mod_subsequent_mut(&mut self) -> &mut ModSubsequent {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub const fn silencer(&self) -> &SilencerHeader {
        unsafe { std::mem::transmute(&self.data) }
    }

    pub fn silencer_mut(&mut self) -> &mut SilencerHeader {
        unsafe { std::mem::transmute(&mut self.data) }
    }
}

impl Default for GlobalHeader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn mod_header_initial() {
        assert_eq!(size_of::<ModInitial>(), 124);

        let header = ModInitial {
            freq_div: 0x01234567,
            data: (0..MOD_HEADER_INITIAL_DATA_SIZE)
                .map(|i| i as u8)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        };

        let mut buf = vec![0x00; 124];
        unsafe {
            std::ptr::copy_nonoverlapping(&header as *const _ as *const u8, buf.as_mut_ptr(), 124);
        }
        assert_eq!(buf[0], 0x67);
        assert_eq!(buf[1], 0x45);
        assert_eq!(buf[2], 0x23);
        assert_eq!(buf[3], 0x01);
        (0..MOD_HEADER_INITIAL_DATA_SIZE).for_each(|i| {
            assert_eq!(buf[4 + i], i as u8);
        });
    }

    #[test]
    fn mod_header_subsequent() {
        assert_eq!(size_of::<ModSubsequent>(), 124);
    }

    #[test]
    fn silencer_header() {
        assert_eq!(size_of::<SilencerHeader>(), 124);

        let header = SilencerHeader {
            _cycle: 0,
            step: 0x4567,
            _unused: [0x00; 120],
        };

        let mut buf = vec![0x00; 124];
        unsafe {
            std::ptr::copy_nonoverlapping(&header as *const _ as *const u8, buf.as_mut_ptr(), 124);
        }
        assert_eq!(buf[2], 0x67);
        assert_eq!(buf[3], 0x45);
    }

    #[test]
    fn global_header() {
        assert_eq!(size_of::<GlobalHeader>(), 128);

        let header = GlobalHeader {
            msg_id: 0x01,
            fpga_flag: FPGAControlFlags::FORCE_FAN,
            cpu_flag: CPUControlFlags::CONFIG_SYNC,
            size: 0x02,
            data: (0..124)
                .map(|i| i as u8)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        };

        let mut buf = vec![0x00; 128];
        unsafe {
            std::ptr::copy_nonoverlapping(&header as *const _ as *const u8, buf.as_mut_ptr(), 128);
        }
        assert_eq!(buf[0], 0x01);
        assert_eq!(buf[1], 1 << 4);
        assert_eq!(buf[2], 1 << 2);
        assert_eq!(buf[3], 0x02);
        (0..124).for_each(|i| {
            assert_eq!(buf[4 + i], i as u8);
        });
    }
}
