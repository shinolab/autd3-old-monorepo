/*
 * File: lightweight_client.rs
 * Project: src
 * Created Date: 30/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::net::SocketAddr;

use autd3_core::geometry::Transducer;

use crate::ToMessage;

pub struct LightweightClient {
    client: crate::pb::ecat_light_client::EcatLightClient<tonic::transport::Channel>,
}

pub struct LightweightClientBuilder {
    transducers: Vec<(
        usize,
        autd3_core::geometry::Vector3,
        autd3_core::geometry::UnitQuaternion,
    )>,
    device_map: Vec<usize>,
}

impl Default for LightweightClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LightweightClientBuilder {
    pub fn new() -> Self {
        Self {
            transducers: vec![],
            device_map: vec![],
        }
    }

    pub fn add_device<D: autd3_core::geometry::Device>(mut self, dev: D) -> Self {
        let id = self.transducers.len();
        let mut t = dev.get_transducers(id);
        self.device_map.push(t.len());
        self.transducers.append(&mut t);
        self
    }

    pub async fn open(
        self,
        addr: SocketAddr,
    ) -> Result<LightweightClient, crate::error::AUTDProtoBufError> {
        LightweightClient::open_impl(
            autd3_core::geometry::Geometry::<autd3_core::geometry::LegacyTransducer>::new(
                self.transducers
                    .iter()
                    .map(|&(id, pos, rot)| {
                        autd3_core::geometry::LegacyTransducer::new(id, pos, rot)
                    })
                    .collect(),
                self.device_map.clone(),
                340. * autd3_core::METER,
                0.,
            )?,
            addr,
        )
        .await
    }
}

impl LightweightClient {
    pub fn builder() -> LightweightClientBuilder {
        LightweightClientBuilder::new()
    }

    async fn open_impl(
        geometry: autd3_core::geometry::Geometry<autd3_core::geometry::LegacyTransducer>,
        addr: SocketAddr,
    ) -> Result<Self, crate::error::AUTDProtoBufError> {
        let mut client =
            crate::pb::ecat_light_client::EcatLightClient::connect(format!("http://{}", addr))
                .await?;
        let res = client.config_geomety(geometry.to_msg()).await?.into_inner();
        if !res.success {
            return Err(crate::error::AUTDProtoBufError::SendError(res.msg));
        }
        Ok(Self { client })
    }

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
