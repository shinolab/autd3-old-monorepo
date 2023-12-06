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

use autd3capi_def::{autd3::gain::Bessel, driver::geometry::Vector3, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainBessel(
    x: float,
    y: float,
    z: float,
    nx: float,
    ny: float,
    nz: float,
    theta_z: float,
) -> GainPtr {
    GainPtr::new(Bessel::new(
        Vector3::new(x, y, z),
        Vector3::new(nx, ny, nz),
        theta_z,
    ))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainBesselWithIntensity(bessel: GainPtr, intensity: u8) -> GainPtr {
    GainPtr::new(take_gain!(bessel, Bessel).with_intensity(intensity))
}
