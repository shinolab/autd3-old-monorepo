/*
 * File: timer_strategy.rs
 * Project: src
 * Created Date: 08/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

/// Timer strategy
#[derive(Clone, Copy)]
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
        let sc = s;

        assert_eq!(s as u8, sc as u8);
    }
}
