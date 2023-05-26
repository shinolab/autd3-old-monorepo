/*
 * File: dynamic_modulation.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::core::modulation::Modulation;

use crate::DynamicDatagram;

pub trait DynamicModulation: DynamicDatagram {
    fn modulation(&self) -> &dyn Modulation;
    fn modulation_mut(&mut self) -> &mut dyn Modulation;
}

pub struct ModulationWrap {
    pub modulation: Box<dyn Modulation>,
}

impl ModulationWrap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<M: 'static + Modulation>(m: M) -> Box<Box<dyn DynamicModulation>> {
        Box::new(Box::new(Self {
            modulation: Box::new(m),
        }))
    }
}

impl DynamicModulation for ModulationWrap {
    fn modulation(&self) -> &dyn Modulation {
        &*self.modulation
    }

    fn modulation_mut(&mut self) -> &mut dyn Modulation {
        &mut *self.modulation
    }
}

impl DynamicDatagram for ModulationWrap {
    fn operation(
        &mut self,
        _: crate::dynamic_transducer::TransMode,
        _: &autd3::prelude::Geometry<crate::DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3::core::Operation>,
            Box<dyn autd3::core::Operation>,
        ),
        autd3::core::error::AUTDInternalError,
    > {
        let freq_div = self.modulation.sampling_frequency_division();
        Ok((
            Box::new(autd3::core::Modulation::new(
                self.modulation.calc()?,
                freq_div,
            )),
            Box::<autd3::core::NullBody>::default(),
        ))
    }

    fn timeout(&self) -> Option<std::time::Duration> {
        None
    }
}
