/*
 * File: build.rs
 * Project: autd3-link-soem
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

#[cfg(feature = "remote")]
fn main() {
    match std::process::Command::new("git")
        .args([
            "submodule",
            "update",
            "--init",
            "--recursive",
            "--",
            "./3rdparty/ADS",
        ])
        .output()
    {
        Ok(r) => {
            if !r.status.success() {
                eprintln!(
                    "Failed to update submodule: {}",
                    String::from_utf8_lossy(&r.stderr)
                );
            } else {
                println!("Submodule updated");
            }
        }
        Err(e) => {
            eprintln!("Failed to update submodule: {}", e);
        }
    }

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=wsock32");
        println!("cargo:rustc-link-lib=ws2_32");
    }

    let mut build = cc::Build::new();
    if cfg!(target_os = "windows") {
        build.flag("/DWIN32").flag("/DNOMINMAX");
    }
    build
        .define("CONFIG_DEFAULT_LOGLEVEL", "1")
        .warnings(true)
        .cpp(true);

    if cfg!(target_os = "macos") {
        build.flag("-std=c++17");
    }

    build
        .file("ads_c.cpp")
        .file("3rdparty/ADS/AdsLib/AdsDef.cpp")
        .file("3rdparty/ADS/AdsLib/AdsDevice.cpp")
        .file("3rdparty/ADS/AdsLib/AdsFile.cpp")
        .file("3rdparty/ADS/AdsLib/AdsLib.cpp")
        .file("3rdparty/ADS/AdsLib/Frame.cpp")
        .file("3rdparty/ADS/AdsLib/LicenseAccess.cpp")
        .file("3rdparty/ADS/AdsLib/Log.cpp")
        .file("3rdparty/ADS/AdsLib/RouterAccess.cpp")
        .file("3rdparty/ADS/AdsLib/RTimeAccess.cpp")
        .file("3rdparty/ADS/AdsLib/Sockets.cpp")
        .file("3rdparty/ADS/AdsLib/standalone/AdsLib.cpp")
        .file("3rdparty/ADS/AdsLib/standalone/AmsConnection.cpp")
        .file("3rdparty/ADS/AdsLib/standalone/AmsNetId.cpp")
        .file("3rdparty/ADS/AdsLib/standalone/AmsPort.cpp")
        .file("3rdparty/ADS/AdsLib/standalone/AmsRouter.cpp")
        .file("3rdparty/ADS/AdsLib/standalone/NotificationDispatcher.cpp");

    build
        .include("3rdparty/ADS/AdsLib")
        .include("3rdparty/ADS/AdsLib/standalone")
        .compile("ads");
}

#[cfg(not(feature = "remote"))]
fn main() {}
