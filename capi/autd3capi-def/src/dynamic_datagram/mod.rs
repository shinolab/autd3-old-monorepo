/*
 * File: mod.rs
 * Project: dynamic_datagram
 * Created Date: 06/12/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

mod debug;
mod force_fan;
mod mod_delay;
mod reads_fpga_info;

pub use debug::*;
pub use force_fan::*;
pub use mod_delay::*;
pub use reads_fpga_info::*;

use std::time::Duration;

use autd3::prelude::*;
use autd3_driver::{
    datagram::Datagram, error::AUTDInternalError, fpga::SILENCER_STEP_DEFAULT, operation::Operation,
};

use crate::{G, M};

pub trait DynamicDatagram {
    #[allow(clippy::type_complexity)]
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError>;

    fn timeout(&self) -> Option<Duration>;
}

pub struct DynamicDatagramPack {
    pub d: Box<Box<dyn DynamicDatagram>>,
    pub timeout: Option<std::time::Duration>,
}

impl Datagram for DynamicDatagramPack {
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        let Self { mut d, .. } = self;
        d.operation()
    }

    fn timeout(&self) -> Option<Duration> {
        if self.timeout.is_some() {
            self.timeout
        } else {
            self.d.timeout()
        }
    }
}

pub struct DynamicDatagramPack2 {
    pub d1: Box<Box<dyn DynamicDatagram>>,
    pub d2: Box<Box<dyn DynamicDatagram>>,
    pub timeout: Option<std::time::Duration>,
}

impl Datagram for DynamicDatagramPack2 {
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        let Self { mut d1, mut d2, .. } = self;
        let (op1, _) = d1.operation()?;
        let (op2, _) = d2.operation()?;
        Ok((op1, op2))
    }

    fn timeout(&self) -> Option<Duration> {
        self.timeout
    }
}

impl DynamicDatagram for Synchronize {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::<autd3_driver::operation::SyncOp>::default(),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for Stop {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::new(<Self as Datagram>::O1::new(
                SILENCER_STEP_DEFAULT,
                SILENCER_STEP_DEFAULT,
            )),
            Box::<autd3_driver::operation::StopOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for Silencer {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::new(<Self as Datagram>::O1::new(
                self.step_intensity(),
                self.step_phase(),
            )),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for Clear {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::<autd3_driver::operation::ClearOp>::default(),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for FocusSTM {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        let freq_div = self.sampling_config().frequency_division();
        Ok((
            Box::new(<Self as Datagram>::O1::new(
                self.clear(),
                freq_div,
                self.start_idx(),
                self.finish_idx(),
            )),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for GainSTM<Box<G>> {
    fn operation(
        &mut self,
    ) -> Result<(Box<dyn Operation>, Box<dyn Operation>), autd3_driver::error::AUTDInternalError>
    {
        let freq_div = self.sampling_config().frequency_division();
        Ok((
            Box::new(<Self as Datagram>::O1::new(
                self.clear(),
                self.mode(),
                freq_div,
                self.start_idx(),
                self.finish_idx(),
            )),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        None
    }
}

impl DynamicDatagram for Box<G> {
    fn operation(
        &mut self,
    ) -> Result<(Box<dyn Operation>, Box<dyn Operation>), autd3_driver::error::AUTDInternalError>
    {
        let mut tmp: Box<G> = Box::<Null>::default();
        std::mem::swap(&mut tmp, self);
        Ok((
            Box::new(autd3_driver::operation::GainOp::new(tmp)),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        None
    }
}

impl DynamicDatagram for Box<M> {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        let freq_div = self.sampling_config().frequency_division();
        let buf = self.calc()?;
        Ok((
            Box::new(autd3_driver::operation::ModulationOp::new(buf, freq_div)),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        None
    }
}
