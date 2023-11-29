/*
 * File: transform.rs
 * Project: modulation
 * Created Date: 21/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{autd3::modulation::IntoTransform, *};
use driver::common::EmitIntensity;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithTransform(
    m: ModulationPtr,
    f: ConstPtr,
    context: ConstPtr,
) -> ModulationPtr {
    ModulationPtr::new(
        Box::from_raw(m.0 as *mut Box<M>).with_transform(move |i, d| {
            let f = std::mem::transmute::<_, unsafe extern "C" fn(ConstPtr, u32, u8) -> u8>(f);
            EmitIntensity::new(f(context, i as u32, d.value()))
        }),
    )
}
