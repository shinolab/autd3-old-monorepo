/*
 * File: link_soem_remote.rs
 * Project: src
 * Created Date: 21/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{net::SocketAddr, time::Duration};

use autd3_derive::LinkSync;
use autd3_driver::{
    cpu::{RxMessage, TxDatagram},
    error::AUTDInternalError,
    geometry::Transducer,
    link::{Link, LinkBuilder},
};

use autd3_protobuf::*;

/// Link to connect to remote SOEMServer
#[derive(LinkSync)]
pub struct RemoteSOEM {
    client: ecat_client::EcatClient<tonic::transport::Channel>,
    timeout: Duration,
    is_open: bool,
}

pub struct RemoteSOEMBuilder {
    addr: SocketAddr,
    timeout: Duration,
}

impl RemoteSOEMBuilder {
    /// Set timeout
    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }
}

#[async_trait::async_trait]
impl<T: Transducer> LinkBuilder<T> for RemoteSOEMBuilder {
    type L = RemoteSOEM;

    async fn open(
        self,
        _: &autd3_driver::geometry::Geometry<T>,
    ) -> Result<Self::L, AUTDInternalError> {
        Ok(Self::L {
            client: ecat_client::EcatClient::connect(format!("http://{}", self.addr))
                .await
                .map_err(|e| AUTDInternalError::from(AUTDProtoBufError::from(e)))?,
            timeout: self.timeout,
            is_open: true,
        })
    }
}

impl RemoteSOEM {
    pub fn builder(addr: SocketAddr) -> RemoteSOEMBuilder {
        RemoteSOEMBuilder {
            addr,
            timeout: Duration::from_millis(200),
        }
    }
}

#[async_trait::async_trait]
impl Link for RemoteSOEM {
    async fn close(&mut self) -> Result<(), AUTDInternalError> {
        self.is_open = false;
        self.client
            .close(CloseRequest {})
            .await
            .map_err(AUTDProtoBufError::from)?;
        Ok(())
    }

    async fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open() {
            return Err(AUTDInternalError::LinkClosed);
        }

        Ok(self
            .client
            .send_data(tx.to_msg())
            .await
            .map_err(AUTDProtoBufError::from)?
            .into_inner()
            .success)
    }

    async fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError> {
        if !self.is_open() {
            return Err(AUTDInternalError::LinkClosed);
        }

        rx.copy_from_slice(&Vec::<RxMessage>::from_msg(
            &self
                .client
                .read_data(ReadRequest {})
                .await
                .map_err(AUTDProtoBufError::from)?
                .into_inner(),
        ));
        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}
