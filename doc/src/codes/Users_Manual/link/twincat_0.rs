# extern crate autd3;
# extern crate tokio;
# extern crate autd3_link_twincat;
# use autd3::prelude::*;
use autd3_link_twincat::TwinCAT;

# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros()))
#     .open_with(
TwinCAT::builder()
# ).await?;
# Ok(())
# }