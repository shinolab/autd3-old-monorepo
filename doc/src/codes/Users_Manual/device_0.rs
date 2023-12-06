# extern crate autd3;
# extern crate tokio;
# use autd3::prelude::*;
# 
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros()))
#    .open_with(autd3::link::Nop::builder()).await?;
let mut dev = &mut autd.geometry[0];
let idx = dev.idx();
dev.enable = false;
dev.sound_speed = 340e3;
dev.set_sound_speed_from_temp(15.);
dev.attenuation = 0.;
let t = Vector3::new(1., 0., 0.);
let r = UnitQuaternion::from_quaternion(Quaternion::new(1., 0., 0., 0.));
dev.translate(t);
dev.rotate(r);
dev.affine(t, r);
# Ok(())
# }