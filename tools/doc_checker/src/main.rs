/*
 * File: main.rs
 * Project: src
 * Created Date: 24/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use anyhow::Result;
use capi_header_parser::{
    cmake,
    parse::{parse, Arg, Function},
    types::{InOut, Type},
};
use regex::Regex;

fn parse_arg(lines: &mut Lines<BufReader<File>>) -> Result<(Type, Vec<Arg>)> {
    let re_arg = Regex::new(r"\|(.*?)\|(.*?)\|(.*?)\|(.*?)\|")?;
    let mut args = Vec::new();
    loop {
        let line = lines.next().unwrap()?;
        let matches = re_arg.captures(&line).unwrap();
        let name = matches
            .get(1)
            .unwrap_or_else(|| panic!("failed to read: {}", &line))
            .as_str()
            .trim();
        let types_tokes = matches
            .get(2)
            .unwrap_or_else(|| panic!("failed to read: {}", &line))
            .as_str()
            .trim();
        let pointer = types_tokes.chars().filter(|&c| c == '*').count();
        let ty: &str = &types_tokes.replace('*', "");
        if name == "return" {
            return Ok((Type::parse(ty, false), args));
        }
        let inout = match matches
            .get(3)
            .unwrap_or_else(|| panic!("failed to read: {}", &line))
            .as_str()
            .trim()
        {
            "in" => InOut::IN,
            "out" => InOut::OUT,
            _ => panic!("Cannot determine in/out:{}", &line),
        };
        args.push(Arg::new(
            name,
            Type::parse(ty, false),
            inout,
            false,
            pointer,
        ))
    }
}

fn parse_function(func_name: &str, lines: &mut Lines<BufReader<File>>) -> Result<Option<Function>> {
    let table_begin = Regex::new(r"\|.*?\|.*?\|.*?\|.*?\|")?;
    loop {
        let line = lines.next().unwrap()?;
        if table_begin.is_match(&line) {
            let _ = lines.next();
            let (return_ty, args) = parse_arg(lines)?;
            return Ok(Some(Function::new(func_name, return_ty, args)));
        }
    }
}

struct Func {
    bin: String,
    func: Function,
}

impl PartialEq for Func {
    fn eq(&self, other: &Self) -> bool {
        self.bin.trim() == other.bin.trim()
            && self.func.name().trim() == other.func.name().trim()
            && self.func.return_ty() == other.func.return_ty()
            && self
                .func
                .args()
                .iter()
                .zip(other.func.args().iter())
                .all(|(l, r)| {
                    l.name().trim() == r.name().trim()
                        && l.ty() == r.ty()
                        && l.inout() == r.inout()
                        && l.pointer() == r.pointer()
                })
    }
}
impl Eq for Func {}

impl std::hash::Hash for Func {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bin.trim().hash(state);
        self.func.name().trim().hash(state);
        self.func.return_ty().hash(state);
        self.func.args().iter().for_each(|a| {
            a.name().trim().hash(state);
            a.ty().hash(state);
            a.inout().hash(state);
            a.pointer().hash(state);
        })
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\tbin: {}", self.bin)?;
        writeln!(f, "\tname: {}", self.func.name())?;
        writeln!(f, "\treturn: {:?}", self.func.return_ty())?;
        for (i, arg) in self.func.args().iter().enumerate() {
            writeln!(f, "\targument[{}]", i)?;
            writeln!(f, "\t\tname: {}", arg.name())?;
            writeln!(f, "\t\tinuot: {:?}", arg.inout())?;
            writeln!(f, "\t\ttype: {:?}", arg.ty())?;
            writeln!(f, "\t\tpointer: {}", arg.pointer())?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let projects = cmake::glob_projects("../../capi", &[])?;
    let mut defined_functions = HashSet::new();
    for proj in projects {
        for func in parse(proj.header(), false)? {
            defined_functions.insert(Func {
                bin: proj.name().to_string(),
                func,
            });
        }
    }

    let doc_paths = vec![
        "../../doc/book/src/en/FFI/reference.md",
        "../../doc/book/src/jp/FFI/reference.md",
    ];

    for path in doc_paths {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut lines_iter = reader.lines();

        let func_begin = Regex::new(r"## (.*?) \((.*)\)")?;

        let mut documented_functions = HashSet::new();
        loop {
            let line = lines_iter.next();
            if line.is_none() {
                break;
            }
            let line = line.unwrap()?;
            if func_begin.is_match(&line) {
                let matches = func_begin.captures(&line).unwrap();
                if let Some(func) =
                    parse_function(matches.get(1).unwrap().as_str(), &mut lines_iter)?
                {
                    documented_functions.insert(Func {
                        bin: matches.get(2).unwrap().as_str().to_string(),
                        func,
                    });
                }
            }
        }

        let diff = defined_functions
            .symmetric_difference(&documented_functions)
            .collect::<Vec<_>>();
        for f in diff.iter() {
            if defined_functions.contains(f) {
                eprintln!("The following function is defined but not documented:");
                eprintln!("{}", f);
            }
            if documented_functions.contains(f) {
                eprintln!("The following function is documented but not defined:");
                eprintln!("{}", f);
            }
        }

        if !diff.is_empty() {
            panic!("Fix document!");
        }
    }

    Ok(())
}
