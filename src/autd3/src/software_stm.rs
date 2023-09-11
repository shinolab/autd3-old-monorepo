/*
 * File: software_stm.rs
 * Project: src
 * Created Date: 23/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Condvar, Mutex,
};

use autd3_driver::{
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
    Fs: FnMut(usize, std::time::Duration) -> Option<S> + Send + 'static,
    Ff: FnMut(usize, std::time::Duration) -> bool + Send + 'static,
    Fe: FnMut(AUTDError) -> bool + Send + 'static,
> {
    controller: &'a mut Controller<T, L>,
    callback_loop: Fs,
    finish_signal: Ff,
    callback_err: Fe,
    timer_strategy: TimerStrategy,
}

impl<
        'a,
        T: Transducer,
        L: Link<T>,
        S: Datagram<T>,
        Fs: FnMut(usize, std::time::Duration) -> Option<S> + Send + 'static,
        Ff: FnMut(usize, std::time::Duration) -> bool + Send + 'static,
        Fe: FnMut(AUTDError) -> bool + Send + 'static,
    > SoftwareSTM<'a, T, L, S, Fs, Ff, Fe>
{
    pub(crate) fn new(
        controller: &'a mut Controller<T, L>,
        callback_loop: Fs,
        finish_signal: Ff,
        callback_err: Fe,
    ) -> Self {
        Self {
            controller,
            callback_loop,
            finish_signal,
            callback_err,
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
    Fs: FnMut(usize, std::time::Duration) -> Option<S> + Send + 'static,
    Fe: FnMut(AUTDError) -> bool + Send + 'static,
> {
    controller: &'a mut Controller<T, L>,
    callback_loop: Fs,
    callback_err: Fe,
    i: Arc<AtomicUsize>,
    wait: Arc<(Mutex<bool>, Condvar)>,
    now: Arc<std::time::Instant>,
}

impl<
        'a,
        T: Transducer,
        L: Link<T>,
        S: Datagram<T>,
        Fs: FnMut(usize, std::time::Duration) -> Option<S> + Send + 'static,
        Fe: FnMut(AUTDError) -> bool + Send + 'static,
    > TimerCallback for SoftwareSTMCallback<'a, T, L, S, Fs, Fe>
{
    fn rt_thread(&mut self) {
        match (self.callback_loop)(self.i.load(Ordering::Acquire), self.now.elapsed()) {
            Some(s) => match self.controller.send(s) {
                Ok(_) => {}
                Err(e) => {
                    if (self.callback_err)(e) {
                        let (lock, cvar) = &*self.wait;
                        *lock.lock().unwrap() = true;
                        cvar.notify_one();
                    }
                }
            },
            None => {}
        }
        self.i.fetch_add(1, Ordering::Release);
    }
}

impl<
        'a,
        T: Transducer,
        L: Link<T>,
        S: Datagram<T>,
        Fs: FnMut(usize, std::time::Duration) -> Option<S> + Send + 'static,
        Ff: FnMut(usize, std::time::Duration) -> bool + Send + 'static,
        Fe: FnMut(AUTDError) -> bool + Send + 'static,
    > SoftwareSTM<'a, T, L, S, Fs, Ff, Fe>
{
    /// Start STM with specified interval
    pub fn start(self, interval: std::time::Duration) -> Result<bool, AUTDError> {
        let Self {
            controller,
            mut callback_loop,
            mut finish_signal,
            mut callback_err,
            timer_strategy,
        } = self;

        let now = Arc::new(std::time::Instant::now());
        let mut next = interval;
        let i = Arc::new(AtomicUsize::new(0));
        let fin = Arc::new(AtomicBool::new(false));
        match timer_strategy {
            TimerStrategy::Sleep => std::thread::scope(|s| {
                s.spawn(|| loop {
                    if finish_signal(i.load(Ordering::Acquire), now.elapsed()) {
                        fin.store(true, Ordering::Release);
                        break;
                    }
                });
                s.spawn(|| -> Result<bool, AUTDError> {
                    Ok(loop {
                        if fin.load(Ordering::Acquire) {
                            break true;
                        }
                        match callback_loop(i.load(Ordering::Acquire), now.elapsed()) {
                            Some(s) => match controller.send(s) {
                                Ok(_) => {}
                                Err(e) => {
                                    if callback_err(e) {
                                        break false;
                                    }
                                }
                            },
                            None => {}
                        }
                        i.fetch_add(1, Ordering::Release);
                        let sleep = next.saturating_sub(now.elapsed());
                        if !sleep.is_zero() {
                            std::thread::sleep(sleep);
                        }
                        next += interval;
                    })
                })
                .join()
                .unwrap()
            }),
            TimerStrategy::BusyWait => std::thread::scope(|s| {
                s.spawn(|| loop {
                    if finish_signal(i.load(Ordering::Acquire), now.elapsed()) {
                        fin.store(true, Ordering::Release);
                        break;
                    }
                });
                s.spawn(|| -> Result<bool, AUTDError> {
                    Ok(loop {
                        if fin.load(Ordering::Acquire) {
                            break true;
                        }

                        match callback_loop(i.load(Ordering::Acquire), now.elapsed()) {
                            Some(s) => match controller.send(s) {
                                Ok(_) => {}
                                Err(e) => {
                                    if callback_err(e) {
                                        break false;
                                    }
                                }
                            },
                            None => {}
                        }
                        i.fetch_add(1, Ordering::Release);
                        while now.elapsed() < next {
                            std::hint::spin_loop();
                        }
                        next += interval;
                    })
                })
                .join()
                .unwrap()
            }),
            TimerStrategy::NativeTimer => std::thread::scope(|s| -> Result<bool, AUTDError> {
                let wait = Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new()));
                let handle = match Timer::start(
                    SoftwareSTMCallback {
                        controller,
                        callback_loop,
                        callback_err,
                        i: i.clone(),
                        wait: wait.clone(),
                        now: now.clone(),
                    },
                    interval,
                ) {
                    Ok(handle) => handle,
                    Err(e) => {
                        return Err(AUTDError::Internal(e));
                    }
                };
                let wait2 = wait.clone();
                let fin2 = fin.clone();
                s.spawn(move || {
                    let (lock, cvar) = &*wait2;
                    loop {
                        if finish_signal(i.load(Ordering::Acquire), now.elapsed()) {
                            fin2.store(true, Ordering::Relaxed);
                            *lock.lock().unwrap() = true;
                            cvar.notify_one();
                            break;
                        }
                    }
                });
                s.spawn(move || -> Result<bool, AUTDError> {
                    let (lock, cvar) = &*wait;
                    let mut exit = lock.lock().unwrap();
                    while !*exit {
                        exit = cvar.wait(exit).unwrap();
                    }
                    match handle.close() {
                        Ok(_) => Ok(fin.load(Ordering::Relaxed)),
                        Err(e) => {
                            return Err(AUTDError::Internal(e));
                        }
                    }
                })
                .join()
                .unwrap()
            }),
        }
    }
}
