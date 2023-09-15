/*
 * File: lib.rs
 * Project: src
 * Created Date: 29/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

pub use autd3capi_common as common;
pub use autd3capi_common::holo;

use autd3capi_common::float;
use common::{
    ConstPtr, DynamicDatagram, DynamicTransducer, Gain, Link, Modulation, STMProps, G, L, M,
};

pub const NUM_TRANS_IN_UNIT: u32 = 249;
pub const NUM_TRANS_IN_X: u32 = 18;
pub const NUM_TRANS_IN_Y: u32 = 14;
pub const TRANS_SPACING_MM: float = 10.16;
pub const DEVICE_HEIGHT_MM: float = 151.4;
pub const DEVICE_WIDTH_MM: float = 192.0;
pub const FPGA_CLK_FREQ: u32 = 163840000;
pub const FPGA_SUB_CLK_FREQ: u32 = 20480000;

pub const AUTD3_ERR: i32 = -1;
pub const AUTD3_TRUE: i32 = 1;
pub const AUTD3_FALSE: i32 = 0;

#[repr(u8)]
pub enum GainSTMMode {
    PhaseDutyFull = 0,
    PhaseFull = 1,
    PhaseHalf = 2,
}

impl From<GainSTMMode> for common::autd3::prelude::GainSTMMode {
    fn from(mode: GainSTMMode) -> Self {
        match mode {
            GainSTMMode::PhaseDutyFull => common::autd3::prelude::GainSTMMode::PhaseDutyFull,
            GainSTMMode::PhaseFull => common::autd3::prelude::GainSTMMode::PhaseFull,
            GainSTMMode::PhaseHalf => common::autd3::prelude::GainSTMMode::PhaseHalf,
        }
    }
}

#[repr(u8)]
pub enum TransMode {
    Legacy = 0,
    Advanced = 1,
    AdvancedPhase = 2,
}

impl From<TransMode> for common::TransMode {
    fn from(value: TransMode) -> Self {
        match value {
            TransMode::Legacy => common::TransMode::Legacy,
            TransMode::Advanced => common::TransMode::Advanced,
            TransMode::AdvancedPhase => common::TransMode::AdvancedPhase,
        }
    }
}

#[repr(u8)]
pub enum Level {
    Critical = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
    Off = 6,
}

impl From<Level> for common::autd3::prelude::LevelFilter {
    fn from(level: Level) -> Self {
        match level {
            Level::Critical => common::autd3::prelude::LevelFilter::MoreSevereEqual(
                common::autd3::prelude::Level::Critical,
            ),
            Level::Error => common::autd3::prelude::LevelFilter::MoreSevereEqual(
                common::autd3::prelude::Level::Error,
            ),
            Level::Warn => common::autd3::prelude::LevelFilter::MoreSevereEqual(
                common::autd3::prelude::Level::Warn,
            ),
            Level::Info => common::autd3::prelude::LevelFilter::MoreSevereEqual(
                common::autd3::prelude::Level::Info,
            ),
            Level::Debug => common::autd3::prelude::LevelFilter::MoreSevereEqual(
                common::autd3::prelude::Level::Debug,
            ),
            Level::Trace => common::autd3::prelude::LevelFilter::MoreSevereEqual(
                common::autd3::prelude::Level::Trace,
            ),
            Level::Off => common::autd3::prelude::LevelFilter::Off,
        }
    }
}

#[repr(u8)]
pub enum TimerStrategy {
    Sleep = 0,
    NativeTimer = 1,
    BusyWait = 2,
}

impl From<TimerStrategy> for common::autd3::prelude::TimerStrategy {
    fn from(strategy: TimerStrategy) -> Self {
        match strategy {
            TimerStrategy::Sleep => common::autd3::prelude::TimerStrategy::Sleep,
            TimerStrategy::NativeTimer => common::autd3::prelude::TimerStrategy::NativeTimer,
            TimerStrategy::BusyWait => common::autd3::prelude::TimerStrategy::BusyWait,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ControllerPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GeometryPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DevicePtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TransducerPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LinkPtr(pub ConstPtr);

impl LinkPtr {
    pub fn new<T: Link<DynamicTransducer> + 'static>(link: T) -> Self {
        let l: Box<Box<L>> = Box::new(Box::new(link));
        Self(Box::into_raw(l) as _)
    }
}

#[macro_export]
macro_rules! take_link {
    ($ptr:expr, $type:ty) => {
        Box::from_raw($ptr.0 as *mut Box<L> as *mut Box<$type>)
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DatagramPtr(pub ConstPtr);

impl DatagramPtr {
    pub fn new<T: DynamicDatagram>(d: T) -> Self {
        let d: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(d));
        Self(Box::into_raw(d) as _)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DatagramSpecialPtr(pub ConstPtr);

impl DatagramSpecialPtr {
    pub fn new<T: DynamicDatagram>(d: T) -> Self {
        let d: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(d));
        Self(Box::into_raw(d) as _)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GainPtr(pub ConstPtr);

impl GainPtr {
    pub fn new<T: Gain<DynamicTransducer> + 'static>(g: T) -> Self {
        let g: Box<Box<G>> = Box::new(Box::new(g));
        Self(Box::into_raw(g) as _)
    }
}

#[macro_export]
macro_rules! take_gain {
    ($ptr:expr, $type:ty) => {
        Box::from_raw($ptr.0 as *mut Box<G> as *mut Box<$type>)
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ModulationPtr(pub ConstPtr);

impl ModulationPtr {
    pub fn new<T: Modulation + 'static>(m: T) -> Self {
        let m: Box<Box<M>> = Box::new(Box::new(m));
        Self(Box::into_raw(m) as _)
    }
}

#[macro_export]
macro_rules! take_mod {
    ($ptr:expr, $type:ty) => {
        Box::from_raw($ptr.0 as *mut Box<M> as *mut Box<$type>)
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct STMPropsPtr(pub ConstPtr);

impl STMPropsPtr {
    pub fn new(props: STMProps) -> Self {
        Self(Box::into_raw(Box::new(props)) as _)
    }
}

#[macro_export]
macro_rules! create_holo {
    ($type:tt, $backend_type:tt, $backend:expr, $points:expr, $amps:expr, $size:expr) => {
        GainPtr::new(
            $type::new(cast!($backend.0, Rc<$backend_type>).clone()).add_foci_from_iter(
                (0..$size as usize).map(|i| {
                    let p = Vector3::new(
                        $points.add(i * 3).read(),
                        $points.add(i * 3 + 1).read(),
                        $points.add(i * 3 + 2).read(),
                    );
                    let amp = *$amps.add(i);
                    (p, amp)
                }),
            ),
        )
    };

    ($type:tt, $points:expr, $amps:expr, $size:expr) => {
        GainPtr::new(
            $type::new().add_foci_from_iter((0..$size as usize).map(|i| {
                let p = Vector3::new(
                    $points.add(i * 3).read(),
                    $points.add(i * 3 + 1).read(),
                    $points.add(i * 3 + 2).read(),
                );
                let amp = *$amps.add(i);
                (p, amp)
            })),
        )
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BackendPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ConstraintPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GroupGainMapPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GroupKVMapPtr(pub ConstPtr);
