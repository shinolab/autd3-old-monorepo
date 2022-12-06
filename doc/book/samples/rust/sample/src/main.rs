use autd3::prelude::*;
use autd3_link_soem::{Config, SOEM};

use anyhow::Result;

fn main() -> Result<()> {
    let mut geometry = GeometryBuilder::new().legacy_mode().build();
    geometry.add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))?;

    let config = Config {
        high_precision_timer: true,
        ..Config::default()
    };
    let link = SOEM::new(config, |msg| {
        eprintln!("unrecoverable error occurred: {}", msg);
        std::process::exit(-1);
    });

    let mut autd = Controller::open(geometry, link).expect("Failed to open");

    autd.ack_check_timeout = std::time::Duration::from_millis(20);

    let mut clear = Clear::new();
    autd.send(&mut clear).flush()?;

    let mut sync = Synchronize::new();
    autd.send(&mut sync).flush()?;

    autd.firmware_infos().unwrap().iter().for_each(|firm_info| {
        println!("{}", firm_info);
    });

    let mut silencer = SilencerConfig::default();
    autd.send(&mut silencer).flush()?;

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0);

    let mut g = Focus::new(center);
    let mut m = Sine::new(150);

    autd.send(&mut m).send(&mut g)?;

    let mut _s = String::new();
    std::io::stdin().read_line(&mut _s)?;

    autd.close()?;

    Ok(())
}
