/*
 * File: link_soem.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    ffi::CStr,
    sync::{
        atomic::{AtomicBool, AtomicI32, Ordering},
        Arc, Mutex,
    },
    thread::JoinHandle,
    time::Duration,
    usize,
};

use crossbeam_channel::{bounded, Receiver, Sender};
use libc::{c_void, timeval};

use autd3_core::{
    autd3_device::NUM_TRANS_IN_UNIT,
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::{get_logger, Link},
    osal_timer::{Timer, TimerCallback},
    spdlog::prelude::*,
    timer_strategy::TimerStrategy,
    RxDatagram, TxDatagram, EC_CYCLE_TIME_BASE_NANO_SEC,
};

use crate::local::{
    ecat::{add_timespec, ec_sync, ecat_setup, gettimeofday},
    error::SOEMError,
    error_handler::EcatErrorHandler,
    iomap::IOMap,
    native_methods::*,
    EthernetAdapters, SyncMode,
};

struct SoemCallback {
    wkc: Arc<AtomicI32>,
    receiver: Receiver<TxDatagram>,
    io_map: Arc<Mutex<IOMap>>,
}

impl TimerCallback for SoemCallback {
    fn rt_thread(&mut self) {
        unsafe {
            ec_send_processdata();
            self.wkc.store(
                ec_receive_processdata(EC_TIMEOUTRET as i32),
                Ordering::Release,
            );

            if let Ok(tx) = self.receiver.try_recv() {
                self.io_map.lock().unwrap().copy_from(&tx);
            }

            ec_send_processdata();
        }
    }
}

type OnLostCallBack = Box<dyn Fn(&str) + Send>;

pub struct SOEM {
    ecatth_handle: Option<JoinHandle<()>>,
    ecat_check_th: Option<JoinHandle<()>>,
    timer_handle: Option<Box<Timer<SoemCallback>>>,
    buf_size: usize,
    timer_strategy: TimerStrategy,
    sync_mode: SyncMode,
    ifname: String,
    state_check_interval: std::time::Duration,
    timeout: std::time::Duration,
    sender: Option<Sender<TxDatagram>>,
    is_open: Arc<AtomicBool>,
    ec_sync0_cycle_time_ns: u32,
    ec_send_cycle_time_ns: u32,
    on_lost: Option<OnLostCallBack>,
    io_map: Arc<Mutex<IOMap>>,
    logger: Logger,
}

impl SOEM {
    pub fn new() -> Self {
        let logger = get_logger();
        logger.set_level_filter(LevelFilter::MoreSevereEqual(Level::Info));
        Self {
            buf_size: 32,
            timer_strategy: TimerStrategy::Sleep,
            sync_mode: SyncMode::FreeRun,
            ifname: String::new(),
            state_check_interval: Duration::from_millis(100),
            on_lost: None,
            timeout: Duration::from_millis(20),
            logger,
            ecatth_handle: None,
            ecat_check_th: None,
            timer_handle: None,
            sender: None,
            is_open: Arc::new(AtomicBool::new(false)),
            ec_sync0_cycle_time_ns: EC_CYCLE_TIME_BASE_NANO_SEC * 2,
            ec_send_cycle_time_ns: EC_CYCLE_TIME_BASE_NANO_SEC * 2,
            io_map: Arc::new(Mutex::new(IOMap::new(&[0]))),
        }
    }

    pub fn with_sync0_cycle(self, sync0_cycle: u16) -> Self {
        Self {
            ec_sync0_cycle_time_ns: EC_CYCLE_TIME_BASE_NANO_SEC * sync0_cycle as u32,
            ..self
        }
    }

    pub fn with_send_cycle(self, send_cycle: u16) -> Self {
        Self {
            ec_send_cycle_time_ns: EC_CYCLE_TIME_BASE_NANO_SEC * send_cycle as u32,
            ..self
        }
    }

    pub fn with_buf_size(self, buf_size: usize) -> Self {
        Self { buf_size, ..self }
    }

    pub fn with_timer_strategy(self, timer_strategy: TimerStrategy) -> Self {
        Self {
            timer_strategy,
            ..self
        }
    }

    pub fn with_sync_mode(self, sync_mode: SyncMode) -> Self {
        Self { sync_mode, ..self }
    }

    pub fn with_ifname<S: Into<String>>(self, ifname: S) -> Self {
        Self {
            ifname: ifname.into(),
            ..self
        }
    }

    pub fn with_state_check_interval(self, state_check_interval: Duration) -> Self {
        Self {
            state_check_interval,
            ..self
        }
    }

    pub fn with_on_lost<F: 'static + Fn(&str) + Send>(self, on_lost: F) -> Self {
        Self {
            on_lost: Some(Box::new(on_lost)),
            ..self
        }
    }

    pub fn with_log_level(self, level: LevelFilter) -> Self {
        self.logger.set_level_filter(level);
        self
    }

    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }

    pub fn with_logger(self, logger: Logger) -> Self {
        Self { logger, ..self }
    }
}

impl Default for SOEM {
    fn default() -> Self {
        Self::new()
    }
}

impl SOEM {
    pub fn open_impl(&mut self, device_map: &[usize]) -> Result<i32, AUTDInternalError> {
        let ifname = if self.ifname.is_empty() {
            lookup_autd()?
        } else {
            self.ifname.clone()
        };

        let ifname = std::ffi::CString::new(ifname).unwrap();

        let (tx_sender, tx_receiver) = bounded(self.buf_size);

        unsafe {
            if ec_init(ifname.as_ptr()) <= 0 {
                return Err(
                    SOEMError::NoSocketConnection(ifname.to_str().unwrap().to_string()).into(),
                );
            }

            let wc = ec_config_init(0);

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
            if self.sync_mode == SyncMode::DC {
                (1..=ec_slavecount as usize).for_each(|i| {
                    ec_slave[i].PO2SOconfigx = Some(dc_config);
                });
            }

            ec_configdc();

            let dev_map = if device_map.is_empty() {
                vec![NUM_TRANS_IN_UNIT; wc as _]
            } else {
                device_map.to_vec()
            };
            self.io_map = Arc::new(Mutex::new(IOMap::new(&dev_map)));
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
                        debug!(logger: self.logger,
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
            match self.timer_strategy {
                TimerStrategy::Sleep => {
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
                TimerStrategy::BusyWait => {
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
                TimerStrategy::NativeTimer => {
                    self.timer_handle = Some(Timer::start(
                        SoemCallback {
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
                        debug!(logger: self.logger,
                            "Slave[{}]: {} (State={:#02x} StatusCode={:#04x})",
                            slave, status, ec_slave[slave].state, ec_slave[slave].ALstatuscode
                        );
                    }
                });
                return Err(SOEMError::NotResponding.into());
            }

            if self.sync_mode == SyncMode::FreeRun {
                (1..=ec_slavecount as u16).for_each(|i| {
                    dc_config(&mut ecx_context as *mut _, i);
                });
            }

            let is_open = self.is_open.clone();
            let on_lost = self.on_lost.take();
            let logger = self.logger.clone();
            let state_check_interval = self.state_check_interval;
            self.ecat_check_th = Some(std::thread::spawn(move || {
                let error_handler = EcatErrorHandler { on_lost, logger };
                while is_open.load(Ordering::Acquire) {
                    if wkc.load(Ordering::Acquire) < expected_wkc || ec_group[0].docheckstate != 0 {
                        error_handler.handle();
                    }
                    std::thread::sleep(state_check_interval);
                }
            }));

            self.sender = Some(tx_sender);

            Ok(wc)
        }
    }
}

fn lookup_autd() -> Result<String, SOEMError> {
    let adapters: EthernetAdapters = Default::default();

    if let Some(adapter) = adapters.into_iter().find(|adapter| unsafe {
        let ifname = std::ffi::CString::new(adapter.name().to_owned()).unwrap();
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
        Ok(adapter.name().to_owned())
    } else {
        Err(SOEMError::NoDeviceFound)
    }
}

unsafe extern "C" fn dc_config(context: *mut ecx_contextt, slave: u16) -> i32 {
    let cyc_time = *((*context).userdata as *mut u32);
    ec_dcsync0(slave, 1, cyc_time, 0);
    0
}

impl<T: Transducer> Link<T> for SOEM {
    fn open(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        if self.is_open.load(Ordering::Acquire) {
            return Ok(());
        }

        let found_dev = self.open_impl(geometry.device_map())?;
        if found_dev <= 0 {
            return Err(SOEMError::SlaveNotFound(0, geometry.num_devices() as _).into());
        }
        if found_dev as usize != geometry.num_devices() {
            return Err(
                SOEMError::SlaveNotFound(found_dev as u16, geometry.num_devices() as _).into(),
            );
        }

        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        if !self.is_open.load(Ordering::Acquire) {
            return Ok(());
        }

        while !self.sender.as_ref().unwrap().is_empty() {
            std::thread::sleep(Duration::from_nanos(self.ec_sync0_cycle_time_ns as _));
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

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open.load(Ordering::Acquire) {
            return Err(AUTDInternalError::LinkClosed);
        }

        let buf = tx.clone();
        self.sender.as_mut().unwrap().send(buf).unwrap();

        Ok(true)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open.load(Ordering::Acquire) {
            return Err(AUTDInternalError::LinkClosed);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.io_map.lock().unwrap().input(),
                rx.as_mut_ptr(),
                rx.len(),
            );
        }
        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.is_open.load(Ordering::Acquire)
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}

impl SOEM {
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
                    std::thread::sleep(Duration::from_nanos(sleep as _));
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
                let expired = std::time::Instant::now() + Duration::from_nanos(sleep as _);
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
