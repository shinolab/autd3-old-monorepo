/*
 * File: debug_link.rs
 * Project: link
 * Created Date: 09/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

use std::time::Duration;

use autd3_core::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::Link,
    CPUControlFlags, RxDatagram, TxDatagram, MSG_CLEAR, MSG_RD_CPU_VERSION,
    MSG_RD_CPU_VERSION_MINOR, MSG_RD_FPGA_FUNCTION, MSG_RD_FPGA_VERSION, MSG_RD_FPGA_VERSION_MINOR,
};
use autd3_firmware_emulator::CPUEmulator;

use spdlog::prelude::*;

pub use spdlog::Level;

pub struct Debug {
    is_open: bool,
    cpus: Vec<CPUEmulator>,
}

impl Debug {
    pub fn new() -> Self {
        // default_logger.set_level_filter(LevelFilter::MoreSevere(Level::Trace));
        Self {
            is_open: false,
            cpus: vec![],
        }
    }
}  

impl Link for Debug {
    fn open<T: Transducer>(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        let default_logger = spdlog::default_logger();
        default_logger.set_level_filter(LevelFilter::All);
        
        log::debug!("Open Debug link");

        if self.is_open() {
            log::warn!("Debug link is already opened.");
            return Ok(());
        }

        self.cpus = geometry
            .device_map()
            .iter()
            .enumerate()
            .map(|(i, &dev)| {
                let mut cpu = CPUEmulator::new(i, dev);
                cpu.init();
                cpu
            })
            .collect();

        log::trace!("Initialize emulator");

        self.is_open = true;

        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        log::debug!("Close Debug link");

        if !self.is_open() {
            log::warn!("Debug link is already closed.");
            return Ok(());
        }

        self.is_open = false;
        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        for cpu in &mut self.cpus {
            cpu.send(tx);
        }

        match tx.header().msg_id {
            MSG_CLEAR => {
                log::debug!("\tOP: CLEAR");
            }
            MSG_RD_CPU_VERSION => {
                log::debug!("\tOP: RD_CPU_VERSION");
            }
            MSG_RD_CPU_VERSION_MINOR => {
                log::debug!("\tOP: RD_CPU_VERSION_MINOR");
            }
            MSG_RD_FPGA_VERSION => {
                log::debug!("\tOP: RD_FPGA_VERSION");
            }
            MSG_RD_FPGA_VERSION_MINOR => {
                log::debug!("\tOP: RD_FPGA_VERSION_MINOR");
            }
            MSG_RD_FPGA_FUNCTION => {
                log::debug!("\tOP: RD_FPGA_FUNCTION");
            }
            _ => {}
        }

        log::debug!("\tCPU Flag: {}", tx.header().cpu_flag);
        log::debug!("\tFPGA Flag: {}", tx.header().fpga_flag);

        self.cpus.iter().for_each(|cpu| {
            log::debug!("Status: {}", cpu.id());
            let fpga = cpu.fpga();  
            if fpga.is_stm_mode() {
                if fpga.is_stm_gain_mode() {
                    if fpga.is_legacy_mode() {
                        log::debug!("\tGain STM Legacy mode");
                    } else {
                        log::debug!("\tGain STM mode");
                    }
                } else {
                    log::debug!("\tFocus STM mode");
                }
                if tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN) {
                    log::debug!("\t\tSTM BEGIN");
                }
                if tx.header().cpu_flag.contains(CPUControlFlags::STM_END) {
                    log::debug!(
                        "\t\tSTM END (cycle = {}, frequency_division = {})",
                        fpga.stm_cycle(),
                        fpga.stm_frequency_division()
                    );
                    if log::max_level() >= log::Level::Trace {
                        let cycles = fpga.cycles();
                        for j in 0..fpga.stm_cycle() {
                            let (duty, phase) = fpga.drives(j);
                            log::trace!("\tSTM[{}]:", j);
                            log::trace!(
                                "{}",
                                duty.iter()
                                    .zip(phase.iter())
                                    .zip(cycles.iter())
                                    .enumerate()
                                    .map(|(i, ((d, p), c))| {
                                        format!("\n\t\t{:<3}: duty = {:<4}, phase = {:<4}, cycle = {:<4}", i, d, p, c)
                                    })
                                    .collect::<Vec<_>>()
                                    .join("")
                            );
                        }
                    }
                }
            } else if fpga.is_legacy_mode() {
                log::debug!("\tNormal Legacy mode");
            } else {
                log::debug!("\tNormal Advanced mode");
            }
            log::debug!(
                "\tSilencer step = {}, cycle={}",
                fpga.silencer_step(),
                fpga.silencer_cycle()
            );
            let m = fpga.modulation();
            let freq_div_m = fpga.modulation_frequency_division();
            log::debug!(
                "\tModulation size = {}, frequency_division = {}",
                m.len(),
                freq_div_m
            );
            if fpga.is_outputting() {
                log::debug!("\t\t modulation = {:?}", m);
                if !fpga.is_stm_mode() && log::max_level() >= log::Level::Trace {
                    let cycles = fpga.cycles();
                    let (duty, phase) = fpga.drives(0);
                    log::trace!(
                        "{}",
                        duty.iter()
                            .zip(phase.iter())
                            .zip(cycles.iter())
                            .enumerate()
                            .map(|(i, ((d, p), c))| {
                                format!("\n\t\t{:<3}: duty = {:<4}, phase = {:<4}, cycle = {:<4}", i, d, p, c)
                            })
                            .collect::<Vec<_>>()
                            .join("")
                    );
                }
            } else {
                log::info!("\tWithout output");
            }
        });

        Ok(true)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        for cpu in &mut self.cpus {
            rx.messages_mut()[cpu.id()].ack = cpu.ack();
            rx.messages_mut()[cpu.id()].msg_id = cpu.msg_id();
        }

        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn timeout(&self) -> Duration {
        Duration::ZERO
    }
}

impl Default for Debug {
    fn default() -> Self {
        Self::new()
    }
}
