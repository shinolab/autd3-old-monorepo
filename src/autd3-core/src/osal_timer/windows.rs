/*
 * File: windows.rs
 * Project: src
 * Created Date: 24/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use windows::Win32::{Media::*, System::Threading::*};

use super::error::TimerError;

pub struct NativeTimerWrapper {
    timer_id: u32,
}

impl NativeTimerWrapper {
    pub fn new() -> NativeTimerWrapper {
        NativeTimerWrapper { timer_id: 0 }
    }

    pub fn start<P>(
        &mut self,
        cb: LPTIMECALLBACK,
        period: std::time::Duration,
        lp_param: *mut P,
    ) -> Result<bool, TimerError> {
        unsafe {
            let h_process = GetCurrentProcess();
            SetPriorityClass(h_process, REALTIME_PRIORITY_CLASS);

            let u_resolution = 1;
            timeBeginPeriod(u_resolution);

            let timer_id = timeSetEvent(
                period.as_millis() as _,
                u_resolution,
                cb,
                lp_param as usize,
                TIME_PERIODIC | TIME_CALLBACK_FUNCTION | TIME_KILL_SYNCHRONOUS,
            );

            if timer_id == 0 {
                return Err(TimerError::CreationFailed());
            }

            self.timer_id = timer_id;
            Ok(true)
        }
    }

    pub fn close(&mut self) -> Result<(), TimerError> {
        unsafe {
            if self.timer_id != 0 && timeKillEvent(self.timer_id) != TIMERR_NOERROR {
                return Err(TimerError::DeleteFailed());
            }
            timeEndPeriod(1);
        }
        Ok(())
    }
}

impl Drop for NativeTimerWrapper {
    fn drop(&mut self) {
        let _ = self.close();
    }
}
