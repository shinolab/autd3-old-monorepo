/*
 * File: debug.rs
 * Project: link
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use autd3_driver::{
    cpu::{RxDatagram, TxDatagram},
    error::AUTDInternalError,
    fpga::{FPGA_SUB_CLK_FREQ, FPGA_SUB_CLK_FREQ_DIV},
    geometry::{Device, Transducer},
    link::Link,
    logger::get_logger,
    operation::{FirmwareInfoType, TypeTag},
};
use autd3_firmware_emulator::CPUEmulator;

use spdlog::prelude::*;

/// Link to debug
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

    /// set timeout
    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }

    /// set log level
    pub fn with_log_level(self, level: LevelFilter) -> Self {
        self.logger.set_level_filter(level);
        self
    }

    /// set logger
    ///
    /// By default, the logger will display log messages on the console.
    pub fn with_logger(self, logger: Logger) -> Self {
        Self { logger, ..self }
    }

    pub fn emulators(&self) -> &[CPUEmulator] {
        &self.cpus
    }
}

impl Default for Debug {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Transducer> Link<T> for Debug {
    fn open(&mut self, devices: &[Device<T>]) -> Result<(), AUTDInternalError> {
        debug!(logger: self.logger,"Open Debug link");

        if self.is_open {
            warn!(logger: self.logger,"Debug link is already opened.");
            return Ok(());
        }

        self.cpus = devices
            .iter()
            .enumerate()
            .map(|(i, dev)| {
                let mut cpu = CPUEmulator::new(i, dev.num_transducers());
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

        self.cpus.iter_mut().for_each(|cpu| {
            cpu.send(tx); 
        });

        tx.headers()
            .zip(tx.payloads())
            .enumerate()
            .for_each(|(i, (h, b))| {
                debug!(logger: self.logger,"\tDevice[{i}]:");
                let fpga_flag = h.fpga_flag;
                debug!(logger: self.logger,"\t\tFlag: {fpga_flag}");
                let print = |slot, tag| match tag {
                    TypeTag::NONE => {
                        debug!(logger: self.logger,"\t\tSlot {slot} Op: None");
                    }
                    TypeTag::Clear => {
                        debug!(logger: self.logger,"\t\tSlot {slot} Op: Clear");
                    }
                    TypeTag::Sync => {
                        debug!(logger: self.logger,"\t\tSlot {slot} Op: Sync");
                    }
                    TypeTag::FirmwareInfo => {
                        let info_type = FirmwareInfoType::from(b[1]);
                        debug!(logger: self.logger,"\t\tSlot {slot} Op: FirmwareInfo ({info_type:?})");
                    }
                    TypeTag::Modulation => {
                        debug!(logger: self.logger,"\t\tSlot {slot} Op: Modulation");
                    }
                    TypeTag::ModDelay => {
                        debug!(logger: self.logger,"\t\tSlot {slot} Op: ModulationDelay");
                    } 
                    TypeTag::Silencer => {
                        debug!(logger: self.logger,"\t\tSlot {slot} Op: Silencer");
                    }
                    TypeTag::Gain => {
                        debug!(logger: self.logger,"\t\tSlot {slot} Op: Gain");
                    }
                    TypeTag::FocusSTM => {
                        debug!(logger: self.logger,"\t\tSlot {slot} Op: FocusSTM");
                    }
                    TypeTag::GainSTM => {
                        debug!(logger: self.logger,"\t\tSlot {slot} Op: GainSTM");
                    }
                    TypeTag::Filter => {
                        debug!(logger: self.logger,"\t\tSlot {slot} Op: Filter");
                    }
                };
                print(1, TypeTag::from(b[0]));
                if h.slot_2_offset != 0 {
                    print(2, TypeTag::from(b[h.slot_2_offset as usize]));
                }
            });

        self.cpus.iter().for_each(|cpu| {
            debug!(logger: self.logger,"Status: {}", cpu.idx());
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
                let freq_div_stm = fpga.stm_frequency_division() as usize / FPGA_SUB_CLK_FREQ_DIV;
                    debug!(logger: self.logger,
                        "\t\tSTM: cycle = {}, sampling_frequency = {} ({}/{})",
                        fpga.stm_cycle(),
                        FPGA_SUB_CLK_FREQ / freq_div_stm,
                        FPGA_SUB_CLK_FREQ,
                        freq_div_stm
                    );
                    if self.logger.should_log(Level::Trace) {
                        let cycles = fpga.cycles();
                        ( 0..fpga.stm_cycle()).for_each(|j| {
                            trace!(logger: self.logger,"\tSTM[{}]:", j);
                            trace!(logger: self.logger,
                                "{}",
                                fpga.duties_and_phases(j).iter()
                                    .zip(cycles.iter())
                                    .enumerate()
                                    .map(|(i, (d, c))| {
                                        format!("\n\t\t{:<3}: duty = {:<4}, phase = {:<4}, cycle = {:<4}", i, d.0, d.1, c)
                                    })
                                    .collect::<Vec<_>>()
                                    .join("")
                            );
                        });
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
                    trace!(logger: self.logger,
                        "{}",
                        fpga.duties_and_phases(0).iter()
                            .zip(fpga.cycles().iter())
                            .enumerate()
                            .map(|(i, (d, c))| {
                                format!("\n\t\t{:<3}: duty = {:<4}, phase = {:<4}, cycle = {:<4}", i, d.0, d.1, c)
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

        self.cpus.iter_mut().for_each(|cpu| {
            rx[cpu.idx()].data = cpu.ack();
            rx[cpu.idx()].ack = cpu.msg_id();
        });

        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }

    fn send_receive(
        &mut self,
        tx: &TxDatagram,
        rx: &mut RxDatagram,
        timeout: Duration,
    ) -> Result<bool, AUTDInternalError> {
        debug!(logger: self.logger, "Send receive data with timeout ({timeout:?})");
        if !<Self as Link<T>>::send(self, tx)? {
            return Ok(false);
        }
        if timeout.is_zero() {
            return <Self as Link<T>>::receive(self, rx);
        }
        <Self as Link<T>>::wait_msg_processed(self, tx, rx, timeout)
    }

    fn wait_msg_processed(
        &mut self,
        tx: &TxDatagram,
        rx: &mut RxDatagram,
        timeout: Duration,
    ) -> Result<bool, AUTDInternalError> {
        let start = std::time::Instant::now();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1));
            if !<Self as Link<T>>::receive(self, rx)? {
                continue;
            }
            if tx
                .headers()
                .zip(rx.iter())
                .all(|(h, r)| h.msg_id == r.ack)
            {
                return Ok(true);
            }
            if start.elapsed() > timeout {
                return Ok(false);
            }
        }
    }
}
