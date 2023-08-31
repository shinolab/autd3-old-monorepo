/*
 * File: filter.rs
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

pub struct FilterPhase {}

impl FilterPhase {
    pub fn write(tx: &mut [u16], filter: &[i16]) {
        tx[0] = TypeTag::FilterPhase as u16;
        unsafe {
            std::ptr::copy_nonoverlapping(
                filter.as_ptr(),
                (&mut tx[1..]).as_mut_ptr() as *mut i16,
                filter.len(),
            )
        }
    }
}

pub struct FilterGain {}

impl FilterGain {
    pub fn write(tx: &mut [u16], filter: &[i16]) {
        tx[0] = TypeTag::FilterGain as u16;
        unsafe {
            std::ptr::copy_nonoverlapping(
                filter.as_ptr(),
                (&mut tx[1..]).as_mut_ptr() as *mut i16,
                filter.len(),
            )
        }
    }
}
