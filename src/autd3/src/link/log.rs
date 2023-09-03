/*
 * File: log.rs
 * Project: link
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    time::Duration,
};

use autd3_driver::{
    cpu::{RxDatagram, TxDatagram},
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::Link,
    operation::TypeTag,
};
use spdlog::prelude::*;

pub use spdlog::Level;

/// Link to logging
pub struct LogImpl<T: Transducer, L: Link<T>> {
    link: L,
    logger: Logger,
    synchronized: Vec<bool>,
    _trans: PhantomData<T>,
}

pub trait Log<T: Transducer, L: Link<T>> {
    /// enable logger
    fn with_log(self) -> LogImpl<T, L>;
}

impl<T: Transducer, L: Link<T>> Log<T, L> for L {
    fn with_log(self) -> LogImpl<T, L> {
        LogImpl::new(self)
    }
}

impl<T: Transducer, L: Link<T>> Deref for LogImpl<T, L> {
    type Target = L;

    fn deref(&self) -> &Self::Target {
        &self.link
    }
}

impl<T: Transducer, L: Link<T>> DerefMut for LogImpl<T, L> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.link
    }
}

impl<T: Transducer, L: Link<T>> LogImpl<T, L> {
    fn new(link: L) -> Self {
        let logger = autd3_driver::logger::get_logger();
        logger.set_level_filter(spdlog::LevelFilter::MoreSevereEqual(spdlog::Level::Info));
        Self {
            link,
            logger,
            synchronized: Default::default(),
            _trans: PhantomData,
        }
    }

    /// set log level
    pub fn with_log_level(self, level: LevelFilter) -> Self {
        self.logger.set_level_filter(level);
        self
    }

    /// set logger
    ///
    /// By default, the logger will display the log on the console.
    pub fn with_logger(self, logger: Logger) -> Self {
        Self { logger, ..self }
    }
}

impl<T: Transducer, L: Link<T>> Link<T> for LogImpl<T, L> {
    fn open(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        trace!(logger: self.logger, "Open Log link");

        if self.is_open() {
            warn!(logger: self.logger, "Log link is already opened.");
            return Ok(());
        }

        let res = self.link.open(geometry);
        if res.is_err() {
            error!(logger: self.logger, "Failed to open link");
            return res;
        }

        self.synchronized = vec![false; geometry.num_devices()];

        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        trace!(logger: self.logger, "Close Log link");

        if !self.is_open() {
            warn!(logger: self.logger, "Log link is already closed.");
            return Ok(());
        }

        self.synchronized.fill(false);

        let res = self.link.close();
        if res.is_err() {
            error!(logger: self.logger, "Failed to close link");
            return res;
        }

        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        debug!(logger: self.logger, "Send data");

        if !self.is_open() {
            warn!(logger: self.logger, "Link is not opened");
            return Ok(false);
        }

        tx.headers()
            .zip(tx.bodies())
            .enumerate()
            .for_each(|(i, (h, b))| {
                let mut print = |tag| match tag {
                    TypeTag::NONE | TypeTag::Clear | TypeTag::FirmwareInfo => (),
                    TypeTag::Sync => {
                        self.synchronized[i] = true;
                    }
                    _ => {
                        if !self.synchronized[i] {
                            warn!(logger: self.logger, "Device ({i}) is not synchronized");
                        }
                    }
                };
                print(TypeTag::from(b[0]));
                if h.slot_2_offset != 0 {
                    print(TypeTag::from(b[h.slot_2_offset as usize]));
                }
            });

        if !self.link.send(tx)? {
            error!(logger: self.logger, "Failed to send data");
            return Ok(false);
        }
        Ok(true)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        debug!(logger: self.logger, "Receive data");

        if !self.is_open() {
            warn!(logger: self.logger, "Link is not opened");
            return Ok(false);
        }

        if !self.link.receive(rx)? {
            error!(logger: self.logger, "Failed to receive data");
            return Ok(false);
        }
        Ok(true)
    }

    fn send_receive(
        &mut self,
        tx: &TxDatagram,
        rx: &mut RxDatagram,
        timeout: Duration,
    ) -> Result<bool, AUTDInternalError> {
        if !self.send(tx)? {
            return Ok(false);
        }
        if timeout.is_zero() {
            return self.receive(rx);
        }
        if !self.wait_msg_processed(tx, rx, timeout)? {
            error!(logger: self.logger, "Failed to confirm that the data was processed");
            return Ok(false);
        }
        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.link.is_open()
    }

    fn timeout(&self) -> Duration {
        Duration::ZERO
    }
}
