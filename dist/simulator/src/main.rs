/*
 * File: main.rs
 * Project: src
 * Created Date: 28/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::{BufReader, Write},
    path::Path,
};

use autd3_simulator::{Simulator, ViewerSettings};

use clap::Parser;

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find(',')
        .ok_or_else(|| format!("no `,` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(
    help_template = "Author: {author-with-newline} {about-section}Version: {version} \n\n {usage-heading} {usage} \n\n {all-args} {tab}"
)]
struct Args {
    /// Windows Size (Optional, if set, overrides settings from file)
    #[arg(short = 'w', long = "window_size", value_name = "Width,Height" , value_parser = parse_key_val::<u32, u32>)]
    window_size: Option<(u32, u32)>,

    /// Port (Optional, if set, overrides settings from file)
    #[arg(short = 'p', long = "port")]
    port: Option<u16>,

    /// Vsync (Optional, if set, overrides settings from file)
    #[arg(short = 'v', long = "vsync")]
    vsync: Option<bool>,

    /// GPU index (Optional, if set, overrides settings from file.)
    #[arg(short = 'g', long = "gpu_idx")]
    index: Option<i32>,

    /// Setting file path
    #[arg(short = 's', long = "setting", default_value = "settings.json")]
    setting: String,

    /// Debug mode
    #[arg(short = 'd', long = "debug", default_value = "false")]
    debug: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Args::parse();

    let port = cli.port;
    let gpu_idx = cli.index;
    let window_size = cli.window_size;
    let settings_path = cli.setting;
    let vsync = cli.vsync;
    let debug = cli.debug;

    if debug {
        spdlog::default_logger()
            .set_level_filter(spdlog::LevelFilter::MoreSevereEqual(spdlog::Level::Debug));
    }

    let settings: ViewerSettings = if Path::new(&settings_path).exists() {
        let file = File::open(&settings_path)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)?
    } else {
        Default::default()
    };

    let mut simulator = Simulator::new().with_settings(settings);

    if let Some(port) = port {
        simulator = simulator.with_port(port);
    }

    if let Some(gpu_idx) = gpu_idx {
        simulator = simulator.with_gpu_idx(gpu_idx);
    }

    if let Some((width, height)) = window_size {
        simulator = simulator.with_window_size(width, height);
    }

    if let Some(vsync) = vsync {
        simulator = simulator.with_vsync(vsync);
    }

    simulator.run();

    {
        let settings = simulator.get_settings();

        let settings_str = serde_json::to_string_pretty(settings)?;

        if Path::new(&settings_path).exists() {
            fs::remove_file(&settings_path).unwrap();
        }

        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .append(false)
            .open(&settings_path)?;

        write!(file, "{}", settings_str)?;
    }

    Ok(())
}
