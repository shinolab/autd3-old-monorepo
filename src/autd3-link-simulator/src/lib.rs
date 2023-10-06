/*
 * File: lib.rs
 * Project: src
 * Created Date: 09/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
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

use autd3_driver::{
    cpu::{RxMessage, TxDatagram},
    error::AUTDInternalError,
    geometry::Transducer,
    link::{Link, LinkBuilder},
};

enum Either {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

/// Link for Simulator
pub struct Simulator {
    client: simulator_client::SimulatorClient<tonic::transport::Channel>,
    runtime: Runtime,
    timeout: Duration,
    is_open: bool,
}

pub struct SimulatorBuilder {
    addr: Either,
    port: u16,
    timeout: Duration,
}

impl<T: Transducer> LinkBuilder<T> for SimulatorBuilder {
    type L = Simulator;

    fn open(
        self,
        geometry: &autd3_driver::geometry::Geometry<T>,
    ) -> Result<Self::L, AUTDInternalError> {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let mut client = runtime
            .block_on(simulator_client::SimulatorClient::connect(format!(
                "http://{}",
                match self.addr {
                    Either::V4(ip) => SocketAddr::V4(SocketAddrV4::new(ip, self.port)),
                    Either::V6(ip) => SocketAddr::V6(SocketAddrV6::new(ip, self.port, 0, 0)),
                }
            )))
            .map_err(|e| AUTDInternalError::from(AUTDProtoBufError::from(e)))?;

        if runtime
            .block_on(client.config_geomety(geometry.to_msg()))
            .is_err()
        {
            return Err(
                AUTDProtoBufError::SendError("Failed to initialize simulator".to_string()).into(),
            );
        }

        Ok(Self::L {
            client,
            runtime,
            timeout: self.timeout,
            is_open: true,
        })
    }
}

impl SimulatorBuilder {
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
}

impl Simulator {
    pub fn builder(port: u16) -> SimulatorBuilder {
        SimulatorBuilder {
            addr: Either::V4(Ipv4Addr::LOCALHOST),
            port,
            timeout: Duration::from_millis(200),
        }
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
    ) -> Result<Vec<RxMessage>, AUTDProtoBufError> {
        let res = runtime.block_on(client.read_data(ReadRequest {}))?;
        Ok(Vec::<RxMessage>::from_msg(&res.into_inner()))
    }
}

impl Link for Simulator {
    fn close(&mut self) -> Result<(), AUTDInternalError> {
        if !self.is_open {
            return Ok(());
        }
        self.is_open = false;

        Self::close_impl(&mut self.client, &self.runtime)?;

        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open {
            return Err(AUTDInternalError::LinkClosed);
        }

        Ok(Self::send_impl(&mut self.client, &self.runtime, tx)?)
    }

    fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError> {
        if !self.is_open {
            return Err(AUTDInternalError::LinkClosed);
        }

        let rx_ = Self::receive_impl(&mut self.client, &self.runtime)?;
        if rx.len() == rx_.len() {
            rx.copy_from_slice(&rx_);
        }

        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }

    // fn update_geometry(&mut self, devices: &[Device<T>]) -> Result<(), AUTDInternalError> {
    //     if let Some(client) = &mut self.client {
    //         if self
    //             .runtime
    //             .block_on(client.update_geomety(devices.to_msg()))
    //             .is_err()
    //         {
    //             return Err(
    //                 AUTDProtoBufError::SendError("Failed to update geometry".to_string()).into(),
    //             );
    //         }
    //         Ok(())
    //     } else {
    //         Err(AUTDInternalError::LinkClosed)
    //     }
    // }
}
