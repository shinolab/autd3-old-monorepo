# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
# let x = 0.;
# let y = 0.;
# let z = 0.;
# let nx = 0.;
# let ny = 0.;
# let nz = 0.;
# let theta = 0.;
let g = autd3::gain::Bessel::new(
            Vector3::new(x, y, z), 
            Vector3::new(nx, ny, nz), 
            theta,
        );
# }