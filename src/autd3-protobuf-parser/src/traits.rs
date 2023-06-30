/*
 * File: traits.rs
 * Project: src
 * Created Date: 30/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::rc::Rc;

use crate::pb::*;
use autd3_core::modulation::ModulationProperty;

pub trait ToMessage {
    type Message: prost::Message;

    fn to_msg(&self) -> Self::Message;
}

pub trait FromMessage<T: prost::Message> {
    fn from_msg(msg: &T) -> Self;
}

impl ToMessage for autd3_core::geometry::Vector3 {
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

impl ToMessage for autd3_core::geometry::Quaternion {
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

impl<T: autd3_core::geometry::Transducer> ToMessage for autd3_core::geometry::Geometry<T> {
    type Message = Geometry;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            geometries: (0..self.num_devices())
                .map(|dev| geometry::Autd3 {
                    position: Some(self.transducers_of(dev).next().unwrap().position().to_msg()),
                    rotation: Some(self.transducers_of(dev).next().unwrap().rotation().to_msg()),
                })
                .collect(),
        }
    }
}

impl ToMessage for autd3_core::TxDatagram {
    type Message = TxRawData;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            data: self.data().to_vec(),
        }
    }
}

impl ToMessage for autd3::modulation::Static {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Modulation(Modulation {
                modulation: Some(modulation::Modulation::Static(Static {
                    amp: self.amp() as _,
                    freq_div: self.sampling_frequency_division() as _,
                })),
            })),
        }
    }
}

impl ToMessage for autd3::modulation::SineLegacy {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Modulation(Modulation {
                modulation: Some(modulation::Modulation::SineLegacy(SineLegacy {
                    freq_div: self.sampling_frequency_division() as _,
                    freq: self.freq() as _,
                    amp: self.amp() as _,
                    offset: self.offset() as _,
                })),
            })),
        }
    }
}

impl ToMessage for autd3::modulation::Sine {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Modulation(Modulation {
                modulation: Some(modulation::Modulation::Sine(Sine {
                    freq_div: self.sampling_frequency_division() as _,
                    freq: self.freq() as _,
                    amp: self.amp() as _,
                    offset: self.offset() as _,
                })),
            })),
        }
    }
}

impl ToMessage for autd3::modulation::SinePressure {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Modulation(Modulation {
                modulation: Some(modulation::Modulation::SinePressure(SinePressure {
                    freq_div: self.sampling_frequency_division() as _,
                    freq: self.freq() as _,
                    amp: self.amp() as _,
                    offset: self.offset() as _,
                })),
            })),
        }
    }
}

impl ToMessage for autd3::modulation::Square {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Modulation(Modulation {
                modulation: Some(modulation::Modulation::Square(Square {
                    freq_div: self.sampling_frequency_division() as _,
                    freq: self.freq() as _,
                    low: self.low() as _,
                    high: self.high() as _,
                    duty: self.duty() as _,
                })),
            })),
        }
    }
}

impl ToMessage for autd3::gain::Focus {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Gain(Gain {
                gain: Some(gain::Gain::Focus(Focus {
                    amp: self.amp() as _,
                    pos: Some(self.pos().to_msg()),
                })),
            })),
        }
    }
}

impl ToMessage for autd3::gain::Bessel {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Gain(Gain {
                gain: Some(gain::Gain::Bessel(Bessel {
                    amp: self.amp() as _,
                    pos: Some(self.pos().to_msg()),
                    dir: Some(self.dir().to_msg()),
                    theta: self.theta() as _,
                })),
            })),
        }
    }
}

impl ToMessage for autd3::gain::Null {
    type Message = Datagram;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Gain(Gain {
                gain: Some(gain::Gain::Null(Null {})),
            })),
        }
    }
}

impl ToMessage for autd3::gain::Plane {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Gain(Gain {
                gain: Some(gain::Gain::Plane(Plane {
                    amp: self.amp() as _,
                    dir: Some(self.dir().to_msg()),
                })),
            })),
        }
    }
}

impl ToMessage for autd3::gain::TransducerTest {
    type Message = Datagram;

    #[allow(clippy::unnecessary_cast)]
    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Gain(Gain {
                gain: Some(gain::Gain::TransTest(TransducerTest {
                    drives: self
                        .test_drive()
                        .iter()
                        .map(|(&idx, &(phase, amp))| transducer_test::TestDrive {
                            idx: idx as _,
                            phase: phase as _,
                            amp: amp as _,
                        })
                        .collect(),
                })),
            })),
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
                    value: *value as _,
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

impl ToMessage for autd3_core::silencer_config::SilencerConfig {
    type Message = Datagram;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::SilencerConfig(SilencerConfig {
                step: self.step() as _,
            })),
        }
    }
}

impl ToMessage for autd3_core::RxDatagram {
    type Message = RxMessage;

    fn to_msg(&self) -> Self::Message {
        let mut data = vec![0; std::mem::size_of::<autd3_core::RxMessage>() * self.len()];
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

impl ToMessage for autd3_core::clear::Clear {
    type Message = Datagram;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Special(SpecialData {
                special: Some(special_data::Special::Clear(Clear {})),
            })),
        }
    }
}

impl ToMessage for autd3_core::synchronize::Synchronize {
    type Message = Datagram;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Special(SpecialData {
                special: Some(special_data::Special::Synchronize(Synchronize {})),
            })),
        }
    }
}

impl ToMessage for autd3_core::stop::Stop {
    type Message = Datagram;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Special(SpecialData {
                special: Some(special_data::Special::Stop(Stop {})),
            })),
        }
    }
}

impl ToMessage for autd3_core::update_flag::UpdateFlags {
    type Message = Datagram;

    fn to_msg(&self) -> Self::Message {
        Self::Message {
            datagram: Some(datagram::Datagram::Special(SpecialData {
                special: Some(special_data::Special::UpdateFlags(UpdateFlags {})),
            })),
        }
    }
}

impl FromMessage<RxMessage> for autd3_core::RxDatagram {
    fn from_msg(msg: &RxMessage) -> Self {
        let mut rx = autd3_core::RxDatagram::new(
            msg.data.len() / std::mem::size_of::<autd3_core::RxMessage>(),
        );
        unsafe {
            std::ptr::copy_nonoverlapping(msg.data.as_ptr(), rx.as_mut_ptr() as _, msg.data.len());
        }
        rx
    }
}

impl FromMessage<Vector3> for autd3_core::geometry::Vector3 {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Vector3) -> Self {
        autd3_core::geometry::Vector3::new(msg.x as _, msg.y as _, msg.z as _)
    }
}

impl FromMessage<Quaternion> for autd3_core::geometry::UnitQuaternion {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Quaternion) -> Self {
        autd3_core::geometry::UnitQuaternion::from_quaternion(
            autd3_core::geometry::Quaternion::new(msg.w as _, msg.x as _, msg.y as _, msg.z as _),
        )
    }
}

impl FromMessage<Static> for autd3::modulation::Static {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Static) -> Self {
        Self::new()
            .with_amp(msg.amp as _)
            .with_sampling_frequency_division(msg.freq_div as _)
    }
}

impl FromMessage<SineLegacy> for autd3::modulation::SineLegacy {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &SineLegacy) -> Self {
        Self::new(msg.freq as _)
            .with_amp(msg.amp as _)
            .with_offset(msg.offset as _)
            .with_sampling_frequency_division(msg.freq_div as _)
    }
}

impl FromMessage<Sine> for autd3::modulation::Sine {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Sine) -> Self {
        Self::new(msg.freq as _)
            .with_amp(msg.amp as _)
            .with_offset(msg.offset as _)
            .with_sampling_frequency_division(msg.freq_div as _)
    }
}

impl FromMessage<SinePressure> for autd3::modulation::SinePressure {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &SinePressure) -> Self {
        Self::new(msg.freq as _)
            .with_amp(msg.amp as _)
            .with_offset(msg.offset as _)
            .with_sampling_frequency_division(msg.freq_div as _)
    }
}

impl FromMessage<Square> for autd3::modulation::Square {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Square) -> Self {
        Self::new(msg.freq as _)
            .with_low(msg.low as _)
            .with_high(msg.high as _)
            .with_duty(msg.duty as _)
            .with_sampling_frequency_division(msg.freq_div as _)
    }
}

impl FromMessage<Focus> for autd3::gain::Focus {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Focus) -> Self {
        Self::new(autd3_core::geometry::Vector3::from_msg(
            msg.pos.as_ref().unwrap(),
        ))
        .with_amp(msg.amp as _)
    }
}

impl FromMessage<Bessel> for autd3::gain::Bessel {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Bessel) -> Self {
        Self::new(
            autd3_core::geometry::Vector3::from_msg(msg.pos.as_ref().unwrap()),
            autd3_core::geometry::Vector3::from_msg(msg.dir.as_ref().unwrap()),
            msg.theta as _,
        )
        .with_amp(msg.amp as _)
    }
}

impl FromMessage<Null> for autd3::gain::Null {
    fn from_msg(_msg: &Null) -> Self {
        Self::new()
    }
}

impl FromMessage<Plane> for autd3::gain::Plane {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Plane) -> Self {
        Self::new(autd3_core::geometry::Vector3::from_msg(
            msg.dir.as_ref().unwrap(),
        ))
        .with_amp(msg.amp as _)
    }
}

impl FromMessage<TransducerTest> for autd3::gain::TransducerTest {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &TransducerTest) -> Self {
        msg.drives.iter().fold(Self::new(), |acc, v| {
            acc.set(v.idx as _, v.phase as _, v.amp as _)
        })
    }
}

impl<'a, T: autd3_core::geometry::Transducer> FromMessage<Grouped> for autd3::gain::Grouped<'a, T> {
    #[allow(clippy::unnecessary_cast)]
    fn from_msg(msg: &Grouped) -> Self {
        msg.groups.iter().fold(Self::new(), |acc, v| {
            if let Some(g) = &v.gain {
                match &g.gain {
                    Some(gain::Gain::Focus(g)) => {
                        acc.add::<autd3::gain::Focus>(v.id as _, autd3::gain::Focus::from_msg(g))
                    }
                    Some(gain::Gain::Bessel(g)) => {
                        acc.add::<autd3::gain::Bessel>(v.id as _, autd3::gain::Bessel::from_msg(g))
                    }
                    Some(gain::Gain::Null(g)) => {
                        acc.add::<autd3::gain::Null>(v.id as _, autd3::gain::Null::from_msg(g))
                    }
                    Some(gain::Gain::Plane(g)) => {
                        acc.add::<autd3::gain::Plane>(v.id as _, autd3::gain::Plane::from_msg(g))
                    }
                    Some(gain::Gain::TransTest(g)) => acc.add::<autd3::gain::TransducerTest>(
                        v.id as _,
                        autd3::gain::TransducerTest::from_msg(g),
                    ),
                    Some(gain::Gain::Sdp(g)) => acc.add::<autd3_gain_holo::SDP<_>>(
                        v.id as _,
                        autd3_gain_holo::SDP::<autd3_gain_holo::NalgebraBackend>::from_msg(g),
                    ),
                    Some(gain::Gain::Evp(g)) => acc.add::<autd3_gain_holo::EVP<_>>(
                        v.id as _,
                        autd3_gain_holo::EVP::<autd3_gain_holo::NalgebraBackend>::from_msg(g),
                    ),
                    Some(gain::Gain::Naive(g)) => acc.add::<autd3_gain_holo::Naive<_>>(
                        v.id as _,
                        autd3_gain_holo::Naive::<autd3_gain_holo::NalgebraBackend>::from_msg(g),
                    ),
                    Some(gain::Gain::Gs(g)) => acc.add::<autd3_gain_holo::GS<_>>(
                        v.id as _,
                        autd3_gain_holo::GS::<autd3_gain_holo::NalgebraBackend>::from_msg(g),
                    ),
                    Some(gain::Gain::Gspat(g)) => acc.add::<autd3_gain_holo::GSPAT<_>>(
                        v.id as _,
                        autd3_gain_holo::GSPAT::<autd3_gain_holo::NalgebraBackend>::from_msg(g),
                    ),
                    Some(gain::Gain::Lm(g)) => acc.add::<autd3_gain_holo::LM<_>>(
                        v.id as _,
                        autd3_gain_holo::LM::<autd3_gain_holo::NalgebraBackend>::from_msg(g),
                    ),
                    Some(gain::Gain::Greedy(g)) => acc.add::<autd3_gain_holo::Greedy>(
                        v.id as _,
                        autd3_gain_holo::Greedy::from_msg(g),
                    ),
                    _ => acc,
                }
            } else {
                acc
            }
        })
    }
}

impl FromMessage<Constraint> for autd3_gain_holo::Constraint {
    fn from_msg(msg: &Constraint) -> Self {
        match &msg.constraint {
            Some(constraint::Constraint::DontCare(_)) => Self::DontCare,
            Some(constraint::Constraint::Normalize(_)) => Self::Normalize,
            Some(constraint::Constraint::Uniform(uniform)) => Self::Uniform(uniform.value as _),
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
                    autd3_core::geometry::Vector3::from_msg(h.pos.as_ref().unwrap()),
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
                    autd3_core::geometry::Vector3::from_msg(h.pos.as_ref().unwrap()),
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
                    autd3_core::geometry::Vector3::from_msg(h.pos.as_ref().unwrap()),
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
                    autd3_core::geometry::Vector3::from_msg(h.pos.as_ref().unwrap()),
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
                    autd3_core::geometry::Vector3::from_msg(h.pos.as_ref().unwrap()),
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
                    autd3_core::geometry::Vector3::from_msg(h.pos.as_ref().unwrap()),
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
                    autd3_core::geometry::Vector3::from_msg(h.pos.as_ref().unwrap()),
                    h.amp as _,
                )
            }))
    }
}

impl FromMessage<Geometry> for Vec<autd3_core::autd3_device::AUTD3> {
    fn from_msg(msg: &Geometry) -> Self {
        msg.geometries
            .iter()
            .map(|dev| {
                autd3_core::autd3_device::AUTD3::with_quaternion(
                    autd3_core::geometry::Vector3::from_msg(dev.position.as_ref().unwrap()),
                    autd3_core::geometry::UnitQuaternion::from_msg(dev.rotation.as_ref().unwrap()),
                )
            })
            .collect()
    }
}

impl FromMessage<TxRawData> for autd3_core::TxDatagram {
    fn from_msg(msg: &TxRawData) -> Self {
        let len = msg.data.len();
        let header_size = std::mem::size_of::<autd3_core::GlobalHeader>();
        let body_size = std::mem::size_of::<u16>() * autd3_core::autd3_device::NUM_TRANS_IN_UNIT;
        let body_num = if len > header_size {
            if (len - header_size) % body_size != 0 {
                0
            } else {
                (len - header_size) / body_size
            }
        } else {
            0
        };
        let mut tx = autd3_core::TxDatagram::new(&vec![
            autd3_core::autd3_device::NUM_TRANS_IN_UNIT;
            body_num
        ]);
        tx.num_bodies = body_num;
        let body_len = body_num * body_size;
        unsafe {
            std::ptr::copy_nonoverlapping(
                msg.data[header_size..].as_ptr(),
                tx.body_raw_mut().as_mut_ptr() as *mut u8,
                body_len,
            );
            std::ptr::copy_nonoverlapping(
                msg.data.as_ptr(),
                tx.header_mut() as *mut _ as *mut u8,
                header_size,
            );
        }
        tx
    }
}

impl FromMessage<SilencerConfig> for autd3_core::silencer_config::SilencerConfig {
    fn from_msg(msg: &SilencerConfig) -> Self {
        Self::new(msg.step as _)
    }
}
