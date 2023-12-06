/*
 * File: reads_fpga_info.rs
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
    operation::{ConfigureReadsFPGAInfoOp, Operation},
};

type Op = ConfigureReadsFPGAInfoOp<Box<dyn Fn(&Device) -> bool>>;

pub struct DynamicConfigureReadsFPGAInfo {
    map: HashMap<usize, bool>,
}

impl DynamicConfigureReadsFPGAInfo {
    pub fn new(map: HashMap<usize, bool>) -> Self {
        Self { map }
    }
}

pub struct DynamicConfigureReadsFPGAInfoOp {
    op: Op,
}

impl Operation for DynamicConfigureReadsFPGAInfoOp {
    fn pack(&mut self, device: &Device, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        self.op.pack(device, tx)
    }

    fn required_size(&self, device: &Device) -> usize {
        self.op.required_size(device)
    }

    fn init(&mut self, geometry: &Geometry) -> Result<(), AUTDInternalError> {
        self.op.init(geometry)
    }

    fn remains(&self, device: &Device) -> usize {
        self.op.remains(device)
    }

    fn commit(&mut self, device: &Device) {
        self.op.commit(device)
    }
}

impl DynamicDatagram for DynamicConfigureReadsFPGAInfo {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        let map = self.map.clone();
        Ok((
            Box::new(DynamicConfigureReadsFPGAInfoOp {
                op: ConfigureReadsFPGAInfoOp::new(Box::new(move |dev: &Device| map[&dev.idx()])),
            }),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(200))
    }
}
