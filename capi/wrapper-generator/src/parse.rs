/*
 * File: parse.rs
 * Project: src
 * Created Date: 25/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::{fs::File, io::Read, path::Path};

use anyhow::Result;
use syn::__private::ToTokens;

use crate::types::{InOut, Type};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Arg {
    pub name: String,
    pub ty: Type,
    pub inout: InOut,
    pub pointer: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Function {
    pub docs: Vec<String>,
    pub name: String,
    pub return_ty: Type,
    pub args: Vec<Arg>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Const {
    pub name: String,
    pub ty: Type,
    pub value: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Enum {
    pub name: String,
    pub ty: Type,
    pub values: Vec<(String, String)>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<(Type, String)>,
}

pub fn parse_func<P>(header: P, use_single: bool) -> Result<Vec<Function>>
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
                let docs = item_fn
                    .attrs
                    .iter()
                    .filter(|attr| attr.path().is_ident("doc"))
                    .map(|attr| {
                        attr.meta
                            .require_name_value()
                            .unwrap()
                            .value
                            .clone()
                            .into_token_stream()
                            .to_string()
                            .trim()
                            .to_string()
                    })
                    .collect();

                let name = item_fn.sig.ident.to_string();
                let return_ty = Type::parse_return(item_fn.sig.output, use_single);
                let args = item_fn
                    .sig
                    .inputs
                    .into_iter()
                    .map(|i| Type::parse_arg(i, use_single))
                    .collect::<Vec<_>>();

                Some(Function {
                    docs,
                    name,
                    return_ty,
                    args,
                })
            }
            _ => None,
        })
        .collect())
}

pub fn parse_const<P>(header: P, use_single: bool) -> Result<Vec<Const>>
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
            syn::Item::Const(item_const) => {
                let name = item_const.ident.to_string();
                let ty =
                    Type::parse_str(&item_const.ty.into_token_stream().to_string(), use_single);
                let mut value = item_const.expr.into_token_stream().to_string();
                value.retain(|c| !c.is_whitespace());

                Some(Const { name, ty, value })
            }
            _ => None,
        })
        .collect())
}

pub fn parse_enum<P>(header: P, use_single: bool) -> Result<Vec<Enum>>
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
            syn::Item::Enum(item_enum) => {
                let name = item_enum.ident.to_string();
                let ty = Type::parse_str(
                    item_enum.attrs[0]
                        .meta
                        .require_list()
                        .unwrap()
                        .tokens
                        .to_string()
                        .as_str(),
                    use_single,
                );
                let values = item_enum
                    .variants
                    .into_iter()
                    .map(|v| {
                        let name = v.ident.to_string();
                        let value = v.discriminant.unwrap().1.into_token_stream().to_string();
                        (name, value)
                    })
                    .collect();

                Some(Enum { name, ty, values })
            }
            _ => None,
        })
        .collect())
}

pub fn parse_struct<P>(header: P, use_single: bool) -> Result<Vec<Struct>>
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
            syn::Item::Struct(item_struct) => match item_struct.vis {
                syn::Visibility::Public(_) => {
                    let name = item_struct.ident.to_string();
                    let fields = item_struct
                        .fields
                        .iter()
                        .filter(|f| f.ident.is_some())
                        .map(|f| {
                            (
                                Type::parse_str(
                                    f.ty.clone().into_token_stream().to_string().as_str(),
                                    use_single,
                                ),
                                f.ident.as_ref().unwrap().to_string(),
                            )
                        })
                        .collect();
                    Some(Struct { name, fields })
                }
                _ => None,
            },
            _ => None,
        })
        .collect())
}
