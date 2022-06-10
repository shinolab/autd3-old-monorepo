/*
 * File: naive_c.rs
 * Project: src
 * Created Date: 07/06/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/06/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use std::io::Write;

use itertools::Itertools;

use capi_header_parser::{
    parse::{Arg, Function},
    types::{InOut, Type},
};

use crate::traits::Generator;

pub struct MatlabGenerator {}

impl MatlabGenerator {
    fn to_return_ty(ty: &Type) -> &str {
        match ty {
            Type::Int8 => "int8_t",
            Type::Int16 => "int16_t",
            Type::Int32 => "int32_t",
            Type::Int64 => "int64_t",
            Type::UInt8 => "uint8_t",
            Type::UInt16 => "uint16_t",
            Type::UInt32 => "uint32_t",
            Type::UInt64 => "uint64_t",
            Type::Void => "void",
            Type::Char => "char",
            Type::Float32 => "float",
            Type::Float64 => "double",
            Type::Bool => "bool",
        }
    }

    fn to_arg_ty(arg: &Arg) -> String {
        let res = match arg.ty() {
            Type::Int8 => "int8_t",
            Type::Int16 => "int16_t",
            Type::Int32 => "int32_t",
            Type::Int64 => "int64_t",
            Type::UInt8 => "uint8_t",
            Type::UInt16 => "uint16_t",
            Type::UInt32 => "uint32_t",
            Type::UInt64 => "uint64_t",
            Type::Char => {
                if arg.inout() == InOut::IN {
                    "char"
                } else {
                    "int8_t"
                }
            }
            Type::Float32 => "float",
            Type::Float64 => "double",
            Type::Bool => "bool",
            Type::Void => "void",
        };
        let mut res = res.to_string();
        res.push_str(&"*".repeat(arg.pointer()));
        res
    }
}

impl Generator for MatlabGenerator {
    fn print_header<W: Write>(w: &mut W, _bin_name: &str) -> Result<()> {
        write!(
            w,
            r"// This file was automatically generated from header file
#ifdef __cplusplus
#include <cstdint>
#else
#include <stdbool.h>
#include <stdint.h>
#endif

"
        )?;
        Ok(())
    }

    fn register_func<W: Write>(w: &mut W, function: &Function) -> Result<()> {
        let args = function
            .args()
            .iter()
            .map(|arg| format!("{} {}", Self::to_arg_ty(arg), arg.name()))
            .join(", ");

        writeln!(
            w,
            r"{} {}({});",
            Self::to_return_ty(&function.return_ty()),
            function.name(),
            args
        )?;
        Ok(())
    }

    fn print_footer<W: Write>(_w: &mut W) -> Result<()> {
        Ok(())
    }

    fn get_filename(name: &str) -> String {
        format!("{}.h", name.replace('-', "_"))
    }
}
