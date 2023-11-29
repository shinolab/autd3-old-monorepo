# extern crate autd3;
# extern crate tokio;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Nop::builder()).await?;
autd.geometry[0][0].mod_delay = 1;
autd.send(ConfigureModDelay::new()).await?;
# Ok(())
# }