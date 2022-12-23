/*
 * File: main.rs
 * Project: src
 * Created Date: 24/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

mod csharp;
mod julia;
mod matlab;
mod nim;
mod python;
mod traits;

use std::{fs::File, io::BufWriter, path::Path};

use anyhow::Result;

use capi_header_parser::{cmake, parse::parse};
use csharp::CSharpGenerator;
use julia::JuliaGenerator;
use matlab::MatlabGenerator;
use nim::NimGenerator;
use python::PythonGenerator;
use traits::Generator;

fn gen<G: Generator, P: AsRef<Path>>(path: P, capi_path: P, use_single: bool) -> Result<()> {
    std::fs::create_dir_all(path.as_ref())?;
    let projects = cmake::glob_projects(capi_path, &["autd3capi-backend-blas".to_string()])?;
    for proj in projects {
        let mut writer = BufWriter::new(File::create(
            path.as_ref().join(G::get_filename(proj.name())),
        )?);
        G::print_header(&mut writer, proj.name())?;
        for func in parse(proj.header(), use_single)? {
            G::register_func(&mut writer, &func)?;
        }
        G::print_footer(&mut writer)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    gen::<PythonGenerator, _>("../../python/pyautd3/native_methods", "../../capi", false)?;
    gen::<CSharpGenerator, _>("../../cs/src/NativeMethods", "../../capi", false)?;
    gen::<CSharpGenerator, _>(
        "../../cs/unity/Assets/autd3/Scripts/NativeMethods",
        "../../capi",
        true,
    )?;
    gen::<MatlabGenerator, _>("../../matlab/bin", "../../capi", false)?;
    gen::<NimGenerator, _>("../../nim/src/autd3/native_methods", "../../capi", false)?;
    gen::<JuliaGenerator, _>("../../julia/src/NativeMethods", "../../capi", false)?;

    Ok(())
}
