/*
 * File: build.rs
 * Project: autd3-link-soem
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
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

#[cfg(target_os = "windows")]
fn main() {
    let home_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-lib=winmm");
    println!("cargo:rustc-link-lib=ws2_32");
    println!("cargo:rustc-link-search={home_dir}\\3rdparty\\SOEM\\oshw\\win32\\wpcap\\Lib\\x64");
    println!("cargo:rustc-link-lib=Packet");
    println!("cargo:rustc-link-lib=wpcap");

    let mut build = cc::Build::new();
    build.flag("/DWIN32").warnings(true).cpp(false);
    add!("3rdparty/SOEM/soem/*.c", path, build.file(path));
    add!("3rdparty/SOEM/osal/win32/*.c", path, build.file(path));
    add!("3rdparty/SOEM/oshw/win32/*.c", path, build.file(path));
    build
        .include("3rdparty/SOEM/soem")
        .include("3rdparty/SOEM/osal")
        .include("3rdparty/SOEM/osal/win32")
        .include("3rdparty/SOEM/oshw/win32")
        .include("3rdparty/SOEM/oshw/win32/wpcap/Include")
        .include("3rdparty/SOEM/oshw/win32/wpcap/Include/pcap")
        .compile("soem");

    let bindings = bindgen::Builder::default()
        .clang_arg("-DWIN32")
        .clang_arg("-I3rdparty/SOEM/soem")
        .clang_arg("-I3rdparty/SOEM/osal")
        .clang_arg("-I3rdparty/SOEM/osal/win32")
        .clang_arg("-I3rdparty/SOEM/oshw/win32")
        .clang_arg("-I3rdparty/SOEM/oshw/win32/wpcap/Include")
        .clang_arg("-I3rdparty/SOEM/oshw/win32/wpcap/Include/pcap")
        .header("3rdparty/SOEM/osal/win32/osal_defs.h")
        .header("3rdparty/SOEM/osal/win32/osal_win32.h")
        .header("3rdparty/SOEM/soem/ethercattype.h")
        .header("3rdparty/SOEM/oshw/win32/nicdrv.h")
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
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("soem_bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=pcap");

    let mut build = cc::Build::new();
    build.warnings(true).cpp(false);
    add!("3rdparty/SOEM/soem/*.c", path, build.file(path));
    add!("3rdparty/SOEM/osal/macosx/*.c", path, build.file(path));
    add!("3rdparty/SOEM/oshw/macosx/*.c", path, build.file(path));
    build
        .include("3rdparty/SOEM/soem")
        .include("3rdparty/SOEM/osal")
        .include("3rdparty/SOEM/osal/macosx")
        .include("3rdparty/SOEM/oshw/macosx")
        .compile("soem");

    let bindings = bindgen::Builder::default()
        .clang_arg("-I3rdparty/SOEM/soem")
        .clang_arg("-I3rdparty/SOEM/osal")
        .clang_arg("-I3rdparty/SOEM/osal/macosx")
        .clang_arg("-I3rdparty/SOEM/oshw/macosx")
        .header("3rdparty/SOEM/osal/macosx/osal_defs.h")
        .header("3rdparty/SOEM/soem/ethercattype.h")
        .header("3rdparty/SOEM/oshw/macosx/nicdrv.h")
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
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("soem_bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(target_os = "linux")]
fn main() {
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=rt");

    let mut build = cc::Build::new();
    build.warnings(true).cpp(false);
    add!("3rdparty/SOEM/soem/*.c", path, build.file(path));
    add!("3rdparty/SOEM/osal/linux/*.c", path, build.file(path));
    add!("3rdparty/SOEM/oshw/linux/*.c", path, build.file(path));
    build
        .include("3rdparty/SOEM/soem")
        .include("3rdparty/SOEM/osal")
        .include("3rdparty/SOEM/osal/linux")
        .include("3rdparty/SOEM/oshw/linux")
        .compile("soem");

    let bindings = bindgen::Builder::default()
        .clang_arg("-I3rdparty/SOEM/soem")
        .clang_arg("-I3rdparty/SOEM/osal")
        .clang_arg("-I3rdparty/SOEM/osal/linux")
        .clang_arg("-I3rdparty/SOEM/oshw/linux")
        .header("3rdparty/SOEM/osal/linux/osal_defs.h")
        .header("3rdparty/SOEM/soem/ethercattype.h")
        .header("3rdparty/SOEM/oshw/linux/nicdrv.h")
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
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("soem_bindings.rs"))
        .expect("Couldn't write bindings!");
}
