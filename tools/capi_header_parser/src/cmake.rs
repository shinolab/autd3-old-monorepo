/*
 * File: cmake.rs
 * Project: src
 * Created Date: 25/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/04/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::{fs::File, io::BufRead, io::BufReader, path::Path};

use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
pub struct CMakeProject {
    name: String,
    header: String,
}

impl CMakeProject {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn header(&self) -> &str {
        &self.header
    }
}

fn parse<P>(path: P, projcts: &mut Vec<CMakeProject>, ignores: &[String]) -> Result<()>
where
    P: AsRef<Path>,
{
    let file = File::open(path.as_ref().join("CMakeLists.txt"))?;
    let reader = BufReader::new(file);
    let re_subdir = Regex::new(r"add_subdirectory\((.*)\)")?;
    let re_library_begin = Regex::new(r"add_library\((.*?)\s")?;
    let mut lines_iter = reader.lines();

    loop {
        let line = lines_iter.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap()?;
        for cap in re_subdir.captures_iter(&line) {
            parse(path.as_ref().join(&cap[1]), projcts, ignores)?;
        }
        for cap in re_library_begin.captures_iter(&line) {
            let name = cap[1].to_string();
            if ignores.contains(&name) {
                continue;
            }

            let files = glob::glob(path.as_ref().join("*.h").to_str().unwrap())
                .unwrap()
                .map(|e| e.unwrap())
                .collect::<Vec<_>>();

            if !files.is_empty() {
                projcts.push(CMakeProject {
                    name,
                    header: files[0].to_str().unwrap().to_string(),
                })
            }
        }
    }

    Ok(())
}

pub fn glob_projects<P>(base: P, ignores: &[String]) -> Result<Vec<CMakeProject>>
where
    P: AsRef<Path>,
{
    let mut projects = Vec::new();
    parse(base, &mut projects, ignores)?;
    Ok(projects)
}
