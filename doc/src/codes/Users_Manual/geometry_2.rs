# extern crate autd3;
# extern crate tokio;
# use autd3::prelude::*;
# 
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::zeros()))
    .add_device(
        AUTD3::new(Vector3::new(0., 0., AUTD3::DEVICE_WIDTH))
            .with_rotation(EulerAngle::ZYZ(0. * Rad, PI/2.0 * Rad, 0. * Rad)))
#    .open_with(autd3::link::Nop::builder()).await?;
# Ok(())
# }