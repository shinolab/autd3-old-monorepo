/*
 * File: traits.rs
 * Project: src
 * Created Date: 30/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::rc::Rc;

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

impl ToMessage for autd3_driver::common::EmitIntensity {
    type Message = EmitIntensity;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            pulse_width: self.pulse_width() as _,
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
                            idx: t.local_idx() as _,
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
                            idx: t.local_idx() as _,
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

impl ToMessage for autd3_gain_holo::Constraint {
    type Message = Constraint;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        match self {
            autd3_gain_holo::Constraint::DontCare => Constraint {
                constraint: Some(constraint::Constraint::DontCare(DontCare {})),
            },
            autd3_gain_holo::Constraint::Normalize => Constraint {
                constraint: Some(constraint::Constraint::Normalize(Normalize {})),
            },
            autd3_gain_holo::Constraint::Uniform(value) => Constraint {
                constraint: Some(constraint::Constraint::Uniform(Uniform {
                    value: Some(value.to_msg()),
                })),
            },
            autd3_gain_holo::Constraint::Clamp(min, max) => Constraint {
                constraint: Some(constraint::Constraint::Clamp(Clamp {
                    min: *min as _,
                    max: *max as _,
                })),
            },
        }
    }
}

macro_rules! to_holo {
    ($self:expr) => {
        $self
            .foci()
            .map(|(p, &a)| Holo {
                pos: Some(p.to_msg()),
                amp: a as _,
            })
            .collect()
    };
}

impl ToMessage for autd3_gain_holo::SDP<autd3_gain_holo::NalgebraBackend> {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Gain(Gain {
                gain: Some(gain::Gain::Sdp(Sdp {
                    backend: Backend::Nalgebra as i32,
                    holo: to_holo!(self),
                    alpha: self.alpha() as _,
                    lambda: self.lambda() as _,
                    repeat: self.repeat() as _,
                    constraint: Some(self.constraint().to_msg()),
                })),
            })),
        }
    }
}

impl ToMessage for autd3_gain_holo::EVP<autd3_gain_holo::NalgebraBackend> {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Gain(Gain {
                gain: Some(gain::Gain::Evp(Evp {
                    backend: Backend::Nalgebra as i32,
                    holo: to_holo!(self),
                    gamma: self.gamma() as _,
                    constraint: Some(self.constraint().to_msg()),
                })),
            })),
        }
    }
}

impl ToMessage for autd3_gain_holo::Naive<autd3_gain_holo::NalgebraBackend> {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Gain(Gain {
                gain: Some(gain::Gain::Naive(Naive {
                    backend: Backend::Nalgebra as i32,
                    holo: to_holo!(self),
                    constraint: Some(self.constraint().to_msg()),
                })),
            })),
        }
    }
}

impl ToMessage for autd3_gain_holo::GS<autd3_gain_holo::NalgebraBackend> {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Gain(Gain {
                gain: Some(gain::Gain::Gs(Gs {
                    backend: Backend::Nalgebra as i32,
                    repeat: self.repeat() as _,
                    holo: to_holo!(self),
                    constraint: Some(self.constraint().to_msg()),
                })),
            })),
        }
    }
}

impl ToMessage for autd3_gain_holo::GSPAT<autd3_gain_holo::NalgebraBackend> {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Gain(Gain {
                gain: Some(gain::Gain::Gspat(Gspat {
                    backend: Backend::Nalgebra as i32,
                    repeat: self.repeat() as _,
                    holo: to_holo!(self),
                    constraint: Some(self.constraint().to_msg()),
                })),
            })),
        }
    }
}

impl ToMessage for autd3_gain_holo::LM<autd3_gain_holo::NalgebraBackend> {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Gain(Gain {
                gain: Some(gain::Gain::Lm(Lm {
                    backend: Backend::Nalgebra as i32,
                    eps_1: self.eps_1() as _,
                    eps_2: self.eps_2() as _,
                    tau: self.tau() as _,
                    k_max: self.k_max() as _,
                    initial: self.initial().iter().map(|&v| v as _).collect(),
                    holo: to_holo!(self),
                    constraint: Some(self.constraint().to_msg()),
                })),
            })),
        }
    }
}

impl ToMessage for autd3_gain_holo::Greedy {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Gain(Gain {
                gain: Some(gain::Gain::Greedy(Greedy {
                    holo: to_holo!(self),
                    phase_div: self.phase_div() as _,
                    constraint: Some(self.constraint().to_msg()),
                })),
            })),
        }
    }
}

impl ToMessage for autd3_driver::datagram::Silencer {
    type Message = Datagram;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::SilencerConfig(SilencerConfig {
                step: self.step() as _,
            })),
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

impl ToMessage for autd3_driver::datagram::Clear {
    type Message = Datagram;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Special(SpecialData {
                special: Some(special_data::Special::Clear(Clear {})),
            })),
        }
    }
}

impl ToMessage for autd3_driver::datagram::Synchronize {
    type Message = Datagram;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Special(SpecialData {
                special: Some(special_data::Special::Synchronize(Synchronize {})),
            })),
        }
    }
}

impl ToMessage for autd3_driver::datagram::Stop {
    type Message = Datagram;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Special(SpecialData {
                special: Some(special_data::Special::Stop(Stop {})),
            })),
        }
    }
}

impl ToMessage for autd3_driver::datagram::UpdateFlags {
    type Message = Datagram;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Special(SpecialData {
                special: Some(special_data::Special::UpdateFlags(UpdateFlags {})),
            })),
        }
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

impl FromMessage<EmitIntensity> for autd3_driver::common::EmitIntensity {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &EmitIntensity) -> Self {
        Self::new_pulse_width(msg.pulse_width as _).unwrap()
    }
}

impl FromMessage<Constraint> for autd3_gain_holo::Constraint {
    fn from_msg(msg: &Constraint) -> Self {
        match &msg.constraint {
            Some(constraint::Constraint::DontCare(_)) => Self::DontCare,
            Some(constraint::Constraint::Normalize(_)) => Self::Normalize,
            Some(constraint::Constraint::Uniform(uniform)) => Self::Uniform(
                autd3_driver::common::EmitIntensity::from_msg(uniform.value.as_ref().unwrap()),
            ),
            Some(constraint::Constraint::Clamp(clamp)) => {
                Self::Clamp(clamp.min as _, clamp.max as _)
            }
            _ => Self::DontCare,
        }
    }
}

impl FromMessage<Sdp> for autd3_gain_holo::SDP<autd3_gain_holo::NalgebraBackend> {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Sdp) -> Self {
        Self::new(Rc::new(autd3_gain_holo::NalgebraBackend::default()))
            .with_alpha(msg.alpha as _)
            .with_lambda(msg.lambda as _)
            .with_repeat(msg.repeat as _)
            .with_constraint(autd3_gain_holo::Constraint::from_msg(
                msg.constraint.as_ref().unwrap(),
            ))
            .add_foci_from_iter(msg.holo.iter().map(|h| {
                (
                    autd3_driver::geometry::Vector3::from_msg(h.pos.as_ref().unwrap()),
                    h.amp as _,
                )
            }))
    }
}

impl FromMessage<Evp> for autd3_gain_holo::EVP<autd3_gain_holo::NalgebraBackend> {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Evp) -> Self {
        Self::new(Rc::new(autd3_gain_holo::NalgebraBackend::default()))
            .with_gamma(msg.gamma as _)
            .with_constraint(autd3_gain_holo::Constraint::from_msg(
                msg.constraint.as_ref().unwrap(),
            ))
            .add_foci_from_iter(msg.holo.iter().map(|h| {
                (
                    autd3_driver::geometry::Vector3::from_msg(h.pos.as_ref().unwrap()),
                    h.amp as _,
                )
            }))
    }
}

impl FromMessage<Naive> for autd3_gain_holo::Naive<autd3_gain_holo::NalgebraBackend> {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Naive) -> Self {
        Self::new(Rc::new(autd3_gain_holo::NalgebraBackend::default()))
            .with_constraint(autd3_gain_holo::Constraint::from_msg(
                msg.constraint.as_ref().unwrap(),
            ))
            .add_foci_from_iter(msg.holo.iter().map(|h| {
                (
                    autd3_driver::geometry::Vector3::from_msg(h.pos.as_ref().unwrap()),
                    h.amp as _,
                )
            }))
    }
}

impl FromMessage<Gs> for autd3_gain_holo::GS<autd3_gain_holo::NalgebraBackend> {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Gs) -> Self {
        Self::new(Rc::new(autd3_gain_holo::NalgebraBackend::default()))
            .with_repeat(msg.repeat as _)
            .with_constraint(autd3_gain_holo::Constraint::from_msg(
                msg.constraint.as_ref().unwrap(),
            ))
            .add_foci_from_iter(msg.holo.iter().map(|h| {
                (
                    autd3_driver::geometry::Vector3::from_msg(h.pos.as_ref().unwrap()),
                    h.amp as _,
                )
            }))
    }
}

impl FromMessage<Gspat> for autd3_gain_holo::GSPAT<autd3_gain_holo::NalgebraBackend> {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Gspat) -> Self {
        Self::new(Rc::new(autd3_gain_holo::NalgebraBackend::default()))
            .with_repeat(msg.repeat as _)
            .with_constraint(autd3_gain_holo::Constraint::from_msg(
                msg.constraint.as_ref().unwrap(),
            ))
            .add_foci_from_iter(msg.holo.iter().map(|h| {
                (
                    autd3_driver::geometry::Vector3::from_msg(h.pos.as_ref().unwrap()),
                    h.amp as _,
                )
            }))
    }
}

impl FromMessage<Lm> for autd3_gain_holo::LM<autd3_gain_holo::NalgebraBackend> {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Lm) -> Self {
        Self::new(Rc::new(autd3_gain_holo::NalgebraBackend::default()))
            .with_eps_1(msg.eps_1 as _)
            .with_eps_2(msg.eps_2 as _)
            .with_tau(msg.tau as _)
            .with_k_max(msg.k_max as _)
            .with_initial(msg.initial.iter().map(|&v| v as _).collect())
            .with_constraint(autd3_gain_holo::Constraint::from_msg(
                msg.constraint.as_ref().unwrap(),
            ))
            .add_foci_from_iter(msg.holo.iter().map(|h| {
                (
                    autd3_driver::geometry::Vector3::from_msg(h.pos.as_ref().unwrap()),
                    h.amp as _,
                )
            }))
    }
}

impl FromMessage<Greedy> for autd3_gain_holo::Greedy {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Greedy) -> Self {
        Self::new()
            .with_phase_div(msg.phase_div as _)
            .with_constraint(autd3_gain_holo::Constraint::from_msg(
                msg.constraint.as_ref().unwrap(),
            ))
            .add_foci_from_iter(msg.holo.iter().map(|h| {
                (
                    autd3_driver::geometry::Vector3::from_msg(h.pos.as_ref().unwrap()),
                    h.amp as _,
                )
            }))
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

impl FromMessage<SilencerConfig> for autd3_driver::datagram::Silencer {
    fn from_msg(msg: &SilencerConfig) -> Self {
        Self::new(msg.step as _)
    }
}

impl FromMessage<FirmwareInfoResponse> for Vec<autd3_driver::firmware_version::FirmwareInfo> {
    fn from_msg(msg: &FirmwareInfoResponse) -> Self {
        msg.firmware_info_list
            .iter()
            .enumerate()
            .map(|(i, f)| {
                autd3_driver::firmware_version::FirmwareInfo::new(
                    i,
                    f.cpu_major_version as _,
                    f.cpu_minor_version as _,
                    f.fpga_major_version as _,
                    f.fpga_minor_version as _,
                    f.fpga_function_bits as _,
                )
            })
            .collect()
    }
}

impl FromMessage<FpgaInfoResponse> for Vec<autd3_driver::fpga::FPGAInfo> {
    fn from_msg(msg: &FpgaInfoResponse) -> Self {
        msg.fpga_info_list
            .iter()
            .map(|f| autd3_driver::fpga::FPGAInfo::new(f.info as _))
            .collect()
    }
}

impl FromMessage<FocusStm> for autd3_driver::datagram::FocusSTM {
    fn from_msg(msg: &FocusStm) -> Self {
        autd3_driver::datagram::FocusSTM::with_sampling_frequency_division(msg.freq_div)
            .add_foci_from_iter(msg.control_points.iter().map(|p| {
                autd3_driver::operation::ControlPoint::new(
                    autd3_driver::geometry::Vector3::from_msg(p.pos.as_ref().unwrap()),
                )
                .with_shift(p.shift as _)
            }))
            .with_start_idx(if msg.start_idx < 0 {
                None
            } else {
                Some(msg.start_idx as _)
            })
            .with_finish_idx(if msg.finish_idx < 0 {
                None
            } else {
                Some(msg.finish_idx as _)
            })
    }
}
