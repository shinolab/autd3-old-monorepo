/*
 * File: logger.rs
 * Project: link
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use atomic::Atomic;
use spin::RwLock as SpinRwLock;
use std::sync::{atomic::*, Arc};

use spdlog::{
    formatter::{Formatter, FullFormatter},
    prelude::*,
    sink::Sink,
    ErrorHandler, StringBuf,
};

type SinkErrorHandler = Atomic<Option<ErrorHandler>>;

/// logger with custom output and flush callback
pub struct CustomSink<O, F>
where
    O: Fn(&str) -> spdlog::Result<()> + Send + Sync,
    F: Fn() -> spdlog::Result<()> + Send + Sync,
{
    level_filter: Atomic<LevelFilter>,
    formatter: SpinRwLock<Box<dyn Formatter>>,
    error_handler: SinkErrorHandler,
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
            level_filter: Atomic::new(LevelFilter::All),
            formatter: SpinRwLock::new(Box::new(FullFormatter::new())),
            error_handler: SinkErrorHandler::new(None),
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
        self.formatter.read().format(record, &mut string_buf)?;
        (self.out)(string_buf.as_str())
    }

    fn flush(&self) -> spdlog::Result<()> {
        (self.flush)()
    }

    fn level_filter(&self) -> LevelFilter {
        self.level_filter.load(Ordering::Relaxed)
    }

    fn set_level_filter(&self, level_filter: LevelFilter) {
        self.level_filter.store(level_filter, Ordering::Relaxed);
    }

    fn set_formatter(&self, formatter: Box<dyn spdlog::formatter::Formatter>) {
        *self.formatter.write() = formatter;
    }

    fn set_error_handler(&self, handler: Option<spdlog::ErrorHandler>) {
        self.error_handler.store(handler, Ordering::Relaxed);
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
