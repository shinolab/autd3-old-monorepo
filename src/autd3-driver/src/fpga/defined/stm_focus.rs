/*
 * File: stm_focus.rs
 * Project: defined
 * Created Date: 05/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{defined::float, derive::prelude::AUTDInternalError};

use super::{FOCUS_STM_FIXED_NUM_LOWER, FOCUS_STM_FIXED_NUM_UNIT, FOCUS_STM_FIXED_NUM_UPPER};

#[repr(C)]
pub struct STMFocus {
    pub(crate) buf: [u16; 4],
}

impl STMFocus {
    pub fn set(
        &mut self,
        x: float,
        y: float,
        z: float,
        duty_shift: u8,
    ) -> Result<(), AUTDInternalError> {
        let ix = (x / FOCUS_STM_FIXED_NUM_UNIT).round() as i32;
        let iy = (y / FOCUS_STM_FIXED_NUM_UNIT).round() as i32;
        let iz = (z / FOCUS_STM_FIXED_NUM_UNIT).round() as i32;
        if !(FOCUS_STM_FIXED_NUM_LOWER..=FOCUS_STM_FIXED_NUM_UPPER).contains(&ix)
            || !(FOCUS_STM_FIXED_NUM_LOWER..=FOCUS_STM_FIXED_NUM_UPPER).contains(&iy)
            || !(FOCUS_STM_FIXED_NUM_LOWER..=FOCUS_STM_FIXED_NUM_UPPER).contains(&iz)
        {
            return Err(AUTDInternalError::FocusSTMPointOutOfRange(x, y, z));
        }
        self.buf[0] = (ix & 0xFFFF) as u16;
        self.buf[1] = ((iy << 2) & 0xFFFC) as u16
            | ((ix >> 30) & 0x0002) as u16
            | ((ix >> 16) & 0x0001) as u16;
        self.buf[2] = ((iz << 4) & 0xFFF0) as u16
            | ((iy >> 28) & 0x0008) as u16
            | ((iy >> 14) & 0x0007) as u16;
        self.buf[3] = (((duty_shift as u16) << 6) & 0x3FC0)
            | ((iz >> 26) & 0x0020) as u16
            | ((iz >> 12) & 0x001F) as u16;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::fpga::FOCUS_STM_FIXED_NUM_WIDTH;

    use super::*;

    #[test]
    fn stm_focus() {
        let mut p = STMFocus { buf: [0; 4] };

        let x = FOCUS_STM_FIXED_NUM_UNIT;
        let y = 2. * FOCUS_STM_FIXED_NUM_UNIT;
        let z = 3. * FOCUS_STM_FIXED_NUM_UNIT;
        let duty_shift = 4;

        assert!(p.set(x, y, z, duty_shift).is_ok());

        assert_eq!(
            (p.buf[0] as u32) & ((1 << FOCUS_STM_FIXED_NUM_WIDTH) - 1),
            1
        );
        assert_eq!(
            ((p.buf[1] >> 2) as u32) & ((1 << FOCUS_STM_FIXED_NUM_WIDTH) - 1),
            2
        );
        assert_eq!(
            ((p.buf[2] >> 4) as u32) & ((1 << FOCUS_STM_FIXED_NUM_WIDTH) - 1),
            3
        );
        assert_eq!((p.buf[3] >> 6) & 0xFF, 4);

        let x = -FOCUS_STM_FIXED_NUM_UNIT;
        let y = -2. * FOCUS_STM_FIXED_NUM_UNIT;
        let z = -3. * FOCUS_STM_FIXED_NUM_UNIT;
        let duty_shift = 0xFF;

        assert!(p.set(x, y, z, duty_shift).is_ok());

        assert_eq!(p.buf[0], 0xFFFF);
        assert_eq!(p.buf[1] & 0b01, 0b01);
        assert_eq!(p.buf[1] & 0b10, 0b10);

        assert_eq!(p.buf[1] & 0b1111111111111100, 0b1111111111111000);
        assert_eq!(p.buf[2] & 0b0111, 0b0111);
        assert_eq!(p.buf[2] & 0b1000, 0b1000);

        assert_eq!(p.buf[2] & 0b1111111111110000, 0b1111111111010000);
        assert_eq!(p.buf[3] & 0b011111, 0b011111);
        assert_eq!(p.buf[3] & 0b100000, 0b100000);

        assert_eq!((p.buf[3] >> 6) & 0xFF, 0xFF);

        let x = FOCUS_STM_FIXED_NUM_UNIT * ((1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1)) - 1) as float;
        let y = FOCUS_STM_FIXED_NUM_UNIT * ((1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1)) - 1) as float;
        let z = FOCUS_STM_FIXED_NUM_UNIT * ((1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1)) - 1) as float;
        let duty_shift = 0;

        assert!(p.set(x, y, z, duty_shift).is_ok());

        assert!(p
            .set(x + FOCUS_STM_FIXED_NUM_UNIT, y, z, duty_shift)
            .is_err());
        assert!(p
            .set(x, y + FOCUS_STM_FIXED_NUM_UNIT, z, duty_shift)
            .is_err());
        assert!(p
            .set(x, y, z + FOCUS_STM_FIXED_NUM_UNIT, duty_shift)
            .is_err());

        let x = -FOCUS_STM_FIXED_NUM_UNIT * (1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1)) as float;
        let y = -FOCUS_STM_FIXED_NUM_UNIT * (1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1)) as float;
        let z = -FOCUS_STM_FIXED_NUM_UNIT * (1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1)) as float;
        let duty_shift = 0;

        assert!(p.set(x, y, z, duty_shift).is_ok());

        assert!(p
            .set(x - FOCUS_STM_FIXED_NUM_UNIT, y, z, duty_shift)
            .is_err());
        assert!(p
            .set(x, y - FOCUS_STM_FIXED_NUM_UNIT, z, duty_shift)
            .is_err());
        assert!(p
            .set(x, y, z - FOCUS_STM_FIXED_NUM_UNIT, duty_shift)
            .is_err());
    }
}
