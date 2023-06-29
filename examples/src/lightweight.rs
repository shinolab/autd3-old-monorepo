/*
 * File: lightweight.rs
 * Project: src
 * Created Date: 29/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;

use autd3_protobuf_parser as pb;

use pb::ToMessage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client =
        pb::ecat_light_client::EcatLightClient::connect("http://127.0.0.1:8080".to_string())
            .await?;

    client
        .config_geomety(pb::Geometry {
            geometries: vec![pb::geometry::Autd3 {
                position: Some(Vector3::zeros().to_msg()),
                rotation: Some(UnitQuaternion::identity().to_msg()),
            }],
        })
        .await?;

    client.send_special(Clear::new().to_msg()).await?;
    client.send_special(Synchronize::new().to_msg()).await?;

    client.send_header(Sine::new(150).to_msg()).await?;
    client
        .send_body(Focus::new(Vector3::new(90., 70., 150.)).to_msg())
        .await?;

    println!("Press enter to exit...");
    let mut i = String::new();
    std::io::stdin().read_line(&mut i)?;

    client.close(pb::CloseRequest {}).await?;

    Ok(())
}
