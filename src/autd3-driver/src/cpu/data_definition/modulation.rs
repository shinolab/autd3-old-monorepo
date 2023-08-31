/*
 * File: modulation.rs
 * Project: data_definition
 * Created Date: 31/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::fmt;

use super::TypeTag;

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct ModulationControlFlags : u8 {
        const NONE      = 0;
        const MOD_BEGIN = 1 << 1;
        const MOD_END   = 1 << 2;
    }
}

impl fmt::Display for ModulationControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();
        if self.contains(ModulationControlFlags::MOD_BEGIN) {
            flags.push("MOD_BEGIN")
        }
        if self.contains(ModulationControlFlags::MOD_END) {
            flags.push("MOD_END")
        }
        if self.is_empty() {
            flags.push("NONE")
        }
        write!(
            f,
            "{}",
            flags
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" | ")
        )
    }
}

pub struct Modulation {}

impl Modulation {
    pub fn write(tx: &mut [u16], freq_div: u32, begin: bool, end: bool, data: &[u8]) {
        let mut f = ModulationControlFlags::NONE;
        f.set(ModulationControlFlags::MOD_BEGIN, begin);
        f.set(ModulationControlFlags::MOD_END, end);
        tx[0] = (f.bits() as u16) << 8 | TypeTag::Modulation as u16;

        tx[1] = data.len() as u16;

        let mut offset = 2;
        if begin {
            tx[2] = (freq_div & 0xFFFF) as u16;
            tx[3] = ((freq_div >> 16) & 0xFFFF) as u16;
            offset += 2;
        }

        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                (&mut tx[offset..]).as_mut_ptr() as *mut u8,
                data.len(),
            )
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::mem::size_of;

//     use super::*;

//     #[test]
//     fn mod_header_initial() {
//         assert_eq!(size_of::<ModInitial>(), 124);

//         let header = ModInitial {
//             freq_div: 0x01234567,
//             data: (0..MOD_HEADER_INITIAL_DATA_SIZE)
//                 .map(|i| i as u8)
//                 .collect::<Vec<_>>()
//                 .try_into()
//                 .unwrap(),
//         };

//         let mut buf = vec![0x00; 124];
//         unsafe {
//             std::ptr::copy_nonoverlapping(&header as *const _ as *const u8, buf.as_mut_ptr(), 124);
//         }
//         assert_eq!(buf[0], 0x67);
//         assert_eq!(buf[1], 0x45);
//         assert_eq!(buf[2], 0x23);
//         assert_eq!(buf[3], 0x01);
//         (0..MOD_HEADER_INITIAL_DATA_SIZE).for_each(|i| {
//             assert_eq!(buf[4 + i], i as u8);
//         });
//     }

//     #[test]
//     fn mod_header_subsequent() {
//         assert_eq!(size_of::<ModSubsequent>(), 124);
//     }
// }
