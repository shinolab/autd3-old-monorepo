/*
 * File: types.rs
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
    Char,
    Float32,
    Float64,
    Bool,
}

impl Type {
    pub fn parse(str: &str, use_single: bool) -> Self {
        match str {
            "int8_t" => Type::Int8,
            "int16_t" => Type::Int16,
            "int32_t" => Type::Int32,
            "int64_t" => Type::Int64,
            "uint8_t" => Type::UInt8,
            "uint16_t" => Type::UInt16,
            "uint32_t" => Type::UInt32,
            "uint64_t" => Type::UInt64,
            "void" => Type::Void,
            "char" => Type::Char,
            "autd3_float_t" => {
                if use_single {
                    Type::Float32
                } else {
                    Type::Float64
                }
            }
            "float" => Type::Float32,
            "double" => Type::Float64,
            "bool" => Type::Bool,
            _ => panic!("Unknown type: {}", str),
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
