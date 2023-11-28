# extern crate autd3;
# extern crate tokio;
# use autd3::prelude::*;
# 
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let autd = 
Controller::builder()
    .add_device(AUTD3::new(Vector3::new(-AUTD3::DEVICE_WIDTH, 0., 0.)))
    .add_device(AUTD3::new(Vector3::zeros()))
#    .open_with(autd3::link::Nop::builder()).await?;
# Ok(())
# }
