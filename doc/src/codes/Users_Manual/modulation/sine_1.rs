# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Sine::new(150)
            .with_amp(1.)
            .with_offset(0.5);
# }