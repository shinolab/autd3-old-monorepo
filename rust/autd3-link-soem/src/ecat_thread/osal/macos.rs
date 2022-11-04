/*
 * File: macos.rs
 * Project: ecat_thread
 * Created Date: 03/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/11/2022
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
use libc::{gettimeofday, timespec, timeval};

use crate::{iomap::IOMap, native_methods::*};

use super::{error_handler::EcatErrorHandler, utils::*};

pub trait Waiter {}
pub struct NormalWaiter {}
pub struct HighPrecisionWaiter {}
impl Waiter for NormalWaiter {}
impl Waiter for HighPrecisionWaiter {}

pub fn add_timespec(ts: &mut timespec, addtime: i64) {
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

pub fn ecat_setup(cycletime_ns: i64) -> timespec {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    gettimeofday(&mut ts as *mut _ as *mut _, std::ptr::null_mut() as *mut _);

    let ht = ((ts.tv_nsec / cycletime_ns) + 1) * cycletime_ns;
    ts.tv_nsec = ht;

    ts
}

fn osal_timed_wait(abs_time: &timespec) {
    let mut tp = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    unsafe {
        gettimeofday(&mut tp as *mut _ as *mut _, std::ptr::null_mut() as *mut _);
    }

    let sleep = (abs_time.tv_sec - tp.tv_sec as i64) * 1000000000
        + (abs_time.tv_nsec - tp.tv_usec as i64 * 1000) as i64;

    if sleep > 0 {
        std::thread::sleep(std::time::Duration::from_nanos(sleep as _));
    }
}

pub struct NormalWaiter {}
pub struct HighPrecisionWaiter {}

impl Waiter for NormalWaiter {
    fn timed_wait(abs_time: &timespec) {
        osal_timed_wait(abs_time)
    }
}

impl Waiter for HighPrecisionWaiter {
    fn timed_wait(abs_time: &timespec) {
        osal_timed_wait(abs_time)
    }
}
