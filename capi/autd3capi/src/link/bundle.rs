/*
 * File: bundle.rs
 * Project: link
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    common::{autd3::link::Bundle, *},
    LinkPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkBundle(main: LinkPtr, sub: LinkPtr) -> LinkPtr {
    let main: Box<Box<L>> = Box::from_raw(main.0 as *mut Box<L>);
    let sub: Box<Box<L>> = Box::from_raw(sub.0 as *mut Box<L>);
    LinkPtr::new(Bundle::new(*main, *sub))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::link::debug::*;

    #[test]
    fn test_link_debug() {
        unsafe {
            let link_1 = AUTDLinkDebug();
            let link_2 = AUTDLinkDebug();

            let _link = AUTDLinkBundle(link_1, link_2);
        }
    }
}
