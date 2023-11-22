/*
 * File: transform.rs
 * Project: modulation
 * Created Date: 21/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    common::{autd3::modulation::IntoTransform, *},
    ModulationPtr,
};
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
            let f = std::mem::transmute::<
                _,
                unsafe extern "C" fn(ConstPtr, u32, EmitIntensity) -> EmitIntensity,
            >(f);
            f(context, i as u32, d)
        }),
    )
}
