/*
 * File: audit.rs
 * Project: link
 * Created Date: 14/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use autd3_driver::{
    cpu::{RxMessage, TxDatagram},
    error::AUTDInternalError,
    geometry::{Device, Transducer},
    link::Link,
};
use autd3_firmware_emulator::CPUEmulator;

/// Link for test
pub struct Audit {
    is_open: bool,
    timeout: Duration,
    last_timeout: Duration,
    cpus: Vec<CPUEmulator>,
    down: bool,
    broken: bool,
}

impl Audit {
    pub fn new() -> Self {
        Self {
            is_open: false,
            timeout: Duration::ZERO,
            last_timeout: Duration::ZERO,
            cpus: Vec::new(),
            down: false,
            broken: false,
        }
    }

    /// set timeout
    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }

    pub fn last_timeout(&self) -> Duration {
        self.last_timeout
    }

    pub fn emulators(&self) -> &[CPUEmulator] {
        &self.cpus
    }

    pub fn emulators_mut(&mut self) -> &mut [CPUEmulator] {
        &mut self.cpus
    }

    pub fn down(&mut self) {
        self.down = true;
    }

    pub fn up(&mut self) {
        self.down = false;
    }

    pub fn break_down(&mut self) {
        self.broken = true;
    }

    pub fn repair(&mut self) {
        self.broken = false;
    }
}

impl Deref for Audit {
    type Target = [CPUEmulator];

    fn deref(&self) -> &Self::Target {
        &self.cpus
    }
}

impl DerefMut for Audit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cpus
    }
}

impl Default for Audit {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Transducer> Link<T> for Audit {
    fn open(&mut self, devices: &[Device<T>]) -> Result<(), AUTDInternalError> {
        self.cpus = devices
            .iter()
            .enumerate()
            .map(|(i, dev)| {
                let mut cpu = CPUEmulator::new(i, dev.num_transducers());
                cpu.init();
                cpu
            })
            .collect();

        self.is_open = true;

        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        self.is_open = false;
        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open {
            return Ok(false);
        }

        if self.broken {
            return Err(AUTDInternalError::LinkError("broken".to_owned()));
        }

        if self.down {
            return Ok(false);
        }

        self.cpus.iter_mut().for_each(|cpu| {
            cpu.send(tx);
        });

        Ok(true)
    }

    fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError> {
        if !self.is_open {
            return Ok(false);
        }

        if self.broken {
            return Err(AUTDInternalError::LinkError("broken".to_owned()));
        }

        if self.down {
            return Ok(false);
        }

        self.cpus.iter_mut().for_each(|cpu| {
            rx[cpu.idx()].data = cpu.rx_data();
            rx[cpu.idx()].ack = cpu.ack();
        });

        Ok(true)
    }

    fn send_receive(
        &mut self,
        tx: &TxDatagram,
        rx: &mut [RxMessage],
        timeout: Duration,
    ) -> Result<bool, AUTDInternalError> {
        self.last_timeout = timeout;
        if !<Self as Link<T>>::send(self, tx)? {
            return Ok(false);
        }
        if timeout.is_zero() {
            return <Self as Link<T>>::receive(self, rx);
        }
        <Self as Link<T>>::wait_msg_processed(self, tx, rx, timeout)
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}
