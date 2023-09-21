/*
 * File: lib.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

pub mod constraint;
pub mod evp;
pub mod greedy;
pub mod gs;
pub mod gspat;
pub mod lm;
pub mod naive;
pub mod nalgebra_backend;
pub mod sdp;

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use autd3capi::{link::debug::*, *};
    use autd3capi_def::{common::*, ControllerPtr, Level, LinkPtr};

    pub unsafe fn make_debug_link() -> LinkPtr {
        let debug = AUTDLinkDebug();
        let debug = AUTDLinkDebugWithLogLevel(debug, Level::Off);
        AUTDLinkDebugWithTimeout(debug, 0)
    }

    pub unsafe fn create_controller() -> ControllerPtr {
        let builder = AUTDControllerBuilder();
        let builder = AUTDControllerBuilderAddDevice(builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let builder = AUTDControllerBuilderAddDeviceQuaternion(builder, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

        let link = make_debug_link();
        let mut err = vec![c_char::default(); 256];
        let cnt = AUTDControllerOpenWith(builder, link, err.as_mut_ptr());
        assert_ne!(cnt.0, NULL);
        cnt
    }
}
