/*
 * File: glob.rs
 * Project: src
 * Created Date: 25/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;
use serde_derive::Deserialize;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

#[derive(Debug, Deserialize)]
struct WorkspaceRoot {
    workspace: Workspace,
}

#[derive(Debug, Deserialize)]
struct Workspace {
    members: Vec<String>,
}

pub struct Project {
    pub path: String,
    pub name: String,
}

pub fn glob_projects<P: AsRef<Path>>(path: P) -> Result<Vec<Project>> {
    let workspace_path = path.as_ref().join("Cargo.toml");

    let mut file_content = String::new();
    let mut fr = File::open(workspace_path)
        .map(BufReader::new)
        .map_err(|e| e.to_string())
        .unwrap();
    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())
        .unwrap();

    let workspace_project: WorkspaceRoot = toml::from_str(&file_content)?;

    Ok(workspace_project
        .workspace
        .members
        .iter()
        .map(|m| Project {
            path: path
                .as_ref()
                .join(m)
                .join("src/lib.rs")
                .to_str()
                .unwrap()
                .to_string(),
            name: m.to_string(),
        })
        .collect())
}
