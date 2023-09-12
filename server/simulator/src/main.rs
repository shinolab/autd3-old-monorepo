/*
 * File: main.rs
 * Project: src
 * Created Date: 28/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/08/2023
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

use simulator::{Simulator, ViewerSettings};

use clap::{Args, Parser, Subcommand};

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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(
    help_template = "Author: {author-with-newline} {about-section}Version: {version} \n\n {usage-heading} {usage} \n\n {all-args} {tab}"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args)]
struct Arg {
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

    /// Config file path
    #[arg(long = "config_path")]
    config_path: Option<String>,

    /// Setting file name
    #[arg(short = 's', long = "setting", default_value = "settings.json")]
    setting: String,

    /// Debug mode
    #[arg(short = 'd', long = "debug", default_value = "false")]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run simulator
    Run(Arg),
    /// List available GPUs
    List,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List => {
            simulator::available_gpus()?
                .iter()
                .for_each(|(idx, name, ty)| {
                    println!("\t{}: {} (type {:?})", idx, name, ty);
                });
        }
        Commands::Run(arg) => {
            let port = arg.port;
            let gpu_idx = arg.index;
            let window_size = arg.window_size;
            let settings_path = if let Some(path) = &arg.config_path {
                Path::new(path).join(&arg.setting)
            } else {
                Path::new(&arg.setting).to_owned()
            };
            let vsync = arg.vsync;
            let debug = arg.debug;

            if debug {
                spdlog::default_logger()
                    .set_level_filter(spdlog::LevelFilter::MoreSevereEqual(spdlog::Level::Debug));
            }

            let settings: ViewerSettings = if settings_path.exists() {
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

            if let Some(path) = &arg.config_path {
                simulator = simulator.with_config_path(path);
            }

            simulator.run();

            {
                let settings = simulator.get_settings();

                let settings_str = serde_json::to_string_pretty(settings)?;

                if settings_path.exists() {
                    fs::remove_file(&settings_path).unwrap();
                }

                std::fs::create_dir_all(settings_path.parent().unwrap())?;

                let mut file = OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .append(false)
                    .open(&settings_path)?;

                write!(file, "{}", settings_str)?;
            }
        }
    }

    Ok(())
}
