/*
 * File: traits.rs
 * Project: src
 * Created Date: 30/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::pb::*;

pub trait ToMessage {
    type Message: prost::Message;

    fn to_msg(&self) -> Self::Message;
}

pub trait FromMessage<T: prost::Message> {
    fn from_msg(msg: &T) -> Self;
}

impl ToMessage for autd3_driver::geometry::Vector3 {
    type Message = Vector3;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            x: self.x as _,
            y: self.y as _,
            z: self.z as _,
        }
    }
}

impl ToMessage for autd3_driver::geometry::Quaternion {
    type Message = Quaternion;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            w: self.w as _,
            x: self.coords.x as _,
            y: self.coords.y as _,
            z: self.coords.z as _,
        }
    }
}

impl ToMessage for autd3_driver::geometry::Geometry {
    type Message = Geometry;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            devices: self
                .iter()
                .map(|dev| geometry::Device {
                    idx: dev.idx() as _,
                    transducers: dev
                        .iter()
                        .map(|t| geometry::Transducer {
                            idx: t.tr_idx() as _,
                            pos: Some(t.position().to_msg()),
                            rot: Some(t.rotation().to_msg()),
                        })
                        .collect(),
                    sound_speed: dev.sound_speed as _,
                    attenuation: dev.attenuation as _,
                })
                .collect(),
        }
    }
}

impl ToMessage for &[autd3_driver::geometry::Device] {
    type Message = Geometry;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            devices: self
                .iter()
                .map(|dev| geometry::Device {
                    idx: dev.idx() as _,
                    transducers: dev
                        .iter()
                        .map(|t| geometry::Transducer {
                            idx: t.tr_idx() as _,
                            pos: Some(t.position().to_msg()),
                            rot: Some(t.rotation().to_msg()),
                        })
                        .collect(),
                    sound_speed: dev.sound_speed as _,
                    attenuation: dev.attenuation as _,
                })
                .collect(),
        }
    }
}

impl ToMessage for autd3_driver::cpu::TxDatagram {
    type Message = TxRawData;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            data: self.all_data().to_vec(),
            num_devices: self.num_devices() as _,
        }
    }
}

impl ToMessage for Vec<autd3_driver::cpu::RxMessage> {
    type Message = RxMessage;

    fn to_msg(&self) -> Self::Message {
        let mut data = vec![0; std::mem::size_of::<autd3_driver::cpu::RxMessage>() * self.len()];
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.as_ptr() as *const u8,
                data.as_mut_ptr(),
                data.len(),
            );
        }
        Self::Message { data }
    }
}

impl FromMessage<RxMessage> for Vec<autd3_driver::cpu::RxMessage> {
    fn from_msg(msg: &RxMessage) -> Self {
        let mut rx = vec![
            autd3_driver::cpu::RxMessage { ack: 0, data: 0 };
            msg.data.len() / std::mem::size_of::<autd3_driver::cpu::RxMessage>()
        ];
        unsafe {
            std::ptr::copy_nonoverlapping(msg.data.as_ptr(), rx.as_mut_ptr() as _, msg.data.len());
        }
        rx
    }
}

impl FromMessage<Vector3> for autd3_driver::geometry::Vector3 {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Vector3) -> Self {
        autd3_driver::geometry::Vector3::new(msg.x as _, msg.y as _, msg.z as _)
    }
}

impl FromMessage<Quaternion> for autd3_driver::geometry::UnitQuaternion {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Quaternion) -> Self {
        autd3_driver::geometry::UnitQuaternion::from_quaternion(
            autd3_driver::geometry::Quaternion::new(msg.w as _, msg.x as _, msg.y as _, msg.z as _),
        )
    }
}

impl FromMessage<Geometry> for autd3_driver::geometry::Geometry {
    fn from_msg(msg: &Geometry) -> Self {
        let devices = msg
            .devices
            .iter()
            .map(|dev| {
                let mut device = autd3_driver::geometry::Device::new(
                    dev.idx as usize,
                    dev.transducers
                        .iter()
                        .map(|tr| {
                            autd3_driver::geometry::Transducer::new(
                                dev.idx as _,
                                tr.idx as _,
                                autd3_driver::geometry::Vector3::from_msg(tr.pos.as_ref().unwrap()),
                                autd3_driver::geometry::UnitQuaternion::from_msg(
                                    tr.rot.as_ref().unwrap(),
                                ),
                            )
                        })
                        .collect(),
                );
                device.sound_speed = dev.sound_speed as _;
                device.attenuation = dev.attenuation as _;
                device
            })
            .collect();
        Self::new(devices)
    }
}

impl FromMessage<TxRawData> for autd3_driver::cpu::TxDatagram {
    fn from_msg(msg: &TxRawData) -> Self {
        let mut tx = autd3_driver::cpu::TxDatagram::new(msg.num_devices as usize);
        unsafe {
            std::ptr::copy_nonoverlapping(
                msg.data.as_ptr(),
                tx.all_data_mut().as_mut_ptr(),
                msg.data.len(),
            );
        }
        tx
    }
}
