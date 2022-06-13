/*
 * File: csharp.rs
 * Project: src
 * Created Date: 25/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/06/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use convert_case::{Case, Casing};
use std::io::Write;

use itertools::Itertools;

use capi_header_parser::{
    parse::{Arg, Function},
    types::{InOut, Type},
};

use crate::traits::Generator;

pub struct JuliaGenerator {}

impl JuliaGenerator {
    fn to_snake(name: &str) -> String {
        let res = name.to_case(Case::Snake);
        res
    }

    fn to_return_ty(ty: &Type) -> &str {
        match ty {
            Type::Int8 => "Int8",
            Type::Int16 => "Int16",
            Type::Int32 => "Int32",
            Type::Int64 => "Int64",
            Type::UInt8 => "UInt8",
            Type::UInt16 => "UInt16",
            Type::UInt32 => "UInt32",
            Type::UInt64 => "UInt64",
            Type::Void => "Cvoid",
            Type::Char => "Int8",
            Type::Float32 => "Float32",
            Type::Float64 => "Float64",
            Type::Bool => "Bool",
        }
    }

    fn to_arg_ty(arg: &Arg) -> &str {
        match arg.pointer() {
            0 => match arg.ty() {
                Type::Int8 => "Int8",
                Type::Int16 => "Int16",
                Type::Int32 => "Int32",
                Type::Int64 => "Int64",
                Type::UInt8 => "UInt8",
                Type::UInt16 => "UInt16",
                Type::UInt32 => "UInt32",
                Type::UInt64 => "UInt64",
                Type::Void => panic!("void is not supported in argument"),
                Type::Char => "Int8",
                Type::Float32 => "Float32",
                Type::Float64 => "Float64",
                Type::Bool => "Bool",
            },
            1 => match arg.ty() {
                Type::Int8 => match arg.inout() {
                    InOut::IN => "Array{Int8,1}",
                    InOut::OUT => "Ref{Int8}",
                    InOut::INOUT => panic!("INOUT is not supported."),
                },
                Type::Int16 => match arg.inout() {
                    InOut::IN => "Array{Int16,1}",
                    InOut::OUT => "Ref{Int16}",
                    InOut::INOUT => panic!("INOUT is not supported."),
                },
                Type::Int32 => match arg.inout() {
                    InOut::IN => "Array{Int32,1}",
                    InOut::OUT => "Ref{Int32}",
                    InOut::INOUT => panic!("INOUT is not supported."),
                },
                Type::Int64 => match arg.inout() {
                    InOut::IN => "Array{Int64,1}",
                    InOut::OUT => "Ref{Int64}",
                    InOut::INOUT => panic!("INOUT is not supported."),
                },
                Type::UInt8 => match arg.inout() {
                    InOut::IN => "Array{UInt8,1}",
                    InOut::OUT => "Ref{UInt8}",
                    InOut::INOUT => panic!("INOUT is not supported."),
                },
                Type::UInt16 => match arg.inout() {
                    InOut::IN => "Array{UInt16,1}",
                    InOut::OUT => "Ref{UInt16}",
                    InOut::INOUT => panic!("INOUT is not supported."),
                },
                Type::UInt32 => match arg.inout() {
                    InOut::IN => "Array{UInt32,1}",
                    InOut::OUT => "Ref{UInt32}",
                    InOut::INOUT => panic!("INOUT is not supported."),
                },
                Type::UInt64 => match arg.inout() {
                    InOut::IN => "Array{UInt64,1}",
                    InOut::OUT => "Ref{UInt64}",
                    InOut::INOUT => panic!("INOUT is not supported."),
                },
                Type::Void => "Ptr{Cvoid}",
                Type::Char => match arg.inout() {
                    InOut::IN => "Cstring",
                    InOut::OUT => "Ref{UInt8}",
                    InOut::INOUT => panic!("INOUT is not supported."),
                },
                Type::Float32 => match arg.inout() {
                    InOut::IN => "Array{Float32,1}",
                    InOut::OUT => "Ref{Float32}",
                    InOut::INOUT => panic!("INOUT is not supported."),
                },
                Type::Float64 => match arg.inout() {
                    InOut::IN => "Array{Float64,1}",
                    InOut::OUT => "Ref{Float64}",
                    InOut::INOUT => panic!("INOUT is not supported."),
                },
                Type::Bool => match arg.inout() {
                    InOut::IN => "Array{Bool,1}",
                    InOut::OUT => "Ref{Bool}",
                    InOut::INOUT => panic!("INOUT is not supported."),
                },
            },
            2 => match arg.ty() {
                Type::Void => "Ref{Ptr{Cvoid}}",
                _ => panic!("double pointer is not supported, but void**"),
            },
            _ => {
                panic!("triple or more pointer is not supported")
            }
        }
    }
}

impl Generator for JuliaGenerator {
    fn print_header<W: Write>(w: &mut W, bin_name: &str) -> Result<()> {
        write!(
            w,
            r#"# This file was automatically generated from header file

function get_lib_ext()
if Sys.iswindows()
    return ".dll"
elseif Sys.isapple()
    return ".dylib"
elseif Sys.islinux()
    return ".so"
end
end

function get_lib_prefix()
if Sys.iswindows()
    return ""
else
    return "lib"
end
end

const _dll = joinpath(@__DIR__, "bin", get_lib_prefix() * "{}" * get_lib_ext())

"#,
            bin_name,
        )?;
        Ok(())
    }

    fn register_func<W: Write>(w: &mut W, function: &Function) -> Result<()> {
        let func_name = Self::to_snake(function.name());
        let args = function.args().iter().map(|arg| arg.name()).join(", ");
        let ret_ty = function.return_ty();
        let arg_types: String = function
            .args()
            .iter()
            .map(|arg| format!("{}, ", Self::to_arg_ty(arg)))
            .collect();

        writeln!(
            w,
            r"{}({}) = ccall((:{}, _dll), {}, ({}), {});",
            func_name,
            args,
            function.name(),
            Self::to_return_ty(&ret_ty),
            arg_types,
            args
        )?;
        Ok(())
    }

    fn print_footer<W: Write>(_w: &mut W) -> Result<()> {
        Ok(())
    }

    fn get_filename(name: &str) -> String {
        format!("{}.jl", name)
    }
}
