/*
 * File: main.rs
 * Project: src
 * Created Date: 24/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

mod csharp;
mod python;
mod traits;

use std::{fs::File, io::BufWriter, path::Path};

use anyhow::Result;

use capi_header_parser::{cmake, parse::parse};
use csharp::CSharpGenerator;
use python::PythonGenerator;
use traits::Generator;

fn gen<G: Generator, P: AsRef<Path>>(path: P, capi_path: P) -> Result<()> {
    std::fs::create_dir_all(path.as_ref())?;
    let projects = cmake::glob_projects(capi_path)?;
    for proj in projects {
        let mut writer = BufWriter::new(File::create(
            path.as_ref().join(G::get_filename(proj.name())),
        )?);
        G::print_header(&mut writer, proj.name())?;
        for func in parse(proj.header())? {
            G::register_func(&mut writer, &func)?;
        }
        G::print_footer(&mut writer)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    gen::<PythonGenerator, _>("python", "../../capi")?;
    gen::<CSharpGenerator, _>("cs", "../../capi")?;

    // let functions = parse::parse(proj.header())?;
    // for func in functions {
    //     dbg!(func);
    // }

    Ok(())
}
