#![allow(non_snake_case)]

use autd3_core::{autd3_device::NUM_TRANS_IN_UNIT, link::Link, timer_strategy::TimerStrategy};
use autd3_link_soem::{SyncMode, SOEM};

use clap::{Args, Parser, Subcommand, ValueEnum};

use spdlog::Level;

use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum SyncModeArg {
    /// DC Sync
    DC,
    /// FreeRun mode
    FreeRun,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum TimerStrategyArg {
    /// use native timer
    NativeTimer,
    /// use sleep
    Sleep,
    /// use busy wait
    BusyWait,
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
    /// Interface name
    #[clap(short = 'i', long = "ifname", default_value = "")]
    ifname: String,
    /// Client port
    #[clap(short = 'p', long = "port")]
    port: u16,
    /// Sync0 cycle time in units of 500us
    #[clap(short = 's', long = "sync0", default_value = "2")]
    sync0: u16,
    /// Send cycle time in units of 500us
    #[clap(short = 'c', long = "send", default_value = "2")]
    send: u16,
    /// Buffer size
    #[clap(short = 'b', long = "buffer_size", default_value = "32")]
    buf_size: usize,
    /// Sync mode
    #[clap(short = 'm', long = "sync_mode", default_value = "free-run")]
    sync_mode: SyncModeArg,
    /// Timer strategy
    #[clap(short = 'w', long = "timer", default_value = "sleep")]
    timer_strategy: TimerStrategyArg,
    /// State check interval in ms
    #[clap(short = 'e', long = "state_check_interval", default_value = "500")]
    state_check_interval: u64,
    /// Timeout in ms
    #[clap(short = 't', long = "timeout", default_value = "20")]
    timeout: u64,
    /// Set debug mode
    #[clap(short = 'd', long = "debug")]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    Run(Arg),
    /// List available interfaces
    List,
}

fn run(soem: SOEM, port: u16) -> anyhow::Result<()> {
    spdlog::info!("Connecting SOEM server...");

    let mut soem = soem;
    let dev_num = soem.open_impl(&[])? as usize;

    spdlog::info!("{} AUTDs found", dev_num);

    let dev_map = vec![NUM_TRANS_IN_UNIT; dev_num];
    let mut tx = autd3_core::TxDatagram::new(&dev_map);
    let mut rx = autd3_core::RxDatagram::new(dev_num);

    let buf_size = 1 + tx.transmitting_size();
    let rx_buf_size = dev_num * std::mem::size_of::<autd3_core::RxMessage>();

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr)?;
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");
    spdlog::info!("Waiting for client connection on {}", port);

    let exit = Arc::new(AtomicBool::new(false));
    let exit_ = exit.clone();
    ctrlc::set_handler(move || exit_.store(true, Ordering::Release))
        .expect("Error setting Ctrl-C handler");
    spdlog::info!("Press Ctrl+C to exit...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut socket) => {
                spdlog::info!("Connected to client: {}", socket.peer_addr()?);
                let mut buf = vec![0x00; buf_size];
                let mut rx_buf = vec![0x00; rx_buf_size];
                loop {
                    let len = match socket.read(&mut buf) {
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            if exit.load(Ordering::Acquire) {
                                break;
                            }
                            continue;
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                        Ok(len) => len,
                    };
                    if len == 0 {
                        spdlog::info!("Client disconnected");
                        spdlog::info!("Waiting for client connection on {}", port);
                        break;
                    }
                    match buf[0] {
                        1 => {
                            let len = len - 1;
                            let header_size = std::mem::size_of::<autd3_core::GlobalHeader>();
                            if len > header_size {
                                let body_size = std::mem::size_of::<u16>() * NUM_TRANS_IN_UNIT;
                                if (len - header_size) % body_size != 0 {
                                    spdlog::warn!("Invalid message size: {}", len);
                                    continue;
                                }
                                let body_len = len - header_size;
                                tx.num_bodies = body_len / body_size;
                                unsafe {
                                    std::ptr::copy_nonoverlapping(
                                        buf[1 + header_size..].as_ptr(),
                                        tx.body_raw_mut().as_mut_ptr() as *mut u8,
                                        body_len,
                                    );
                                }
                            } else {
                                tx.num_bodies = 0;
                            }
                            unsafe {
                                std::ptr::copy_nonoverlapping(
                                    buf[1..].as_ptr(),
                                    tx.header_mut() as *mut _ as *mut u8,
                                    header_size,
                                );
                            }
                            Link::<autd3_core::geometry::LegacyTransducer>::send(&mut soem, &tx)?;
                        }
                        2 => {
                            Link::<autd3_core::geometry::LegacyTransducer>::receive(
                                &mut soem, &mut rx,
                            )?;
                            unsafe {
                                std::ptr::copy_nonoverlapping(
                                    rx.messages().as_ptr() as *const u8,
                                    rx_buf.as_mut_ptr(),
                                    rx_buf_size,
                                );
                            }
                            let _ = socket.write(&rx_buf)?;
                        }
                        _ => {}
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                if exit.load(Ordering::Acquire) {
                    spdlog::info!("Shutting down server...");

                    Link::<autd3_core::geometry::LegacyTransducer>::close(&mut soem)?;

                    spdlog::info!("Shutting down server...done");

                    break;
                }
                continue;
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}

fn main_() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List => {
            println!("Available interfaces:");
            let adapters = autd3_link_soem::EthernetAdapters::new();
            let name_len = adapters
                .into_iter()
                .map(|adapter| adapter.name().len())
                .max()
                .unwrap_or(0);
            for adapter in &adapters {
                println!("\t{:name_len$}\t{}", adapter.name(), adapter.desc());
            }
        }
        Commands::Run(args) => {
            let port = args.port;
            let ifname = args.ifname.to_string();
            let sync0_cycle = args.sync0;
            let send_cycle = args.send;
            let state_check_interval = args.state_check_interval;
            let sync_mode = match args.sync_mode {
                SyncModeArg::DC => SyncMode::DC,
                SyncModeArg::FreeRun => SyncMode::FreeRun,
            };
            let timer_strategy = match args.timer_strategy {
                TimerStrategyArg::NativeTimer => TimerStrategy::NativeTimer,
                TimerStrategyArg::Sleep => TimerStrategy::Sleep,
                TimerStrategyArg::BusyWait => TimerStrategy::BusyWait,
            };
            let buf_size = args.buf_size;
            let level = if args.debug {
                Level::Debug
            } else {
                Level::Info
            };
            let timeout = args.timeout;

            let soem = autd3_link_soem::SOEM::new()
                .with_buf_size(buf_size)
                .with_ifname(ifname)
                .with_log_level(spdlog::LevelFilter::MoreSevereEqual(level))
                .with_send_cycle(send_cycle)
                .with_state_check_interval(std::time::Duration::from_millis(state_check_interval))
                .with_sync0_cycle(sync0_cycle)
                .with_timer_strategy(timer_strategy)
                .with_sync_mode(sync_mode)
                .with_timeout(std::time::Duration::from_millis(timeout))
                .with_on_lost(|msg| {
                    spdlog::error!("{}", msg);
                    std::process::exit(-1);
                });

            run(soem, port)?;
        }
    }

    Ok(())
}

fn main() {
    match main_() {
        Ok(_) => {}
        Err(e) => {
            spdlog::error!("{}", e);
            std::process::exit(-1);
        }
    }
}
