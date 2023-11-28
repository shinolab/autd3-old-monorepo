# extern crate autd3;
# extern crate tokio;
# use autd3::prelude::*;
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().add_device(AUTD3::new(Vector3::zeros())).open_with(autd3::link::Nop::builder()).await?;
let center = autd.geometry.center() + Vector3::new(0., 0., 150.0 * MILLIMETER);
let point_num = 200;
let radius = 30.0 * MILLIMETER;
let stm = FocusSTM::new(1.0).add_foci_from_iter((0..point_num).map(|i| {
    let theta = 2.0 * PI * i as float / point_num as float;
    let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);
    center + p
}))?;
# Ok(())
# }