# extern crate autd3;
# extern crate tokio;
# use autd3::prelude::*;
# 
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros()))
#    .open_with(autd3::link::Nop::builder()).await?;
let tr = &autd.geometry[0][0];
for tr in &autd.geometry[0] {
    // do something
}
# Ok(())
# }