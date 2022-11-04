/*
 * File: ecat_thread.rs
 * Project: ecat_thread
 * Created Date: 04/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crossbeam_channel::{Receiver, Sender};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;

use autd3_core::{RxDatagram, TxDatagram};

use crate::iomap::IOMap;

use super::osal::*;
use super::utils::ec_sync;
use super::waiter::Waiter;
use crate::native_methods::*;

pub struct EcatThreadHandler<W: Waiter> {
    io_map: Box<IOMap>,
    is_running: Arc<AtomicBool>,
    wkc: Arc<AtomicI32>,
    receiver: Receiver<TxDatagram>,
    sender: Sender<RxDatagram>,
    expected_wkc: i32,
    cycletime: i64,
    _phantom_data: PhantomData<W>,
}

impl<W: Waiter> EcatThreadHandler<W> {
    pub fn new(
        io_map: Box<IOMap>,
        is_running: Arc<AtomicBool>,
        wkc: Arc<AtomicI32>,
        receiver: Receiver<TxDatagram>,
        sender: Sender<RxDatagram>,
        expected_wkc: i32,
        cycletime: i64,
    ) -> Self {
        Self {
            io_map,
            is_running,
            wkc,
            receiver,
            sender,
            expected_wkc,
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
                    self.io_map.copy_from(tx);
                }

                ec_send_processdata();
            }
        }
    }
}
