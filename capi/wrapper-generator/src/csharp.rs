/*
 * File: csharp.rs
 * Project: src
 * Created Date: 25/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use convert_case::{Case, Casing};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use itertools::Itertools;

use crate::{
    parse::{Arg, Const, Enum, Function},
    types::{InOut, Type},
};

use crate::traits::Generator;

pub struct CSharpGenerator {
    functions: Vec<Function>,
    constants: Vec<Const>,
    enums: Vec<Enum>,
}

impl CSharpGenerator {
    fn sub_abbr(str: String) -> String {
        str.replace("Cuda", "CUDA")
            .replace("Blas", "BLAS")
            .replace("Twincat", "TwinCAT")
            .replace("Soem", "SOEM")
    }

    fn sub_reserve(str: String) -> String {
        if str == "out" {
            str.replace("out", "@out")
        } else {
            str
        }
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
            Type::VoidPtr => "IntPtr",
            Type::Custom(ref s) => s,
        }
    }

    fn to_arg_ty(arg: &Arg) -> &str {
        match arg.pointer {
            0 => match arg.ty {
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
                Type::VoidPtr => "IntPtr",
                Type::Custom(ref s) => s,
            },
            1 => match arg.ty {
                Type::Int8 => match arg.inout {
                    InOut::In => "sbyte[]?",
                    InOut::Out => "out sbyte",
                    InOut::InOut => panic!("INOUT sbyte is not supported."),
                },
                Type::Int16 => match arg.inout {
                    InOut::In => "short[]?",
                    InOut::Out => "out short",
                    InOut::InOut => panic!("INOUT short is not supported."),
                },
                Type::Int32 => match arg.inout {
                    InOut::In => "short[]?",
                    InOut::Out => "out short",
                    InOut::InOut => panic!("INOUT short is not supported."),
                },
                Type::Int64 => match arg.inout {
                    InOut::In => "short[]?",
                    InOut::Out => "out short",
                    InOut::InOut => panic!("INOUT short is not supported."),
                },
                Type::UInt8 => match arg.inout {
                    InOut::In => "byte[]?",
                    InOut::Out => "out byte",
                    InOut::InOut => panic!("INOUT byte is not supported."),
                },
                Type::UInt16 => match arg.inout {
                    InOut::In => "ushort[]?",
                    InOut::Out => "out ushort",
                    InOut::InOut => panic!("INOUT ushort is not supported."),
                },
                Type::UInt32 => match arg.inout {
                    InOut::In => "uint[]?",
                    InOut::Out => "out uint",
                    InOut::InOut => panic!("INOUT uint is not supported."),
                },
                Type::UInt64 => match arg.inout {
                    InOut::In => "ulong[]?",
                    InOut::Out => "out ulong",
                    InOut::InOut => panic!("INOUT ulong is not supported."),
                },
                Type::Char => match arg.inout {
                    InOut::In => "string",
                    InOut::Out => "byte[]",
                    InOut::InOut => panic!("INOUT char* is not supported."),
                },
                Type::Float32 => match arg.inout {
                    InOut::In => "float[]?",
                    InOut::Out => "out float",
                    InOut::InOut => panic!("INOUT float is not supported."),
                },
                Type::Float64 => match arg.inout {
                    InOut::In => "double[]?",
                    InOut::Out => "out double",
                    InOut::InOut => panic!("INOUT double is not supported."),
                },
                Type::Bool => match arg.inout {
                    InOut::In => "bool[]?",
                    InOut::Out => "out bool",
                    InOut::InOut => panic!("INOUT bool is not supported."),
                },
                Type::VoidPtr => match arg.inout {
                    InOut::In => panic!("void** is not supported."),
                    InOut::Out => "out IntPtr",
                    InOut::InOut => panic!("INOUT double is not supported."),
                },
                _ => unimplemented!(),
            },
            _ => {
                panic!("double or more pointer is not supported")
            }
        }
    }

    fn get_filename(name: &str) -> String {
        format!("{}.cs", Self::to_class_name(name))
    }
}

impl Generator for CSharpGenerator {
    fn register_const(mut self, constants: Vec<Const>) -> Self {
        self.constants = constants;
        self
    }

    fn register_enum(mut self, enums: Vec<Enum>) -> Self {
        self.enums = enums;
        self
    }

    fn register_func(mut self, functions: Vec<Function>) -> Self {
        self.functions = functions;
        self
    }

    fn new() -> Self {
        Self {
            functions: Vec::new(),
            constants: Vec::new(),
            enums: Vec::new(),
        }
    }

    fn write<P: AsRef<Path>>(self, path: P, crate_name: &str) -> Result<()> {
        let mut w = BufWriter::new(File::create(
            path.as_ref().join(Self::get_filename(crate_name)),
        )?);

        write!(
            w,
            r#"// This file is autogenerated
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp
{{
    namespace NativeMethods
    {{
        internal static class {}
        {{
            private const string DLL = "{}";
"#,
            Self::to_class_name(crate_name),
            crate_name.replace('-', "_"),
        )?;

        for constant in self.constants {
            writeln!(
                w,
                r"
            public const {} {} = {};",
                Self::to_return_ty(&constant.ty),
                Self::to_pascal(&constant.name),
                if Self::to_return_ty(&constant.ty) == "float" {
                    format!("{}f", constant.value)
                } else {
                    constant.value.to_string()
                }
            )?;
        }

        for function in self.functions {
            let args = function
                .args
                .iter()
                .map(|arg| format!("{} {}", Self::to_arg_ty(arg), Self::to_camel(&arg.name)))
                .join(", ");

            let attr = "[DllImport(DLL, CallingConvention = CallingConvention.Cdecl)]";
            let ret_attr = if function.return_ty == Type::Bool {
                "[return: MarshalAs(UnmanagedType.U1)]"
            } else {
                ""
            };

            writeln!(
                w,
                r"
            {}{} public static extern {} {}({});",
                attr,
                ret_attr,
                Self::to_return_ty(&function.return_ty),
                function.name,
                args
            )?;
        }

        writeln!(
            w,
            r"        }}
    }}"
        )?;

        for e in self.enums {
            write!(
                w,
                r"
    public enum {} : {}",
                e.name,
                Self::to_return_ty(&e.ty)
            )?;

            writeln!(
                w,
                r"
    {{",
            )?;

            for (i, v) in &e.values {
                writeln!(w, r"        {} = {},", Self::to_pascal(i), v)?;
            }

            writeln!(w, r"    }}",)?;
        }

        writeln!(
            w,
            r"
}}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif

"
        )?;

        Ok(())
    }
}
