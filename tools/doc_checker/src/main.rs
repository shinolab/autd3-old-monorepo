/*
 * File: main.rs
 * Project: src
 * Created Date: 24/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/06/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::{
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
            return Ok((ty.into(), args));
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
        args.push(Arg::new(name, ty.into(), inout, false, pointer))
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

fn main() -> Result<()> {
    let projects = cmake::glob_projects("../../capi")?;
    let mut defined_functions = Vec::new();
    for proj in projects {
        for func in parse(proj.header())? {
            defined_functions.push(Func {
                bin: proj.name().to_string(),
                func,
            });
        }
    }

    let path = "../../book/src/jp/Software/FFI/reference.md";

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines();

    let func_begin = Regex::new(r"## (.*?) \((.*)\)")?;

    let mut documented_functions = Vec::new();
    loop {
        let line = lines_iter.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap()?;
        if func_begin.is_match(&line) {
            let matches = func_begin.captures(&line).unwrap();
            if let Some(func) = parse_function(matches.get(1).unwrap().as_str(), &mut lines_iter)? {
                documented_functions.push(Func {
                    bin: matches.get(2).unwrap().as_str().to_string(),
                    func,
                });
            }
        }
    }

    for def in defined_functions.iter() {
        if !documented_functions.iter().any(|f| f == def) {
            eprintln!("There is a difference between header and document:");
            eprintln!("================== defined =======================");
            eprintln!("\tbin: {}", def.bin);
            eprintln!("\tname: {}", def.func.name());
            eprintln!("\treturn: {:?}", def.func.return_ty());
            for (i, arg) in def.func.args().iter().enumerate() {
                eprintln!("\targument[{}]", i);
                eprintln!("\t\tname: {}", arg.name());
                eprintln!("\t\tinuot: {:?}", arg.inout());
                eprintln!("\t\ttype: {:?}", arg.ty());
                eprintln!("\t\tpointer: {}", arg.pointer());
            }
        };
    }
    for doc in documented_functions.iter() {
        if !defined_functions.iter().any(|f| f == doc) {
            eprintln!("There is a difference between header and document:");
            eprintln!("================== documented =======================");
            eprintln!("\tbin: {}", doc.bin);
            eprintln!("\tname: {}", doc.func.name());
            eprintln!("\treturn: {:?}", doc.func.return_ty());
            for (i, arg) in doc.func.args().iter().enumerate() {
                eprintln!("\targument[{}]", i);
                eprintln!("\t\tname: {}", arg.name());
                eprintln!("\t\tinuot: {:?}", arg.inout());
                eprintln!("\t\ttype: {:?}", arg.ty());
                eprintln!("\t\tpointer: {}", arg.pointer());
            }
        };
    }

    Ok(())
}
