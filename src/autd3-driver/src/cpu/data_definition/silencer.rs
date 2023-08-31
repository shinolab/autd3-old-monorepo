/*
 * File: silencer.rs
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

use super::TypeTag;

pub struct Silencer {}

impl Silencer {
    pub fn write(tx: &mut [u16], step: u16) {
        tx[0] = TypeTag::Silencer as u16;
        tx[1] = step;
    }
}

// #[cfg(test)]
// mod tests {
//     use std::mem::size_of;

//     use super::*;

//     #[test]
//     fn silencer_header() {
//         assert_eq!(size_of::<SilencerHeader>(), 124);

//         let header = SilencerHeader {
//             _cycle: 0,
//             step: 0x4567,
//             _unused: [0x00; 120],
//         };

//         let mut buf = vec![0x00; 124];
//         unsafe {
//             std::ptr::copy_nonoverlapping(&header as *const _ as *const u8, buf.as_mut_ptr(), 124);
//         }
//         assert_eq!(buf[2], 0x67);
//         assert_eq!(buf[3], 0x45);
//     }
// }
