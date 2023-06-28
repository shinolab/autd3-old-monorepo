pub mod pb {
    tonic::include_proto!("autd3");
}

use std::error::Error;

pub use pb::*;

use thiserror::Error;
use tonic::Status;

pub fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn Error + 'static) = err_status;
    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }
        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}

pub trait ToMessage {
    type Message: prost::Message;

    fn to_msg(&self) -> Self::Message;
}

pub trait FromMessage<T: prost::Message> {
    fn from_msg(msg: &T) -> Self;
}

impl<T: autd3_core::geometry::Transducer> ToMessage for autd3_core::geometry::Geometry<T> {
    type Message = Geometry;

    fn to_msg(&self) -> Self::Message {
        pb::Geometry {
            geometries: (0..self.num_devices())
                .map(|dev| {
                    let pos = self.transducers_of(dev).next().unwrap().position();
                    let rot = self.transducers_of(dev).next().unwrap().rotation();
                    #[allow(clippy::unnecessary_cast)]
                    geometry::Autd3 {
                        position: Some(Vector3 {
                            x: pos.x as f64,
                            y: pos.y as f64,
                            z: pos.z as f64,
                        }),
                        rotation: Some(Quaternion {
                            w: rot.w as f64,
                            x: rot.coords.x as f64,
                            y: rot.coords.y as f64,
                            z: rot.coords.z as f64,
                        }),
                    }
                })
                .collect(),
        }
    }
}

impl ToMessage for autd3_core::TxDatagram {
    type Message = TxRawData;

    fn to_msg(&self) -> Self::Message {
        TxRawData {
            data: self.data().to_vec(),
        }
    }
}

impl FromMessage<RxMessage> for autd3_core::RxDatagram {
    fn from_msg(msg: &RxMessage) -> Self {
        let mut rx = autd3_core::RxDatagram::new(
            msg.data.len() / std::mem::size_of::<autd3_core::RxMessage>(),
        );
        unsafe {
            std::ptr::copy_nonoverlapping(msg.data.as_ptr(), rx.as_mut_ptr() as _, msg.data.len());
        }
        rx
    }
}

impl FromMessage<Geometry> for Vec<autd3_core::autd3_device::AUTD3> {
    fn from_msg(msg: &Geometry) -> Self {
        msg.geometries
            .iter()
            .map(|dev| {
                let pos = dev.position.as_ref().unwrap();
                let pos = autd3_core::geometry::Vector3::new(pos.x as _, pos.y as _, pos.z as _);
                let rot = dev.rotation.as_ref().unwrap();
                let rot = autd3_core::geometry::UnitQuaternion::from_quaternion(
                    autd3_core::geometry::Quaternion::new(
                        rot.w as _, rot.x as _, rot.y as _, rot.z as _,
                    ),
                );
                autd3_core::autd3_device::AUTD3::with_quaternion(pos, rot)
            })
            .collect()
    }
}

impl FromMessage<TxRawData> for autd3_core::TxDatagram {
    fn from_msg(msg: &TxRawData) -> Self {
        let len = msg.data.len();
        let header_size = std::mem::size_of::<autd3_core::GlobalHeader>();
        let body_size = std::mem::size_of::<u16>() * autd3_core::autd3_device::NUM_TRANS_IN_UNIT;
        let body_num = if len > header_size {
            if (len - header_size) % body_size != 0 {
                0
            } else {
                (len - header_size) / body_size
            }
        } else {
            0
        };
        let mut tx = autd3_core::TxDatagram::new(&vec![
            autd3_core::autd3_device::NUM_TRANS_IN_UNIT;
            body_num
        ]);
        tx.num_bodies = body_num;
        let body_len = body_num * body_size;
        unsafe {
            std::ptr::copy_nonoverlapping(
                msg.data[header_size..].as_ptr(),
                tx.body_raw_mut().as_mut_ptr() as *mut u8,
                body_len,
            );
            std::ptr::copy_nonoverlapping(
                msg.data.as_ptr(),
                tx.header_mut() as *mut _ as *mut u8,
                header_size,
            );
        }
        tx
    }
}

#[derive(Error, Debug)]
pub enum AUTDProtoBufError {
    #[error("{0}")]
    SendError(String),
    #[error("{0}")]
    TokioSendError(String),
    #[error("{0}")]
    TransportError(String),
    #[error("{0}")]
    TokioJoinError(String),
}

impl<T> From<std::sync::mpsc::SendError<T>> for AUTDProtoBufError {
    fn from(e: std::sync::mpsc::SendError<T>) -> Self {
        AUTDProtoBufError::SendError(e.to_string())
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for AUTDProtoBufError {
    fn from(e: tokio::sync::mpsc::error::SendError<T>) -> Self {
        AUTDProtoBufError::TokioSendError(e.to_string())
    }
}

impl From<tokio::task::JoinError> for AUTDProtoBufError {
    fn from(e: tokio::task::JoinError) -> Self {
        AUTDProtoBufError::TokioJoinError(e.to_string())
    }
}

impl From<tonic::transport::Error> for AUTDProtoBufError {
    fn from(e: tonic::transport::Error) -> Self {
        match e.source() {
            Some(source) => AUTDProtoBufError::TransportError(source.to_string()),
            None => AUTDProtoBufError::TransportError(e.to_string()),
        }
    }
}

impl From<AUTDProtoBufError> for autd3_core::error::AUTDInternalError {
    fn from(e: AUTDProtoBufError) -> Self {
        autd3_core::error::AUTDInternalError::LinkError(e.to_string())
    }
}
