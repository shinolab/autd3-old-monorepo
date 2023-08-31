/*
 * File: sync.rs
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

pub struct Sync {}

impl Sync {
    pub fn write(tx: &mut [u16], cycle: &[u16]) {
        tx[0] = TypeTag::Sync as u16;
        unsafe {
            std::ptr::copy_nonoverlapping(cycle.as_ptr(), (&mut tx[1..]).as_mut_ptr(), cycle.len())
        }
    }
}
