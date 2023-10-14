/*
 * File: group.rs
 * Project: controller
 * Created Date: 05/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{collections::HashMap, hash::Hash, time::Duration};

use autd3_driver::{
    datagram::Datagram,
    error::AUTDInternalError,
    geometry::{Device, Transducer},
    link::Link,
    operation::{Operation, OperationHandler},
};

use super::Controller;

#[allow(clippy::type_complexity)]
pub struct GroupGuard<
    'a,
    K: Hash + Eq + Clone,
    T: Transducer,
    L: Link,
    F: Fn(&Device<T>) -> Option<K>,
> {
    pub(crate) cnt: &'a mut Controller<T, L>,
    pub(crate) f: F,
    pub(crate) timeout: Option<Duration>,
    pub(crate) op: HashMap<K, (Box<dyn Operation<T>>, Box<dyn Operation<T>>)>,
}

impl<'a, K: Hash + Eq + Clone, T: Transducer, L: Link, F: Fn(&Device<T>) -> Option<K>>
    GroupGuard<'a, K, T, L, F>
{
    pub fn set<D: Datagram<T>>(mut self, k: K, d: D) -> Result<Self, AUTDInternalError>
    where
        D::O1: 'static,
        D::O2: 'static,
    {
        self.timeout = match (self.timeout, d.timeout()) {
            (None, None) => None,
            (None, Some(t)) => Some(t),
            (Some(t), None) => Some(t),
            (Some(t1), Some(t2)) => Some(t1.max(t2)),
        };
        let (op1, op2) = d.operation()?;
        self.op.insert(k, (Box::new(op1), Box::new(op2)));
        Ok(self)
    }

    #[doc(hidden)]
    pub fn set_boxed_op(
        mut self,
        k: K,
        op1: Box<dyn autd3_driver::operation::Operation<T>>,
        op2: Box<dyn autd3_driver::operation::Operation<T>>,
        timeout: Option<Duration>,
    ) -> Result<Self, AUTDInternalError> {
        self.timeout = match (self.timeout, timeout) {
            (None, None) => None,
            (None, Some(t)) => Some(t),
            (Some(t), None) => Some(t),
            (Some(t1), Some(t2)) => Some(t1.max(t2)),
        };
        self.op.insert(k, (op1, op2));
        Ok(self)
    }

    pub fn send(mut self) -> Result<bool, AUTDInternalError> {
        let timeout = self.timeout;

        let enable_flags_store = self
            .cnt
            .geometry
            .iter()
            .map(|dev| dev.enable)
            .collect::<Vec<_>>();

        let enable_flags_map: HashMap<K, Vec<bool>> = self
            .op
            .keys()
            .map(|k| {
                (
                    k.clone(),
                    self.cnt
                        .geometry
                        .iter()
                        .map(|dev| {
                            if !dev.enable {
                                return false;
                            }
                            if let Some(kk) = (self.f)(dev) {
                                kk == *k
                            } else {
                                false
                            }
                        })
                        .collect(),
                )
            })
            .collect();

        self.op.iter_mut().try_for_each(|(k, (op1, op2))| {
            self.cnt.geometry_mut().iter_mut().for_each(|dev| {
                dev.enable = enable_flags_map[k][dev.idx()];
            });
            OperationHandler::init(op1, op2, &self.cnt.geometry)
        })?;

        let r = loop {
            let start = std::time::Instant::now();
            self.op.iter_mut().try_for_each(|(k, (op1, op2))| {
                self.cnt.geometry_mut().iter_mut().for_each(|dev| {
                    dev.enable = enable_flags_map[k][dev.idx()];
                });
                OperationHandler::pack(op1, op2, &self.cnt.geometry, &mut self.cnt.tx_buf)
            })?;

            if !self
                .cnt
                .link
                .send_receive(&self.cnt.tx_buf, &mut self.cnt.rx_buf, timeout)?
            {
                break false;
            }
            if self.op.iter_mut().all(|(k, (op1, op2))| {
                self.cnt.geometry_mut().iter_mut().for_each(|dev| {
                    dev.enable = enable_flags_map[k][dev.idx()];
                });
                OperationHandler::is_finished(op1, op2, &self.cnt.geometry)
            }) {
                break true;
            }
            if start.elapsed() < Duration::from_millis(1) {
                std::thread::sleep(Duration::from_millis(1));
            }
        };

        self.cnt
            .geometry
            .iter_mut()
            .zip(enable_flags_store.iter())
            .for_each(|(dev, &enable)| dev.enable = enable);

        Ok(r)
    }
}
