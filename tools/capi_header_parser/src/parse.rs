/*
 * File: parse.rs
 * Project: src
 * Created Date: 25/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::{fs, path::Path};

use anyhow::Result;
use regex::Regex;

use crate::types::{InOut, Type};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Arg {
    name: String,
    ty: Type,
    inout: InOut,
    is_const: bool,
    pointer: usize,
}

impl Arg {
    pub fn new(name: &str, ty: Type, inout: InOut, is_const: bool, pointer: usize) -> Self {
        Self {
            name: name.to_string(),
            ty,
            inout,
            is_const,
            pointer,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn ty(&self) -> Type {
        self.ty
    }
    pub fn inout(&self) -> InOut {
        self.inout
    }
    pub fn is_const(&self) -> bool {
        self.is_const
    }
    pub fn pointer(&self) -> usize {
        self.pointer
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Function {
    name: String,
    return_ty: Type,
    args: Vec<Arg>,
}

impl Function {
    pub fn new(name: &str, return_ty: Type, args: Vec<Arg>) -> Self {
        Self {
            name: name.to_string(),
            return_ty,
            args,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn return_ty(&self) -> Type {
        self.return_ty
    }

    pub fn args(&self) -> &[Arg] {
        &self.args
    }
}

pub fn parse<P>(header: P, use_single: bool) -> Result<Vec<Function>>
where
    P: AsRef<Path>,
{
    let mut funcutions = Vec::new();
    let data = fs::read_to_string(header)?;

    let re = Regex::new(r"(?ms)EXPORT_AUTD (.*?) (.*?)\((.*?)\);").unwrap();
    let re_arg = Regex::new(r"(IN |OUT |INOUT )?(const)? ?(.*?) (.*)$").unwrap();

    for cap in re.captures_iter(&data) {
        let return_ty = Type::parse(&cap[1], use_single);
        let name = cap[2].to_string();
        let mut args = Vec::new();
        for arg in cap[3].split(',') {
            if let Some(matches) = re_arg.captures(arg.trim()) {
                let inout = if let Some(m) = matches.get(1) {
                    m.as_str().trim().into()
                } else {
                    panic!("Cannot find IN/OUT attribute: {} of {}", &arg, &name);
                };
                let is_const = matches.get(2).is_some();
                let name = matches.get(4).unwrap().as_str().to_string();
                let types_token = matches.get(3).unwrap().as_str();
                let pointer = types_token.chars().filter(|&c| c == '*').count();
                let ty = Type::parse(types_token.replace('*', "").trim(), use_single);
                args.push(Arg {
                    name,
                    ty,
                    inout,
                    is_const,
                    pointer,
                });
            } else {
                panic!("Cannot parse argument: {}", arg.trim());
            }
        }
        funcutions.push(Function {
            name,
            return_ty,
            args,
        });
    }

    Ok(funcutions)
}
