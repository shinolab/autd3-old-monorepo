/*
 * File: main.rs
 * Project: src
 * Created Date: 26/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use cargo_license::{get_dependencies_from_cargo_lock, GetDependenciesOpt};
use cargo_metadata::MetadataCommand;
use license_file::Package;

mod diff;
mod license_file;
mod npm;

fn check<P>(
    path: P,
    filename: &str,
    license_file_map: &[Package],
    additional_deps: &[(&str, &str)],
) -> anyhow::Result<()>
where
    P: Into<PathBuf>,
{
    let mut cmd = MetadataCommand::new();

    let path: PathBuf = path.into();
    cmd.manifest_path(&path);

    let get_opts = GetDependenciesOpt {
        avoid_dev_deps: false,
        avoid_build_deps: false,
        direct_deps_only: false,
        root_only: false,
    };

    let dependencies = get_dependencies_from_cargo_lock(cmd, get_opts)?;

    let mut licenses = std::collections::HashSet::new();
    for dependency in &dependencies {
        match (
            dependency.license.as_ref(),
            dependency.license_file.as_ref(),
        ) {
            (None, None) => panic!("no license found for {}", dependency.name),
            (_, Some(_)) => {}
            (Some(license), _) => {
                licenses.insert(license.to_owned());
            }
        }
    }
    licenses.extend(
        additional_deps
            .iter()
            .map(|(license, _)| license.to_string()),
    );

    let old = path.parent().unwrap().join(format!("{}.txt", filename));
    let new = path.parent().unwrap().join(format!("{}-new.txt", filename));

    let mut writer = BufWriter::new(File::create(&new)?);

    writeln!(writer, "THIRD-PARTY SOFTWARE NOTICES AND INFORMATION")?;
    writeln!(writer)?;
    writeln!(
        writer,
        "This software includes the following third-party components."
    )?;
    writeln!(
        writer,
        "The license terms for each of these components are provided later in this notice."
    )?;
    writeln!(writer)?;
    writeln!(writer)?;

    for dependency in dependencies {
        writeln!(
            writer,
            "---------------------------------------------------------"
        )?;
        writeln!(writer)?;

        writeln!(
            writer,
            "{} {}{}",
            dependency.name,
            dependency.version,
            dependency
                .license
                .map(|license| format!(" ({})", license))
                .unwrap_or_default()
        )?;
        if let Some(rep) = dependency.repository {
            writeln!(writer, "{}", rep)?;
        }

        if dependency.license_file.is_some() {
            let license_file_content = license_file_map
                .iter()
                .find(|p| p.name == dependency.name)
                .ok_or(anyhow::anyhow!(
                    "license file not found for {}",
                    dependency.name
                ))?
                .license_file_content
                .to_owned();

            writeln!(writer)?;
            writeln!(writer, "---")?;
            writeln!(writer)?;

            writeln!(writer, "{}", license_file_content)?;
        }
    }

    for additionnal_dep in additional_deps {
        writeln!(
            writer,
            "---------------------------------------------------------"
        )?;
        writeln!(writer)?;
        writeln!(writer, "{}", additionnal_dep.1)?;
    }

    writeln!(writer)?;

    for entry in glob::glob(concat!(env!("CARGO_MANIFEST_DIR"), "/licenses/*.txt"))? {
        let entry = entry?;
        let path = Path::new(&entry);
        let name = path.file_stem().unwrap().to_str().unwrap();

        if !licenses.iter().any(|license| license.contains(name)) {
            continue;
        }

        writeln!(
            writer,
            "---------------------------------------------------------"
        )?;
        writeln!(writer)?;
        writeln!(writer, "{}", name)?;
        writeln!(writer)?;
        writeln!(writer, "---")?;
        writeln!(writer)?;

        let mut file_content = String::new();
        fs::File::open(path)
            .map(BufReader::new)?
            .read_to_string(&mut file_content)?;
        writer.write_all(file_content.as_bytes())?;
    }
    writeln!(
        writer,
        "---------------------------------------------------------"
    )?;

    writer.flush()?;

    if old.exists() {
        let mut old_license = String::new();
        fs::File::open(&old)
            .map(BufReader::new)?
            .read_to_string(&mut old_license)?;
        let old_license = old_license.replace("\r", "");

        let mut new_license = String::new();
        fs::File::open(&new)
            .map(BufReader::new)?
            .read_to_string(&mut new_license)?;
        let new_license = new_license.replace("\r", "");

        if diff::show_diff(&old_license, &new_license) {
            return Err(anyhow::anyhow!(
                "ThirdPartyNotice.txt is not up to date. Please check {} manually.",
                new.canonicalize().unwrap().to_str().unwrap()
            ));
        }

        fs::remove_file(&old)?;
    }

    std::fs::rename(new, old)?;

    Ok(())
}

fn check_npm<P>(path: P, filename: &str) -> anyhow::Result<()>
where
    P: Into<PathBuf>,
{
    let path: PathBuf = path.into();

    let dependencies = npm::glob_node_modules(&path)?;

    let mut licenses = std::collections::HashSet::new();
    for dependency in &dependencies {
        licenses.insert(dependency.license.to_owned());
    }

    let old = path.parent().unwrap().join(format!("{}.txt", filename));
    let new = path.parent().unwrap().join(format!("{}-new.txt", filename));

    let mut writer = BufWriter::new(File::create(&new)?);

    writeln!(writer, "THIRD-PARTY SOFTWARE NOTICES AND INFORMATION")?;
    writeln!(writer)?;
    writeln!(
        writer,
        "This software includes the following third-party components."
    )?;
    writeln!(
        writer,
        "The license terms for each of these components are provided later in this notice."
    )?;
    writeln!(writer)?;
    writeln!(writer)?;

    for dependency in dependencies {
        writeln!(
            writer,
            "---------------------------------------------------------"
        )?;
        writeln!(writer)?;

        writeln!(
            writer,
            "{} {} {}",
            dependency.name, dependency.version, dependency.license
        )?;
        writeln!(writer, "{}", dependency.repository)?;
    }

    writeln!(writer)?;

    for entry in glob::glob(concat!(env!("CARGO_MANIFEST_DIR"), "/licenses/*.txt"))? {
        let entry = entry?;
        let path = Path::new(&entry);
        let name = path.file_stem().unwrap().to_str().unwrap();

        if !licenses.iter().any(|license| license.contains(name)) {
            continue;
        }

        writeln!(
            writer,
            "---------------------------------------------------------"
        )?;
        writeln!(writer)?;
        writeln!(writer, "{}", name)?;
        writeln!(writer)?;
        writeln!(writer, "---")?;
        writeln!(writer)?;

        let mut file_content = String::new();
        fs::File::open(path)
            .map(BufReader::new)?
            .read_to_string(&mut file_content)?;
        writer.write_all(file_content.as_bytes())?;
    }
    writeln!(
        writer,
        "---------------------------------------------------------"
    )?;

    writer.flush()?;

    if old.exists() {
        let mut old_license = String::new();
        fs::File::open(&old)
            .map(BufReader::new)?
            .read_to_string(&mut old_license)?;
        let old_license = old_license.replace("\r", "");

        let mut new_license = String::new();
        fs::File::open(&new)
            .map(BufReader::new)?
            .read_to_string(&mut new_license)?;
        let new_license = new_license.replace("\r", "");

        if diff::show_diff(&old_license, &new_license) {
            return Err(anyhow::anyhow!(
                "ThirdPartyNotice.txt is not up to date. Please check {} manually.",
                new.canonicalize().unwrap().to_str().unwrap()
            ));
        }

        fs::remove_file(&old)?;
    }

    std::fs::rename(new, old)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let license_file_map = license_file::load_license_file_map()?;

    let imgui_vulkano_render = (
        "MIT",
        r"imgui-vulkano-renderer 0.9.0 (MIT)
https://github.com/Tenebryo/imgui-vulkano-renderer
Modification of the original version by Shun Suzuki <suzuki@hapis.k.u-tokyo.ac.jp>",
    );
    let imgui_winit_support = (
        "MIT",
        r"imgui-winit-support 0.11.0 (MIT)
https://github.com/imgui-rs/imgui-rs",
    );
    let sdr_rs_dep = (
        "MIT",
        r"sdr 0.7.0 (MIT)
https://github.com/adamgreig/sdr-rs
Modification of the original version by Shun Suzuki <suzuki@hapis.k.u-tokyo.ac.jp>",
    );
    let notofont_dep = ("OFL", "Noto Sans v2.012 (OFL)");

    check(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../capi/Cargo.toml"),
        "ThirdPartyNotice",
        &license_file_map,
        &[
            imgui_vulkano_render,
            imgui_winit_support,
            sdr_rs_dep,
            notofont_dep,
        ],
    )?;

    check(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../server/simulator/Cargo.toml"),
        "ThirdPartyNotice",
        &license_file_map,
        &[
            imgui_vulkano_render,
            imgui_winit_support,
            sdr_rs_dep,
            notofont_dep,
        ],
    )?;

    check(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../server/SOEMAUTDServer/Cargo.toml"),
        "ThirdPartyNotice",
        &license_file_map,
        &[sdr_rs_dep],
    )?;

    check(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../server/LightweightTwinCATAUTDServer/Cargo.toml"),
        "ThirdPartyNotice",
        &license_file_map,
        &[sdr_rs_dep],
    )?;

    check(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../server/src-tauri/Cargo.toml"),
        "ThirdPartyNotice",
        &license_file_map,
        &[sdr_rs_dep],
    )?;

    check_npm(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../server/node_modules"),
        "ThirdPartyNotice",
    )?;

    Ok(())
}
