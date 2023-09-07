/*
 * File: bundle.rs
 * Project: link
 * Created Date: 14/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{marker::PhantomData, time::Duration};

use autd3_driver::{
    cpu::{RxDatagram, TxDatagram},
    error::AUTDInternalError,
    geometry::{Device, Transducer},
    link::Link,
};

/// Link to bundle two links
pub struct Bundle<T: Transducer, L1: Link<T>, L2: Link<T>> {
    main_link: L1,
    sub_link: L2,
    _t: PhantomData<T>,
}

impl<T: Transducer, L1: Link<T>, L2: Link<T>> Bundle<T, L1, L2> {
    /// constructor
    ///
    /// Data is sent to both `main_link` and `sub_link`, but the received data is used from `main_link`. Also, the larger timeout of `main_link` and `sub_link` is used.
    ///
    /// # Arguments
    ///
    /// * `main_link` - main link
    /// * `sub_link` - sub link
    ///
    pub fn new(main_link: L1, sub_link: L2) -> Self {
        Self {
            main_link,
            sub_link,
            _t: PhantomData,
        }
    }

    pub fn link_main(&self) -> &L1 {
        &self.main_link
    }

    pub fn link_main_mut(&mut self) -> &mut L1 {
        &mut self.main_link
    }

    pub fn link_sub(&self) -> &L2 {
        &self.sub_link
    }

    pub fn link_sub_mut(&mut self) -> &mut L2 {
        &mut self.sub_link
    }
}

impl<T: Transducer, L1: Link<T>, L2: Link<T>> Link<T> for Bundle<T, L1, L2> {
    fn open(&mut self, devices: &[Device<T>]) -> Result<(), AUTDInternalError> {
        if self.is_open() {
            return Ok(());
        }

        self.main_link.open(devices)?;
        self.sub_link.open(devices)?;

        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        if !self.is_open() {
            return Ok(());
        }

        self.main_link.close()?;
        self.sub_link.close()?;

        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open() {
            return Ok(false);
        }

        self.main_link.send(tx)?;
        self.sub_link.send(tx)?;

        Ok(true)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open() {
            return Ok(false);
        }

        self.main_link.receive(rx)?;

        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.main_link.is_open() && self.sub_link.is_open()
    }

    fn timeout(&self) -> Duration {
        self.main_link.timeout().max(self.sub_link.timeout())
    }
}
