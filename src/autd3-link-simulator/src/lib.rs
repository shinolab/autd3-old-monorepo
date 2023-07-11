/*
 * File: lib.rs
 * Project: src
 * Created Date: 09/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use tokio::runtime::{Builder, Runtime};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

use autd3_protobuf::*;

use crossbeam_channel::bounded;
// use std::sync::atomic::{AtomicBool, Ordering};
// use std::sync::{Arc, RwLock};
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
    tx: Option<mpsc::Sender<SimulatorTx>>,
    rx: Option<mpsc::Receiver<SimulatorTx>>,
    // rx_buf: Arc<RwLock<autd3_core::RxDatagram>>,
    receive_stream_th: Option<JoinHandle<()>>,
    timeout: Duration,
    runtime: Runtime,
    rx_receiver: Option<crossbeam_channel::Receiver<SimulatorRx>>,
}

impl Simulator {
    pub fn new(port: u16) -> Self {
        let (tx, rx) = mpsc::channel(128);
        Self {
            addr: Either::V4(Ipv4Addr::LOCALHOST),
            port,
            tx: Some(tx),
            rx: Some(rx),
            // rx_buf: Arc::new(RwLock::new(autd3_core::RxDatagram::new(0))),
            receive_stream_th: None,
            timeout: Duration::from_millis(200),
            runtime: Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap(),
            rx_receiver: None,
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

    fn open_impl<T: Transducer>(
        &mut self,
        geometry: &Geometry<T>,
    ) -> Result<(), AUTDProtoBufError> {
        let (rx_sender, rx_receiver) = bounded(128);

        let mut client = self
            .runtime
            .block_on(simulator_client::SimulatorClient::connect(format!(
                "http://{}",
                match self.addr {
                    Either::V4(ip) => SocketAddr::V4(SocketAddrV4::new(ip, self.port)),
                    Either::V6(ip) => SocketAddr::V6(SocketAddrV6::new(ip, self.port, 0, 0)),
                }
            )))?;
        let rx = self.rx.take().unwrap();
        self.receive_stream_th = Some(self.runtime.spawn(async move {
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
        }));

        if !(0..20).any(|_| {
            if self
                .tx
                .as_mut()
                .unwrap()
                .blocking_send(SimulatorTx {
                    data: Some(simulator_tx::Data::Geometry(geometry.to_msg())),
                })
                .is_err()
            {
                return false;
            }
            std::thread::sleep(Duration::from_millis(100));
            if let Ok(rx) = rx_receiver.try_recv() {
                if let Some(rx) = rx.data {
                    return matches!(rx, simulator_rx::Data::Geometry(_));
                }
            }
            false
        }) {
            return Err(AUTDProtoBufError::SendError(
                "Failed to initialize simulator".to_string(),
            ));
        }

        // self.rx_buf = Arc::new(RwLock::new(autd3_core::RxDatagram::new(
        //     geometry.num_devices(),
        // )));

        self.rx_receiver = Some(rx_receiver);

        // self.run.store(true, Ordering::Release);
        // let run = self.run.clone();
        // let rx_buf = self.rx_buf.clone();
        // self.receive_th = Some(std::thread::spawn(move || {
        //     while run.load(Ordering::Acquire) {
        //         if let Ok(rx) = rx_receiver.try_recv() {
        //             if let Some(simulator_rx::Data::Rx(rx)) = rx.data {
        //                 *rx_buf.write().unwrap() = autd3_core::RxDatagram::from_msg(&rx);
        //             }
        //         }
        //         std::thread::sleep(Duration::from_millis(1));
        //     }
        // }));

        Ok(())
    }

    fn close_impl(&mut self) -> Result<(), AUTDProtoBufError> {
        // self.run.store(false, Ordering::Release);
        if let Some(tx) = self.tx.take() {
            tx.blocking_send(SimulatorTx {
                data: Some(simulator_tx::Data::Close(CloseRequest {})),
            })?;
            drop(tx);
        }
        // if let Some(th) = self.receive_th.take() {
        //     th.join().unwrap();
        // }
        if let Some(th) = self.receive_stream_th.take() {
            self.runtime.block_on(th)?;
        }
        Ok(())
    }

    fn send_impl(
        sender: &mut tokio::sync::mpsc::Sender<SimulatorTx>,
        tx: &TxDatagram,
    ) -> Result<(), AUTDProtoBufError> {
        sender.blocking_send(SimulatorTx {
            data: Some(simulator_tx::Data::Raw(tx.to_msg())),
        })?;
        Ok(())
    }

    fn receive_impl(
        sender: &mut tokio::sync::mpsc::Sender<SimulatorTx>,
    ) -> Result<(), AUTDProtoBufError> {
        sender.blocking_send(SimulatorTx {
            data: Some(simulator_tx::Data::Read(ReadRequest {})),
        })?;
        Ok(())
    }
}

impl<T: Transducer> Link<T> for Simulator {
    fn open(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        self.open_impl(geometry)?;
        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        self.close_impl()?;
        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        if let Some(tx_) = &mut self.tx {
            Self::send_impl(tx_, tx)?;
            Ok(true)
        } else {
            Err(AUTDInternalError::LinkClosed)
        }
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        if let Some(tx_) = &mut self.tx {
            Self::receive_impl(tx_)?;
        } else {
            return Err(AUTDInternalError::LinkClosed);
        }

        if let Some(receiver) = self.rx_receiver.as_ref() {
            if let Ok(rx_) = receiver.try_recv() {
                if let Some(simulator_rx::Data::Rx(rx_)) = rx_.data {
                    rx.copy_from(&autd3_core::RxDatagram::from_msg(&rx_));
                }
            }
        }

        Ok(true)
    }

    fn is_open(&self) -> bool {
        // self.run.load(Ordering::Acquire)
        self.tx.is_some()
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}
