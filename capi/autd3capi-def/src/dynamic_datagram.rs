/*
 * File: dynamic_Datagram.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::{collections::HashMap, time::Duration};

use autd3::prelude::*;
use autd3_driver::{
    datagram::Datagram,
    error::AUTDInternalError,
    fpga::SILENCER_STEP_DEFAULT,
    operation::{Operation, TypeTag},
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

impl DynamicDatagram for UpdateFlags {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::<autd3_driver::operation::UpdateFlagsOp>::default(),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
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

impl DynamicDatagram for ConfigureModDelay {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::<autd3_driver::operation::ConfigureModDelayOp>::default(),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

pub struct DynamicConfigureDebugOutputIdx {
    idx_map: HashMap<usize, u8>,
}

impl DynamicConfigureDebugOutputIdx {
    pub fn new(idx_map: HashMap<usize, u8>) -> Self {
        Self { idx_map }
    }
}

pub struct DynamicDebugOutIdxOp {
    remains: HashMap<usize, usize>,
    idx_map: HashMap<usize, u8>,
}

impl Operation for DynamicDebugOutIdxOp {
    fn pack(&mut self, device: &Device, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        assert_eq!(self.remains[&device.idx()], 1);
        tx[0] = TypeTag::Debug as u8;
        tx[2] = self.idx_map.get(&device.idx()).cloned().unwrap_or(0xFF);
        Ok(4)
    }

    fn required_size(&self, _: &Device) -> usize {
        4
    }

    fn init(&mut self, geometry: &Geometry) -> Result<(), AUTDInternalError> {
        self.remains = geometry.devices().map(|device| (device.idx(), 1)).collect();
        Ok(())
    }

    fn remains(&self, device: &Device) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device) {
        self.remains.insert(device.idx(), 0);
    }
}

impl DynamicDatagram for DynamicConfigureDebugOutputIdx {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::new(DynamicDebugOutIdxOp {
                remains: Default::default(),
                idx_map: self.idx_map.clone(),
            }),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(200))
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
