# extern crate autd3;
# extern crate tokio;
# extern crate autd3_link_twincat;
# use autd3::prelude::*;
use autd3_link_twincat::RemoteTwinCAT;

# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros()))
#      .open_with(
RemoteTwinCAT::builder("172.16.99.111.1.1")
            .with_server_ip("172.16.99.104")
            .with_client_ams_net_id("172.16.99.62.1.1")
# ).await?;
# Ok(())
# }