/*
 * File: lightweight_server.rs
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

use std::sync::RwLock;

use crate::{error::*, pb::*, traits::*};

use tonic::{Request, Response, Status};

pub struct LightweightServer<
    L: autd3_core::link::Link<autd3::prelude::LegacyTransducer> + Sync + 'static,
    F: Fn() -> L + Send + Sync + 'static,
> {
    autd: RwLock<Option<autd3::Controller<autd3::prelude::LegacyTransducer, L>>>,
    link: F,
}

impl<
        L: autd3_core::link::Link<autd3::prelude::LegacyTransducer> + Sync + 'static,
        F: Fn() -> L + Send + Sync + 'static,
    > LightweightServer<L, F>
{
    pub fn new(f: F) -> Self {
        LightweightServer {
            autd: RwLock::new(None),
            link: f,
        }
    }

    fn send_special(
        autd: &mut autd3::Controller<autd3::prelude::LegacyTransducer, L>,
        msg: &SpecialData,
    ) -> Result<bool, AUTDProtoBufError> {
        Ok(match msg.special {
            Some(special_data::Special::Clear(_)) => autd.send(autd3::prelude::Clear::new()),
            Some(special_data::Special::Stop(_)) => autd.send(autd3::prelude::Stop::new()),
            Some(special_data::Special::Synchronize(_)) => {
                autd.send(autd3::prelude::Synchronize::new())
            }
            Some(special_data::Special::UpdateFlags(_)) => {
                autd.send(autd3::prelude::UpdateFlags::new())
            }
            _ => return Err(AUTDProtoBufError::NotSupportedData),
        }?)
    }

    fn send_modulation(
        autd: &mut autd3::Controller<autd3::prelude::LegacyTransducer, L>,
        modulation: &Modulation,
    ) -> Result<bool, AUTDProtoBufError> {
        Ok(match &modulation.modulation {
            Some(modulation::Modulation::Static(msg)) => {
                autd.send(autd3::prelude::Static::from_msg(msg))
            }
            Some(modulation::Modulation::SineLegacy(msg)) => {
                autd.send(autd3::prelude::SineLegacy::from_msg(msg))
            }
            Some(modulation::Modulation::Sine(msg)) => {
                autd.send(autd3::prelude::Sine::from_msg(msg))
            }
            Some(modulation::Modulation::SinePressure(msg)) => {
                autd.send(autd3::prelude::SinePressure::from_msg(msg))
            }
            Some(modulation::Modulation::Square(msg)) => {
                autd.send(autd3::prelude::Square::from_msg(msg))
            }
            None => return Err(AUTDProtoBufError::NotSupportedData),
        }?)
    }

    fn send_silencer(
        autd: &mut autd3::Controller<autd3::prelude::LegacyTransducer, L>,
        msg: &SilencerConfig,
    ) -> Result<bool, AUTDProtoBufError> {
        Ok(autd.send(autd3::prelude::SilencerConfig::from_msg(msg))?)
    }

    fn send_gain(
        autd: &mut autd3::Controller<autd3::prelude::LegacyTransducer, L>,
        gain: &Gain,
    ) -> Result<bool, AUTDProtoBufError> {
        Ok(match &gain.gain {
            Some(gain::Gain::Focus(msg)) => autd.send(autd3::prelude::Focus::from_msg(msg)),
            Some(gain::Gain::Bessel(msg)) => autd.send(autd3::prelude::Bessel::from_msg(msg)),
            Some(gain::Gain::Null(msg)) => autd.send(autd3::prelude::Null::from_msg(msg)),
            Some(gain::Gain::Plane(msg)) => autd.send(autd3::prelude::Plane::from_msg(msg)),
            Some(gain::Gain::TransTest(msg)) => {
                autd.send(autd3::prelude::TransducerTest::from_msg(msg))
            }
            Some(gain::Gain::Grouped(msg)) => autd.send(autd3::prelude::Grouped::from_msg(msg)),
            Some(gain::Gain::Sdp(msg)) => autd.send(autd3_gain_holo::SDP::from_msg(msg)),
            Some(gain::Gain::Evp(msg)) => autd.send(autd3_gain_holo::EVP::from_msg(msg)),
            Some(gain::Gain::Naive(msg)) => autd.send(autd3_gain_holo::Naive::from_msg(msg)),
            Some(gain::Gain::Gs(msg)) => autd.send(autd3_gain_holo::GS::from_msg(msg)),
            Some(gain::Gain::Gspat(msg)) => autd.send(autd3_gain_holo::GSPAT::from_msg(msg)),
            Some(gain::Gain::Lm(msg)) => autd.send(autd3_gain_holo::LM::from_msg(msg)),
            Some(gain::Gain::Greedy(msg)) => autd.send(autd3_gain_holo::Greedy::from_msg(msg)),
            None => return Err(AUTDProtoBufError::NotSupportedData),
        }?)
    }
}

#[tonic::async_trait]
impl<
        L: autd3_core::link::Link<autd3::prelude::LegacyTransducer> + Sync + 'static,
        F: Fn() -> L + Send + Sync + 'static,
    > ecat_light_server::EcatLight for LightweightServer<L, F>
{
    async fn config_geomety(
        &self,
        req: Request<Geometry>,
    ) -> Result<Response<GeometryLightResponse>, Status> {
        if let Some(mut autd) = self.autd.write().unwrap().take() {
            match autd.close() {
                Ok(_) => {}
                Err(e) => {
                    return Ok(Response::new(GeometryLightResponse {
                        success: false,
                        msg: format!("{}", e),
                    }))
                }
            }
        }
        *self.autd.write().unwrap() =
            match Vec::<autd3::prelude::AUTD3>::from_msg(&req.into_inner())
                .iter()
                .fold(autd3::prelude::Controller::builder(), |acc, &dev| {
                    acc.add_device(dev)
                })
                .open_with((self.link)())
            {
                Ok(autd) => Some(autd),
                Err(e) => {
                    return Ok(Response::new(GeometryLightResponse {
                        success: false,
                        msg: format!("{}", e),
                    }))
                }
            };

        Ok(Response::new(GeometryLightResponse {
            success: true,
            msg: String::new(),
        }))
    }

    async fn send(&self, req: Request<Datagram>) -> Result<Response<SendLightResponse>, Status> {
        if let Some(autd) = self.autd.write().unwrap().as_mut() {
            match match req.into_inner().datagram {
                Some(datagram::Datagram::SilencerConfig(ref msg)) => Self::send_silencer(autd, msg),
                Some(datagram::Datagram::Gain(ref msg)) => Self::send_gain(autd, msg),
                Some(datagram::Datagram::Modulation(ref msg)) => Self::send_modulation(autd, msg),
                Some(datagram::Datagram::Special(ref msg)) => Self::send_special(autd, msg),
                None => Err(AUTDProtoBufError::NotSupportedData),
            } {
                Ok(res) => Ok(Response::new(SendLightResponse {
                    success: res,
                    err: false,
                    msg: String::new(),
                })),
                Err(e) => Ok(Response::new(SendLightResponse {
                    success: false,
                    err: true,
                    msg: format!("{}", e),
                })),
            }
        } else {
            Ok(Response::new(SendLightResponse {
                success: false,
                err: true,
                msg: "Geometry is not configured".to_string(),
            }))
        }
    }

    async fn close(
        &self,
        _: Request<CloseRequest>,
    ) -> Result<Response<GeometryLightResponse>, Status> {
        if let Some(mut autd) = self.autd.write().unwrap().take() {
            match autd.close() {
                Ok(_) => Ok(Response::new(GeometryLightResponse {
                    success: true,
                    msg: String::new(),
                })),
                Err(e) => Ok(Response::new(GeometryLightResponse {
                    success: false,
                    msg: format!("{}", e),
                })),
            }
        } else {
            Ok(Response::new(GeometryLightResponse {
                success: false,
                msg: "Controller is not opened".to_string(),
            }))
        }
    }
}
