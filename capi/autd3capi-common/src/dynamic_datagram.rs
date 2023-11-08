/*
 * File: dynamic_Datagram.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::time::Duration;

use autd3::prelude::*;
use autd3_driver::{datagram::Datagram, error::AUTDInternalError, operation::Operation};

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

unsafe impl Send for DynamicDatagramPack {}
unsafe impl Sync for DynamicDatagramPack {}

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

unsafe impl Send for DynamicDatagramPack2 {}
unsafe impl Sync for DynamicDatagramPack2 {}

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

impl DynamicDatagram for UpdateFlags {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::<crate::driver::operation::UpdateFlagsOp>::default(),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for Synchronize {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::<crate::driver::operation::SyncOp>::default(),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for Stop {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::new(<Self as Datagram>::O1::new(10)),
            Box::<crate::driver::operation::StopOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for Silencer {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::new(<Self as Datagram>::O1::new(self.step())),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for Clear {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::<crate::driver::operation::ClearOp>::default(),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for ConfigureModDelay {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::<crate::driver::operation::ConfigureModDelayOp>::default(),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for ConfigureAmpFilter {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::<crate::driver::operation::ConfigureAmpFilterOp>::default(),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for ConfigurePhaseFilter {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::<crate::driver::operation::ConfigurePhaseFilterOp>::default(),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for FocusSTM {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        let freq_div = self.sampling_frequency_division();
        Ok((
            Box::new(<Self as Datagram>::O1::new(
                self.clear(),
                freq_div,
                self.start_idx(),
                self.finish_idx(),
            )),
            Box::<crate::driver::operation::NullOp>::default(),
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
        let freq_div = self.sampling_frequency_division();
        Ok((
            Box::new(<Self as Datagram>::O1::new(
                self.clear(),
                self.mode(),
                freq_div,
                self.start_idx(),
                self.finish_idx(),
            )),
            Box::<crate::driver::operation::NullOp>::default(),
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
            Box::new(crate::driver::operation::GainOp::new(tmp)),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        None
    }
}

impl DynamicDatagram for Box<M> {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        let freq_div = self.sampling_frequency_division();
        let buf = self.calc()?;
        Ok((
            Box::new(crate::driver::operation::ModulationOp::new(buf, freq_div)),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        None
    }
}
