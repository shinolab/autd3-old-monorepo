# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let stm = GainSTM::new(1.0).with_mode(GainSTMMode::PhaseFull);
# let stm = stm.add_gain(Null::new())?;
# Ok(())
# }