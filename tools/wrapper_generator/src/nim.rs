/*
 * File: nim.rs
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
    types::Type,
};

use crate::traits::Generator;

pub struct NimGenerator {}

impl NimGenerator {
    fn to_return_ty(ty: &Type) -> &str {
        match ty {
            Type::Int8 => "int8",
            Type::Int16 => "int16",
            Type::Int32 => "int32",
            Type::Int64 => "int64",
            Type::UInt8 => "uint8",
            Type::UInt16 => "uint16",
            Type::UInt32 => "uint32",
            Type::UInt64 => "uint64",
            Type::Void => "void",
            Type::Char => "char",
            Type::Float32 => "float32",
            Type::Float64 => "float64",
            Type::Bool => "bool",
        }
    }

    fn to_arg_ty(arg: &Arg) -> String {
        let res = match arg.ty() {
            Type::Int8 => "int8",
            Type::Int16 => "int16",
            Type::Int32 => "int32",
            Type::Int64 => "int64",
            Type::UInt8 => "uint8",
            Type::UInt16 => "uint16",
            Type::UInt32 => "uint32",
            Type::UInt64 => "uint64",
            Type::Char => "char",
            Type::Float32 => "float32",
            Type::Float64 => "float64",
            Type::Bool => "bool",
            Type::Void => "void",
        };
        let mut res = res.to_string();
        res.push_str(&"*".repeat(arg.pointer()));
        res
    }
}

impl Generator for NimGenerator {
    fn print_header<W: Write>(w: &mut W, bin_name: &str) -> Result<()> {
        write!(
            w,
            r#"// This file was automatically generated from header file

#ifdef C2NIM
#  dynlib dll
#  cdecl
#  if defined(windows)
#    define dll "{0}.dll"
#  elif defined(macosx)
#    define dll "lib{0}.dylib"
#  else
#    define dll "lib{0}.so"
#  endif
#endif

"#,
            bin_name
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
        format!("{}.h", name)
    }
}
