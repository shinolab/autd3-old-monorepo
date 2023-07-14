/*
 * File: python.rs
 * Project: src
 * Created Date: 25/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;
use convert_case::{Case, Casing};

use std::{
    collections::HashSet,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use itertools::Itertools;

use crate::{
    parse::{Arg, Const, Enum, Function, Struct},
    types::{InOut, Type},
};

use crate::traits::Generator;

pub struct PythonGenerator {
    functions: Vec<Function>,
    constants: Vec<Const>,
    enums: Vec<Enum>,
    structs: Vec<Struct>,
}

impl PythonGenerator {
    fn sub_reserve(str: String) -> String {
        if str == "lambda" {
            "lambda_".to_string()
        } else {
            str
        }
    }

    fn to_python_func_name(name: &str) -> String {
        let name = name[4..].to_string();
        name.to_case(Case::Snake)
    }

    fn to_return_ty(ty: &Type) -> String {
        match ty {
            Type::Int8 => "ctypes.c_int8".to_string(),
            Type::Int16 => "ctypes.c_int16".to_string(),
            Type::Int32 => "ctypes.c_int32".to_string(),
            Type::Int64 => "ctypes.c_int64".to_string(),
            Type::UInt8 => "ctypes.c_uint8".to_string(),
            Type::UInt16 => "ctypes.c_uint16".to_string(),
            Type::UInt32 => "ctypes.c_uint32".to_string(),
            Type::UInt64 => "ctypes.c_uint64".to_string(),
            Type::Void => "None".to_string(),
            Type::Char => "ctypes.c_char".to_string(),
            Type::Float32 => "ctypes.c_float".to_string(),
            Type::Float64 => "ctypes.c_double".to_string(),
            Type::Bool => "ctypes.c_bool".to_string(),
            Type::VoidPtr => "ctypes.c_void_p".to_string(),
            Type::Custom(ref s) => s.to_owned(),
        }
    }

    fn to_python_ty(ty: &Type) -> String {
        match ty {
            Type::Int8 => "int".to_string(),
            Type::Int16 => "int".to_string(),
            Type::Int32 => "int".to_string(),
            Type::Int64 => "int".to_string(),
            Type::UInt8 => "int".to_string(),
            Type::UInt16 => "int".to_string(),
            Type::UInt32 => "int".to_string(),
            Type::UInt64 => "int".to_string(),
            Type::Float32 => "float".to_string(),
            Type::Float64 => "float".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Custom(ref s) => format!("\"{}\"", s),
            Type::VoidPtr => "ctypes.c_void_p".to_string(),
            t => unimplemented!("{:?}", t),
        }
    }

    fn to_arg(arg: &Arg) -> String {
        match arg.pointer {
            0 => match arg.ty {
                Type::Int8 => "ctypes.c_int8".to_owned(),
                Type::Int16 => "ctypes.c_int16".to_owned(),
                Type::Int32 => "ctypes.c_int32".to_owned(),
                Type::Int64 => "ctypes.c_int64".to_owned(),
                Type::UInt8 => "ctypes.c_uint8".to_owned(),
                Type::UInt16 => "ctypes.c_uint16".to_owned(),
                Type::UInt32 => "ctypes.c_uint32".to_owned(),
                Type::UInt64 => "ctypes.c_uint64".to_owned(),
                Type::Void => panic!("void is not supported in argument"),
                Type::Char => "ctypes.c_char".to_owned(),
                Type::Float32 => "ctypes.c_float".to_owned(),
                Type::Float64 => "ctypes.c_double".to_owned(),
                Type::Bool => "ctypes.c_bool".to_owned(),
                Type::VoidPtr => "ctypes.c_void_p".to_owned(),
                Type::Custom(ref s) => s.to_owned(),
            },
            1 => match arg.ty {
                Type::Int8 => "ctypes.POINTER(ctypes.c_int8)".to_owned(),
                Type::Int16 => "ctypes.POINTER(ctypes.c_int16)".to_owned(),
                Type::Int32 => "ctypes.POINTER(ctypes.c_int32)".to_owned(),
                Type::Int64 => "ctypes.POINTER(ctypes.c_int64)".to_owned(),
                Type::UInt8 => "ctypes.POINTER(ctypes.c_uint8)".to_owned(),
                Type::UInt16 => "ctypes.POINTER(ctypes.c_uint16)".to_owned(),
                Type::UInt32 => "ctypes.POINTER(ctypes.c_uint32)".to_owned(),
                Type::UInt64 => "ctypes.POINTER(ctypes.c_uint64)".to_owned(),
                Type::Void => unimplemented!(),
                Type::Char => "ctypes.c_char_p".to_owned(),
                Type::Float32 => "ctypes.POINTER(ctypes.c_float)".to_owned(),
                Type::Float64 => "ctypes.POINTER(ctypes.c_double)".to_owned(),
                Type::Bool => "ctypes.POINTER(ctypes.c_bool)".to_owned(),
                Type::VoidPtr => "ctypes.POINTER(ctypes.c_void_p)".to_owned(),
                Type::Custom(ref s) => format!("ctypes.POINTER({})", s),
            },
            2 => match arg.ty {
                Type::Float32 => "ctypes.POINTER(ctypes.POINTER(ctypes.c_float))".to_owned(),
                Type::Float64 => "ctypes.POINTER(ctypes.POINTER(ctypes.c_double))".to_owned(),
                Type::Custom(ref s) => format!("ctypes.POINTER(ctypes.POINTER({}))", s),
                _ => unimplemented!(),
            },
            _ => {
                panic!("truple or more pointer is not supported")
            }
        }
    }

    fn to_python_arg(arg: &Arg) -> &str {
        match arg.pointer {
            0 => match arg.ty {
                Type::Int8 => "int",
                Type::Int16 => "int",
                Type::Int32 => "int",
                Type::Int64 => "int",
                Type::UInt8 => "int",
                Type::UInt16 => "int",
                Type::UInt32 => "int",
                Type::UInt64 => "int",
                Type::Void => panic!("void is not supported in argument"),
                Type::Char => panic!("void is not supported in argument"),
                Type::Float32 => "float",
                Type::Float64 => "float",
                Type::Bool => "bool",
                Type::VoidPtr => "ctypes.c_void_p",
                Type::Custom(ref s) => s,
            },
            1 => match arg.ty {
                Type::Int8 => "Any",
                Type::Int16 => "Any",
                Type::Int32 => "Any",
                Type::Int64 => "Any",
                Type::UInt8 => "Any",
                Type::UInt16 => "Any",
                Type::UInt32 => "Any",
                Type::UInt64 => "Any",
                Type::Void => unimplemented!(),
                Type::Char => match arg.inout {
                    InOut::In => "bytes",
                    InOut::Out => "ctypes.Array[ctypes.c_char]",
                    _ => "Any",
                },
                Type::Float32 => "Any",
                Type::Float64 => "Any",
                Type::Bool => "Any",
                Type::VoidPtr => "Any",
                Type::Custom(_) => "Any",
            },
            2 => "Any",
            _ => {
                panic!("triple or more pointer is not supported")
            }
        }
    }

    fn get_filename(name: &str) -> String {
        format!("{}.py", name.replace('-', "_"))
    }
}

impl Generator for PythonGenerator {
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

    fn register_struct(mut self, e: Vec<crate::parse::Struct>) -> Self {
        self.structs = e;
        self
    }

    fn new() -> Self {
        Self {
            functions: Vec::new(),
            constants: Vec::new(),
            enums: Vec::new(),
            structs: Vec::new(),
        }
    }

    fn write<P: AsRef<Path>>(self, path: P, crate_name: &str) -> Result<()> {
        let mut w = BufWriter::new(File::create(
            path.as_ref().join(Self::get_filename(crate_name)),
        )?);

        writeln!(
            w,
            r"# This file is autogenerated
import threading
import ctypes
import os"
        )?;

        if self
            .functions
            .iter()
            .any(|f| f.args.iter().any(|arg| Self::to_python_arg(arg) == "Any"))
        {
            writeln!(w, r"from typing import Any")?;
        }

        let owns = |ty: &Type| {
            if let Type::Custom(ref s) = ty {
                if self.enums.iter().any(|e| &e.name == s)
                    || self.structs.iter().any(|e| &e.name == s)
                {
                    return None;
                }
                Some(s.to_string())
            } else {
                None
            }
        };
        let used_ty: HashSet<_> = self
            .functions
            .iter()
            .flat_map(|f| {
                f.args
                    .iter()
                    .filter_map(|arg| owns(&arg.ty))
                    .chain([&f.return_ty].iter().filter_map(|&ty| owns(ty)))
                    .collect::<Vec<_>>()
            })
            .collect();

        if crate_name != "autd3capi-def" && !used_ty.is_empty() {
            writeln!(
                w,
                r"from .autd3capi_def import {}
",
                used_ty.iter().sorted().join(", ")
            )?;
        }

        if !self.enums.is_empty() {
            writeln!(w, r"from enum import IntEnum")?;
        }

        self.enums
            .iter()
            .map(|e| {
                writeln!(
                    w,
                    r"
class {}(IntEnum):",
                    e.name
                )?;

                e.values
                    .iter()
                    .map(|(i, v)| writeln!(w, r"    {} = {}", i, v))
                    .try_collect()?;

                writeln!(
                    w,
                    r"
    @classmethod
    def from_param(cls, obj):
        return int(obj)
"
                )
            })
            .try_collect()?;

        self.structs
            .iter()
            .filter(|e| e.name.ends_with("Ptr"))
            .map(|p| {
                writeln!(
                    w,
                    r"
class {}(ctypes.Structure):",
                    p.name
                )?;

                writeln!(
                    w,
                    "    _fields_ = [(\"_0\", ctypes.c_void_p)]
"
                )
            })
            .try_collect()?;

        self.structs
            .iter()
            .filter(|e| !e.name.ends_with("Ptr"))
            .map(|p| {
                writeln!(
                    w,
                    r"
class {}(ctypes.Structure):",
                    p.name
                )?;

                writeln!(
                    w,
                    "    _fields_ = [{}]
",
                    p.fields
                        .iter()
                        .map(|(ty, name)| format!("(\"{}\", {})", name, Self::to_return_ty(ty)))
                        .join(", ")
                )
            })
            .try_collect()?;

        self.constants
            .iter()
            .map(|constant| {
                write!(
                    w,
                    r"
{}: {} = {}",
                    constant.name,
                    Self::to_python_ty(&constant.ty),
                    constant.value
                )
            })
            .try_collect()?;

        if crate_name == "autd3capi-def" {
            writeln!(w)?;
            return Ok(());
        }

        write!(
            w,
            r"
class Singleton(type):
    _instances = {{}}  # type: ignore
    _lock = threading.Lock()

    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            with cls._lock:
                if cls not in cls._instances:
                    cls._instances[cls] = super(Singleton, cls).__call__(*args, **kwargs)
        return cls._instances[cls]


class NativeMethods(metaclass=Singleton):",
        )?;

        write!(
            w,
            r"

    def init_dll(self, bin_location: str, bin_prefix: str, bin_ext: str):
        try:
            self.dll = ctypes.CDLL(os.path.join(bin_location, f'{{bin_prefix}}{}{{bin_ext}}'))
        except Exception:
            return",
            crate_name.replace('-', "_")
        )?;

        self.functions
            .iter()
            .map(|function| {
                writeln!(w)?;
                let args = function.args.iter().map(Self::to_arg).join(", ");
                write!(
                    w,
                    r"
        self.dll.{}.argtypes = [{}]{}",
                    function.name,
                    args,
                    if function
                        .args
                        .iter()
                        .any(|arg| matches!(arg.ty, Type::Custom(_)))
                    {
                        "  # type: ignore"
                    } else {
                        ""
                    }
                )?;
                write!(
                    w,
                    r" 
        self.dll.{}.restype = {}",
                    function.name,
                    Self::to_return_ty(&function.return_ty)
                )
            })
            .try_collect()?;

        self.functions
            .iter()
            .map(|function| {
                writeln!(w)?;
                let args = function
                    .args
                    .iter()
                    .map(|arg| {
                        format!(
                            "{}: {}",
                            Self::sub_reserve(arg.name.to_owned()),
                            Self::to_python_arg(arg)
                        )
                    })
                    .join(", ");
                let arg_names = function
                    .args
                    .iter()
                    .map(|arg| Self::sub_reserve(arg.name.to_owned()))
                    .join(", ");

                write!(
                    w,
                    r"
    def {}(self{}{}) -> {}:
        return self.dll.{}({})",
                    Self::to_python_func_name(&function.name),
                    if function.args.is_empty() { "" } else { ", " },
                    args,
                    if function.return_ty == Type::Void {
                        "None".to_string()
                    } else {
                        Self::to_return_ty(&function.return_ty)
                    },
                    function.name,
                    arg_names
                )
            })
            .try_collect()?;

        writeln!(w)?;

        Ok(())
    }
}
