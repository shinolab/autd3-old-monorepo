/*
 * File: focus.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::{autd3::gain::Focus, driver::geometry::Vector3, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainFocus(x: float, y: float, z: float) -> GainPtr {
    GainPtr::new(Focus::new(Vector3::new(x, y, z)))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainFocusWithIntensity(focus: GainPtr, intensity: u8) -> GainPtr {
    GainPtr::new(take_gain!(focus, Focus).with_intensity(intensity))
}
