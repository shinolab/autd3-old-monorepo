/*
 * File: main.rs
 * Project: autd-server
 * Created Date: 27/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(non_snake_case)]

mod log_formatter;

use log_formatter::LogFormatter;

use autd3_driver::{
    cpu::TxDatagram,
    link::{Link, LinkBuilder},
    timer_strategy::TimerStrategy,
};
use autd3_link_soem::{SyncMode, SOEM};
use autd3_protobuf::*;

use clap::{Args, Parser, Subcommand, ValueEnum};

use tokio::{
    runtime::Runtime,
    sync::{mpsc, RwLock},
};
use tonic::{transport::Server, Request, Response, Status};

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
    sync0: u64,
    /// Send cycle time in units of 500us
    #[clap(short = 'c', long = "send", default_value = "2")]
    send: u64,
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
}

#[derive(Subcommand)]
enum Commands {
    Run(Arg),
    /// List available interfaces
    List,
}

struct SOEMServer {
    num_dev: usize,
    soem: RwLock<SOEM>,
}

#[tonic::async_trait]
impl ecat_server::Ecat for SOEMServer {
    async fn send_data(
        &self,
        request: Request<TxRawData>,
    ) -> Result<Response<SendResponse>, Status> {
        let tx = TxDatagram::from_msg(&request.into_inner());
        Ok(Response::new(SendResponse {
            success: Link::send(&mut *self.soem.write().await, &tx)
                .await
                .unwrap_or(false),
        }))
    }

    async fn read_data(&self, _: Request<ReadRequest>) -> Result<Response<RxMessage>, Status> {
        let mut rx = vec![autd3_driver::cpu::RxMessage { ack: 0, data: 0 }; self.num_dev];
        Link::receive(&mut *self.soem.write().await, &mut rx)
            .await
            .unwrap_or(false);
        Ok(Response::new(rx.to_msg()))
    }

    async fn close(&self, _: Request<CloseRequest>) -> Result<Response<CloseResponse>, Status> {
        self.soem.write().await.clear_iomap();
        Ok(Response::new(CloseResponse { success: true }))
    }
}

async fn main_() -> anyhow::Result<()> {
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
            adapters.into_iter().for_each(|adapter| {
                println!("\t{:name_len$}\t{}", adapter.name(), adapter.desc());
            });
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
            let timeout = args.timeout;
            let f = move || -> autd3_link_soem::local::link_soem::SOEMBuilder {
                autd3_link_soem::SOEM::builder()
                    .with_buf_size(buf_size)
                    .with_ifname(ifname.clone())
                    .with_send_cycle(send_cycle)
                    .with_state_check_interval(std::time::Duration::from_millis(
                        state_check_interval,
                    ))
                    .with_sync0_cycle(sync0_cycle)
                    .with_timer_strategy(timer_strategy)
                    .with_sync_mode(sync_mode)
                    .with_timeout(std::time::Duration::from_millis(timeout))
                    .with_on_lost(|msg| {
                        tracing::error!("{}", msg);
                        std::process::exit(-1);
                    })
            };
            let (tx, mut rx) = mpsc::channel(1);
            ctrlc::set_handler(move || {
                let rt = Runtime::new().expect("failed to obtain a new Runtime object");
                rt.block_on(tx.send(())).unwrap();
            })
            .expect("Error setting Ctrl-C handler");

            let addr = format!("0.0.0.0:{}", port).parse()?;
            tracing::info!("Waiting for client connection on {}", addr);
            let rt = Runtime::new().expect("failed to obtain a new Runtime object");

            tracing::info!("Starting SOEM server...");

            let soem = f()
                .open(&autd3_driver::geometry::Geometry::new(vec![]))
                .await?;
            let num_dev = SOEM::num_devices();

            tracing::info!("{} AUTDs found", num_dev);

            let server_future = Server::builder()
                .add_service(ecat_server::EcatServer::new(SOEMServer {
                    num_dev,
                    soem: RwLock::new(soem),
                }))
                .serve_with_shutdown(addr, async {
                    let _ = rx.recv().await;
                });
            rt.block_on(server_future)?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().event_format(LogFormatter).init();

    match main_().await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("{}", e);
            std::process::exit(-1);
        }
    }
}
