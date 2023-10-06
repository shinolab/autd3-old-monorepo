/*
 * File: link.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use crate::{
    cpu::{RxMessage, TxDatagram},
    derive::prelude::Geometry,
    error::AUTDInternalError,
    geometry::Transducer,
};

/// Link is a interface to the AUTD device
pub trait Link: Send {
    /// Close link
    fn close(&mut self) -> Result<(), AUTDInternalError>;
    /// Send data to devices
    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError>;
    /// Receive data from devices
    fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError>;
    /// Check if link is open
    #[must_use]
    fn is_open(&self) -> bool;
    /// Get timeout
    #[must_use]
    fn timeout(&self) -> Duration;
    /// Send and receive data
    fn send_receive(
        &mut self,
        tx: &TxDatagram,
        rx: &mut [RxMessage],
        timeout: Option<Duration>,
    ) -> Result<bool, AUTDInternalError> {
        let timeout = timeout.unwrap_or(self.timeout());
        if !self.send(tx)? {
            return Ok(false);
        }
        if timeout.is_zero() {
            return self.receive(rx);
        }
        self.wait_msg_processed(tx, rx, timeout)
    }

    /// Wait until message is processed
    fn wait_msg_processed(
        &mut self,
        tx: &TxDatagram,
        rx: &mut [RxMessage],
        timeout: Duration,
    ) -> Result<bool, AUTDInternalError> {
        let start = std::time::Instant::now();
        let _ = self.receive(rx)?;
        if tx.headers().zip(rx.iter()).all(|(h, r)| h.msg_id == r.ack) {
            return Ok(true);
        }
        loop {
            if start.elapsed() > timeout {
                return Ok(false);
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
            if !self.receive(rx)? {
                continue;
            }
            if tx.headers().zip(rx.iter()).all(|(h, r)| h.msg_id == r.ack) {
                return Ok(true);
            }
        }
    }
}

pub trait LinkBuilder<T: Transducer> {
    type L: Link;

    /// Open link
    fn open(self, geometry: &Geometry<T>) -> Result<Self::L, AUTDInternalError>;
}

impl Link for Box<dyn Link> {
    #[cfg_attr(coverage_nightly, no_coverage)]
    fn close(&mut self) -> Result<(), AUTDInternalError> {
        self.as_mut().close()
    }

    #[cfg_attr(coverage_nightly, no_coverage)]
    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        self.as_mut().send(tx)
    }

    #[cfg_attr(coverage_nightly, no_coverage)]
    fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError> {
        self.as_mut().receive(rx)
    }

    #[cfg_attr(coverage_nightly, no_coverage)]
    fn is_open(&self) -> bool {
        self.as_ref().is_open()
    }

    #[cfg_attr(coverage_nightly, no_coverage)]
    fn timeout(&self) -> Duration {
        self.as_ref().timeout()
    }

    #[cfg_attr(coverage_nightly, no_coverage)]
    fn send_receive(
        &mut self,
        tx: &TxDatagram,
        rx: &mut [RxMessage],
        timeout: Option<Duration>,
    ) -> Result<bool, AUTDInternalError> {
        self.as_mut().send_receive(tx, rx, timeout)
    }

    #[cfg_attr(coverage_nightly, no_coverage)]
    fn wait_msg_processed(
        &mut self,
        tx: &TxDatagram,
        rx: &mut [RxMessage],
        timeout: Duration,
    ) -> Result<bool, AUTDInternalError> {
        self.as_mut().wait_msg_processed(tx, rx, timeout)
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

    impl Link for MockLink {
        #[cfg_attr(coverage_nightly, no_coverage)]
        fn close(&mut self) -> Result<(), AUTDInternalError> {
            self.is_open = false;
            Ok(())
        }

        fn send(&mut self, _: &TxDatagram) -> Result<bool, AUTDInternalError> {
            if !self.is_open {
                return Err(AUTDInternalError::LinkClosed);
            }

            self.send_cnt += 1;
            Ok(!self.down)
        }

        fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError> {
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

        #[cfg_attr(coverage_nightly, no_coverage)]
        fn is_open(&self) -> bool {
            self.is_open
        }

        fn timeout(&self) -> Duration {
            self.timeout
        }
    }

    #[test]
    fn send_receive() {
        let mut link = MockLink {
            is_open: true,
            timeout: Duration::from_millis(0),
            send_cnt: 0,
            recv_cnt: 0,
            down: false,
        };

        let tx = TxDatagram::new(0);
        let mut rx = Vec::new();
        assert_eq!(link.send_receive(&tx, &mut rx, None), Ok(true));

        link.is_open = false;
        assert_eq!(
            link.send_receive(&tx, &mut rx, None),
            Err(AUTDInternalError::LinkClosed)
        );

        link.is_open = true;
        link.down = true;
        assert_eq!(link.send_receive(&tx, &mut rx, None), Ok(false));

        link.down = false;
        assert_eq!(
            link.send_receive(&tx, &mut rx, Some(Duration::from_millis(1))),
            Ok(true)
        );
    }

    #[test]
    fn wait_msg_processed() {
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
            link.wait_msg_processed(&tx, &mut rx, Duration::from_millis(10)),
            Ok(true)
        );

        link.recv_cnt = 0;
        link.is_open = false;
        assert_eq!(
            link.wait_msg_processed(&tx, &mut rx, Duration::from_millis(10)),
            Err(AUTDInternalError::LinkClosed)
        );

        link.recv_cnt = 0;
        link.is_open = true;
        link.down = true;
        assert_eq!(
            link.wait_msg_processed(&tx, &mut rx, Duration::from_millis(10)),
            Ok(false)
        );

        link.down = false;
        link.recv_cnt = 0;
        tx.header_mut(0).msg_id = 20;
        assert_eq!(
            link.wait_msg_processed(&tx, &mut rx, Duration::from_secs(10)),
            Err(AUTDInternalError::LinkError("too many".to_owned()))
        );
    }
}
