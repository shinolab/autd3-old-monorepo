use autd3::prelude::*;
use autd3_link_soem::{Config, SOEM};

fn main() {
    let mut geometry = GeometryBuilder::new().legacy_mode().build();
    geometry.add_device(Vector3::zeros(), Vector3::zeros());

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

    autd.clear().unwrap();

    autd.synchronize().unwrap();

    println!("***** Firmware information *****");
    autd.firmware_infos().unwrap().iter().for_each(|firm_info| {
        println!("{}", firm_info);
    });
    println!("********************************");

    let silencer_config = SilencerConfig::default();
    autd.config_silencer(silencer_config).unwrap();

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0);

    let mut g = Focus::new(center);
    let mut m = Sine::new(150);

    autd.send(&mut m).send(&mut g).unwrap();

    let mut _s = String::new();
    io::stdin().read_line(&mut _s).unwrap();

    autd.close().unwrap();
}
