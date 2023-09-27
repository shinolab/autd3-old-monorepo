/*
 * File: dynamic_Datagram.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::time::Duration;

use autd3::prelude::*;
use autd3_driver::{datagram::Datagram, error::AUTDInternalError, operation::Operation};

use crate::{
    dynamic_op::{DynamicGainOp, DynamicGainSTMOp},
    dynamic_transducer::{DynamicTransducer, TransMode},
    G, M,
};

pub trait DynamicDatagram {
    #[allow(clippy::type_complexity)]
    fn operation(
        &mut self,
        mode: TransMode,
    ) -> Result<
        (
            Box<dyn Operation<DynamicTransducer>>,
            Box<dyn Operation<DynamicTransducer>>,
        ),
        AUTDInternalError,
    >;

    fn timeout(&self) -> Option<Duration>;
}

impl Datagram<DynamicTransducer>
    for (
        TransMode,
        Box<Box<dyn DynamicDatagram>>,
        Option<std::time::Duration>,
    )
{
    type O1 = Box<dyn Operation<DynamicTransducer>>;
    type O2 = Box<dyn Operation<DynamicTransducer>>;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        let (mode, mut op, _) = self;
        op.operation(mode)
    }

    fn timeout(&self) -> Option<Duration> {
        if self.2.is_some() {
            self.2
        } else {
            self.1.timeout()
        }
    }
}

impl Datagram<DynamicTransducer>
    for (
        TransMode,
        Box<Box<dyn DynamicDatagram>>,
        Box<Box<dyn DynamicDatagram>>,
        Option<std::time::Duration>,
    )
{
    type O1 = Box<dyn Operation<DynamicTransducer>>;
    type O2 = Box<dyn Operation<DynamicTransducer>>;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        let (mode, mut op1, mut op2, _) = self;
        let (op1, _) = op1.operation(mode)?;
        let (op2, _) = op2.operation(mode)?;
        Ok((op1, op2))
    }

    fn timeout(&self) -> Option<Duration> {
        self.3
    }
}

impl DynamicDatagram for UpdateFlags {
    fn operation(
        &mut self,
        _: TransMode,
    ) -> Result<
        (
            Box<dyn Operation<DynamicTransducer>>,
            Box<dyn Operation<DynamicTransducer>>,
        ),
        AUTDInternalError,
    > {
        Ok((
            Box::<crate::driver::operation::UpdateFlagsOp>::default(),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram<DynamicTransducer>>::timeout(self)
    }
}

impl DynamicDatagram for Synchronize {
    fn operation(
        &mut self,
        _: TransMode,
    ) -> Result<
        (
            Box<dyn Operation<DynamicTransducer>>,
            Box<dyn Operation<DynamicTransducer>>,
        ),
        AUTDInternalError,
    > {
        Ok((
            Box::<crate::driver::operation::SyncOp>::default(),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram<LegacyTransducer>>::timeout(self)
    }
}

impl DynamicDatagram for Stop {
    fn operation(
        &mut self,
        _: TransMode,
    ) -> Result<
        (
            Box<dyn Operation<DynamicTransducer>>,
            Box<dyn Operation<DynamicTransducer>>,
        ),
        AUTDInternalError,
    > {
        Ok((
            Box::new(<Self as Datagram<DynamicTransducer>>::O1::new(10)),
            Box::<crate::driver::operation::StopOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram<DynamicTransducer>>::timeout(self)
    }
}

impl DynamicDatagram for Silencer {
    fn operation(
        &mut self,
        _: TransMode,
    ) -> Result<
        (
            Box<dyn Operation<DynamicTransducer>>,
            Box<dyn Operation<DynamicTransducer>>,
        ),
        AUTDInternalError,
    > {
        Ok((
            Box::new(<Self as Datagram<DynamicTransducer>>::O1::new(self.step())),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram<DynamicTransducer>>::timeout(self)
    }
}

impl DynamicDatagram for Clear {
    fn operation(
        &mut self,
        _: TransMode,
    ) -> Result<
        (
            Box<dyn Operation<DynamicTransducer>>,
            Box<dyn Operation<DynamicTransducer>>,
        ),
        AUTDInternalError,
    > {
        Ok((
            Box::<crate::driver::operation::ClearOp>::default(),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram<DynamicTransducer>>::timeout(self)
    }
}

impl DynamicDatagram for ConfigureModDelay {
    fn operation(
        &mut self,
        _: TransMode,
    ) -> Result<
        (
            Box<dyn Operation<DynamicTransducer>>,
            Box<dyn Operation<DynamicTransducer>>,
        ),
        AUTDInternalError,
    > {
        Ok((
            Box::<crate::driver::operation::ConfigureModDelayOp>::default(),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram<DynamicTransducer>>::timeout(self)
    }
}

impl DynamicDatagram for ConfigureAmpFilter {
    fn operation(
        &mut self,
        _: TransMode,
    ) -> Result<
        (
            Box<dyn Operation<DynamicTransducer>>,
            Box<dyn Operation<DynamicTransducer>>,
        ),
        AUTDInternalError,
    > {
        Ok((
            Box::<crate::driver::operation::ConfigureAmpFilterOp>::default(),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram<DynamicTransducer>>::timeout(self)
    }
}

impl DynamicDatagram for ConfigurePhaseFilter {
    fn operation(
        &mut self,
        _: TransMode,
    ) -> Result<
        (
            Box<dyn Operation<DynamicTransducer>>,
            Box<dyn Operation<DynamicTransducer>>,
        ),
        AUTDInternalError,
    > {
        Ok((
            Box::<crate::driver::operation::ConfigurePhaseFilterOp>::default(),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram<DynamicTransducer>>::timeout(self)
    }
}

impl DynamicDatagram for FocusSTM {
    fn operation(
        &mut self,
        _: TransMode,
    ) -> Result<
        (
            Box<dyn Operation<DynamicTransducer>>,
            Box<dyn Operation<DynamicTransducer>>,
        ),
        AUTDInternalError,
    > {
        let freq_div = self.sampling_frequency_division();
        Ok((
            Box::new(<Self as Datagram<DynamicTransducer>>::O1::new(
                self.take_control_points(),
                freq_div,
                self.start_idx(),
                self.finish_idx(),
            )),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram<DynamicTransducer>>::timeout(self)
    }
}

impl DynamicDatagram for GainSTM<DynamicTransducer, Box<G>> {
    fn operation(
        &mut self,
        mode: TransMode,
    ) -> Result<
        (
            Box<dyn Operation<DynamicTransducer>>,
            Box<dyn Operation<DynamicTransducer>>,
        ),
        autd3_driver::error::AUTDInternalError,
    > {
        let freq_div = self.sampling_frequency_division();
        Ok((
            Box::new(DynamicGainSTMOp::new(
                mode,
                self.take_gains(),
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

impl DynamicDatagram for Amplitudes {
    fn operation(
        &mut self,
        mode: TransMode,
    ) -> Result<
        (
            Box<dyn Operation<DynamicTransducer>>,
            Box<dyn Operation<DynamicTransducer>>,
        ),
        autd3_driver::error::AUTDInternalError,
    > {
        match mode {
            TransMode::Legacy | TransMode::Advanced => {
                Err(autd3_driver::error::AUTDInternalError::NotSupported(
                    "Amplitudes can not be used in Legacy or Advanced mode".to_string(),
                ))
            }
            TransMode::AdvancedPhase => Ok((
                Box::new(<Self as Datagram<AdvancedPhaseTransducer>>::O1::new(
                    self.amp(),
                )),
                Box::<crate::driver::operation::NullOp>::default(),
            )),
        }
    }
    fn timeout(&self) -> Option<Duration> {
        None
    }
}

impl DynamicDatagram for Box<G> {
    fn operation(
        &mut self,
        mode: TransMode,
    ) -> Result<
        (
            Box<dyn Operation<DynamicTransducer>>,
            Box<dyn Operation<DynamicTransducer>>,
        ),
        autd3_driver::error::AUTDInternalError,
    > {
        let mut tmp: Box<G> = Box::<Null>::default();
        std::mem::swap(&mut tmp, self);
        Ok((
            Box::new(DynamicGainOp::new(tmp, mode)),
            Box::<crate::driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        None
    }
}

impl DynamicDatagram for Box<M> {
    fn operation(
        &mut self,
        _: TransMode,
    ) -> Result<
        (
            Box<dyn Operation<DynamicTransducer>>,
            Box<dyn Operation<DynamicTransducer>>,
        ),
        AUTDInternalError,
    > {
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
