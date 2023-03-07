use autd3::prelude::*;
use autd3_link_soem::{Config, SOEM};

use anyhow::Result;

fn main() -> Result<()> {
    let mut geometry = GeometryBuilder::new().build();
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

    let mut clear = Clear::new();
    autd.timeout(std::time::Duration::from_millis(20))
        .send(&mut clear)
        .flush()?;
    let mut sync = Synchronize::new();
    autd.timeout(std::time::Duration::from_millis(20))
        .send(&mut sync)
        .flush()?;

    autd.firmware_infos().unwrap().iter().for_each(|firm_info| {
        println!("{}", firm_info);
    });

    let mut silencer = SilencerConfig::default();
    autd.timeout(std::time::Duration::from_millis(20))
        .send(&mut silencer)
        .flush()?;

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0);
    let mut g = Focus::new(center);
    let mut m = Sine::new(150);

    autd.timeout(std::time::Duration::from_millis(20))
        .send(&mut m)
        .send(&mut g)?;

    let mut _s = String::new();
    std::io::stdin().read_line(&mut _s)?;

    autd.close()?;

    Ok(())
}
