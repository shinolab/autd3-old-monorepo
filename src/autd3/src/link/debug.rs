/*
 * File: debug.rs
 * Project: link
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use autd3_core::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::{get_logger, Link},
    CPUControlFlags, RxDatagram, TxDatagram, FPGA_SUB_CLK_FREQ, FPGA_SUB_CLK_FREQ_DIV, MSG_CLEAR,
    MSG_RD_CPU_VERSION, MSG_RD_CPU_VERSION_MINOR, MSG_RD_FPGA_FUNCTION, MSG_RD_FPGA_VERSION,
    MSG_RD_FPGA_VERSION_MINOR,
};
use autd3_firmware_emulator::CPUEmulator;

use spdlog::prelude::*;

pub struct Debug {
    is_open: bool,
    timeout: Duration,
    logger: Logger,
    cpus: Vec<CPUEmulator>,
}

impl Debug {
    pub fn new() -> Self {
        let logger = get_logger();
        logger.set_level_filter(LevelFilter::MoreSevereEqual(Level::Debug));
        Self {
            is_open: false,
            timeout: Duration::ZERO,
            logger,
            cpus: Vec::new(),
        }
    }

    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }

    pub fn with_log_level(self, level: LevelFilter) -> Self {
        self.logger.set_level_filter(level);
        self
    }

    pub fn with_logger(self, logger: Logger) -> Self {
        Self { logger, ..self }
    }
}

impl Default for Debug {
    fn default() -> Self {
        Self::new()
    }
}

impl Link for Debug {
    fn open<T: Transducer>(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        debug!(logger: self.logger,"Open Debug link");

        if self.is_open {
            warn!(logger: self.logger,"Debug link is already opened.");
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
        trace!(logger: self.logger,"Initialize emulator");

        self.is_open = true;

        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        debug!(logger: self.logger,"Close Debug link");

        if !self.is_open {
            warn!(logger: self.logger,"Debug link is already closed.");
            return Ok(());
        }

        self.is_open = false;
        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        debug!(logger: self.logger, "Send data");

        if !self.is_open {
            warn!(logger: self.logger, "Link is not opened");
            return Ok(false);
        }

        for cpu in &mut self.cpus {
            cpu.send(tx);
        }

        match tx.header().msg_id {
            MSG_CLEAR => {
                debug!(logger: self.logger,"\tOP: CLEAR");
            }
            MSG_RD_CPU_VERSION => {
                debug!(logger: self.logger,"\tOP: RD_CPU_VERSION");
            }
            MSG_RD_CPU_VERSION_MINOR => {
                debug!(logger: self.logger,"\tOP: RD_CPU_VERSION_MINOR");
            }
            MSG_RD_FPGA_VERSION => {
                debug!(logger: self.logger,"\tOP: RD_FPGA_VERSION");
            }
            MSG_RD_FPGA_VERSION_MINOR => {
                debug!(logger: self.logger,"\tOP: RD_FPGA_VERSION_MINOR");
            }
            MSG_RD_FPGA_FUNCTION => {
                debug!(logger: self.logger,"\tOP: RD_FPGA_FUNCTION");
            }
            _ => {}
        }

        debug!(logger: self.logger,"\tCPU Flag: {}", tx.header().cpu_flag);
        debug!(logger: self.logger,"\tFPGA Flag: {}", tx.header().fpga_flag);

        self.cpus.iter().for_each(|cpu| {
            debug!(logger: self.logger,"Status: {}", cpu.id());
            let fpga = cpu.fpga();
            if fpga.is_stm_mode() {
                if fpga.is_stm_gain_mode() {
                    if fpga.is_legacy_mode() {
                        debug!(logger: self.logger,"\tGain STM Legacy mode");
                    } else {
                        debug!(logger: self.logger,"\tGain STM mode");
                    }
                } else {
                    debug!(logger: self.logger,"\tFocus STM mode"); 
                }
                if tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN) {
                    debug!(logger: self.logger,"\t\tSTM BEGIN");
                }
                if tx.header().cpu_flag.contains(CPUControlFlags::STM_END) {
                    let freq_div_stm = fpga.stm_frequency_division() as usize / FPGA_SUB_CLK_FREQ_DIV;
                    debug!(logger: self.logger,
                        "\t\tSTM END: cycle = {}, sampling_frequency = {} ({}/{}))",
                        fpga.stm_cycle(),
                        FPGA_SUB_CLK_FREQ / freq_div_stm,
                        FPGA_SUB_CLK_FREQ,
                        freq_div_stm
                    );
                    if self.logger.should_log(Level::Trace) {
                        let cycles = fpga.cycles();
                        for j in 0..fpga.stm_cycle() {
                            let (duty, phase) = fpga.drives(j);
                            trace!(logger: self.logger,"\tSTM[{}]:", j);
                            trace!(logger: self.logger,
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
                debug!(logger: self.logger,"\tNormal Legacy mode");
            } else {
                debug!(logger: self.logger,"\tNormal Advanced mode");
            }
            debug!(logger: self.logger,
                "\tSilencer step = {}",
                fpga.silencer_step(),
            );
            let m = fpga.modulation();
            let freq_div_m = fpga.modulation_frequency_division() as usize / FPGA_SUB_CLK_FREQ_DIV;
            debug!(logger: self.logger,
                "\tModulation size = {}, sampling_frequency = {} ({}/{})",
                m.len(),
                FPGA_SUB_CLK_FREQ / freq_div_m,
                FPGA_SUB_CLK_FREQ,
                freq_div_m
            );
            if fpga.is_outputting() {
                debug!(logger: self.logger,"\t\t modulation = {:?}", m);
                if !fpga.is_stm_mode() && self.logger.should_log(Level::Trace) {
                    let cycles = fpga.cycles();
                    let (duty, phase) = fpga.drives(0);
                    trace!(logger: self.logger,
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
                info!(logger: self.logger,"\tWithout output");
            }
        });

        Ok(true)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        debug!(logger: self.logger, "Receive data");

        if !self.is_open {
            warn!(logger: self.logger, "Link is not opened");
            return Ok(false);
        }

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
        self.timeout
    }
}
