/*
 * File: link.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use async_trait::async_trait;
use std::time::Duration;

use crate::{
    cpu::{RxMessage, TxDatagram},
    derive::prelude::Geometry,
    error::AUTDInternalError,
    geometry::Transducer,
};

/// Link is a interface to the AUTD device
#[async_trait]
pub trait Link: Send + Sync {
    /// Close link
    async fn close(&mut self) -> Result<(), AUTDInternalError>;
    /// Send data to devices
    async fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError>;
    /// Receive data from devices
    async fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError>;
    /// Check if link is open
    #[must_use]
    fn is_open(&self) -> bool;
    /// Get timeout
    #[must_use]
    fn timeout(&self) -> Duration;
    /// Send and receive data
    async fn send_receive(
        &mut self,
        tx: &TxDatagram,
        rx: &mut [RxMessage],
        timeout: Option<Duration>,
    ) -> Result<bool, AUTDInternalError> {
        let timeout = timeout.unwrap_or(self.timeout());
        if !self.send(tx).await? {
            return Ok(false);
        }
        if timeout.is_zero() {
            return self.receive(rx).await;
        }
        self.wait_msg_processed(tx, rx, timeout).await
    }

    /// Wait until message is processed
    async fn wait_msg_processed(
        &mut self,
        tx: &TxDatagram,
        rx: &mut [RxMessage],
        timeout: Duration,
    ) -> Result<bool, AUTDInternalError> {
        let start = std::time::Instant::now();
        let _ = self.receive(rx).await?;
        if tx.headers().zip(rx.iter()).all(|(h, r)| h.msg_id == r.ack) {
            return Ok(true);
        }
        loop {
            if start.elapsed() > timeout {
                return Ok(false);
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
            if !self.receive(rx).await? {
                continue;
            }
            if tx.headers().zip(rx.iter()).all(|(h, r)| h.msg_id == r.ack) {
                return Ok(true);
            }
        }
    }
}

#[async_trait]
pub trait LinkBuilder<T: Transducer> {
    type L: Link;

    /// Open link
    async fn open(self, geometry: &Geometry<T>) -> Result<Self::L, AUTDInternalError>;
}

#[async_trait]
impl Link for Box<dyn Link> {
    #[cfg_attr(coverage_nightly, coverage(off))]
    async fn close(&mut self) -> Result<(), AUTDInternalError> {
        self.as_mut().close().await
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    async fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        self.as_mut().send(tx).await
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    async fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError> {
        self.as_mut().receive(rx).await
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn is_open(&self) -> bool {
        self.as_ref().is_open()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn timeout(&self) -> Duration {
        self.as_ref().timeout()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    async fn send_receive(
        &mut self,
        tx: &TxDatagram,
        rx: &mut [RxMessage],
        timeout: Option<Duration>,
    ) -> Result<bool, AUTDInternalError> {
        self.as_mut().send_receive(tx, rx, timeout).await
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    async fn wait_msg_processed(
        &mut self,
        tx: &TxDatagram,
        rx: &mut [RxMessage],
        timeout: Duration,
    ) -> Result<bool, AUTDInternalError> {
        self.as_mut().wait_msg_processed(tx, rx, timeout).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockLink {
        pub is_open: bool,
        pub timeout: Duration,
        pub send_cnt: usize,
        pub recv_cnt: usize,
        pub down: bool,
    }

    #[async_trait]
    impl Link for MockLink {
        #[cfg_attr(coverage_nightly, coverage(off))]
        async fn close(&mut self) -> Result<(), AUTDInternalError> {
            self.is_open = false;
            Ok(())
        }

        async fn send(&mut self, _: &TxDatagram) -> Result<bool, AUTDInternalError> {
            if !self.is_open {
                return Err(AUTDInternalError::LinkClosed);
            }

            self.send_cnt += 1;
            Ok(!self.down)
        }

        async fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError> {
            if !self.is_open {
                return Err(AUTDInternalError::LinkClosed);
            }

            if self.recv_cnt > 10 {
                return Err(AUTDInternalError::LinkError("too many".to_owned()));
            }

            self.recv_cnt += 1;
            rx.iter_mut().for_each(|r| r.ack = self.recv_cnt as u8);

            Ok(!self.down)
        }

        #[cfg_attr(coverage_nightly, coverage(off))]
        fn is_open(&self) -> bool {
            self.is_open
        }

        fn timeout(&self) -> Duration {
            self.timeout
        }
    }

    #[tokio::test]
    async fn send_receive() {
        let mut link = MockLink {
            is_open: true,
            timeout: Duration::from_millis(0),
            send_cnt: 0,
            recv_cnt: 0,
            down: false,
        };

        let tx = TxDatagram::new(0);
        let mut rx = Vec::new();
        assert_eq!(link.send_receive(&tx, &mut rx, None).await, Ok(true));

        link.is_open = false;
        assert_eq!(
            link.send_receive(&tx, &mut rx, None).await,
            Err(AUTDInternalError::LinkClosed)
        );

        link.is_open = true;
        link.down = true;
        assert_eq!(link.send_receive(&tx, &mut rx, None).await, Ok(false));

        link.down = false;
        assert_eq!(
            link.send_receive(&tx, &mut rx, Some(Duration::from_millis(1)))
                .await,
            Ok(true)
        );
    }

    #[tokio::test]
    async fn wait_msg_processed() {
        let mut link = MockLink {
            is_open: true,
            timeout: Duration::from_millis(0),
            send_cnt: 0,
            recv_cnt: 0,
            down: false,
        };

        let mut tx = TxDatagram::new(1);
        tx.header_mut(0).msg_id = 2;
        let mut rx = vec![RxMessage { ack: 0, data: 0 }];
        assert_eq!(
            link.wait_msg_processed(&tx, &mut rx, Duration::from_millis(10))
                .await,
            Ok(true)
        );

        link.recv_cnt = 0;
        link.is_open = false;
        assert_eq!(
            link.wait_msg_processed(&tx, &mut rx, Duration::from_millis(10))
                .await,
            Err(AUTDInternalError::LinkClosed)
        );

        link.recv_cnt = 0;
        link.is_open = true;
        link.down = true;
        assert_eq!(
            link.wait_msg_processed(&tx, &mut rx, Duration::from_millis(10))
                .await,
            Ok(false)
        );

        link.down = false;
        link.recv_cnt = 0;
        tx.header_mut(0).msg_id = 20;
        assert_eq!(
            link.wait_msg_processed(&tx, &mut rx, Duration::from_secs(10))
                .await,
            Err(AUTDInternalError::LinkError("too many".to_owned()))
        );
    }
}
