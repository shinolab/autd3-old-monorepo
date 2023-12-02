# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let m = autd3::modulation::Sine::new(150.)
            .with_sampling_config(SamplingConfiguration::from_frequency(4e3)?);
# Ok(())
# } 