/*
 * File: lib.rs
 * Project: src
 * Created Date: 09/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use tokio::runtime::{Builder, Runtime};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::transport::Channel;

pub mod pb {
    tonic::include_proto!("autd3");
}

use pb::*;

use crossbeam_channel::{bounded, Sender};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
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

pub struct Simulator {
    addr: Either,
    port: u16,
    tx: Option<mpsc::Sender<Tx>>,
    rx: Option<mpsc::Receiver<Tx>>,
    rx_buf: Arc<RwLock<Vec<u8>>>,
    receive_th: Option<std::thread::JoinHandle<()>>,
    receive_stream_th: Option<JoinHandle<()>>,
    timeout: Duration,
    runtime: Runtime,
    run: Arc<AtomicBool>,
}

impl Simulator {
    pub fn new(port: u16) -> Self {
        let (tx, rx) = mpsc::channel(128);
        Self {
            addr: Either::V4(Ipv4Addr::LOCALHOST),
            port,
            tx: Some(tx),
            rx: Some(rx),
            rx_buf: Arc::new(RwLock::new(Vec::new())),
            receive_th: None,
            receive_stream_th: None,
            timeout: Duration::from_millis(200),
            runtime: Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap(),
            run: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn with_server_ip(self, ipv4: Ipv4Addr) -> Self {
        self.with_server_ipv4(ipv4)
    }

    pub fn with_server_ipv4(self, ipv4: Ipv4Addr) -> Self {
        Self {
            addr: Either::V4(ipv4),
            ..self
        }
    }

    pub fn with_server_ipv6(self, ipv6: Ipv6Addr) -> Self {
        Self {
            addr: Either::V6(ipv6),
            ..self
        }
    }

    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }

    async fn rt(
        mut client: simulator_client::SimulatorClient<Channel>,
        rx: tokio::sync::mpsc::Receiver<Tx>,
        rx_sender: Sender<Rx>,
    ) {
        let response = client.receive_data(ReceiverStream::new(rx)).await.unwrap();
        let mut resp_stream = response.into_inner();
        while let Some(received) = resp_stream.next().await {
            match received {
                Ok(received) => match rx_sender.send(received) {
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                },
                Err(_) => {
                    break;
                }
            }
        }
    }
}

impl<T: Transducer> Link<T> for Simulator {
    fn open(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        let sockect_addr = match self.addr {
            Either::V4(ip) => SocketAddr::V4(SocketAddrV4::new(ip, self.port)),
            Either::V6(ip) => SocketAddr::V6(SocketAddrV6::new(ip, self.port, 0, 0)),
        };

        let client = match self
            .runtime
            .block_on(simulator_client::SimulatorClient::connect(format!(
                "http://{}",
                sockect_addr
            ))) {
            Ok(client) => client,
            Err(err) => return Err(AUTDInternalError::LinkError(err.to_string())),
        };

        let (rx_sender, rx_receiver) = bounded(128);

        self.receive_stream_th = Some(self.runtime.spawn(Self::rt(
            client,
            self.rx.take().unwrap(),
            rx_sender,
        )));

        if let Err(err) = self.tx.as_mut().unwrap().blocking_send(Tx {
            data: Some(tx::Data::Geometry(pb::Geometry {
                geometries: (0..geometry.num_devices())
                    .map(|dev| {
                        let pos = geometry.transducers_of(dev).next().unwrap().position();
                        let rot = geometry.transducers_of(dev).next().unwrap().rotation();
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
            })),
        }) {
            return Err(AUTDInternalError::LinkError(err.to_string()));
        }

        if !(0..50).any(|_| {
            std::thread::sleep(Duration::from_millis(100));
            if let Ok(rx) = rx_receiver.try_recv() {
                if let Some(rx) = rx.data {
                    return matches!(rx, rx::Data::Geometry(_));
                }
            }
            false
        }) {
            return Err(AUTDInternalError::LinkError(
                "Failed to initialize simulator".to_string(),
            ));
        }

        let rx_buf_size = std::mem::size_of::<autd3_core::RxMessage>() * geometry.num_devices();
        self.rx_buf = Arc::new(RwLock::new(vec![0; rx_buf_size]));

        self.run.store(true, Ordering::Release);
        let run = self.run.clone();
        let rx_buf = self.rx_buf.clone();
        self.receive_th = Some(std::thread::spawn(move || {
            while run.load(Ordering::Acquire) {
                if let Ok(rx) = rx_receiver.try_recv() {
                    if let Some(rx::Data::Rx(rx)) = rx.data {
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                rx.data.as_ptr(),
                                rx_buf.write().unwrap().as_mut_ptr(),
                                rx.data.len(),
                            );
                        }
                    }
                }
                std::thread::sleep(Duration::from_millis(1));
            }
        }));

        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        if let Some(tx) = self.tx.take() {
            if let Err(e) = tx.blocking_send(Tx {
                data: Some(tx::Data::Close(Close {})),
            }) {
                return Err(AUTDInternalError::LinkError(format!(
                    "Failed to close simulator: {}",
                    e
                )));
            }
            drop(tx);
        }
        self.run.store(false, Ordering::Release);
        if let Some(th) = self.receive_th.take() {
            th.join().unwrap();
        }
        if let Some(th) = self.receive_stream_th.take() {
            match self.runtime.block_on(th) {
                Ok(_) => Ok(()),
                Err(err) => Err(AUTDInternalError::LinkError(err.to_string())),
            }
        } else {
            Ok(())
        }
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        if let Some(tx_) = &mut self.tx {
            match tx_.blocking_send(Tx {
                data: Some(tx::Data::Raw(TxRawData {
                    data: tx.data().to_vec(),
                })),
            }) {
                Ok(_) => Ok(true),
                Err(e) => Err(AUTDInternalError::LinkError(format!(
                    "Failed to send data: {}",
                    e
                ))),
            }
        } else {
            Err(AUTDInternalError::LinkClosed)
        }
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        if let Some(tx_) = &mut self.tx {
            if let Err(e) = tx_.blocking_send(Tx {
                data: Some(tx::Data::Read(Read {})),
            }) {
                return Err(AUTDInternalError::LinkError(format!(
                    "Failed to receive data: {}",
                    e
                )));
            }
        } else {
            return Err(AUTDInternalError::LinkClosed);
        }

        unsafe {
            let rx_buf = self.rx_buf.read().unwrap();
            std::ptr::copy_nonoverlapping(
                rx_buf.as_ptr(),
                rx.as_mut_ptr() as *mut u8,
                rx_buf.len(),
            );
        }
        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.receive_stream_th.is_some()
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}
