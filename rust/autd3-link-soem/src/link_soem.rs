/*
 * File: link_soem.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    ffi::CStr,
    sync::{
        atomic::{AtomicBool, AtomicI32, Ordering},
        Arc, Mutex,
    },
    thread::JoinHandle,
    usize,
};

use anyhow::Result;
use autd3_timer::{Timer, TimerCallback};
use crossbeam_channel::{bounded, Receiver, Sender};
use libc::{c_void, timeval};

use autd3_core::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::Link,
    RxDatagram, TxDatagram, EC_CYCLE_TIME_BASE_NANO_SEC,
};

use crate::{
    ecat::{add_timespec, ec_sync, ecat_setup, gettimeofday},
    error::SOEMError,
    error_handler::EcatErrorHandler,
    iomap::IOMap,
    native_methods::*,
    Config, EthernetAdapters, SyncMode,
};

struct SoemCallback {
    lock: AtomicBool,
    wkc: Arc<AtomicI32>,
    receiver: Receiver<TxDatagram>,
    io_map: Arc<Mutex<IOMap>>,
}

impl TimerCallback for SoemCallback {
    fn rt_thread(&mut self) {
        unsafe {
            if let Ok(false) =
                self.lock
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            {
                ec_send_processdata();
                self.wkc.store(
                    ec_receive_processdata(EC_TIMEOUTRET as i32),
                    Ordering::Release,
                );

                if let Ok(tx) = self.receiver.try_recv() {
                    self.io_map.lock().unwrap().copy_from(&tx);
                }

                ec_send_processdata();

                self.lock.store(false, Ordering::Release);
            }
        }
    }
}

pub struct SOEM<F: Fn(&str) + Send> {
    ecatth_handle: Option<JoinHandle<()>>,
    ecat_check_th: Option<JoinHandle<()>>,
    timer_handle: Option<Box<Timer<SoemCallback>>>,
    error_handle: Option<F>,
    config: Config,
    sender: Option<Sender<TxDatagram>>,
    is_open: Arc<AtomicBool>,
    ec_sync0_cycle_time_ns: u32,
    ec_send_cycle_time_ns: u32,
    io_map: Arc<Mutex<IOMap>>,
}

impl<F: Fn(&str) + Send> SOEM<F> {
    pub fn new(config: Config, error_handle: F) -> Self {
        let ec_send_cycle_time_ns = EC_CYCLE_TIME_BASE_NANO_SEC * config.send_cycle as u32;
        let ec_sync0_cycle_time_ns = EC_CYCLE_TIME_BASE_NANO_SEC * config.sync0_cycle as u32;
        Self {
            ecatth_handle: None,
            ecat_check_th: None,
            timer_handle: None,
            error_handle: Some(error_handle),
            sender: None,
            is_open: Arc::new(AtomicBool::new(false)),
            config,
            ec_sync0_cycle_time_ns,
            ec_send_cycle_time_ns,
            io_map: Arc::new(Mutex::new(IOMap::new(&[0]))),
        }
    }
}

fn lookup_autd() -> anyhow::Result<String> {
    let adapters: EthernetAdapters = Default::default();

    if let Some(adapter) = adapters.into_iter().find(|adapter| unsafe {
        let ifname = std::ffi::CString::new(adapter.name.to_owned()).unwrap();
        if ec_init(ifname.as_ptr()) <= 0 {
            ec_close();
            return false;
        }
        let wc = ec_config_init(0);
        if wc <= 0 {
            ec_close();
            return false;
        }
        let found = (1..=wc).all(|i| {
            let slave_name = String::from_utf8(
                ec_slave[i as usize]
                    .name
                    .iter()
                    .take_while(|&&c| c != 0)
                    .map(|&c| c as u8)
                    .collect(),
            )
            .unwrap();
            slave_name == "AUTD"
        });
        ec_close();
        found
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
        if self.is_open() {
            return Ok(());
        }

        self.io_map = Arc::new(Mutex::new(IOMap::new(geometry.device_map())));

        let ifname = if self.config.ifname.is_empty() {
            lookup_autd()?
        } else {
            self.config.ifname.clone()
        };

        let ifname = std::ffi::CString::new(ifname).unwrap();

        let dev_num = geometry.num_devices() as u16;

        let (tx_sender, tx_receiver) = bounded(self.config.buf_size);

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

            if (1..=wc as usize).any(|i| {
                if let Ok(name) = String::from_utf8(
                    ec_slave[i]
                        .name
                        .iter()
                        .map(|&c| c as u8)
                        .take_while(|&c| c != 0)
                        .collect(),
                ) {
                    name != "AUTD"
                } else {
                    false
                }
            }) {
                return Err(SOEMError::NotAUTD3Device.into());
            }

            ecx_context.userdata = &mut self.ec_sync0_cycle_time_ns as *mut _ as *mut c_void;
            if self.config.sync_mode == SyncMode::DC {
                (1..=ec_slavecount as usize).for_each(|i| {
                    ec_slave[i].PO2SOconfigx = Some(dc_config);
                });
            }

            ec_configdc();

            ec_config_map(self.io_map.lock().unwrap().data() as *mut c_void);

            ec_statecheck(0, ec_state_EC_STATE_SAFE_OP as u16, EC_TIMEOUTSTATE as i32);
            if ec_slave[0].state != ec_state_EC_STATE_SAFE_OP as _ {
                return Err(SOEMError::NotReachedSafeOp(ec_slave[0].state).into());
            }
            ec_readstate();
            if ec_slave[0].state != ec_state_EC_STATE_SAFE_OP as u16 {
                (1..=wc as usize).for_each(|slave| {
                    if ec_slave[slave].state != ec_state_EC_STATE_SAFE_OP as u16 {
                        let c_status: &CStr =
                            CStr::from_ptr(ec_ALstatuscode2string(ec_slave[slave].ALstatuscode));
                        let status: &str = c_status.to_str().unwrap();
                        eprintln!(
                            "Slave[{}]: {} (State={:#02x} StatusCode={:#04x})",
                            slave, status, ec_slave[slave].state, ec_slave[slave].ALstatuscode
                        );
                    }
                });
                return Err(SOEMError::NotResponding.into());
            }

            ec_slave[0].state = ec_state_EC_STATE_OPERATIONAL as u16;
            ec_writestate(0);

            self.is_open.store(true, Ordering::Release);
            let expected_wkc = (ec_group[0].outputsWKC * 2 + ec_group[0].inputsWKC) as i32;
            let cycletime = self.ec_send_cycle_time_ns;
            let is_open = self.is_open.clone();
            let wkc = Arc::new(AtomicI32::new(0));
            let wkc_clone = wkc.clone();
            let io_map = self.io_map.clone();
            match self.config.timer_strategy {
                crate::TimerStrategy::Sleep => {
                    self.ecatth_handle = Some(std::thread::spawn(move || {
                        Self::ecat_run_with_sleep(
                            is_open,
                            io_map,
                            wkc_clone,
                            tx_receiver,
                            cycletime,
                        )
                    }))
                }
                crate::TimerStrategy::BusyWait => {
                    self.ecatth_handle = Some(std::thread::spawn(move || {
                        Self::ecat_run_with_busywait(
                            is_open,
                            io_map,
                            wkc_clone,
                            tx_receiver,
                            cycletime,
                        )
                    }))
                }
                crate::TimerStrategy::NativeTimer => {
                    self.timer_handle = Some(Timer::start(
                        SoemCallback {
                            lock: AtomicBool::new(false),
                            wkc: wkc_clone,
                            receiver: tx_receiver,
                            io_map,
                        },
                        self.ec_send_cycle_time_ns,
                    )?)
                }
            }

            ec_statecheck(
                0,
                ec_state_EC_STATE_OPERATIONAL as u16,
                5 * EC_TIMEOUTSTATE as i32,
            );
            if ec_slave[0].state != ec_state_EC_STATE_OPERATIONAL as _ {
                self.is_open.store(false, Ordering::Release);
                if let Some(timer) = self.ecatth_handle.take() {
                    let _ = timer.join();
                }
                if let Some(timer) = self.timer_handle.take() {
                    timer.close()?;
                }
                (1..=wc as usize).for_each(|slave| {
                    if ec_slave[slave].state != ec_state_EC_STATE_SAFE_OP as u16 {
                        let c_status: &CStr =
                            CStr::from_ptr(ec_ALstatuscode2string(ec_slave[slave].ALstatuscode));
                        let status: &str = c_status.to_str().unwrap();
                        eprintln!(
                            "Slave[{}]: {} (State={:#02x} StatusCode={:#04x})",
                            slave, status, ec_slave[slave].state, ec_slave[slave].ALstatuscode
                        );
                    }
                });
                return Err(SOEMError::NotResponding.into());
            }

            if self.config.sync_mode == SyncMode::FreeRun {
                (1..=ec_slavecount as u16).for_each(|i| {
                    dc_config(&mut ecx_context as *mut _, i);
                });
            }

            let is_open = self.is_open.clone();
            let error_handle = self.error_handle.take();
            let state_check_interval = self.config.check_interval;
            self.ecat_check_th = Some(std::thread::spawn(move || {
                let error_handler = EcatErrorHandler { error_handle };
                while is_open.load(Ordering::Acquire) {
                    if wkc.load(Ordering::Acquire) < expected_wkc || ec_group[0].docheckstate != 0 {
                        error_handler.handle();
                    }
                    std::thread::sleep(state_check_interval);
                }
            }));
        }

        self.sender = Some(tx_sender);

        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        if !self.is_open() {
            return Ok(());
        }

        while !self.sender.as_ref().unwrap().is_empty() {
            std::thread::sleep(std::time::Duration::from_nanos(
                self.ec_sync0_cycle_time_ns as _,
            ));
        }

        self.is_open.store(false, Ordering::Release);
        if let Some(timer) = self.ecatth_handle.take() {
            let _ = timer.join();
        }
        if let Some(th) = self.ecat_check_th.take() {
            let _ = th.join();
        }
        if let Some(timer) = self.timer_handle.take() {
            timer.close()?;
        }

        unsafe {
            let cyc_time = *(ecx_context.userdata as *mut u32);
            (1..=ec_slavecount as u16).for_each(|i| {
                ec_dcsync0(i, 0, cyc_time, 0);
            });

            ec_slave[0].state = ec_state_EC_STATE_INIT as _;
            ec_writestate(0);

            ec_close();
        }

        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool> {
        if !self.is_open() {
            return Err(AUTDInternalError::LinkClosed.into());
        }

        let buf = tx.clone();
        self.sender.as_mut().unwrap().send(buf)?;

        Ok(true)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool> {
        if !self.is_open() {
            return Err(AUTDInternalError::LinkClosed.into());
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.io_map.lock().unwrap().input(),
                rx.messages_mut().as_mut_ptr(),
                rx.messages().len(),
            );
        }
        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.is_open.load(Ordering::Acquire)
    }
}

impl<F: 'static + Fn(&str) + Send> SOEM<F> {
    #[allow(clippy::unnecessary_cast)]
    fn ecat_run_with_sleep(
        is_open: Arc<AtomicBool>,
        io_map: Arc<Mutex<IOMap>>,
        wkc: Arc<AtomicI32>,
        receiver: Receiver<TxDatagram>,
        cycletime: u32,
    ) {
        unsafe {
            let mut ts = ecat_setup(cycletime as _);

            let mut toff = 0;
            ec_send_processdata();
            while is_open.load(Ordering::Acquire) {
                add_timespec(&mut ts, cycletime as i64 + toff);

                let mut tp = timeval {
                    tv_sec: 0,
                    tv_usec: 0,
                };
                gettimeofday(&mut tp as *mut _ as *mut _, std::ptr::null_mut());
                let sleep = (ts.tv_sec - tp.tv_sec as i64) * 1000000000i64
                    + (ts.tv_nsec as i64 - tp.tv_usec as i64 * 1000i64);
                if sleep > 0 {
                    std::thread::sleep(std::time::Duration::from_nanos(sleep as _));
                }

                wkc.store(
                    ec_receive_processdata(EC_TIMEOUTRET as i32),
                    Ordering::Release,
                );
                ec_sync(ec_DCtime, cycletime as _, &mut toff);

                if let Ok(tx) = receiver.try_recv() {
                    io_map.lock().unwrap().copy_from(&tx);
                }

                ec_send_processdata();
            }
        }
    }

    #[allow(clippy::unnecessary_cast)]
    fn ecat_run_with_busywait(
        is_open: Arc<AtomicBool>,
        io_map: Arc<Mutex<IOMap>>,
        wkc: Arc<AtomicI32>,
        receiver: Receiver<TxDatagram>,
        cycletime: u32,
    ) {
        unsafe {
            let mut ts = ecat_setup(cycletime as _);

            let mut toff = 0;
            ec_send_processdata();
            while is_open.load(Ordering::Acquire) {
                add_timespec(&mut ts, cycletime as i64 + toff);

                let mut tp = timeval {
                    tv_sec: 0,
                    tv_usec: 0,
                };
                gettimeofday(&mut tp as *mut _ as *mut _, std::ptr::null_mut());
                let sleep = (ts.tv_sec - tp.tv_sec as i64) * 1000000000i64
                    + (ts.tv_nsec as i64 - tp.tv_usec as i64 * 1000i64);
                let expired =
                    std::time::Instant::now() + std::time::Duration::from_nanos(sleep as _);
                while std::time::Instant::now() < expired {
                    std::hint::spin_loop();
                }

                wkc.store(
                    ec_receive_processdata(EC_TIMEOUTRET as i32),
                    Ordering::Release,
                );
                ec_sync(ec_DCtime, cycletime as _, &mut toff);

                if let Ok(tx) = receiver.try_recv() {
                    io_map.lock().unwrap().copy_from(&tx);
                }

                ec_send_processdata();
            }
        }
    }
}
