/*
 * File: types.rs
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

use syn::{FnArg, ReturnType, __private::ToTokens};

use crate::parse::Arg;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Void,
    VoidPtr,
    Char,
    Float32,
    Float64,
    Bool,
}

impl Type {
    fn parse_str(str: &str, use_single: bool) -> Type {
        match str {
            "i8" => Type::Int8,
            "i16" => Type::Int16,
            "i32" => Type::Int32,
            "i64" => Type::Int64,
            "u8" => Type::UInt8,
            "u16" => Type::UInt16,
            "u32" => Type::UInt32,
            "u64" => Type::UInt64,
            "c_char" => Type::Char,
            "ConstPtr" => Type::VoidPtr,
            "float" => {
                if use_single {
                    Type::Float32
                } else {
                    Type::Float64
                }
            }
            "f32" => Type::Float32,
            "f64" => Type::Float64,
            "bool" => Type::Bool,
            s => panic!("Unknown type: {}", s),
        }
    }

    pub fn parse_return(ty: ReturnType, use_single: bool) -> Self {
        match ty {
            ReturnType::Default => Type::Void,
            ReturnType::Type(_, ty) => match *ty {
                syn::Type::Path(path) => Self::parse_str(
                    path.path
                        .segments
                        .first()
                        .unwrap()
                        .ident
                        .to_string()
                        .as_str(),
                    use_single,
                ),
                _ => panic!("Unknown type: {}", ty.to_token_stream()),
            },
        }
    }

    pub(crate) fn parse_arg(i: FnArg, use_single: bool) -> Arg {
        match i {
            syn::FnArg::Receiver(_) => panic!("self is not allowed!"),
            syn::FnArg::Typed(pat) => {
                let name = pat.pat.into_token_stream().to_string();
                match *pat.ty {
                    syn::Type::Path(path) => {
                        return Arg::new(
                            name,
                            Self::parse_str(
                                path.path
                                    .segments
                                    .first()
                                    .unwrap()
                                    .ident
                                    .to_string()
                                    .as_str(),
                                use_single,
                            ),
                            InOut::IN,
                            0,
                        );
                    }
                    syn::Type::Ptr(ptr) => {
                        let inout = if ptr.mutability.is_some() {
                            InOut::OUT
                        } else {
                            InOut::IN
                        };

                        return Arg::new(
                            name,
                            Self::parse_str(
                                ptr.elem.into_token_stream().to_string().as_str(),
                                use_single,
                            ),
                            inout,
                            1,
                        );
                    }
                    _ => panic!("Unknown type: {}", pat.ty.to_token_stream()),
                };
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InOut {
    IN,
    OUT,
    INOUT,
}

impl From<&str> for InOut {
    fn from(str: &str) -> Self {
        match str {
            "IN" => InOut::IN,
            "OUT" => InOut::OUT,
            "INOUT" => InOut::INOUT,
            _ => panic!("Unknown attribute: {}", str),
        }
    }
}
