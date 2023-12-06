# extern crate autd3;
# extern crate tokio;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().add_device(AUTD3::new(Vector3::zeros())).open_with(autd3::link::Nop::builder()).await?;
autd.send(ConfigureForceFan::new(|_dev| true)).await?;
# Ok(())
# }