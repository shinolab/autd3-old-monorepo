# extern crate autd3;
# use autd3::prelude::*;
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let stm = FocusSTM::new_with_sampling_config(SamplingConfiguration::new_with_frequency(1.0)?);
# Ok(())
# }