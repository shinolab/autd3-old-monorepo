# extern crate autd3;
# extern crate tokio;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Nop::builder()).await?;
# let m = Static::new();
# let g = Null::new();
autd.send((m, g).with_timeout(std::time::Duration::from_millis(20))).await?;
# Ok(())
# }