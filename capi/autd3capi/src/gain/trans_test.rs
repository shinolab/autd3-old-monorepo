/*
 * File: trans_test.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::{autd3::gain::TransducerTest, *};
use driver::common::{EmitIntensity, Phase};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ContextPtr(pub ConstPtr);

unsafe impl Send for ContextPtr {}
unsafe impl Sync for ContextPtr {}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainTransducerTest(
    f: ConstPtr,
    context: ContextPtr,
    geometry: GeometryPtr,
) -> GainPtr {
    let f = std::mem::transmute::<
        _,
        unsafe extern "C" fn(ContextPtr, GeometryPtr, u32, u8, *mut Drive),
    >(f);
    GainPtr::new(TransducerTest::new(move |dev, tr| {
        let mut d = driver::common::Drive {
            phase: Phase::new(0),
            intensity: EmitIntensity::new(0),
        };
        f(
            context,
            geometry,
            dev.idx() as u32,
            tr.idx() as u8,
            &mut d as *mut _ as *mut _,
        );
        Some(d)
    }))
}
