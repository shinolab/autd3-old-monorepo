/*
 * File: cmake.rs
 * Project: src
 * Created Date: 25/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 *
 */

use std::{
    fs::File,
    io::BufRead,
    io::{BufReader, Lines},
    path::Path,
};

use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
pub struct CMakeProject {
    name: String,
    header: String,
    depends_ext_lib: bool,
}

impl CMakeProject {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn header(&self) -> &str {
        &self.header
    }
    pub fn depends_ext_lib(&self) -> bool {
        self.depends_ext_lib
    }
}

fn find_header(lines: &mut Lines<BufReader<File>>) -> Result<Option<String>> {
    let re_header = Regex::new(r"^\s*(.*\.h)$")?;
    loop {
        let line = lines.next();
        if line.is_none() {
            return Ok(None);
        }
        let line = line.unwrap()?;
        if let Some(cap) = re_header.captures_iter(&line).next() {
            return Ok(Some(cap[1].to_string()));
        }
    }
}

fn check_dependency_to_ext_lib<P>(path: P) -> Result<bool>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.contains("AUTD_DEPENDS_EXT_LIB") {
            return Ok(true);
        }
    }

    Ok(false)
}

fn parse<P>(path: P, projcts: &mut Vec<CMakeProject>) -> Result<()>
where
    P: AsRef<Path>,
{
    let file = File::open(path.as_ref().join("CMakeLists.txt"))?;
    let reader = BufReader::new(file);
    let re_subdir = Regex::new(r"add_subdirectory\((.*)\)")?;
    let re_library_begin = Regex::new(r"add_library\((.*)\s")?;
    let mut lines_iter = reader.lines();

    loop {
        let line = lines_iter.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap()?;
        for cap in re_subdir.captures_iter(&line) {
            parse(path.as_ref().join(&cap[1]), projcts)?;
        }
        for cap in re_library_begin.captures_iter(&line) {
            let name = cap[1].to_string();
            if let Some(header) = find_header(&mut lines_iter)? {
                let header_path = path.as_ref().join(header);
                let depends_ext_lib = check_dependency_to_ext_lib(&header_path)?;
                projcts.push(CMakeProject {
                    name,
                    header: header_path.to_str().unwrap().to_string(),
                    depends_ext_lib,
                })
            }
        }
    }

    Ok(())
}

pub fn glob_projects<P>(base: P) -> Result<Vec<CMakeProject>>
where
    P: AsRef<Path>,
{
    let mut projects = Vec::new();
    parse(base, &mut projects)?;
    Ok(projects)
}
