/*
 * File: nop.rs
 * Project: link
 * Created Date: 06/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{
    cpu::{RxMessage, TxDatagram},
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::{Link, LinkBuilder},
};
use autd3_firmware_emulator::CPUEmulator;

/// Link to do nothing
#[derive(Default)]
pub struct Nop {
    is_open: bool,
    cpus: Vec<CPUEmulator>,
}

pub struct NopBuilder {}

impl Nop {
    pub fn builder() -> NopBuilder {
        NopBuilder {}
    }
}

impl<T: Transducer> LinkBuilder<T> for NopBuilder {
    type L = Nop;

    fn open(self, geometry: &Geometry<T>) -> Result<Self::L, AUTDInternalError> {
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

impl Link for Nop {
    fn close(&mut self) -> Result<(), AUTDInternalError> {
        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open {
            return Err(AUTDInternalError::LinkClosed);
        }

        self.cpus.iter_mut().for_each(|cpu| {
            cpu.send(tx);
        });

        Ok(true)
    }

    fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError> {
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
