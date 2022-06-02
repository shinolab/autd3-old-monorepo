/*
 * File: csharp.rs
 * Project: src
 * Created Date: 25/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2022
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

pub struct CSharpGenerator {}

impl CSharpGenerator {
    fn sub_abbr(str: String) -> String {
        str.replace("Cuda", "CUDA")
            .replace("Blas", "BLAS")
            .replace("Twincat", "TwinCAT")
            .replace("Soem", "SOEM")
    }

    fn sub_reserve(str: String) -> String {
        str.replace("out", "@out")
    }

    fn to_pascal(name: &str) -> String {
        let res = name.to_case(Case::Pascal);
        Self::sub_abbr(res)
    }

    fn to_camel(name: &str) -> String {
        let res = name.to_case(Case::Camel);
        Self::sub_reserve(Self::sub_abbr(res))
    }

    fn to_class_name(name: &str) -> String {
        if name.split('-').count() == 1 {
            return "Base".to_string();
        }
        Self::to_pascal(&name.replace("autd3capi-", ""))
    }

    fn to_return_ty(ty: &Type) -> &str {
        match ty {
            Type::Int8 => "sbyte",
            Type::Int16 => "short",
            Type::Int32 => "int",
            Type::Int64 => "long",
            Type::UInt8 => "byte",
            Type::UInt16 => "ushort",
            Type::UInt32 => "uint",
            Type::UInt64 => "ulong",
            Type::Void => "void",
            Type::Char => "char",
            Type::Float32 => "float",
            Type::Float64 => "double",
            Type::Bool => "bool",
        }
    }

    fn to_arg_ty(arg: &Arg) -> &str {
        match arg.pointer() {
            0 => match arg.ty() {
                Type::Int8 => "sbyte",
                Type::Int16 => "short",
                Type::Int32 => "int",
                Type::Int64 => "long",
                Type::UInt8 => "byte",
                Type::UInt16 => "ushort",
                Type::UInt32 => "uint",
                Type::UInt64 => "ulong",
                Type::Void => panic!("void is not supported in argument"),
                Type::Char => "char",
                Type::Float32 => "float",
                Type::Float64 => "double",
                Type::Bool => "[MarshalAs(UnmanagedType.U1)] bool",
            },
            1 => match arg.ty() {
                Type::Int8 => match arg.inout() {
                    InOut::IN => "sbyte[]?",
                    InOut::OUT => "out sbyte",
                    InOut::INOUT => panic!("INOUT sbyte is not supported."),
                },
                Type::Int16 => match arg.inout() {
                    InOut::IN => "short[]?",
                    InOut::OUT => "out short",
                    InOut::INOUT => panic!("INOUT short is not supported."),
                },
                Type::Int32 => match arg.inout() {
                    InOut::IN => "short[]?",
                    InOut::OUT => "out short",
                    InOut::INOUT => panic!("INOUT short is not supported."),
                },
                Type::Int64 => match arg.inout() {
                    InOut::IN => "short[]?",
                    InOut::OUT => "out short",
                    InOut::INOUT => panic!("INOUT short is not supported."),
                },
                Type::UInt8 => match arg.inout() {
                    InOut::IN => "byte[]?",
                    InOut::OUT => "out byte",
                    InOut::INOUT => panic!("INOUT byte is not supported."),
                },
                Type::UInt16 => match arg.inout() {
                    InOut::IN => "ushort[]?",
                    InOut::OUT => "out ushort",
                    InOut::INOUT => panic!("INOUT ushort is not supported."),
                },
                Type::UInt32 => match arg.inout() {
                    InOut::IN => "uint[]?",
                    InOut::OUT => "out uint",
                    InOut::INOUT => panic!("INOUT uint is not supported."),
                },
                Type::UInt64 => match arg.inout() {
                    InOut::IN => "ulong[]?",
                    InOut::OUT => "out ulong",
                    InOut::INOUT => panic!("INOUT ulong is not supported."),
                },
                Type::Void => "IntPtr",
                Type::Char => match arg.inout() {
                    InOut::IN => "string",
                    InOut::OUT => "System.Text.StringBuilder?",
                    InOut::INOUT => panic!("INOUT char* is not supported."),
                },
                Type::Float32 => match arg.inout() {
                    InOut::IN => "float[]?",
                    InOut::OUT => "out float",
                    InOut::INOUT => panic!("INOUT float is not supported."),
                },
                Type::Float64 => match arg.inout() {
                    InOut::IN => "double[]?",
                    InOut::OUT => "out double",
                    InOut::INOUT => panic!("INOUT double is not supported."),
                },
                Type::Bool => match arg.inout() {
                    InOut::IN => "bool[]?",
                    InOut::OUT => "out bool",
                    InOut::INOUT => panic!("INOUT bool is not supported."),
                },
            },
            2 => match arg.ty() {
                Type::Void => "out IntPtr",
                _ => panic!("double pointer is not supported, but void**"),
            },
            _ => {
                panic!("triple or more pointer is not supported")
            }
        }
    }
}

impl Generator for CSharpGenerator {
    fn print_header<W: Write>(w: &mut W, bin_name: &str) -> Result<()> {
        write!(
            w,
            r#"// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;
            
namespace AUTD3Sharp.NativeMethods
{{
    internal static class {}
    {{
        const string DLL = "{}";

"#,
            Self::to_class_name(bin_name),
            bin_name,
        )?;
        Ok(())
    }

    fn register_func<W: Write>(w: &mut W, function: &Function) -> Result<()> {
        let args = function
            .args()
            .iter()
            .map(|arg| format!("{} {}", Self::to_arg_ty(arg), Self::to_camel(arg.name())))
            .join(", ");

        let attr = if function
            .args()
            .iter()
            .any(|arg| (arg.ty() == Type::Char) && (arg.pointer() == 1))
        {
            "[DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)]"
        } else {
            "[DllImport(DLL, CallingConvention = CallingConvention.Cdecl)]"
        };
        let ret_attr = if function.return_ty() == Type::Bool {
            "[return: MarshalAs(UnmanagedType.U1)]"
        } else {
            ""
        };

        write!(
            w,
            r"        {}{} public static extern {} {}({});
",
            attr,
            ret_attr,
            Self::to_return_ty(&function.return_ty()),
            function.name(),
            args
        )?;
        Ok(())
    }

    fn print_footer<W: Write>(w: &mut W) -> Result<()> {
        writeln!(
            w,
            r"
    }}
}}"
        )?;
        Ok(())
    }

    fn get_filename(name: &str) -> String {
        format!("{}.cs", Self::to_class_name(name))
    }
}
