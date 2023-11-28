# extern crate autd3;
# extern crate autd3_gain_holo;
# use autd3::prelude::*;
use autd3_gain_holo::{LinAlgBackend, NalgebraBackend, GSPAT, Pascal};

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let x1 = 0.;
# let y1 = 0.;
# let z1 = 0.;
# let x2 = 0.;
# let y2 = 0.;
# let z2 = 0.;
let backend = NalgebraBackend::new()?;
let g = GSPAT::new(backend)
      .add_focus(Vector3::new(x1, y1, z1), 5e3 * Pascal)
      .add_focus(Vector3::new(x2, y2, z2), 5e3 * Pascal);
# Ok(())
# }