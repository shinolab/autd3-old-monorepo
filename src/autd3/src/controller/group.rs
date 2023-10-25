/*
 * File: group.rs
 * Project: controller
 * Created Date: 05/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{collections::HashMap, hash::Hash, time::Duration};

use autd3_driver::{
    cpu::{RxMessage, TxDatagram},
    datagram::Datagram,
    error::AUTDInternalError,
    geometry::{Device, Geometry, Transducer},
    link::Link,
    operation::{Operation, OperationHandler},
};

use super::Controller;

type OpMap<K, T> = HashMap<K, (Box<dyn Operation<T>>, Box<dyn Operation<T>>)>;

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
    pub(crate) op: OpMap<K, T>,
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

    pub async fn send_impl(
        link: &'a mut L,
        geometry: &'a mut Geometry<T>,
        tx_buf: &'a mut TxDatagram,
        rx_buf: &'a mut [RxMessage],
        f: F,
        timeout: Option<Duration>,
        mut op: OpMap<K, T>,
    ) -> Result<bool, AUTDInternalError> {
        let enable_flags_store = geometry.iter().map(|dev| dev.enable).collect::<Vec<_>>();

        let enable_flags_map: HashMap<K, Vec<bool>> = op
            .keys()
            .map(|k| {
                (
                    k.clone(),
                    geometry
                        .iter()
                        .map(|dev| {
                            if !dev.enable {
                                return false;
                            }
                            if let Some(kk) = f(dev) {
                                kk == *k
                            } else {
                                false
                            }
                        })
                        .collect(),
                )
            })
            .collect();

        op.iter_mut().try_for_each(|(k, (op1, op2))| {
            geometry.iter_mut().for_each(|dev| {
                dev.enable = enable_flags_map[k][dev.idx()];
            });
            OperationHandler::init(op1, op2, geometry)
        })?;

        let r = loop {
            let start = std::time::Instant::now();
            op.iter_mut().try_for_each(|(k, (op1, op2))| {
                geometry.iter_mut().for_each(|dev| {
                    dev.enable = enable_flags_map[k][dev.idx()];
                });
                OperationHandler::pack(op1, op2, geometry, tx_buf)
            })?;

            if !link.send_receive(tx_buf, rx_buf, timeout).await? {
                break false;
            }
            if op.iter_mut().all(|(k, (op1, op2))| {
                geometry.iter_mut().for_each(|dev| {
                    dev.enable = enable_flags_map[k][dev.idx()];
                });
                OperationHandler::is_finished(op1, op2, geometry)
            }) {
                break true;
            }
            if start.elapsed() < Duration::from_millis(1) {
                std::thread::sleep(Duration::from_millis(1));
            }
        };

        geometry
            .iter_mut()
            .zip(enable_flags_store.iter())
            .for_each(|(dev, &enable)| dev.enable = enable);

        Ok(r)
    }

    #[cfg(feature = "async")]
    pub async fn send(self) -> Result<bool, AUTDInternalError> {
        let Self {
            cnt,
            f,
            timeout,
            op,
        } = self;
        Self::send_impl(
            &mut cnt.link,
            &mut cnt.geometry,
            &mut cnt.tx_buf,
            &mut cnt.rx_buf,
            f,
            timeout,
            op,
        )
        .await
    }

    #[cfg(not(feature = "async"))]
    pub fn send(self) -> Result<bool, AUTDInternalError> {
        let Self {
            cnt,
            f,
            timeout,
            op,
        } = self;
        cnt.runtime.block_on(Self::send_impl(
            &mut cnt.link,
            &mut cnt.geometry,
            &mut cnt.tx_buf,
            &mut cnt.rx_buf,
            f,
            timeout,
            op,
        ))
    }
}
