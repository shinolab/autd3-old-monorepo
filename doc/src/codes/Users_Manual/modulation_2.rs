# extern crate autd3;
# extern crate tokio;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Nop::builder()).await?;
autd.send(ConfigureModDelay::new(|dev, tr| {
    if dev.idx() == 0 && tr.idx() == 0 {
        1
    } else {
        0
    }
})).await?;
# Ok(())
# }