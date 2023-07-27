/*
 * File: npm.rs
 * Project: src
 * Created Date: 27/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    fs,
    io::{BufReader, Read},
    path::Path,
};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct PackageJson {
    pub name: String,
    pub version: String,
    pub repository: serde_json::Value,
    pub license: String,
    pub author: String,
}

#[derive(Debug)]
pub struct NodeDependencyDetails {
    pub name: String,
    pub version: String,
    pub repository: String,
    pub license: String,
    pub author: String,
}

pub fn glob_node_modules<P>(node_modules_path: P) -> anyhow::Result<Vec<NodeDependencyDetails>>
where
    P: AsRef<Path>,
{
    let mut details = Vec::new();
    for entry in glob::glob(&format!(
        "{}/{}",
        node_modules_path.as_ref().to_str().unwrap(),
        "**/package.json"
    ))? {
        let entry = entry?;
        let mut file_content = String::new();
        fs::File::open(&entry)
            .map(BufReader::new)?
            .read_to_string(&mut file_content)?;
        if let Ok(package) = serde_json::from_str::<PackageJson>(&file_content) {
            details.push(NodeDependencyDetails {
                name: package.name,
                version: package.version,
                repository: match package.repository {
                    serde_json::Value::String(rep) => rep,
                    serde_json::Value::Object(map) => {
                        if let Some(rep) = map.get("url") {
                            rep.as_str().unwrap().to_owned()
                        } else {
                            return Err(anyhow::anyhow!("invalid repository type"));
                        }
                    }
                    _ => return Err(anyhow::anyhow!("invalid repository type")),
                },
                license: package.license,
                author: package.author,
            });
        }
    }

    Ok(details)
}
