/*
 * File: log.rs
 * Project: link
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{marker::PhantomData, time::Duration};

use crate::core::{
    error::AUTDInternalError, link::Link, CPUControlFlags, RxDatagram, TxDatagram, MSG_CLEAR,
    MSG_RD_CPU_VERSION, MSG_RD_CPU_VERSION_MINOR, MSG_RD_FPGA_FUNCTION, MSG_RD_FPGA_VERSION,
    MSG_RD_FPGA_VERSION_MINOR,
};

use autd3_core::geometry::{Geometry, Transducer};
use spdlog::prelude::*;

pub use spdlog::Level;

pub struct Log<T: Transducer, L: Link<T>> {
    link: L,
    logger: Logger,
    synchronized: bool,
    _trans: PhantomData<T>,
}

impl<T: Transducer, L: Link<T>> Log<T, L> {
    pub fn new(link: L) -> Self {
        let logger = crate::core::link::get_logger();
        logger.set_level_filter(spdlog::LevelFilter::MoreSevereEqual(spdlog::Level::Info));
        Self {
            link,
            logger,
            synchronized: false,
            _trans: PhantomData,
        }
    }

    pub fn with_level(self, level: LevelFilter) -> Self {
        self.logger.set_level_filter(level);
        self
    }

    pub fn with_logger(self, logger: Logger) -> Self {
        Self { logger, ..self }
    }
}

impl<T: Transducer, L: Link<T>> Link<T> for Log<T, L> {
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

        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        trace!(logger: self.logger, "Close Log link");

        if !self.is_open() {
            warn!(logger: self.logger, "Log link is already closed.");
            return Ok(());
        }

        self.synchronized = false;

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

        match tx.header().msg_id {
            MSG_CLEAR
            | MSG_RD_CPU_VERSION
            | MSG_RD_CPU_VERSION_MINOR
            | MSG_RD_FPGA_VERSION
            | MSG_RD_FPGA_VERSION_MINOR
            | MSG_RD_FPGA_FUNCTION => {}
            _ => {
                if !tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_EN_N)
                    && tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_SYNC)
                {
                    self.synchronized = true;
                }
                if !self.synchronized {
                    warn!(logger: self.logger, "Devices are not not synchronized");
                }
            }
        }
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
        if !self.wait_msg_processed(tx.header().msg_id, rx, timeout)? {
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
