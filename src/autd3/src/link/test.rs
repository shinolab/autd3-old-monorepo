/*
 * File: debug.rs
 * Project: link
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/09/2023
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
    cpu::{RxDatagram, TxDatagram},
    error::AUTDInternalError,
    geometry::{Device, Transducer},
    link::Link,
};
use autd3_firmware_emulator::CPUEmulator;

/// Link for test
pub struct Test {
    is_open: bool,
    timeout: Duration,
    cpus: Vec<CPUEmulator>,
}

impl Test {
    pub fn new() -> Self {
        Self {
            is_open: false,
            timeout: Duration::ZERO,
            cpus: Vec::new(),
        }
    }

    /// set timeout
    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }

    pub fn emulators(&self) -> &[CPUEmulator] {
        &self.cpus
    }

    pub fn emulators_mut(&mut self) -> &mut [CPUEmulator] {
        &mut self.cpus
    }
}

impl Deref for Test {
    type Target = [CPUEmulator];

    fn deref(&self) -> &Self::Target {
        &self.cpus
    }
}

impl DerefMut for Test {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cpus
    }
}

impl Default for Test {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Transducer> Link<T> for Test {
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

        self.cpus.iter_mut().for_each(|cpu| {
            cpu.send(tx);
        });

        Ok(true)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open {
            return Ok(false);
        }

        self.cpus.iter_mut().for_each(|cpu| {
            rx[cpu.idx()].data = cpu.rx_data();
            rx[cpu.idx()].ack = cpu.ack();
        });

        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}
