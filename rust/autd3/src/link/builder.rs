/*
 * File: builder.rs
 * Project: link
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use autd3_core::link::Link;
use spdlog::prelude::*;

use super::Log;

pub trait LinkBuilder {
    type L: Link;

    fn timeout(self, timeout: Duration) -> Self;
    fn build(self) -> Self::L;
    fn build_with_custom_logger<O, F>(self, level: Level, out: O, flush: F) -> Log<Self::L>
    where
        Self: Sized,
        O: Fn(&str) -> spdlog::Result<()> + Send + Sync + 'static,
        F: Fn() -> spdlog::Result<()> + Send + Sync + 'static,
    {
        let logger = super::logger::get_logger_with_custom_func(level, out, flush);
        Log::with_logger(self.build(), logger)
    }
    fn build_with_default_logger(self, level: Level) -> Log<Self::L>
    where
        Self: Sized,
    {
        Log::new(self.build(), level)
    }
}
