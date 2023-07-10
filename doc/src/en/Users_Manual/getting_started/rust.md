# Rust tutorial


First, make a new project and add `autd3` and `autd3-link-soem` libraries as dependencies.

```shell
cargo new --bin autd3-sample
cd autd3-sample
cargo add autd3
cargo add autd3-link-soem
```

Next, edit `src/main.rs` file as follows.
This is the source code for generating a focus with $\SI{150}{Hz}$ AM modulation. 

```rust,should_panic,filename=main.rs
use autd3::prelude::*;
use autd3_link_soem::SOEM;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Make a controller to control AUTD
    let mut autd = Controller::builder()
        // Configure the devices
        // The first argument of AUTD3::new is position, and the second argument is rotation
        // Here, the device is placed at the origin and does not rotate
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        // Open controller with SOEM link
        // The callback specified by with_on_lost is called when SOEM loses the device
        .open_with(SOEM::new().with_on_lost(|msg| {
            eprintln!("Unrecoverable error occurred: {msg}");
            std::process::exit(-1);
        }))?;

    // Check firmware version
    // This code assumes that the version is v2.9
    autd.firmware_infos()?.iter().for_each(|firm_info| {
        println!("{}", firm_info);
    });

    // Enable silencer
    // Note that this is enabled by default, so it is not actually necessary
    // To disable, send SilencerConfig::none()
    autd.send(SilencerConfig::default())?;

    // A focus at 150mm directly above the center of the device
    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);
    let g = Focus::new(center);

    // 150Hz sine wave modulation
    let m = Sine::new(150);


    // Send data
    autd.send((m, g))?;

    println!("press enter to quit...");
    let mut _s = String::new();
    std::io::stdin().read_line(&mut _s)?;

    // Close controller
    autd.close()?;

    Ok(())
}
```

Then, run the program.

```shell
cargo run --release
```

## For Linux, macOS users

You may need to run with administrator privileges when using SOEM on Linux or macOS.

```shell
cargo build --release && sudo ./target/release/autd3_sample
```
