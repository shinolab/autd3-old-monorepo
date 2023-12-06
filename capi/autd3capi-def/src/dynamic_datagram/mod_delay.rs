/*
 * File: mod_delay.rs
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
    operation::{ConfigureModDelayOp, Operation},
};

type Op = ConfigureModDelayOp<Box<dyn Fn(&Device, &Transducer) -> u16>>;

pub struct DynamicConfigureModDelay {
    map: HashMap<(usize, usize), u16>,
}

impl DynamicConfigureModDelay {
    pub fn new(map: HashMap<(usize, usize), u16>) -> Self {
        Self { map }
    }
}

pub struct DynamicConfigureModDelayOp {
    op: Op,
}

impl Operation for DynamicConfigureModDelayOp {
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

impl DynamicDatagram for DynamicConfigureModDelay {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        let map = self.map.clone();
        Ok((
            Box::new(DynamicConfigureModDelayOp {
                op: ConfigureModDelayOp::new(Box::new(move |device, transducer| {
                    map.get(&(device.idx(), transducer.idx()))
                        .cloned()
                        .unwrap_or(0)
                })),
            }),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        None
    }
}
