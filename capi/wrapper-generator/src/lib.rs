/*
 * File: lib.rs
 * Project: src
 * Created Date: 10/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod csharp;
mod parse;
mod python;
mod traits;
mod types;

use std::path::Path;

use anyhow::Result;

use csharp::CSharpGenerator;
use parse::{parse_const, parse_enum, parse_func, parse_struct};
use python::PythonGenerator;
use traits::Generator;

fn gen<G: Generator, P1: AsRef<Path>, P2: AsRef<Path>>(
    path: P1,
    crate_path: P2,
    use_single: bool,
) -> Result<()> {
    std::fs::create_dir_all(path.as_ref())?;

    let crate_name = crate_path.as_ref().file_name().unwrap().to_str().unwrap();

    glob::glob(&format!(
        "{}/**/*.rs",
        crate_path.as_ref().join("src").display()
    ))?
    .try_fold(G::new(), |acc, path| -> Result<_> {
        let path = path?;
        Ok(acc
            .register_func(parse_func(&path, use_single)?)
            .register_const(parse_const(&path, use_single)?)
            .register_enum(parse_enum(&path, use_single)?)
            .register_struct(parse_struct(&path, use_single)?))
    })?
    .write(path, crate_name)
}

pub fn generate<P: AsRef<Path>>(crate_path: P) -> Result<()> {
    gen::<PythonGenerator, _, _>("../../python/pyautd3/native_methods", &crate_path, false)?;
    gen::<CSharpGenerator, _, _>("../../dotnet/cs/src/NativeMethods", &crate_path, false)?;
    gen::<CSharpGenerator, _, _>(
        "../../dotnet/unity/Assets/Scripts/NativeMethods",
        &crate_path,
        true,
    )?;
    gen::<CSharpGenerator, _, _>(
        "../../dotnet/unity-linux/Assets/Scripts/NativeMethods",
        &crate_path,
        true,
    )?;
    if crate_path.as_ref().file_name().unwrap().to_str().unwrap() != "autd3capi-backend-cuda" {
        gen::<CSharpGenerator, _, _>(
            "../../dotnet/unity-mac/Assets/Scripts/NativeMethods",
            &crate_path,
            true,
        )?;
    }

    let out_file = Path::new("../../cpp/include/autd3/internal/native_methods").join(format!(
        "{}.h",
        crate_path.as_ref().file_name().unwrap().to_str().unwrap()
    ));
    let mut config = cbindgen::Config::default();
    config.language = cbindgen::Language::Cxx;
    config.pragma_once = true;
    config.autogen_warning = Some(
        "/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */"
            .to_string(),
    );
    config.namespace = Some("autd3::internal::native_methods".to_string());
    config.no_includes = true;
    config.sys_includes = vec!["cstdint".to_string()];
    config.sort_by = cbindgen::SortKey::None;
    config.usize_is_size_t = true;
    config.export = cbindgen::ExportConfig {
        include: vec![
            "TransMode".to_string(),
            "Level".to_string(),
            "TimerStrategy".to_string(),
            "GainSTMMode".to_string(),
            "ControllerPtr".to_string(),
            "ConstraintPtr".to_string(),
            "DevicePtr".to_string(),
            "TransducerPtr".to_string(),
            "GeometryPtr".to_string(),
            "ModulationPtr".to_string(),
            "GainPtr".to_string(),
            "LinkPtr".to_string(),
            "DatagramPtr".to_string(),
            "DatagramSpecialPtr".to_string(),
            "STMPropsPtr".to_string(),
            "BackendPtr".to_string(),
            "GroupGainMapPtr".to_string(),
            "GroupKVMapPtr".to_string(),
        ],
        rename: vec![
            ("float".to_string(), "double".to_string()),
            ("ConstPtr".to_string(), "void*".to_string()),
        ]
        .into_iter()
        .collect(),
        ..Default::default()
    };
    config.function = cbindgen::FunctionConfig {
        sort_by: None,
        must_use: Some("[[nodiscard]]".to_string()),
        ..Default::default()
    };
    config.constant = cbindgen::ConstantConfig {
        allow_static_const: false,
        allow_constexpr: true,
        sort_by: Some(cbindgen::SortKey::None),
    };

    cbindgen::Builder::new()
        .with_crate(crate_path)
        .with_config(config)
        .generate()?
        .write_to_file(out_file);

    Ok(())
}
