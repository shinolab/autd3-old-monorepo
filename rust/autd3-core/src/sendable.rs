/*
 * File: sendable.rs
 * Project: src
 * Created Date: 06/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use crate::{error::AUTDInternalError, geometry::*};

#[cfg(not(feature = "dynamic"))]
mod internal {
    use super::*;

    pub trait Sendable<T: Transducer> {
        type H: autd3_driver::Operation;
        type B: autd3_driver::Operation;

        fn operation(self, geometry: &Geometry<T>)
            -> Result<(Self::H, Self::B), AUTDInternalError>;

        fn timeout() -> Option<Duration> {
            None
        }
    }

    impl<T: Transducer, B> Sendable<T> for Box<B>
    where
        B: Sendable<T>,
    {
        type H = B::H;
        type B = B::B;

        fn operation(
            self,
            geometry: &Geometry<T>,
        ) -> Result<(Self::H, Self::B), AUTDInternalError> {
            B::operation(*self, geometry)
        }
    }

    impl<T: Transducer, H, B> Sendable<T> for (H, B)
    where
        H: Sendable<T, B = autd3_driver::NullBody>,
        B: Sendable<T, H = autd3_driver::NullHeader>,
    {
        type H = H::H;
        type B = B::B;

        fn operation(
            self,
            geometry: &Geometry<T>,
        ) -> Result<(Self::H, Self::B), AUTDInternalError> {
            let (h, _) = self.0.operation(geometry)?;
            let (_, b) = self.1.operation(geometry)?;
            Ok((h, b))
        }
    }

    #[derive(Default)]
    pub struct NullHeader {}

    impl NullHeader {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl<T: Transducer> Sendable<T> for NullHeader {
        type H = autd3_driver::NullHeader;
        type B = autd3_driver::NullBody;

        fn operation(self, _: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
            Ok((Self::H::default(), Self::B::default()))
        }
    }

    #[derive(Default)]
    pub struct NullBody {}

    impl NullBody {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl<T: Transducer> Sendable<T> for NullBody {
        type H = autd3_driver::NullHeader;
        type B = autd3_driver::NullBody;

        fn operation(self, _: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
            Ok((Self::H::default(), Self::B::default()))
        }
    }
}

#[cfg(feature = "dynamic")]
mod internal {
    use autd3_driver::Operation;

    use super::*;

    pub trait Sendable {
        fn operation(
            &mut self,
            geometry: &Geometry<DynamicTransducer>,
        ) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError>;

        fn timeout(&self) -> Option<Duration> {
            None
        }
    }

    impl Sendable for (&mut Box<dyn Sendable>, &mut Box<dyn Sendable>) {
        fn operation(
            &mut self,
            geometry: &Geometry<DynamicTransducer>,
        ) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
            let (h, _) = self.0.operation(geometry)?;
            let (_, b) = self.1.operation(geometry)?;
            Ok((h, b))
        }
    }

    impl Sendable for (&mut Box<dyn Sendable>,) {
        fn operation(
            &mut self,
            geometry: &Geometry<DynamicTransducer>,
        ) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
            self.0.operation(geometry)
        }
    }

    #[derive(Default)]
    pub struct NullHeader {}

    impl NullHeader {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Sendable for NullHeader {
        fn operation(
            &mut self,
            _: &Geometry<DynamicTransducer>,
        ) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
            Ok((
                Box::new(autd3_driver::NullHeader::default()),
                Box::new(autd3_driver::NullBody::default()),
            ))
        }
    }

    #[derive(Default)]
    pub struct NullBody {}

    impl NullBody {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Sendable for NullBody {
        fn operation(
            &mut self,
            _: &Geometry<DynamicTransducer>,
        ) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
            Ok((
                Box::new(autd3_driver::NullHeader::default()),
                Box::new(autd3_driver::NullBody::default()),
            ))
        }
    }
}

pub use internal::*;
