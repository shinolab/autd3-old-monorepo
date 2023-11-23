/*
 * File: lib.rs
 * Project: src
 * Created Date: 10/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod parse;
mod python;
mod traits;
mod types;

use std::path::Path;

use anyhow::Result;

use convert_case::{Case, Casing};
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

fn generate_c<P: AsRef<Path>>(crate_path: P) -> Result<()> {
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
            "TimerStrategy".to_string(),
            "GainSTMMode".to_string(),
            "ControllerPtr".to_string(),
            "EmissionConstraintPtr".to_string(),
            "FirmwareInfoListPtr".to_string(),
            "GroupKVMapPtr".to_string(),
            "CachePtr".to_string(),
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
            "GainCalcDrivesMapPtr".to_string(),
            "LinkBuilderPtr".to_string(),
            "ResultI32".to_string(),
            "ResultModulation".to_string(),
            "ResultBackend".to_string(),
            "ResultController".to_string(),
            "ResultGainCalcDrivesMap".to_string(),
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

pub fn generate_cs<P1: AsRef<Path>, P2: AsRef<Path>>(
    path: P1,
    crate_path: P2,
    use_single: bool,
) -> Result<()> {
    let sub_abbr = |str: String| -> String {
        str.replace("Cuda", "CUDA")
            .replace("Blas", "BLAS")
            .replace("Twincat", "TwinCAT")
            .replace("Soem", "SOEM")
    };

    let to_pascal = |name: &str| -> String {
        let res = name.to_case(Case::Pascal);
        sub_abbr(res)
    };

    let to_class_name = |name: &str| {
        if name.split('-').count() == 1 {
            return "Base".to_string();
        }
        to_pascal(&name.replace("autd3capi-", ""))
    };

    let crate_name = crate_path.as_ref().file_name().unwrap().to_str().unwrap();
    let out_file = Path::new(path.as_ref()).join(format!("{}.cs", to_class_name(crate_name)));
    let dll_name = crate_name.replace('-', "_");
    let class_name = to_class_name(crate_name);

    glob::glob(&format!(
        "{}/**/*.rs",
        crate_path.as_ref().join("src").display()
    ))?
    .try_fold(csbindgen::Builder::default(), |acc, path| -> Result<_> {
        let path = path?;
        Ok(acc.input_extern_file(path))
    })?
    .csharp_dll_name(dll_name)
    .csharp_class_name(format!("NativeMethods{}", class_name))
    .csharp_namespace("AUTD3Sharp")
    .csharp_generate_const_filter(|_| true)
    .generate_csharp_file(&out_file)
    .map_err(|_| anyhow::anyhow!("failed to generate cs wrapper"))?;

    let content = std::fs::read_to_string(&out_file)?;
    let content = content.replace("@float", if use_single { "float" } else { "double" });
    let content = content.replace("ConstPtr", "IntPtr");
    let content = content.replace(
        "internal unsafe partial struct Drive",
        "public struct Drive",
    );
    let content = content.replace("public double phase;", "public double Phase;");
    let content = content.replace("public ushort amp;", "public ushort Amp;");

    let content = content.replace("internal enum CMap : byte", "public enum CMap : byte");
    let content = content.replace(
        "internal enum SyncMode : byte",
        "public enum SyncMode : byte",
    );

    let content = if use_single {
        let re = regex::Regex::new(r"internal const float (.*) = (.*);").unwrap();
        re.replace_all(&content, "internal const float $1 = ${2}f;")
            .to_string()
    } else {
        content
    };

    std::fs::write(&out_file, content)?;

    Ok(())
}

pub fn generate<P: AsRef<Path>>(crate_path: P) -> Result<()> {
    gen::<PythonGenerator, _, _>("../../python/pyautd3/native_methods", &crate_path, false)?;

    generate_c(&crate_path)?;
    generate_cs("../../dotnet/cs/src/NativeMethods", &crate_path, false)?;
    generate_cs(
        "../../dotnet/unity/Assets/Scripts/NativeMethods",
        &crate_path,
        true,
    )?;

    Ok(())
}
