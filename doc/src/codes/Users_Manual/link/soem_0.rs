# extern crate autd3;
# extern crate tokio;
# extern crate autd3_link_soem;
use autd3::prelude::*;
use autd3_link_soem::{SOEM, SyncMode};

# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros()))
#            .open_with(
SOEM::builder()
    .with_ifname("")
    .with_buf_size(32)
    .with_on_err(|msg| {
        eprintln!("Unrecoverable error occurred: {msg}");
    })
    .with_state_check_interval(std::time::Duration::from_millis(100))
    .with_on_lost(|msg| {
        eprintln!("Unrecoverable error occurred: {msg}");
        std::process::exit(-1);
    })
    .with_sync0_cycle(2)
    .with_send_cycle(2)
    .with_timer_strategy(TimerStrategy::BusyWait)
    .with_sync_mode(SyncMode::DC)
# ).await?;
# Ok(())
# }