# extern crate autd3;
# use autd3::prelude::*;
use autd3::modulation::Fourier;

# #[allow(unused_variables)]
# fn main()  {
let m = Fourier::from(Sine::new(100.))
        .add_component(Sine::new(150.))
        .add_components_from_iter([Sine::new(200.)]);
# }