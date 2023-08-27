/*
 * File: mod.rs
 * Project: geometry
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

pub mod transducer;

use autd3capi_def::{common::*, ControllerPtr, GeometryPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetGeometry(cnt: ControllerPtr) -> GeometryPtr {
    GeometryPtr(cast!(cnt.0, Cnt).geometry() as *const _ as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetSoundSpeed(geo: GeometryPtr) -> float {
    cast!(geo.0, Geo).sound_speed
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetSoundSpeed(geo: GeometryPtr, value: float) {
    cast_mut!(geo.0, Geo).sound_speed = value;
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetSoundSpeedFromTemp(
    geo: GeometryPtr,
    temp: float,
    k: float,
    r: float,
    m: float,
) {
    cast_mut!(geo.0, Geo).set_sound_speed_from_temp_with(temp, k, r, m);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetAttenuation(geo: GeometryPtr) -> float {
    cast!(geo.0, Geo).attenuation
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetAttenuation(geo: GeometryPtr, value: float) {
    cast_mut!(geo.0, Geo).attenuation = value;
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDNumTransducers(geo: GeometryPtr) -> u32 {
    cast!(geo.0, Geo).num_transducers() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDNumDevices(geo: GeometryPtr) -> u32 {
    cast!(geo.0, Geo).num_devices() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryCenter(geo: GeometryPtr, center: *mut float) {
    let c = cast!(geo.0, Geo).center();
    center.add(0).write(c.x);
    center.add(1).write(c.y);
    center.add(2).write(c.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryCenterOf(geo: GeometryPtr, dev_idx: u32, center: *mut float) {
    let c = cast!(geo.0, Geo).center_of(dev_idx as usize);
    center.add(0).write(c.x);
    center.add(1).write(c.y);
    center.add(2).write(c.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryTranslate(geo: GeometryPtr, x: float, y: float, z: float) {
    cast_mut!(geo.0, Geo).translate(Vector3::new(x, y, z));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryRotate(
    geo: GeometryPtr,
    w: float,
    i: float,
    j: float,
    k: float,
) {
    cast_mut!(geo.0, Geo).rotate(UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k)));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryAffine(
    geo: GeometryPtr,
    x: float,
    y: float,
    z: float,
    w: float,
    i: float,
    j: float,
    k: float,
) {
    cast_mut!(geo.0, Geo).affine(
        Vector3::new(x, y, z),
        UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k)),
    );
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryTranslateOf(
    geo: GeometryPtr,
    dev_idx: u32,
    x: float,
    y: float,
    z: float,
) {
    cast_mut!(geo.0, Geo).translate_of(dev_idx as _, Vector3::new(x, y, z));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryRotateOf(
    geo: GeometryPtr,
    dev_idx: u32,
    w: float,
    i: float,
    j: float,
    k: float,
) {
    cast_mut!(geo.0, Geo).rotate_of(
        dev_idx as _,
        UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k)),
    );
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryAffineOf(
    geo: GeometryPtr,
    dev_idx: u32,
    x: float,
    y: float,
    z: float,
    w: float,
    i: float,
    j: float,
    k: float,
) {
    cast_mut!(geo.0, Geo).affine_of(
        dev_idx as _,
        Vector3::new(x, y, z),
        UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k)),
    );
}

#[cfg(test)]
mod tests {

    use super::{transducer::AUTDTransRotation, *};
    use crate::{geometry::transducer::AUTDTransPosition, tests::*, *};

    #[test]
    fn test_geometry() {
        unsafe {
            let cnt = create_controller();
            let geo = AUTDGetGeometry(cnt);

            let c = 300e3;
            AUTDSetSoundSpeed(geo, c);
            assert_eq!(c, AUTDGetSoundSpeed(geo));

            AUTDSetSoundSpeedFromTemp(geo, 15.0, 1.4, 8.314_463, 28.9647e-3);
            assert_approx_eq::assert_approx_eq!(AUTDGetSoundSpeed(geo), 340295.27186788846);

            let num_transducers = AUTDNumTransducers(geo);
            assert_eq!(num_transducers, 2 * 249);
            let num_devices = AUTDNumDevices(geo) as usize;
            assert_eq!(num_devices, 2);

            let mut v = [0., 0., 0.];
            AUTDGeometryCenter(geo, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 86.62522088353415);
            assert_approx_eq::assert_approx_eq!(v[1], 66.71325301204786);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);
            AUTDGeometryCenterOf(geo, 0, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 86.62522088353415);
            assert_approx_eq::assert_approx_eq!(v[1], 66.71325301204786);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);

            AUTDFreeController(cnt);
        }
    }

    #[test]
    fn geometry_affine() {
        unsafe {
            let cnt = create_controller();
            let geo = AUTDGetGeometry(cnt);

            let num_trans = AUTDNumTransducers(geo) as usize;

            let mut v = vec![[0., 0., 0.]; num_trans];
            (0..num_trans).for_each(|t| AUTDTransPosition(geo, t as _, v[t].as_mut_ptr()));
            AUTDGeometryTranslate(geo, 1., 2., 3.);
            (0..num_trans).for_each(|t| {
                let mut v_new = [0., 0., 0.];
                AUTDTransPosition(geo, t as _, v_new.as_mut_ptr());
                assert_approx_eq::assert_approx_eq!(v_new[0], v[t][0] + 1.);
                assert_approx_eq::assert_approx_eq!(v_new[1], v[t][1] + 2.);
                assert_approx_eq::assert_approx_eq!(v_new[2], v[t][2] + 3.);
            });

            let q = UnitQuaternion::from_axis_angle(&UnitVector3::new_normalize(Vector3::z()), 1.0);
            AUTDGeometryRotate(geo, q.w, q.i, q.j, q.k);
            (0..num_trans).for_each(|t| {
                let mut v_new = [0., 0., 0., 0.];
                AUTDTransRotation(geo, t as _, v_new.as_mut_ptr());
                assert_approx_eq::assert_approx_eq!(v_new[0], q.w);
                assert_approx_eq::assert_approx_eq!(v_new[1], q.i);
                assert_approx_eq::assert_approx_eq!(v_new[2], q.j);
                assert_approx_eq::assert_approx_eq!(v_new[3], q.k);
            });

            AUTDFreeController(cnt);

            let cnt = create_controller();
            let geo = AUTDGetGeometry(cnt);

            let mut v = vec![[0., 0., 0.]; num_trans];
            (0..num_trans).for_each(|t| AUTDTransPosition(geo, t as _, v[t].as_mut_ptr()));

            let rot =
                UnitQuaternion::from_axis_angle(&UnitVector3::new_normalize(Vector3::z()), PI / 2.);
            AUTDGeometryAffine(geo, 1., 2., 3., rot.w, rot.i, rot.j, rot.k);
            (0..num_trans).for_each(|t| {
                let mut v_new = [0., 0., 0.];
                let mut q_new = [0., 0., 0., 0.];
                AUTDTransPosition(geo, t as _, v_new.as_mut_ptr());
                AUTDTransRotation(geo, t as _, q_new.as_mut_ptr());
                assert_approx_eq::assert_approx_eq!(v_new[0], -v[t][1] + 1.);
                assert_approx_eq::assert_approx_eq!(v_new[1], v[t][0] + 2.);
                assert_approx_eq::assert_approx_eq!(v_new[2], v[t][2] + 3.);
                assert_approx_eq::assert_approx_eq!(q_new[0], rot.w);
                assert_approx_eq::assert_approx_eq!(q_new[1], rot.i);
                assert_approx_eq::assert_approx_eq!(q_new[2], rot.j);
                assert_approx_eq::assert_approx_eq!(q_new[3], rot.k);
            });

            AUTDFreeController(cnt);
        }
    }

    #[test]
    fn geometry_affine_of() {
        unsafe {
            let cnt = create_controller();
            let geo = AUTDGetGeometry(cnt);

            let num_trans = AUTDNumTransducers(geo) as usize;

            let mut v = vec![[0., 0., 0.]; num_trans];
            (0..num_trans).for_each(|t| AUTDTransPosition(geo, t as _, v[t].as_mut_ptr()));
            AUTDGeometryTranslateOf(geo, 0, 1., 2., 3.);
            (0..num_trans / 2).for_each(|t| {
                let mut v_new = [0., 0., 0.];
                AUTDTransPosition(geo, t as _, v_new.as_mut_ptr());
                assert_approx_eq::assert_approx_eq!(v_new[0], v[t][0] + 1.);
                assert_approx_eq::assert_approx_eq!(v_new[1], v[t][1] + 2.);
                assert_approx_eq::assert_approx_eq!(v_new[2], v[t][2] + 3.);
            });
            (num_trans / 2..num_trans).for_each(|t| {
                let mut v_new = [0., 0., 0.];
                AUTDTransPosition(geo, t as _, v_new.as_mut_ptr());
                assert_approx_eq::assert_approx_eq!(v_new[0], v[t][0]);
                assert_approx_eq::assert_approx_eq!(v_new[1], v[t][1]);
                assert_approx_eq::assert_approx_eq!(v_new[2], v[t][2]);
            });

            let q = UnitQuaternion::from_axis_angle(&UnitVector3::new_normalize(Vector3::z()), 1.0);
            AUTDGeometryRotateOf(geo, 1, q.w, q.i, q.j, q.k);
            (0..num_trans / 2).for_each(|t| {
                let mut v_new = [0., 0., 0., 0.];
                AUTDTransRotation(geo, t as _, v_new.as_mut_ptr());
                assert_approx_eq::assert_approx_eq!(v_new[0], 1.);
                assert_approx_eq::assert_approx_eq!(v_new[1], 0.);
                assert_approx_eq::assert_approx_eq!(v_new[2], 0.);
                assert_approx_eq::assert_approx_eq!(v_new[3], 0.);
            });
            (num_trans / 2..num_trans).for_each(|t| {
                let mut v_new = [0., 0., 0., 0.];
                AUTDTransRotation(geo, t as _, v_new.as_mut_ptr());
                assert_approx_eq::assert_approx_eq!(v_new[0], q.w);
                assert_approx_eq::assert_approx_eq!(v_new[1], q.i);
                assert_approx_eq::assert_approx_eq!(v_new[2], q.j);
                assert_approx_eq::assert_approx_eq!(v_new[3], q.k);
            });

            let mut v = vec![[0., 0., 0.]; num_trans];
            (0..num_trans).for_each(|t| AUTDTransPosition(geo, t as _, v[t].as_mut_ptr()));
            let rot =
                UnitQuaternion::from_axis_angle(&UnitVector3::new_normalize(Vector3::z()), PI / 2.);
            AUTDGeometryAffineOf(geo, 0, 1., 2., 3., rot.w, rot.i, rot.j, rot.k);
            (0..num_trans / 2).for_each(|t| {
                let mut v_new = [0., 0., 0.];
                let mut q_new = [0., 0., 0., 0.];
                AUTDTransPosition(geo, t as _, v_new.as_mut_ptr());
                AUTDTransRotation(geo, t as _, q_new.as_mut_ptr());
                assert_approx_eq::assert_approx_eq!(v_new[0], -v[t][1] + 1.);
                assert_approx_eq::assert_approx_eq!(v_new[1], v[t][0] + 2.);
                assert_approx_eq::assert_approx_eq!(v_new[2], v[t][2] + 3.);
                assert_approx_eq::assert_approx_eq!(q_new[0], rot.w);
                assert_approx_eq::assert_approx_eq!(q_new[1], rot.i);
                assert_approx_eq::assert_approx_eq!(q_new[2], rot.j);
                assert_approx_eq::assert_approx_eq!(q_new[3], rot.k);
            });
            (num_trans / 2..num_trans).for_each(|t| {
                let mut v_new = [0., 0., 0.];
                let mut q_new = [0., 0., 0., 0.];
                AUTDTransPosition(geo, t as _, v_new.as_mut_ptr());
                AUTDTransRotation(geo, t as _, q_new.as_mut_ptr());
                assert_approx_eq::assert_approx_eq!(v_new[0], v[t][0]);
                assert_approx_eq::assert_approx_eq!(v_new[1], v[t][1]);
                assert_approx_eq::assert_approx_eq!(v_new[2], v[t][2]);
                assert_approx_eq::assert_approx_eq!(q_new[0], q.w);
                assert_approx_eq::assert_approx_eq!(q_new[1], q.i);
                assert_approx_eq::assert_approx_eq!(q_new[2], q.j);
                assert_approx_eq::assert_approx_eq!(q_new[3], q.k);
            });

            AUTDFreeController(cnt);
        }
    }
}
