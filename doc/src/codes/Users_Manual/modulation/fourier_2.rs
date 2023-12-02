# extern crate autd3;
# use autd3::prelude::*;
use autd3::modulation::Fourier;

# #[allow(unused_variables)]
# fn main()  {
let m: Fourier = Sine::new(100.) + Sine::new(150.).with_phase(PI / 2.0);
# }