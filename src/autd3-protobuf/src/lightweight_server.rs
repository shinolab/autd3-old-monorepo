/*
 * File: lightweight_server.rs
 * Project: src
 * Created Date: 30/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::sync::RwLock;

use crate::{error::*, pb::*, traits::*};

use tonic::{Request, Response, Status};

#[doc(hidden)]
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
    pub const fn new(f: F) -> Self {
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

    fn send_focus_stm(
        autd: &mut autd3::Controller<autd3::prelude::LegacyTransducer, L>,
        msg: &FocusStm,
    ) -> Result<bool, AUTDProtoBufError> {
        if msg.control_points.is_empty() {
            return Err(AUTDProtoBufError::NotSupportedData);
        }
        Ok(autd.send(autd3::prelude::FocusSTM::from_msg(msg))?)
    }

    fn send_gain_stm(
        autd: &mut autd3::Controller<autd3::prelude::LegacyTransducer, L>,
        msg: &GainStm,
    ) -> Result<bool, AUTDProtoBufError> {
        if msg.gains.is_empty() {
            return Err(AUTDProtoBufError::NotSupportedData);
        }
        Ok(autd.send(autd3::prelude::GainSTM::from_msg(msg))?)
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
        *self.autd.write().unwrap() = match autd3::Controller::open_impl(
            autd3::core::geometry::Geometry::from_msg(&req.into_inner()),
            (self.link)(),
        ) {
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

    async fn firmware_info(
        &self,
        _req: Request<FirmwareInfoRequest>,
    ) -> Result<Response<FirmwareInfoResponse>, Status> {
        if let Some(autd) = self.autd.write().unwrap().as_mut() {
            match autd.firmware_infos() {
                Ok(list) => Ok(Response::new(FirmwareInfoResponse {
                    success: true,
                    msg: String::new(),
                    firmware_info_list: list
                        .iter()
                        .map(|f| firmware_info_response::FirmwareInfo {
                            cpu_major_version: f.cpu_version_number_major() as _,
                            cpu_minor_version: f.cpu_version_number_minor() as _,
                            fpga_major_version: f.fpga_version_number_major() as _,
                            fpga_minor_version: f.fpga_version_number_minor() as _,
                            fpga_function_bits: f.fpga_function_bits() as _,
                        })
                        .collect(),
                })),
                Err(e) => {
                    return Ok(Response::new(FirmwareInfoResponse {
                        success: false,
                        msg: format!("{}", e),
                        firmware_info_list: Vec::new(),
                    }))
                }
            }
        } else {
            Ok(Response::new(FirmwareInfoResponse {
                success: false,
                msg: "Geometry is not configured".to_string(),
                firmware_info_list: Vec::new(),
            }))
        }
    }

    async fn force_fan(
        &self,
        req: Request<ForceFanRequest>,
    ) -> Result<Response<ForceFanResponse>, Status> {
        if let Some(autd) = self.autd.write().unwrap().as_mut() {
            autd.force_fan(req.into_inner().value);
            return Ok(Response::new(ForceFanResponse {
                success: false,
                msg: String::new(),
            }));
        } else {
            Ok(Response::new(ForceFanResponse {
                success: false,
                msg: "Geometry is not configured".to_string(),
            }))
        }
    }

    async fn reads_fpga_info(
        &self,
        req: Request<ReadsFpgaInfoRequest>,
    ) -> Result<Response<ReadsFpgaInfoResponse>, Status> {
        if let Some(autd) = self.autd.write().unwrap().as_mut() {
            autd.reads_fpga_info(req.into_inner().value);
            return Ok(Response::new(ReadsFpgaInfoResponse {
                success: false,
                msg: String::new(),
            }));
        } else {
            Ok(Response::new(ReadsFpgaInfoResponse {
                success: false,
                msg: "Geometry is not configured".to_string(),
            }))
        }
    }

    async fn fpga_info(
        &self,
        _req: Request<FpgaInfoRequest>,
    ) -> Result<Response<FpgaInfoResponse>, Status> {
        if let Some(autd) = self.autd.write().unwrap().as_mut() {
            match autd.fpga_info() {
                Ok(list) => Ok(Response::new(FpgaInfoResponse {
                    success: true,
                    msg: String::new(),
                    fpga_info_list: list
                        .iter()
                        .map(|f| fpga_info_response::FpgaInfo {
                            info: f.info() as _,
                        })
                        .collect(),
                })),
                Err(e) => {
                    return Ok(Response::new(FpgaInfoResponse {
                        success: false,
                        msg: format!("{}", e),
                        fpga_info_list: Vec::new(),
                    }))
                }
            }
        } else {
            Ok(Response::new(FpgaInfoResponse {
                success: false,
                msg: "Geometry is not configured".to_string(),
                fpga_info_list: Vec::new(),
            }))
        }
    }

    async fn send(&self, req: Request<Datagram>) -> Result<Response<SendLightResponse>, Status> {
        if let Some(autd) = self.autd.write().unwrap().as_mut() {
            match match req.into_inner().datagram {
                Some(datagram::Datagram::SilencerConfig(ref msg)) => Self::send_silencer(autd, msg),
                Some(datagram::Datagram::Gain(ref msg)) => Self::send_gain(autd, msg),
                Some(datagram::Datagram::Modulation(ref msg)) => Self::send_modulation(autd, msg),
                Some(datagram::Datagram::Special(ref msg)) => Self::send_special(autd, msg),
                Some(datagram::Datagram::FocusStm(ref msg)) => Self::send_focus_stm(autd, msg),
                Some(datagram::Datagram::GainStm(ref msg)) => Self::send_gain_stm(autd, msg),
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
