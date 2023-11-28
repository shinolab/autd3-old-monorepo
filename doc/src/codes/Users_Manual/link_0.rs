# extern crate autd3;
# extern crate autd3_link_soem;
# use autd3::prelude::*;
# use autd3_link_soem::SOEM;
# #[allow(unused_variables)]
# fn main() {
let link = SOEM::builder().with_timeout(std::time::Duration::from_millis(20));
# }