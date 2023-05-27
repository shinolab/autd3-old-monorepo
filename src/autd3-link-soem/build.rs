/*
 * File: build.rs
 * Project: autd3-link-soem
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::env;
use std::path::PathBuf;

macro_rules! add {
    ($path:expr, $p:ident, $work: expr) => {
        for entry in glob::glob($path).unwrap() {
            match entry {
                Ok($p) => {
                    $work;
                }
                Err(e) => println!("{:?}", e),
            }
        }
    };
}

#[cfg(feature = "local")]
fn main() {
    match std::process::Command::new("git")
        .args([
            "submodule",
            "update",
            "--init",
            "--recursive",
            "--",
            "./3rdparty/SOEM",
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

    let home_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let os = if cfg!(target_os = "windows") {
        "win32"
    } else if cfg!(target_os = "macos") {
        "macosx"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        panic!("Unsupported OS");
    };

    let mut build = cc::Build::new();
    build.cpp(false);
    add!("3rdparty/SOEM/soem/*.c", path, build.file(path));
    add!(
        &format!("3rdparty/SOEM/osal/{}/*.c", os),
        path,
        build.file(path)
    );
    add!(
        &format!("3rdparty/SOEM/oshw/{}/*.c", os),
        path,
        build.file(path)
    );
    build
        .include("3rdparty/SOEM/soem")
        .include("3rdparty/SOEM/osal")
        .include(format!("3rdparty/SOEM/osal/{}", os))
        .include(format!("3rdparty/SOEM/oshw/{}", os));
    #[cfg(target_os = "windows")]
    {
        build
            .include("3rdparty/SOEM/oshw/win32/wpcap/Include")
            .include("3rdparty/SOEM/oshw/win32/wpcap/Include/pcap")
            .flag("/DWIN32");
    }
    build.compile("soem");

    let mut bindings = bindgen::Builder::default()
        .clang_arg("-I3rdparty/SOEM/soem")
        .clang_arg("-I3rdparty/SOEM/osal")
        .clang_arg(format!("-I3rdparty/SOEM/osal/{}", os))
        .clang_arg(format!("-I3rdparty/SOEM/oshw/{}", os));
    #[cfg(target_os = "windows")]
    {
        bindings = bindings
            .clang_arg("-I3rdparty/SOEM/oshw/win32/wpcap/Include")
            .clang_arg("-I3rdparty/SOEM/oshw/win32/wpcap/Include/pcap")
            .clang_arg("-DWIN32")
            .header("3rdparty/SOEM/osal/win32/osal_win32.h")
    }

    bindings = bindings
        .header(format!("3rdparty/SOEM/osal/{}/osal_defs.h", os))
        .header("3rdparty/SOEM/soem/ethercattype.h")
        .header(format!("3rdparty/SOEM/oshw/{}/nicdrv.h", os))
        .header("3rdparty/SOEM/soem/ethercatmain.h")
        .header("3rdparty/SOEM/soem/ethercatdc.h")
        .header("3rdparty/SOEM/soem/ethercatconfig.h")
        .header("3rdparty/SOEM/soem/ethercatprint.h")
        .allowlist_function("ec_init")
        .allowlist_function("ec_find_adapters")
        .allowlist_function("ec_free_adapters")
        .allowlist_function("ec_send_processdata")
        .allowlist_function("ec_receive_processdata")
        .allowlist_function("ec_config_init")
        .allowlist_function("ec_config_map")
        .allowlist_function("ec_dcsync0")
        .allowlist_function("ec_configdc")
        .allowlist_function("ec_writestate")
        .allowlist_function("ec_statecheck")
        .allowlist_function("ec_close")
        .allowlist_function("ec_readstate")
        .allowlist_function("ec_reconfig_slave")
        .allowlist_function("ec_recover_slave")
        .allowlist_function("ec_ALstatuscode2string")
        .allowlist_var("ec_slave")
        .allowlist_var("ec_group")
        .allowlist_var("ec_DCtime")
        .allowlist_var("ecx_context")
        .allowlist_var("ec_slavecount")
        .allowlist_var("EC_TIMEOUTSTATE")
        .allowlist_var("EC_TIMEOUTRET")
        .allowlist_type("ec_state")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=ws2_32");
        if cfg!(target_arch = "arm") || cfg!(target_arch = "aarch64") {
            println!("cargo:rustc-link-search={home_dir}\\Lib\\ARM64");
        } else {
            println!(
                "cargo:rustc-link-search={home_dir}\\3rdparty\\SOEM\\oshw\\win32\\wpcap\\Lib\\x64"
            );
        }
        println!("cargo:rustc-link-lib=Packet");
        println!("cargo:rustc-link-lib=wpcap");
    }
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=pcap");
    }
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=rt");
    }

    let bindings = bindings.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("soem_bindings.rs"))
        .expect("Couldn't write bindings!");
}
