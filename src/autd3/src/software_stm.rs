/*
 * File: software_stm.rs
 * Project: src
 * Created Date: 23/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::sync::{
    atomic::{AtomicBool, AtomicUsize},
    Arc,
};

use atomic::Ordering;
use autd3_core::{
    datagram::Datagram,
    geometry::Transducer,
    link::Link,
    osal_timer::{Timer, TimerCallback},
    timer_strategy::TimerStrategy,
};

use crate::{error::AUTDError, Controller};

/// Software Spatio-Temporal Modulation (STM)
pub struct SoftwareSTM<
    'a,
    T: Transducer,
    L: Link<T>,
    S: Datagram<T>,
    Fs: FnMut(usize, std::time::Duration) -> S + Send + 'static,
    Ff: FnMut(usize, std::time::Duration) -> bool + Send + 'static,
> {
    controller: &'a mut Controller<T, L>,
    callback_loop: Fs,
    callback_finish: Ff,
    timer_strategy: TimerStrategy,
}

impl<
        'a,
        T: Transducer,
        L: Link<T>,
        S: Datagram<T>,
        Fs: FnMut(usize, std::time::Duration) -> S + Send + 'static,
        Ff: FnMut(usize, std::time::Duration) -> bool + Send + 'static,
    > SoftwareSTM<'a, T, L, S, Fs, Ff>
{
    pub(crate) fn new(
        controller: &'a mut Controller<T, L>,
        callback_loop: Fs,
        callback_finish: Ff,
    ) -> Self {
        Self {
            controller,
            callback_loop,
            callback_finish,
            timer_strategy: TimerStrategy::Sleep,
        }
    }

    /// Set timer strategy
    pub fn with_timer_strategy(self, timer_strategy: TimerStrategy) -> Self {
        Self {
            timer_strategy,
            ..self
        }
    }
}

struct SoftwareSTMCallback<
    'a,
    T: Transducer,
    L: Link<T>,
    S: Datagram<T>,
    Fs: FnMut(usize, std::time::Duration) -> S + Send + 'static,
> {
    controller: &'a mut Controller<T, L>,
    callback_loop: Fs,
    i: Arc<AtomicUsize>,
    now: Arc<std::time::Instant>,
}

impl<
        'a,
        T: Transducer,
        L: Link<T>,
        S: Datagram<T>,
        Fs: FnMut(usize, std::time::Duration) -> S + Send + 'static,
    > TimerCallback for SoftwareSTMCallback<'a, T, L, S, Fs>
{
    fn rt_thread(&mut self) {
        let s = (self.callback_loop)(self.i.load(Ordering::Acquire), self.now.elapsed());
        let _ = self.controller.send(s);
        self.i.fetch_add(1, Ordering::Release);
    }
}

impl<
        'a,
        T: Transducer,
        L: Link<T>,
        S: Datagram<T>,
        Fs: FnMut(usize, std::time::Duration) -> S + Send + 'static,
        Ff: FnMut(usize, std::time::Duration) -> bool + Send + 'static,
    > SoftwareSTM<'a, T, L, S, Fs, Ff>
{
    /// Start STM with specified interval
    pub fn start(self, interval: std::time::Duration) -> Result<bool, AUTDError> {
        let Self {
            controller,
            mut callback_loop,
            mut callback_finish,
            timer_strategy,
        } = self;

        let now = Arc::new(std::time::Instant::now());
        let mut next = interval;
        let i = Arc::new(AtomicUsize::new(0));
        let fin = Arc::new(AtomicBool::new(false));
        match timer_strategy {
            TimerStrategy::Sleep => std::thread::scope(|s| {
                s.spawn(|| loop {
                    if callback_finish(i.load(Ordering::Acquire), now.elapsed()) {
                        fin.store(true, Ordering::Release);
                        break;
                    }
                });
                s.spawn(|| -> Result<bool, AUTDError> {
                    while !fin.load(Ordering::Acquire) {
                        let s = callback_loop(i.load(Ordering::Acquire), now.elapsed());
                        controller.send(s)?;
                        i.fetch_add(1, Ordering::Release);
                        let sleep = next.saturating_sub(now.elapsed());
                        if !sleep.is_zero() {
                            std::thread::sleep(sleep);
                        }
                        next += interval;
                    }
                    Ok(true)
                })
                .join()
                .unwrap()
            }),
            TimerStrategy::BusyWait => std::thread::scope(|s| {
                s.spawn(|| loop {
                    if callback_finish(i.load(Ordering::Acquire), now.elapsed()) {
                        fin.store(true, Ordering::Release);
                        break;
                    }
                });
                s.spawn(|| -> Result<bool, AUTDError> {
                    while !fin.load(Ordering::Acquire) {
                        let s = callback_loop(i.load(Ordering::Acquire), now.elapsed());
                        controller.send(s)?;
                        i.fetch_add(1, Ordering::Release);
                        while now.elapsed() < next {
                            std::hint::spin_loop();
                        }
                        next += interval;
                    }
                    Ok(true)
                })
                .join()
                .unwrap()
            }),
            TimerStrategy::NativeTimer => std::thread::scope(|s| -> Result<bool, AUTDError> {
                let handle = match Timer::start(
                    SoftwareSTMCallback {
                        controller,
                        callback_loop,
                        i: i.clone(),
                        now: now.clone(),
                    },
                    interval.as_nanos() as _,
                ) {
                    Ok(handle) => handle,
                    Err(e) => {
                        return Err(AUTDError::Internal(
                            autd3_core::error::AUTDInternalError::TimerError(e),
                        ));
                    }
                };
                s.spawn(|| -> Result<bool, AUTDError> {
                    loop {
                        if callback_finish(i.load(Ordering::Acquire), now.elapsed()) {
                            match handle.close() {
                                Ok(_) => return Ok(true),
                                Err(e) => {
                                    return Err(AUTDError::Internal(
                                        autd3_core::error::AUTDInternalError::TimerError(e),
                                    ))
                                }
                            }
                        }
                    }
                })
                .join()
                .unwrap()
            }),
        }
    }
}
