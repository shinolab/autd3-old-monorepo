# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Sine::new(150.)
            .with_intensity(EmitIntensity::MAX)
            .with_offset(EmitIntensity::MAX / 2);
# }