/*
 * File: debug.rs
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

use std::{collections::HashMap, time::Duration};

use super::DynamicDatagram;
use autd3::prelude::*;
use autd3_driver::{
    error::AUTDInternalError,
    operation::{Operation, TypeTag},
};

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
