# extern crate autd3;
# extern crate tokio;
# extern crate autd3_driver;
use autd3::{
    derive::Modulation,
};
use autd3_driver::derive::prelude::*;

#[derive(Modulation)]
pub struct Burst {
    config: SamplingConfiguration,
}

impl Burst {
    pub fn new() -> Self {
        Self { config: SamplingConfiguration::from_frequency(4e3).unwrap() }
    }
}

impl Modulation for Burst {
    fn calc(&self) -> Result<Vec<EmitIntensity>, AUTDInternalError> {
        Ok((0..4000)
            .map(|i| if i == 3999 { EmitIntensity::MAX } else { EmitIntensity::MIN })
            .collect())
    }
}
# fn main() { 
# }