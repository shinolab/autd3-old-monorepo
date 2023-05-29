/*
 * File: lib.rs
 * Project: src
 * Created Date: 29/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_common::float;

pub const NUM_TRANS_IN_UNIT: u32 = 249;
pub const NUM_TRANS_IN_X: u32 = 18;
pub const NUM_TRANS_IN_Y: u32 = 14;
pub const TRANS_SPACING_MM: float = 10.16;
pub const DEVICE_HEIGHT: float = 151.4;
pub const DEVICE_WIDTH: float = 192.0;
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

impl From<GainSTMMode> for autd3::prelude::Mode {
    fn from(mode: GainSTMMode) -> Self {
        match mode {
            GainSTMMode::PhaseDutyFull => autd3::prelude::Mode::PhaseDutyFull,
            GainSTMMode::PhaseFull => autd3::prelude::Mode::PhaseFull,
            GainSTMMode::PhaseHalf => autd3::prelude::Mode::PhaseHalf,
        }
    }
}

#[repr(u8)]
pub enum TransMode {
    Legacy = 0,
    Advanced = 1,
    AdvancedPhase = 2,
}

impl From<TransMode> for autd3capi_common::dynamic_transducer::TransMode {
    fn from(value: TransMode) -> Self {
        match value {
            TransMode::Legacy => autd3capi_common::dynamic_transducer::TransMode::Legacy,
            TransMode::Advanced => autd3capi_common::dynamic_transducer::TransMode::Advanced,
            TransMode::AdvancedPhase => {
                autd3capi_common::dynamic_transducer::TransMode::AdvancedPhase
            }
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

impl From<Level> for autd3::prelude::LevelFilter {
    fn from(level: Level) -> Self {
        match level {
            Level::Critical => {
                autd3::prelude::LevelFilter::MoreSevereEqual(autd3::prelude::Level::Critical)
            }
            Level::Error => {
                autd3::prelude::LevelFilter::MoreSevereEqual(autd3::prelude::Level::Error)
            }
            Level::Warn => {
                autd3::prelude::LevelFilter::MoreSevereEqual(autd3::prelude::Level::Warn)
            }
            Level::Info => {
                autd3::prelude::LevelFilter::MoreSevereEqual(autd3::prelude::Level::Info)
            }
            Level::Debug => {
                autd3::prelude::LevelFilter::MoreSevereEqual(autd3::prelude::Level::Debug)
            }
            Level::Trace => {
                autd3::prelude::LevelFilter::MoreSevereEqual(autd3::prelude::Level::Trace)
            }
            Level::Off => autd3::prelude::LevelFilter::Off,
        }
    }
}

#[repr(u8)]
pub enum TimerStrategy {
    Sleep = 0,
    NativeTimer = 1,
    BusyWait = 2,
}

impl From<TimerStrategy> for autd3::prelude::TimerStrategy {
    fn from(strategy: TimerStrategy) -> Self {
        match strategy {
            TimerStrategy::Sleep => autd3::prelude::TimerStrategy::Sleep,
            TimerStrategy::NativeTimer => autd3::prelude::TimerStrategy::NativeTimer,
            TimerStrategy::BusyWait => autd3::prelude::TimerStrategy::BusyWait,
        }
    }
}
