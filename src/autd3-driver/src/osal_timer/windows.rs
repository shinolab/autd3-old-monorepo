/*
 * File: windows.rs
 * Project: src
 * Created Date: 24/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use windows::Win32::{Foundation::*, Media::*, System::Threading::*};

use crate::error::AUTDInternalError;

pub struct NativeTimerWrapper {
    h_queue: HANDLE,
    h_timer: HANDLE,
    h_process: HANDLE,
    priority: u32,
}

impl NativeTimerWrapper {
    pub fn new() -> NativeTimerWrapper {
        NativeTimerWrapper {
            h_queue: HANDLE::default(),
            h_timer: HANDLE::default(),
            h_process: HANDLE::default(),
            priority: 0,
        }
    }

    pub fn start<P>(
        &mut self,
        cb: WAITORTIMERCALLBACK,
        period: std::time::Duration,
        lp_param: *mut P,
    ) -> Result<bool, AUTDInternalError> {
        unsafe {
            self.h_process = GetCurrentProcess();
            self.priority = GetPriorityClass(self.h_process);
            let _ = SetPriorityClass(self.h_process, REALTIME_PRIORITY_CLASS);

            let u_resolution = 1;
            timeBeginPeriod(u_resolution);

            let interval = (period.as_nanos() / 1000 / 1000) as u32;

            self.h_queue = CreateTimerQueue()?;
            CreateTimerQueueTimer(
                &mut self.h_timer as *mut _,
                self.h_queue,
                cb,
                Some(lp_param as *const _),
                0,
                interval.max(1),
                WORKER_THREAD_FLAGS(0),
            )?;

            Ok(true)
        }
    }

    pub fn close(&mut self) -> Result<(), AUTDInternalError> {
        unsafe {
            if !self.h_timer.is_invalid() {
                DeleteTimerQueueTimer(self.h_queue, self.h_timer, None)?;
                DeleteTimerQueue(self.h_queue)?;

                timeEndPeriod(1);
                let _ = SetPriorityClass(
                    self.h_process,
                    windows::Win32::System::Threading::PROCESS_CREATION_FLAGS(self.priority),
                );

                self.h_queue = HANDLE::default();
                self.h_timer = HANDLE::default();
            }
        }
        Ok(())
    }
}

impl Drop for NativeTimerWrapper {
    fn drop(&mut self) {
        let _ = self.close();
    }
}
