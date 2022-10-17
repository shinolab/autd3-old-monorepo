# Rust

[Rust-autd](https://github.com/shinolab/autd3/tree/master/rust) provides a Rust version of the library.

The Rust version of the library is not a wrapping of the C++ version, but a re-implementation in Rust.
Therefore, some features may be different.

## Installation

The Rust version is available at [crate.io](https://crates.io/crates/autd3), so you can install as follows.

```
[dependencies]
autd3 = "2.4.3"
```

Also, you can add links, gains, etc. to dependencies as needed, since they are available as separate crates.

```
[dependencies]
autd3-link-soem = "2.4.3"
autd3-link-twincat = "2.4.3"
autd3-link-simulator = "2.4.3"
autd3-gain-holo = "2.4.3"
```

## Usage

Basically, this is designed to be the same as the C++ version.

For example, the following code is equivalent to [Getting Started](../Users_Manual/getting_started.md).

```rust
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

    autd.check_trials = 50;

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
```

Note that the Rust version of the `send` function takes only one argument. 
If you want to send header and body data at the same time, chain `send`; otherwise, call `flush`.
```rust
    autd.send(&mut m).flush().unwrap();
```

See [rust-autd example](https://github.com/shinolab/autd3/tree/master/rust/autd3-examples) for a more detailed sample.

## Troubleshooting

If you have any questions, please send them to [GitHub issue](https://github.com/shinolab/autd3/issues).
