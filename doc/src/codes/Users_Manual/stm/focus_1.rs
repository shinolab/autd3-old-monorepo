# extern crate autd3;
# use autd3::prelude::*;
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let stm = FocusSTM::from_sampling_config(SamplingConfiguration::from_frequency(1.0)?);
# Ok(())
# }