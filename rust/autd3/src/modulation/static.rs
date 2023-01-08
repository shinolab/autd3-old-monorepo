/*
 * File: static.rs
 * Project: modulation
 * Created Date: 30/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;
use autd3_traits::Modulation;

/// Sine wave modulation in ultrasound amplitude
#[derive(Modulation)]
pub struct Static {
    op: autd3_driver::Modulation,
    duty: u8,
}

impl Static {
    /// constructor.
    pub fn new(duty: u8) -> Self {
        Self {
            op: Default::default(),
            duty,
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn calc(&mut self) -> Result<()> {
        self.op.mod_data.resize(2, self.duty);
        Ok(())
    }
}
