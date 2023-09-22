/*
 * File: software_stm.rs
 * Project: src
 * Created Date: 23/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::sync::{Arc, Condvar, Mutex};

use autd3_driver::{
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
    F: FnMut(&mut Controller<T, L>, usize, std::time::Duration) -> bool + Send + 'static,
> {
    controller: &'a mut Controller<T, L>,
    callback: F,
    timer_strategy: TimerStrategy,
}

impl<
        'a,
        T: Transducer,
        L: Link<T>,
        F: FnMut(&mut Controller<T, L>, usize, std::time::Duration) -> bool + Send + 'static,
    > SoftwareSTM<'a, T, L, F>
{
    pub(crate) fn new(controller: &'a mut Controller<T, L>, callback: F) -> Self {
        Self {
            controller,
            callback,
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
    F: FnMut(&mut Controller<T, L>, usize, std::time::Duration) -> bool + Send + 'static,
> {
    controller: &'a mut Controller<T, L>,
    callback: F,
    i: usize,
    wait: Arc<(Mutex<bool>, Condvar)>,
    now: std::time::Instant,
}

impl<
        'a,
        T: Transducer,
        L: Link<T>,
        F: FnMut(&mut Controller<T, L>, usize, std::time::Duration) -> bool + Send + 'static,
    > TimerCallback for SoftwareSTMCallback<'a, T, L, F>
{
    fn rt_thread(&mut self) {
        if !(self.callback)(self.controller, self.i, self.now.elapsed()) {
            let (lock, cvar) = &*self.wait;
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        }
        self.i += 1;
    }
}

impl<
        'a,
        T: Transducer,
        L: Link<T>,
        F: FnMut(&mut Controller<T, L>, usize, std::time::Duration) -> bool + Send + 'static,
    > SoftwareSTM<'a, T, L, F>
{
    /// Start STM with specified interval
    pub fn start(self, interval: std::time::Duration) -> Result<(), AUTDError> {
        let Self {
            controller,
            mut callback,
            timer_strategy,
        } = self;

        let now = std::time::Instant::now();
        let mut next = interval;
        let mut i = 0;
        match timer_strategy {
            TimerStrategy::Sleep => std::thread::scope(|s| {
                s.spawn(|| -> Result<(), AUTDError> {
                    loop {
                        if !callback(controller, i, now.elapsed()) {
                            break;
                        }
                        i += 1;
                        let sleep = next.saturating_sub(now.elapsed());
                        if !sleep.is_zero() {
                            std::thread::sleep(sleep);
                        }
                        next += interval;
                    }
                    Ok(())
                })
                .join()
                .unwrap()
            }),
            TimerStrategy::BusyWait => std::thread::scope(|s| {
                s.spawn(|| -> Result<(), AUTDError> {
                    loop {
                        if !callback(controller, i, now.elapsed()) {
                            break;
                        }
                        i += 1;
                        while now.elapsed() < next {
                            std::hint::spin_loop();
                        }
                        next += interval;
                    }
                    Ok(())
                })
                .join()
                .unwrap()
            }),
            TimerStrategy::NativeTimer => std::thread::scope(|s| -> Result<(), AUTDError> {
                let wait = Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new()));
                let handle = match Timer::start(
                    SoftwareSTMCallback {
                        controller,
                        callback,
                        i,
                        wait: wait.clone(),
                        now,
                    },
                    interval,
                ) {
                    Ok(handle) => handle,
                    Err(e) => {
                        return Err(AUTDError::Internal(e));
                    }
                };
                s.spawn(move || -> Result<(), AUTDError> {
                    let (lock, cvar) = &*wait;
                    let mut exit = lock.lock().unwrap();
                    while !*exit {
                        exit = cvar.wait(exit).unwrap();
                    }
                    match handle.close() {
                        Ok(_) => Ok(()),
                        Err(e) => Err(AUTDError::Internal(e)),
                    }
                })
                .join()
                .unwrap()
            }),
        }
    }
}
