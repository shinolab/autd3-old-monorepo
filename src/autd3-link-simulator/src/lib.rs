/*
 * File: lib.rs
 * Project: src
 * Created Date: 09/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use tokio::runtime::{Builder, Runtime};

use autd3_protobuf::{simulator_client::SimulatorClient, *};

use std::{
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    time::Duration,
};

use autd3_core::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::Link,
    RxDatagram, TxDatagram,
};

enum Either {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

/// Link for Simulator
pub struct Simulator {
    client: Option<simulator_client::SimulatorClient<tonic::transport::Channel>>,
    addr: Either,
    port: u16,
    timeout: Duration,
    runtime: Runtime,
}

impl Simulator {
    pub fn new(port: u16) -> Self {
        Self {
            client: None,
            addr: Either::V4(Ipv4Addr::LOCALHOST),
            port,
            timeout: Duration::from_millis(200),
            runtime: Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap(),
        }
    }

    /// Set server IP address
    pub fn with_server_ip(self, ipv4: Ipv4Addr) -> Self {
        self.with_server_ipv4(ipv4)
    }

    /// Set server IP address
    pub fn with_server_ipv4(self, ipv4: Ipv4Addr) -> Self {
        Self {
            addr: Either::V4(ipv4),
            ..self
        }
    }

    /// Set server IP address
    pub fn with_server_ipv6(self, ipv6: Ipv6Addr) -> Self {
        Self {
            addr: Either::V6(ipv6),
            ..self
        }
    }

    /// Set timeout
    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }

    fn open_impl<T: Transducer>(
        &mut self,
        geometry: &Geometry<T>,
    ) -> Result<(), AUTDProtoBufError> {
        let mut client = self
            .runtime
            .block_on(simulator_client::SimulatorClient::connect(format!(
                "http://{}",
                match self.addr {
                    Either::V4(ip) => SocketAddr::V4(SocketAddrV4::new(ip, self.port)),
                    Either::V6(ip) => SocketAddr::V6(SocketAddrV6::new(ip, self.port, 0, 0)),
                }
            )))?;

        if self
            .runtime
            .block_on(client.config_geomety(geometry.to_msg()))
            .is_err()
        {
            return Err(AUTDProtoBufError::SendError(
                "Failed to initialize simulator".to_string(),
            ));
        }

        self.client = Some(client);

        Ok(())
    }

    fn close_impl(
        client: &mut SimulatorClient<tonic::transport::Channel>,
        runtime: &Runtime,
    ) -> Result<bool, AUTDProtoBufError> {
        let res = runtime.block_on(client.close(CloseRequest {}))?;
        Ok(res.into_inner().success)
    }

    fn send_impl(
        client: &mut SimulatorClient<tonic::transport::Channel>,
        runtime: &Runtime,
        tx: &TxDatagram,
    ) -> Result<bool, AUTDProtoBufError> {
        let res = runtime.block_on(client.send_data(tx.to_msg()))?;
        Ok(res.into_inner().success)
    }

    fn receive_impl(
        client: &mut SimulatorClient<tonic::transport::Channel>,
        runtime: &Runtime,
    ) -> Result<RxDatagram, AUTDProtoBufError> {
        let res = runtime.block_on(client.read_data(ReadRequest {}))?;
        Ok(autd3_core::RxDatagram::from_msg(&res.into_inner()))
    }
}

impl<T: Transducer> Link<T> for Simulator {
    fn open(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        self.open_impl(geometry)?;
        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        if let Some(client) = &mut self.client {
            Self::close_impl(client, &self.runtime)?;
        }
        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        if let Some(client) = &mut self.client {
            Ok(Self::send_impl(client, &self.runtime, tx)?)
        } else {
            Err(AUTDInternalError::LinkClosed)
        }
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        if let Some(client) = &mut self.client {
            let rx_ = Self::receive_impl(client, &self.runtime)?;
            if rx.len() == rx_.len() {
                rx.copy_from(&rx_);
            }
        } else {
            return Err(AUTDInternalError::LinkClosed);
        }

        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.client.is_some()
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}
