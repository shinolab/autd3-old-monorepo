/*
 * File: lib.rs
 * Project: src
 * Created Date: 29/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

pub use autd3capi_common as common;

use autd3capi_common::float;

pub const NUM_TRANS_IN_UNIT: u32 = 249;
pub const NUM_TRANS_IN_X: u32 = 18;
pub const NUM_TRANS_IN_Y: u32 = 14;
pub const TRANS_SPACING_MM: float = 10.16;
pub const DEVICE_HEIGHT_MM: float = 151.4;
pub const DEVICE_WIDTH_MM: float = 192.0;
pub const FPGA_CLK_FREQ: u32 = 163840000;
pub const FPGA_SUB_CLK_FREQ: u32 = 20480000;

pub const ERR: i32 = -1;
pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;

#[repr(u8)]
pub enum GainSTMMode {
    PhaseDutyFull = 0,
    PhaseFull = 1,
    PhaseHalf = 2,
}

impl From<GainSTMMode> for common::autd3::prelude::Mode {
    fn from(mode: GainSTMMode) -> Self {
        match mode {
            GainSTMMode::PhaseDutyFull => common::autd3::prelude::Mode::PhaseDutyFull,
            GainSTMMode::PhaseFull => common::autd3::prelude::Mode::PhaseFull,
            GainSTMMode::PhaseHalf => common::autd3::prelude::Mode::PhaseHalf,
        }
    }
}

#[repr(u8)]
pub enum TransMode {
    Legacy = 0,
    Advanced = 1,
    AdvancedPhase = 2,
}

impl From<TransMode> for common::dynamic_transducer::TransMode {
    fn from(value: TransMode) -> Self {
        match value {
            TransMode::Legacy => common::dynamic_transducer::TransMode::Legacy,
            TransMode::Advanced => common::dynamic_transducer::TransMode::Advanced,
            TransMode::AdvancedPhase => common::dynamic_transducer::TransMode::AdvancedPhase,
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
