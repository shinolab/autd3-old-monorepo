/*
 * File: unix.rs
 * Project: ecat_thread
 * Created Date: 03/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    marker::PhantomData,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use autd3_core::{RxDatagram, TxDatagram};
use crossbeam_channel::{Receiver, Sender};
use libc::{clock_gettime, clock_nanosleep, timespec, CLOCK_MONOTONIC, TIMER_ABSTIME};

use crate::{iomap::IOMap, native_methods::*};

use super::{error_handler::EcatErrorHandler, utils::*};

pub trait Waiter {}
pub struct NormalWaiter {}
pub struct HighPrecisionWaiter {}
impl Waiter for NormalWaiter {}
impl Waiter for HighPrecisionWaiter {}

fn add_timespec(ts: &mut timespec, addtime: i64) {
    let nsec = addtime % 1000000000;
    let sec = (addtime - nsec) / 1000000000;
    ts.tv_sec += sec;
    ts.tv_nsec += nsec;
    if ts.tv_nsec >= 1000000000 {
        let nsec = ts.tv_nsec % 1000000000;
        ts.tv_sec += ((ts.tv_nsec - nsec) / 1000000000) as i64;
        ts.tv_nsec = nsec;
    }
}

pub struct EcatThreadHandler<F: Fn(&str), W: Waiter> {
    pub io_map: Box<IOMap>,
    pub is_running: Arc<AtomicBool>,
    pub receiver: Receiver<TxDatagram>,
    pub sender: Sender<RxDatagram>,
    pub expected_wkc: i32,
    pub cycletime: i64,
    pub error_handler: EcatErrorHandler<F>,
    _phantom_data: PhantomData<W>,
}

impl<F: Fn(&str) + Send, W: Waiter> EcatThreadHandler<F, W> {
    pub fn new(
        io_map: Box<IOMap>,
        is_running: Arc<AtomicBool>,
        receiver: Receiver<TxDatagram>,
        sender: Sender<RxDatagram>,
        expected_wkc: i32,
        cycletime: i64,
        error_handler: EcatErrorHandler<F>,
    ) -> Self {
        Self {
            io_map,
            is_running,
            receiver,
            sender,
            expected_wkc,
            cycletime,
            error_handler,
            _phantom_data: PhantomData,
        }
    }

    pub fn run(&mut self) {
        unsafe {
            let mut ts = timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            let mut tleft = timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };

            clock_gettime(CLOCK_MONOTONIC, &mut ts as *mut _);

            let ht = ((ts.tv_nsec / self.cycletime) + 1) * self.cycletime;
            ts.tv_nsec = ht;

            let mut toff = 0;
            while self.is_running.load(Ordering::Acquire) {
                add_timespec(&mut ts, self.cycletime + toff);

                clock_nanosleep(CLOCK_MONOTONIC, TIMER_ABSTIME, &ts, &mut tleft as *mut _);

                if ec_slave[0].state == ec_state_EC_STATE_SAFE_OP as _ {
                    ec_slave[0].state = ec_state_EC_STATE_OPERATIONAL as _;
                    ec_writestate(0);
                }

                if let Ok(tx) = self.receiver.try_recv() {
                    self.io_map.copy_from(tx);
                }

                ec_send_processdata();
                if ec_receive_processdata(EC_TIMEOUTRET as i32) != self.expected_wkc
                    && !self.error_handler.handle()
                {
                    return;
                }

                self.sender.send(self.io_map.input()).unwrap();

                ec_sync(ec_DCtime, self.cycletime, &mut toff);
            }
        }
    }
}
