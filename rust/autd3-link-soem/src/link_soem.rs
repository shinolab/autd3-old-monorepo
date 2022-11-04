/*
 * File: link_soem.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
    usize,
};

use anyhow::Result;
use crossbeam_channel::{bounded, Sender};
use libc::c_void;

use autd3_core::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::Link,
    RxDatagram, TxDatagram, EC_CYCLE_TIME_BASE_NANO_SEC,
};

use crate::{
    ecat_thread::{EcatErrorHandler, EcatThreadHandler, HighPrecisionWaiter, NormalWaiter},
    error::SOEMError,
    iomap::IOMap,
    native_methods::*,
    Config, EthernetAdapters, SyncMode,
};

const SEND_BUF_SIZE: usize = 32;

pub struct SOEM<F: Fn(&str) + Send> {
    ecatth_handle: Option<JoinHandle<()>>,
    error_handle: Option<F>,
    is_open: bool,
    config: Config,
    sender: Option<Sender<TxDatagram>>,
    thread_running: Arc<AtomicBool>,
    ec_sync0_cycle_time_ns: u32,
    ec_send_cycle_time_ns: u32,
}

impl<F: Fn(&str) + Send> SOEM<F> {
    pub fn new(config: Config, error_handle: F) -> Self {
        let ec_send_cycle_time_ns = EC_CYCLE_TIME_BASE_NANO_SEC * config.send_cycle as u32;
        let ec_sync0_cycle_time_ns = EC_CYCLE_TIME_BASE_NANO_SEC * config.sync0_cycle as u32;
        Self {
            ecatth_handle: None,
            error_handle: Some(error_handle),
            is_open: false,
            sender: None,
            thread_running: Arc::new(AtomicBool::new(false)),
            config,
            ec_sync0_cycle_time_ns,
            ec_send_cycle_time_ns,
        }
    }
}

fn lookup_autd() -> anyhow::Result<String> {
    let adapters: EthernetAdapters = Default::default();

    if let Some(adapter) = adapters.into_iter().find(|adapter| unsafe {
        let ifname = std::ffi::CString::new(adapter.name.to_owned()).unwrap();
        if ec_init(ifname.as_ptr()) <= 0 {
            return false;
        }
        let wc = ec_config_init(0);
        if wc <= 0 {
            return false;
        }
        let slave_name = String::from_utf8(
            ec_slave[1]
                .name
                .iter()
                .take_while(|&&c| c != 0)
                .map(|&c| c as u8)
                .collect(),
        )
        .unwrap();
        if slave_name == "AUTD" {
            return true;
        }
        false
    }) {
        Ok(adapter.name.to_owned())
    } else {
        Err(SOEMError::NoDeviceFound.into())
    }
}

unsafe extern "C" fn dc_config(context: *mut ecx_contextt, slave: u16) -> i32 {
    let cyc_time = *((*context).userdata as *mut u32);
    ec_dcsync0(slave, 1, cyc_time, 0);
    0
}

impl<F: 'static + Fn(&str) + Send> Link for SOEM<F> {
    fn open<T: Transducer>(&mut self, geometry: &Geometry<T>) -> anyhow::Result<()> {
        let dev_num = geometry.num_devices() as u16;

        let mut io_map = Box::new(IOMap::new(dev_num as _));

        let (tx_sender, tx_receiver) = bounded(SEND_BUF_SIZE);

        let ifname = if self.config.ifname.is_empty() {
            lookup_autd()?
        } else {
            self.config.ifname.clone()
        };
        let ifname = std::ffi::CString::new(ifname).unwrap();

        unsafe {
            if ec_init(ifname.as_ptr()) <= 0 {
                return Err(
                    SOEMError::NoSocketConnection(ifname.to_str().unwrap().to_string()).into(),
                );
            }

            let wc = ec_config_init(0);
            if wc <= 0 {
                return Err(SOEMError::SlaveNotFound(0, dev_num).into());
            }
            if wc as u16 != dev_num {
                return Err(SOEMError::SlaveNotFound(wc as u16, dev_num).into());
            }

            ecx_context.userdata = &mut self.ec_sync0_cycle_time_ns as *mut _ as *mut c_void;

            if self.config.sync_mode == SyncMode::DC {
                (1..=ec_slavecount as usize).for_each(|i| {
                    ec_slave[i].PO2SOconfigx = Some(dc_config);
                });
            }

            ec_configdc();

            ec_config_map(io_map.data() as *mut c_void);

            ec_statecheck(
                0,
                ec_state_EC_STATE_SAFE_OP as u16,
                EC_TIMEOUTSTATE as i32 * 4,
            );

            ec_readstate();

            ec_slave[0].state = ec_state_EC_STATE_OPERATIONAL as u16;

            ec_send_processdata();
            ec_receive_processdata(EC_TIMEOUTRET as _);

            ec_writestate(0);

            let expected_wkc = (ec_group[0].outputsWKC * 2 + ec_group[0].inputsWKC) as i32;
            let cycletime = self.ec_send_cycle_time_ns as i64;
            let error_handle = self.error_handle.take();
            let thread_running = self.thread_running.clone();
            let is_high_precision = self.config.high_precision_timer;
            self.ecatth_handle = Some(std::thread::spawn(move || {
                let error_handler = EcatErrorHandler { error_handle };
                if is_high_precision {
                    let mut callback = EcatThreadHandler::<HighPrecisionWaiter>::new(
                        io_map,
                        thread_running,
                        tx_receiver,
                        expected_wkc,
                        cycletime,
                    );
                    callback.run();
                } else {
                    let mut callback = EcatThreadHandler::<NormalWaiter>::new(
                        io_map,
                        thread_running,
                        tx_receiver,
                        expected_wkc,
                        cycletime,
                    );
                    callback.run();
                }
            }));

            std::thread::sleep(std::time::Duration::from_millis(100));

            ec_statecheck(
                0,
                ec_state_EC_STATE_OPERATIONAL as u16,
                EC_TIMEOUTSTATE as i32 * 5,
            );

            if ec_slave[0].state != ec_state_EC_STATE_OPERATIONAL as u16 {
                return Err(SOEMError::NotResponding.into());
            }

            if self.config.sync_mode == SyncMode::FreeRun {
                (1..=ec_slavecount as u16).for_each(|i| {
                    dc_config(&mut ecx_context as *mut _, i);
                });
            }
        }

        self.is_open = true;
        self.sender = Some(tx_sender);

        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        if !self.is_open {
            return Ok(());
        }

        while !self.sender.as_ref().unwrap().is_empty() {
            std::thread::sleep(std::time::Duration::from_nanos(
                self.ec_sync0_cycle_time_ns as _,
            ));
        }

        self.is_open = false;

        std::thread::sleep(std::time::Duration::from_millis(200));

        self.thread_running.store(false, Ordering::Release);
        if let Some(timer) = self.ecatth_handle.take() {
            let _ = timer.join();
        }

        if let Some(th) = self.recv_thread.take() {
            let _ = th.join();
        }

        unsafe {
            let cyc_time = *(ecx_context.userdata as *mut u32);
            (1..=ec_slavecount as u16).for_each(|i| {
                ec_dcsync0(i, 0, cyc_time, 0);
            });

            ec_slave[0].state = ec_state_EC_STATE_SAFE_OP as _;
            ec_writestate(0);
            ec_statecheck(0, ec_state_EC_STATE_SAFE_OP as _, EC_TIMEOUTSTATE as _);

            ec_slave[0].state = ec_state_EC_STATE_PRE_OP as _;
            ec_writestate(0);
            ec_statecheck(0, ec_state_EC_STATE_PRE_OP as _, EC_TIMEOUTSTATE as _);

            ec_close();
        }

        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool> {
        let buf = tx.clone();

        self.sender.as_mut().unwrap().send(buf)?;

        Ok(true)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool> {
        if !self.is_open {
            return Err(AUTDInternalError::LinkClosed.into());
        }

        rx.copy_from(&self.rx.lock().unwrap());

        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.is_open
    }
}
