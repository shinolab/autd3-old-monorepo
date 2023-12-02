# extern crate autd3;
# extern crate autd3_driver;
use std::collections::HashMap;
use autd3::{
    derive::Gain,
    prelude::*,
};
use autd3_driver::derive::prelude::*;

#[derive(Gain)]
pub struct FocalPoint {
    position: Vector3,
}

impl FocalPoint {
    pub fn new(position: Vector3) -> Self {
        Self {position}
    }
}

impl Gain for FocalPoint {
    fn calc(&self, geometry: &Geometry, filter: GainFilter) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(Self::transform(geometry, filter, |dev, tr| Drive {
            phase: Phase::from_rad((tr.position() - self.position).norm() * tr.wavelength(dev.sound_speed)),
            intensity: EmitIntensity::MAX,
        }))
    }
}
# fn main() { 
# }
#