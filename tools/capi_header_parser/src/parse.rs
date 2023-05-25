/*
 * File: parse.rs
 * Project: src
 * Created Date: 25/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::{fs::File, io::Read, path::Path};

use anyhow::Result;

use crate::types::{InOut, Type};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Arg {
    name: String,
    ty: Type,
    inout: InOut,
    pointer: usize,
}

impl Arg {
    pub fn new(name: String, ty: Type, inout: InOut, pointer: usize) -> Self {
        Self {
            name,
            ty,
            inout,
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
    let mut file = File::open(header)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let syntax_tree = syn::parse_file(&contents)?;

    Ok(syntax_tree
        .items
        .into_iter()
        .filter_map(|item| match item {
            syn::Item::Fn(item_fn) => {
                let name = item_fn.sig.ident.to_string();
                let return_ty = Type::parse_return(item_fn.sig.output, use_single);
                let args = item_fn
                    .sig
                    .inputs
                    .into_iter()
                    .map(|i| Type::parse_arg(i, use_single))
                    .collect::<Vec<_>>();

                Some(Function {
                    name,
                    return_ty,
                    args,
                })
            }
            _ => None,
        })
        .collect())
}
