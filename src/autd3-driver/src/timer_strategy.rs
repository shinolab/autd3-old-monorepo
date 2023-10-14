/*
 * File: timer_strategy.rs
 * Project: src
 * Created Date: 08/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use serde::{Deserialize, Serialize};

/// Timer strategy
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(u8)]
pub enum TimerStrategy {
    /// Use `std::thread::sleep`
    Sleep = 0,
    /// Use busy wait
    BusyWait = 1,
    /// Use OS-native timer
    NativeTimer = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_strategy() {
        assert_eq!(std::mem::size_of::<TimerStrategy>(), 1);

        let s = TimerStrategy::Sleep;

        let sc = Clone::clone(&s);
        assert_eq!(s as u8, sc as u8);
    }

    #[test]
    fn debug() {
        let s = TimerStrategy::Sleep;
        assert_eq!(format!("{:?}", s), "Sleep");
    }

    #[test]
    fn serde() {
        let s = TimerStrategy::Sleep;

        let serialized = serde_json::to_string(&s).unwrap();
        assert_eq!(serialized, "\"Sleep\"");

        let deserialized: TimerStrategy = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, TimerStrategy::Sleep);
    }
}
