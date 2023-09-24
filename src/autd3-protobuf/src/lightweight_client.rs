/*
 * File: lightweight_client.rs
 * Project: src
 * Created Date: 30/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::net::SocketAddr;

use autd3_driver::{
    datagram::{Clear, Synchronize},
    geometry::{Device, Geometry, IntoDevice, LegacyTransducer},
};

use crate::traits::*;

/// Client of AUTD with lightweight mode
pub struct LightweightClient {
    client: crate::pb::ecat_light_client::EcatLightClient<tonic::transport::Channel>,
}

pub struct LightweightClientBuilder {
    devices: Vec<Device<LegacyTransducer>>,
}

impl Default for LightweightClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LightweightClientBuilder {
    const fn new() -> Self {
        Self { devices: vec![] }
    }

    /// Add device
    pub fn add_device<D: IntoDevice<LegacyTransducer>>(mut self, dev: D) -> Self {
        self.devices.push(dev.into_device(self.devices.len()));
        self
    }

    /// Open connection
    pub async fn open(
        self,
        addr: SocketAddr,
    ) -> Result<LightweightClient, crate::error::AUTDProtoBufError> {
        LightweightClient::open_impl(Geometry::<LegacyTransducer>::new(self.devices), addr).await
    }
}

impl LightweightClient {
    /// Create Client builder
    pub const fn builder() -> LightweightClientBuilder {
        LightweightClientBuilder::new()
    }

    async fn open_impl(
        geometry: Geometry<LegacyTransducer>,
        addr: SocketAddr,
    ) -> Result<Self, crate::error::AUTDProtoBufError> {
        let mut client =
            crate::pb::ecat_light_client::EcatLightClient::connect(format!("http://{}", addr))
                .await?;
        let res = client.config_geomety(geometry.to_msg()).await?.into_inner();
        if !res.success {
            return Err(crate::error::AUTDProtoBufError::SendError(res.msg));
        }

        let mut client = Self { client };

        client.send(Clear::new()).await?;
        client.send(Synchronize::new()).await?;

        Ok(client)
    }

    /// Get firmware information
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<FirmwareInfo>)` - List of firmware information
    ///
    pub async fn firmware_infos(
        &mut self,
    ) -> Result<Vec<autd3_driver::firmware_version::FirmwareInfo>, crate::error::AUTDProtoBufError>
    {
        let res = self
            .client
            .firmware_info(tonic::Request::new(crate::pb::FirmwareInfoRequest {}))
            .await?
            .into_inner();
        if !res.success {
            return Err(crate::error::AUTDProtoBufError::SendError(res.msg));
        }
        Ok(Vec::from_msg(&res))
    }

    /// set force fan flag
    pub async fn force_fan(&mut self, value: bool) -> Result<(), crate::error::AUTDProtoBufError> {
        let res = self
            .client
            .force_fan(crate::pb::ForceFanRequest { value })
            .await?
            .into_inner();
        if !res.success {
            return Err(crate::error::AUTDProtoBufError::SendError(res.msg));
        }
        Ok(())
    }

    /// set reads fpga info flag
    pub async fn reads_fpga_info(
        &mut self,
        value: bool,
    ) -> Result<(), crate::error::AUTDProtoBufError> {
        let res = self
            .client
            .reads_fpga_info(crate::pb::ReadsFpgaInfoRequest { value })
            .await?
            .into_inner();
        if !res.success {
            return Err(crate::error::AUTDProtoBufError::SendError(res.msg));
        }
        Ok(())
    }

    /// Get FPGA information
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<FPGAInfo>)` - List of FPGA information
    ///
    pub async fn fpga_info(
        &mut self,
    ) -> Result<Vec<autd3_driver::fpga::FPGAInfo>, crate::error::AUTDProtoBufError> {
        let res = self
            .client
            .fpga_info(crate::pb::FpgaInfoRequest {})
            .await?
            .into_inner();
        if !res.success {
            return Err(crate::error::AUTDProtoBufError::SendError(res.msg));
        }
        Ok(Vec::from_msg(&res))
    }

    /// Send data to the devices
    ///
    /// # Arguments
    ///
    /// * `s` - Datagram
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - It is confirmed that the data has been successfully transmitted
    /// * `Ok(false)` - There are no errors, but it is unclear whether the data has been sent reliably or not
    ///
    pub async fn send<D: ToMessage<Message = crate::pb::Datagram>>(
        &mut self,
        datagram: D,
    ) -> Result<bool, crate::error::AUTDProtoBufError> {
        let res = self
            .client
            .send(tonic::Request::new(datagram.to_msg()))
            .await?
            .into_inner();
        if res.err {
            return Err(crate::error::AUTDProtoBufError::SendError(res.msg));
        }
        Ok(res.success)
    }

    // Close connection
    pub async fn close(mut self) -> Result<(), crate::error::AUTDProtoBufError> {
        let res = self
            .client
            .close(crate::pb::CloseRequest {})
            .await?
            .into_inner();
        if !res.success {
            return Err(crate::error::AUTDProtoBufError::SendError(res.msg));
        }
        Ok(())
    }
}
