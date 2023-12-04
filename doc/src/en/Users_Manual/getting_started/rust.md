# Rust tutorial

First, make a new project and add `autd3` and `autd3-link-soem` libraries as dependencies.

```shell
cargo new --bin autd3-sample
cd autd3-sample
cargo add autd3
cargo add autd3-link-soem
cargo add tokio --features full
```

Next, edit `src/main.rs` file as follows.
This is the source code for generating a focus with $\SI{150}{Hz}$ AM modulation. 

```rust,should_panic,filename=main.rs,edition2021
# extern crate autd3;
# extern crate tokio;
# extern crate autd3_link_soem;
use autd3::prelude::*;
use autd3_link_soem::SOEM;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Make a controller to control AUTD
    let mut autd = Controller::builder()
        // Configure the devices
        // The argument of AUTD3::new is position
        // Here, the device is placed at the origin
        .add_device(AUTD3::new(Vector3::zeros()))
        // Open controller with SOEM link
        // The callback specified by with_on_lost is called when SOEM loses the device
        .open_with(SOEM::builder().with_on_lost(|msg| {
            eprintln!("Unrecoverable error occurred: {msg}");
            std::process::exit(-1);
        })).await?;

    // Check firmware version
    // This code assumes that the version is v4.0.x
    autd.firmware_infos().await?.iter().for_each(|firm_info| {
        println!("{}", firm_info);
    });

    // Enable silencer
    // Note that this is enabled by default, so it is not actually necessary
    // To disable, send Silencer::disable()
    autd.send(Silencer::default()).await?;

    // A focus at 150mm directly above the center of the device
    let center = autd.geometry.center() + Vector3::new(0., 0., 150.0 * MILLIMETER);
    let g = Focus::new(center);

    // 150Hz sine wave modulation
    let m = Sine::new(150.);

    // Send data
    autd.send((m, g)).await?;

    println!("press enter to quit...");
    let mut _s = String::new();
    std::io::stdin().read_line(&mut _s)?;

    // Close controller
    autd.close().await?;

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
