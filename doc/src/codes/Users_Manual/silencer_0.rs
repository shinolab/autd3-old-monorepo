# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let step_intensity = 256;
# let step_phase = 256;
let config = Silencer::default();
let config = Silencer::new(step_intensity, step_phase);
let config = Silencer::disable();
# Ok(())
# }