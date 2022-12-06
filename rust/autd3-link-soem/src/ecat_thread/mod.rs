/*
 * File: mod.rs
 * Project: ecat_thread
 * Created Date: 03/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

mod error_handler;
mod utils;
mod waiter;

#[cfg(windows)]
mod osal {
    mod win32;
    pub use win32::*;
}
#[cfg(target_os = "macos")]
mod osal {
    mod macos;
    pub use macos::*;
}
#[cfg(all(unix, not(target_os = "macos")))]
mod osal {
    mod unix;
    pub use unix::*;
}

pub use error_handler::EcatErrorHandler;

use crossbeam_channel::Receiver;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::{Arc, Mutex};

use autd3_core::TxDatagram;

use crate::iomap::IOMap;

use crate::native_methods::*;
pub use osal::*;
use utils::ec_sync;
use waiter::Waiter;

pub struct EcatThreadHandler<W: Waiter> {
    io_map: Arc<Mutex<IOMap>>,
    is_running: Arc<AtomicBool>,
    wkc: Arc<AtomicI32>,
    receiver: Receiver<TxDatagram>,
    cycletime: i64,
    _phantom_data: PhantomData<W>,
}

impl<W: Waiter> EcatThreadHandler<W> {
    pub fn new(
        io_map: Arc<Mutex<IOMap>>,
        is_running: Arc<AtomicBool>,
        wkc: Arc<AtomicI32>,
        receiver: Receiver<TxDatagram>,
        cycletime: i64,
    ) -> Self {
        Self {
            io_map,
            is_running,
            wkc,
            receiver,
            cycletime,
            _phantom_data: PhantomData,
        }
    }

    pub fn run(&mut self) {
        unsafe {
            let mut ts = ecat_setup(self.cycletime);

            let mut toff = 0;
            ec_send_processdata();
            while self.is_running.load(Ordering::Acquire) {
                add_timespec(&mut ts, self.cycletime + toff);

                W::timed_wait(&ts);

                self.wkc.store(
                    ec_receive_processdata(EC_TIMEOUTRET as i32),
                    Ordering::Release,
                );
                ec_sync(ec_DCtime, self.cycletime, &mut toff);

                if let Ok(tx) = self.receiver.try_recv() {
                    self.io_map.lock().unwrap().copy_from(&tx);
                }

                ec_send_processdata();
            }
        }
    }
}
