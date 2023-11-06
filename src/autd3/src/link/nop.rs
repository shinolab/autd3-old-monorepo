/*
 * File: nop.rs
 * Project: link
 * Created Date: 06/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_derive::LinkSync;
use autd3_driver::{
    cpu::{RxMessage, TxDatagram},
    error::AUTDInternalError,
    geometry::{Geometry},
    link::{Link, LinkBuilder},
};
use autd3_firmware_emulator::CPUEmulator;

/// Link to do nothing
#[derive(LinkSync)]
pub struct Nop {
    is_open: bool,
    cpus: Vec<CPUEmulator>,
}

pub struct NopBuilder {}

impl NopBuilder {
    pub fn with_timeout(self, _timeout: std::time::Duration) -> Self {
        self
    }
}

#[async_trait::async_trait]
impl LinkBuilder for NopBuilder {
    type L = Nop;

    async fn open(self, geometry: &Geometry) -> Result<Self::L, AUTDInternalError> {
        Ok(Nop {
            is_open: true,
            cpus: geometry
                .iter()
                .enumerate()
                .map(|(i, dev)| {
                    let mut cpu = CPUEmulator::new(i, dev.num_transducers());
                    cpu.init();
                    cpu
                })
                .collect(),
        })
    }
}

#[async_trait::async_trait]
impl Link for Nop {
    async fn close(&mut self) -> Result<(), AUTDInternalError> {
        Ok(())
    }

    async fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open {
            return Err(AUTDInternalError::LinkClosed);
        }

        self.cpus.iter_mut().for_each(|cpu| {
            cpu.send(tx);
        });

        Ok(true)
    }

    async fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError> {
        if !self.is_open {
            return Err(AUTDInternalError::LinkClosed);
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

    fn timeout(&self) -> std::time::Duration {
        std::time::Duration::ZERO
    }
}

impl Nop {
    pub fn builder() -> NopBuilder {
        NopBuilder {}
    }
}
