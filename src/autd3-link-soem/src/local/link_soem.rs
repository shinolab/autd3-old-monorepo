/*
 * File: link_soem.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    ffi::{c_void, CStr},
    sync::{
        atomic::{AtomicBool, AtomicI32, Ordering},
        Arc, Mutex,
    },
    thread::JoinHandle,
    time::Duration,
    usize,
};

use crossbeam_channel::{bounded, Receiver, Sender};
use time::ext::NumericalDuration;

use autd3_core::{
    autd3_device::AUTD3,
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::{get_logger, Link},
    osal_timer::{Timer, TimerCallback},
    spdlog::prelude::*,
    timer_strategy::TimerStrategy,
    RxDatagram, TxDatagram, EC_CYCLE_TIME_BASE_NANO_SEC,
};

use crate::local::{
    error::SOEMError, error_handler::EcatErrorHandler, iomap::IOMap, soem_bindings::*,
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
        }
    }
}

type OnLostCallBack = Box<dyn Fn(&str) + Send + Sync>;

/// Link using [SOEM](https://github.com/OpenEtherCATsociety/SOEM)
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
    ec_sync0_cycle: std::time::Duration,
    ec_send_cycle: std::time::Duration,
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
            ec_sync0_cycle: std::time::Duration::from_nanos(EC_CYCLE_TIME_BASE_NANO_SEC as u64 * 2),
            ec_send_cycle: std::time::Duration::from_nanos(EC_CYCLE_TIME_BASE_NANO_SEC as u64 * 2),
            io_map: Arc::new(Mutex::new(IOMap::new(&[0]))),
        }
    }

    /// Set sync0 cycle (the unit is 500us)
    pub fn with_sync0_cycle(self, sync0_cycle: u16) -> Self {
        Self {
            ec_sync0_cycle: std::time::Duration::from_nanos(
                EC_CYCLE_TIME_BASE_NANO_SEC as u64 * sync0_cycle as u64,
            ),
            ..self
        }
    }

    /// Set send cycle (the unit is 500us)
    pub fn with_send_cycle(self, send_cycle: u16) -> Self {
        Self {
            ec_send_cycle: std::time::Duration::from_nanos(
                EC_CYCLE_TIME_BASE_NANO_SEC as u64 * send_cycle as u64,
            ),
            ..self
        }
    }

    /// Set send buffer size
    pub fn with_buf_size(self, buf_size: usize) -> Self {
        Self { buf_size, ..self }
    }

    /// Set timer strategy
    pub fn with_timer_strategy(self, timer_strategy: TimerStrategy) -> Self {
        Self {
            timer_strategy,
            ..self
        }
    }

    /// Set sync mode
    ///
    /// See [Beckhoff's site](https://infosys.beckhoff.com/content/1033/ethercatsystem/2469122443.html) for more details.
    pub fn with_sync_mode(self, sync_mode: SyncMode) -> Self {
        Self { sync_mode, ..self }
    }

    /// Set network interface name
    ///
    /// If empty, this link will automatically find the network interface that is connected to AUTD3 devices.
    ///
    pub fn with_ifname<S: Into<String>>(self, ifname: S) -> Self {
        Self {
            ifname: ifname.into(),
            ..self
        }
    }

    /// Set state check interval
    pub fn with_state_check_interval(self, state_check_interval: Duration) -> Self {
        Self {
            state_check_interval,
            ..self
        }
    }

    /// Set callback function when the link is lost
    pub fn with_on_lost<F: 'static + Fn(&str) + Send + Sync>(self, on_lost: F) -> Self {
        Self {
            on_lost: Some(Box::new(on_lost)),
            ..self
        }
    }

    /// Set log level
    pub fn with_log_level(self, level: LevelFilter) -> Self {
        self.logger.set_level_filter(level);
        self
    }

    /// Set timeout
    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }

    /// Set logger for debugging
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

            if let Err(e) = (1..=wc as usize)
                .map(|i| {
                    if let Ok(name) = String::from_utf8(
                        ec_slave[i]
                            .name
                            .iter()
                            .map(|&c| c as u8)
                            .take_while(|&c| c != 0)
                            .collect(),
                    ) {
                        if name.is_empty() {
                            Err(SOEMError::NoDeviceFound)
                        } else if name == "AUTD" {
                            Ok(())
                        } else {
                            Err(SOEMError::NotAUTD3Device(name))
                        }
                    } else {
                        Err(SOEMError::NoDeviceFound)
                    }
                })
                .collect::<Result<Vec<()>, SOEMError>>()
            {
                return Err(e.into());
            }

            ecx_context.userdata = &mut self.ec_sync0_cycle as *mut _ as *mut c_void;
            if self.sync_mode == SyncMode::DC {
                (1..=ec_slavecount as usize).for_each(|i| {
                    ec_slave[i].PO2SOconfigx = Some(dc_config);
                });
            }

            ec_configdc();

            let dev_map = if device_map.is_empty() {
                vec![AUTD3::NUM_TRANS_IN_UNIT; wc as _]
            } else {
                device_map.to_vec()
            };
            self.io_map = Arc::new(Mutex::new(IOMap::new(&dev_map)));
            ec_config_map(self.io_map.lock().unwrap().data() as *mut c_void);

            ec_statecheck(0, ec_state_EC_STATE_SAFE_OP as u16, EC_TIMEOUTSTATE as i32);
            if ec_slave[0].state != ec_state_EC_STATE_SAFE_OP as u16 {
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
            let is_open = self.is_open.clone();
            let cycle = self.ec_send_cycle;
            let wkc = Arc::new(AtomicI32::new(0));
            let wkc_clone = wkc.clone();
            let io_map = self.io_map.clone();
            match self.timer_strategy {
                TimerStrategy::Sleep => {
                    self.ecatth_handle = Some(std::thread::spawn(move || {
                        Self::ecat_run::<StdSleep>(is_open, io_map, wkc_clone, tx_receiver, cycle)
                    }))
                }
                TimerStrategy::BusyWait => {
                    self.ecatth_handle = Some(std::thread::spawn(move || {
                        Self::ecat_run::<BusyWait>(is_open, io_map, wkc_clone, tx_receiver, cycle)
                    }))
                }
                TimerStrategy::NativeTimer => {
                    self.timer_handle = Some(Timer::start(
                        SoemCallback {
                            wkc: wkc_clone,
                            receiver: tx_receiver,
                            io_map,
                        },
                        self.ec_send_cycle,
                    )?)
                }
            }

            ec_statecheck(
                0,
                ec_state_EC_STATE_OPERATIONAL as u16,
                5 * EC_TIMEOUTSTATE as i32,
            );
            if ec_slave[0].state != ec_state_EC_STATE_OPERATIONAL as u16 {
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

    pub fn clear_iomap(&mut self) {
        while !self.sender.as_mut().unwrap().is_empty() {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        self.io_map.lock().unwrap().clear();
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
    let cyc_time = *((*context).userdata as *mut std::time::Duration);
    ec_dcsync0(slave, 1, cyc_time.as_nanos() as _, 0);
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
            std::thread::sleep(self.ec_sync0_cycle);
        }

        self.is_open.store(false, Ordering::Release);
        if let Some(timer) = self.ecatth_handle.take() {
            let _ = timer.join();
        }
        if let Some(timer) = self.timer_handle.take() {
            timer.close()?;
        }
        if let Some(th) = self.ecat_check_th.take() {
            let _ = th.join();
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

trait Sleep {
    fn sleep(duration: time::Duration);
}

struct StdSleep {}

impl Sleep for StdSleep {
    fn sleep(duration: time::Duration) {
        if duration > time::Duration::ZERO {
            std::thread::sleep(std::time::Duration::from_nanos(
                duration.whole_nanoseconds() as _,
            ));
        }
    }
}

struct BusyWait {}

impl Sleep for BusyWait {
    fn sleep(duration: time::Duration) {
        let expired = time::OffsetDateTime::now_utc() + duration;
        while time::OffsetDateTime::now_utc() < expired {
            std::hint::spin_loop();
        }
    }
}

impl SOEM {
    #[allow(clippy::unnecessary_cast)]
    fn ecat_run<S: Sleep>(
        is_open: Arc<AtomicBool>,
        io_map: Arc<Mutex<IOMap>>,
        wkc: Arc<AtomicI32>,
        receiver: Receiver<TxDatagram>,
        cycle: std::time::Duration,
    ) {
        unsafe {
            #[cfg(target_os = "windows")]
            let priority = {
                let priority = windows::Win32::System::Threading::GetPriorityClass(
                    windows::Win32::System::Threading::GetCurrentProcess(),
                );
                windows::Win32::System::Threading::SetPriorityClass(
                    windows::Win32::System::Threading::GetCurrentProcess(),
                    windows::Win32::System::Threading::REALTIME_PRIORITY_CLASS,
                );
                windows::Win32::System::Threading::SetThreadPriority(
                    windows::Win32::System::Threading::GetCurrentThread(),
                    windows::Win32::System::Threading::THREAD_PRIORITY_TIME_CRITICAL,
                );
                windows::Win32::Media::timeBeginPeriod(1);
                priority
            };

            let mut ts = {
                let tp = time::OffsetDateTime::now_utc();
                let tp_unix_ns = tp.unix_timestamp_nanos();

                let cycle_ns = cycle.as_nanos() as i128;
                let ts_unix_ns = (tp_unix_ns / cycle_ns + 1) * cycle_ns;
                time::OffsetDateTime::from_unix_timestamp_nanos(ts_unix_ns).unwrap()
            };

            let mut toff = time::Duration::ZERO;
            let mut integral = 0;
            ec_send_processdata();
            while is_open.load(Ordering::Acquire) {
                ts += cycle;
                ts += toff;

                S::sleep(ts - time::OffsetDateTime::now_utc());

                wkc.store(
                    ec_receive_processdata(EC_TIMEOUTRET as i32),
                    Ordering::Release,
                );

                toff = Self::ec_sync(ec_DCtime, cycle.as_nanos() as _, &mut integral);

                if let Ok(tx) = receiver.try_recv() {
                    io_map.lock().unwrap().copy_from(&tx);
                }
                ec_send_processdata();
            }

            #[cfg(target_os = "windows")]
            {
                windows::Win32::Media::timeEndPeriod(1);
                windows::Win32::System::Threading::SetPriorityClass(
                    windows::Win32::System::Threading::GetCurrentProcess(),
                    windows::Win32::System::Threading::PROCESS_CREATION_FLAGS(priority),
                );
            }
        }
    }

    fn ec_sync(reftime: i64, cycletime: i64, integral: &mut i64) -> time::Duration {
        let mut delta = (reftime - 50000) % cycletime;
        if delta > (cycletime / 2) {
            delta -= cycletime;
        }
        if delta > 0 {
            *integral += 1;
        }
        if delta < 0 {
            *integral -= 1;
        }
        (-(delta / 100) - (*integral / 20)).nanoseconds()
    }
}
