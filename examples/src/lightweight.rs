/*
 * File: lightweight.rs
 * Project: src
 * Created Date: 29/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;

use autd3_protobuf::LightweightClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = LightweightClient::builder()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .open("127.0.0.1:8080".parse()?)
        .await?;

    println!("*********************************** Firmware information ****************************************");
    client.firmware_infos().await?.iter().for_each(|firm_info| {
        println!("{}", firm_info);
    });
    println!("*************************************************************************************************");

    client.send(Sine::new(150)).await?;
    client
        .send(Focus::new(Vector3::new(90., 70., 150.)))
        .await?;

    println!("Press enter to exit...");
    let mut i = String::new();
    std::io::stdin().read_line(&mut i)?;

    client.close().await?;

    Ok(())
}
