/*
 * File: build.rs
 * Project: src
 * Created Date: 28/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

fn main() -> std::io::Result<()> {
    tonic_build::compile_protos("./proto/autd3.proto")?;
    Ok(())
}
