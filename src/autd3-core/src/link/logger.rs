/*
 * File: logger.rs
 * Project: link
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::sync::{Arc, RwLock};

use spdlog::{
    formatter::{Formatter, FullFormatter},
    prelude::*,
    sink::Sink,
    ErrorHandler, StringBuf,
};

type SinkErrorHandler = Option<ErrorHandler>;

/// logger with custom output and flush callback
pub struct CustomSink<O, F>
where
    O: Fn(&str) -> spdlog::Result<()> + Send + Sync,
    F: Fn() -> spdlog::Result<()> + Send + Sync,
{
    level_filter: RwLock<LevelFilter>,
    formatter: RwLock<Box<dyn Formatter>>,
    error_handler: RwLock<SinkErrorHandler>,
    out: O,
    flush: F,
}

impl<O, F> CustomSink<O, F>
where
    O: Fn(&str) -> spdlog::Result<()> + Send + Sync,
    F: Fn() -> spdlog::Result<()> + Send + Sync,
{
    pub fn new(out: O, flush: F) -> Self {
        Self {
            level_filter: RwLock::new(LevelFilter::All),
            formatter: RwLock::new(Box::new(FullFormatter::new())),
            error_handler: RwLock::new(None),
            out,
            flush,
        }
    }
}

impl<O, F> Sink for CustomSink<O, F>
where
    O: Fn(&str) -> spdlog::Result<()> + Send + Sync,
    F: Fn() -> spdlog::Result<()> + Send + Sync,
{
    fn log(&self, record: &spdlog::Record) -> spdlog::Result<()> {
        if !self.should_log(record.level()) {
            return Ok(());
        }
        let mut string_buf = StringBuf::new();
        self.formatter
            .read()
            .unwrap()
            .format(record, &mut string_buf)?;
        (self.out)(string_buf.as_str())
    }

    fn flush(&self) -> spdlog::Result<()> {
        (self.flush)()
    }

    fn level_filter(&self) -> LevelFilter {
        *self.level_filter.read().unwrap()
    }

    fn set_level_filter(&self, level_filter: LevelFilter) {
        *self.level_filter.write().unwrap() = level_filter;
    }

    fn set_formatter(&self, formatter: Box<dyn spdlog::formatter::Formatter>) {
        *self.formatter.write().unwrap() = formatter;
    }

    fn set_error_handler(&self, handler: Option<spdlog::ErrorHandler>) {
        *self.error_handler.write().unwrap() = handler;
    }
}

fn get_default_sink() -> Vec<Arc<dyn Sink>> {
    spdlog::default_logger().sinks().to_owned()
}

fn get_custom_sink<O, F>(out: O, flush: F) -> Vec<Arc<dyn Sink>>
where
    O: Fn(&str) -> spdlog::Result<()> + Send + Sync + 'static,
    F: Fn() -> spdlog::Result<()> + Send + Sync + 'static,
{
    vec![Arc::new(CustomSink::new(out, flush))]
}

/// Create default logger
pub fn get_logger() -> Logger {
    Logger::builder()
        .sinks(get_default_sink())
        .name("AUTD3")
        .build()
        .unwrap()
}

/// Create logger with custom output and flush callback
pub fn get_logger_with_custom_func<O, F>(out: O, flush: F) -> Logger
where
    O: Fn(&str) -> spdlog::Result<()> + Send + Sync + 'static,
    F: Fn() -> spdlog::Result<()> + Send + Sync + 'static,
{
    Logger::builder()
        .sinks(get_custom_sink(out, flush))
        .name("AUTD3")
        .build()
        .unwrap()
}
