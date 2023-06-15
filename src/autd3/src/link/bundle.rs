/*
 * File: bundle.rs
 * Project: link
 * Created Date: 14/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{marker::PhantomData, time::Duration};

use autd3_core::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::Link,
    RxDatagram, TxDatagram,
};

pub struct Bundle<T: Transducer, L1: Link<T>, L2: Link<T>> {
    main_link: L1,
    sub_link: L2,
    _t: PhantomData<T>,
}

impl<T: Transducer, L1: Link<T>, L2: Link<T>> Bundle<T, L1, L2> {
    pub fn new(main_link: L1, sub_link: L2) -> Self {
        Self {
            main_link,
            sub_link,
            _t: PhantomData,
        }
    }
}

impl<T: Transducer, L1: Link<T>, L2: Link<T>> Link<T> for Bundle<T, L1, L2> {
    fn open(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        if self.is_open() {
            return Ok(());
        }

        self.main_link.open(geometry)?;
        self.sub_link.open(geometry)?;

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
