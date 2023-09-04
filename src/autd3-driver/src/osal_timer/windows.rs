/*
 * File: windows.rs
 * Project: src
 * Created Date: 24/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use windows::Win32::{Foundation::HANDLE, Media::*, System::Threading::*};

use crate::error::AUTDInternalError;

pub struct NativeTimerWrapper {
    timer_id: u32,
    h_process: HANDLE,
    priority: u32,
}

impl NativeTimerWrapper {
    pub fn new() -> NativeTimerWrapper {
        NativeTimerWrapper {
            timer_id: 0,
            h_process: HANDLE::default(),
            priority: 0,
        }
    }

    pub fn start<P>(
        &mut self,
        cb: LPTIMECALLBACK,
        period: std::time::Duration,
        lp_param: *mut P,
    ) -> Result<bool, AUTDInternalError> {
        unsafe {
            self.h_process = GetCurrentProcess();
            self.priority = GetPriorityClass(self.h_process);
            let _ = SetPriorityClass(self.h_process, REALTIME_PRIORITY_CLASS);

            let u_resolution = 1;
            timeBeginPeriod(u_resolution);

            self.timer_id = timeSetEvent(
                period.as_millis() as _,
                u_resolution,
                cb,
                lp_param as usize,
                TIME_PERIODIC | TIME_CALLBACK_FUNCTION | TIME_KILL_SYNCHRONOUS,
            );

            if self.timer_id == 0 {
                timeEndPeriod(1);
                return Err(AUTDInternalError::TimerCreationFailed());
            }

            Ok(true)
        }
    }

    pub fn close(&mut self) -> Result<(), AUTDInternalError> {
        unsafe {
            if self.timer_id != 0 {
                timeEndPeriod(1);
                let _ = SetPriorityClass(
                    self.h_process,
                    windows::Win32::System::Threading::PROCESS_CREATION_FLAGS(self.priority),
                );
                if timeKillEvent(self.timer_id) != TIMERR_NOERROR {
                    return Err(AUTDInternalError::TimerDeleteFailed());
                }
                self.timer_id = 0;
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
