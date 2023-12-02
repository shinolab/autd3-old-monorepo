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
let idx = tr.idx();
let position = tr.position();
let rotation = tr.rotation();
let x_dir = tr.x_direction();
let y_dir = tr.y_direction();
let z_dir = tr.z_direction();

let sound_speed = autd.geometry[0].sound_speed;
let wavelen = tr.wavelength(sound_speed);
let wavenum = tr.wavenumber(sound_speed);
# Ok(())
# }