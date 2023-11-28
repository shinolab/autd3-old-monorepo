# extern crate autd3;
# extern crate tokio;
# use autd3::prelude::*;
# 
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros()))
#    .open_with(autd3::link::Nop::builder()).await?;
let num_dev = autd.geometry.num_devices();
let num_tr = autd.geometry.num_transducers();
let center = autd.geometry.center();
# Ok(())
# }