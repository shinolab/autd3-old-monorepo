# extern crate autd3;
use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() {
let m = Sine::new(150.).with_transform(|i, d| EmitIntensity::new(d.value() / 2));
# }