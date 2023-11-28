# extern crate autd3;
# extern crate tokio;
# extern crate autd3_link_soem;
# use autd3::prelude::*;
use autd3_link_soem::RemoteSOEM;

# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros()))
#            .open_with(
RemoteSOEM::builder("172.16.99.104:8080".parse()?)
# ).await?;
# Ok(())
# }