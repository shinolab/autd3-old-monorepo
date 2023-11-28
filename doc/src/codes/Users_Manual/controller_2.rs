# extern crate autd3;
# extern crate tokio;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().add_device(AUTD3::new(Vector3::zeros())).add_device(AUTD3::new(Vector3::zeros())).open_with(autd3::link::Nop::builder()).await?;
# let x = 0.;
# let y = 0.;
# let z = 0.;
autd.group(|dev| match dev.idx() {
        0 => Some("focus"),
        1 => Some("null"),
        _ => None,
    })
    .set("null", Null::new())?
    .set("focus", Focus::new(Vector3::new(x, y, z)))?
    .send().await?;
# Ok(())
# }