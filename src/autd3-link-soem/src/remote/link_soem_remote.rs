/*
 * File: link_soem_remote.rs
 * Project: src
 * Created Date: 21/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{net::SocketAddr, time::Duration};

use autd3_driver::{
    cpu::{RxMessage, TxDatagram},
    error::AUTDInternalError,
    geometry::{Device, Transducer},
    link::Link,
};
use tokio::runtime::{Builder, Runtime};

use autd3_protobuf::*;
use tonic::Response;

/// Link to connect to remote SOEMServer
pub struct RemoteSOEM {
    client: ecat_client::EcatClient<tonic::transport::Channel>,
    runtime: Runtime,
    timeout: Duration,
    is_open: bool,
}

impl RemoteSOEM {
    /// Constructor
    ///
    /// # Arguments
    ///
    /// * `addr` - IP address and port of SOEMServer
    ///
    pub fn new(addr: SocketAddr) -> Result<Self, AUTDProtoBufError> {
        let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
        Ok(Self {
            client: runtime
                .block_on(ecat_client::EcatClient::connect(format!("http://{}", addr)))?,
            runtime,
            timeout: Duration::from_millis(200),
            is_open: false,
        })
    }

    /// Set timeout
    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }

    fn send_impl(&mut self, tx: &TxDatagram) -> Result<Response<SendResponse>, AUTDProtoBufError> {
        Ok(self.runtime.block_on(self.client.send_data(tx.to_msg()))?)
    }

    fn receive_impl(&mut self) -> Result<Response<autd3_protobuf::RxMessage>, AUTDProtoBufError> {
        Ok(self
            .runtime
            .block_on(self.client.read_data(ReadRequest {}))?)
    }

    fn close_impl(&mut self) -> Result<Response<CloseResponse>, AUTDProtoBufError> {
        Ok(self.runtime.block_on(self.client.close(CloseRequest {}))?)
    }
}

impl<T: Transducer> Link<T> for RemoteSOEM {
    fn open(&mut self, _devices: &[Device<T>]) -> Result<(), AUTDInternalError> {
        self.is_open = true;
        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        self.is_open = false;
        self.close_impl()?;
        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        if <Self as Link<T>>::is_open(self) {
            Ok(self.send_impl(tx)?.into_inner().success)
        } else {
            Err(AUTDInternalError::LinkClosed)
        }
    }

    fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError> {
        if <Self as Link<T>>::is_open(self) {
            rx.copy_from_slice(&Vec::<RxMessage>::from_msg(
                &self.receive_impl()?.into_inner(),
            ));
            Ok(true)
        } else {
            Err(AUTDInternalError::LinkClosed)
        }
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}
