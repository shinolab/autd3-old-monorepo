/*
 * File: python.rs
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

pub struct PythonGenerator {
    functions: Vec<Function>,
    constants: Vec<Const>,
    enums: Vec<Enum>,
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

    fn to_return_ty(ty: &Type) -> &str {
        match ty {
            Type::Int8 => "ctypes.c_int8",
            Type::Int16 => "ctypes.c_int16",
            Type::Int32 => "ctypes.c_int32",
            Type::Int64 => "ctypes.c_int64",
            Type::UInt8 => "ctypes.c_uint8",
            Type::UInt16 => "ctypes.c_uint16",
            Type::UInt32 => "ctypes.c_uint32",
            Type::UInt64 => "ctypes.c_uint64",
            Type::Void => "None",
            Type::Char => "ctypes.c_char",
            Type::Float32 => "ctypes.c_float",
            Type::Float64 => "ctypes.c_double",
            Type::Bool => "ctypes.c_bool",
            Type::VoidPtr => "ctypes.c_void_p",
            _ => unimplemented!(),
        }
    }

    fn to_python_ty(ty: &Type) -> &str {
        match ty {
            Type::Int8 => "int",
            Type::Int16 => "int",
            Type::Int32 => "int",
            Type::Int64 => "int",
            Type::UInt8 => "int",
            Type::UInt16 => "int",
            Type::UInt32 => "int",
            Type::UInt64 => "int",
            Type::Float32 => "float",
            Type::Float64 => "float",
            Type::Bool => "bool",
            _ => unimplemented!(),
        }
    }

    fn to_arg(arg: &Arg) -> &str {
        match arg.pointer {
            0 => match arg.ty {
                Type::Int8 => "ctypes.c_int8",
                Type::Int16 => "ctypes.c_int16",
                Type::Int32 => "ctypes.c_int32",
                Type::Int64 => "ctypes.c_int64",
                Type::UInt8 => "ctypes.c_uint8",
                Type::UInt16 => "ctypes.c_uint16",
                Type::UInt32 => "ctypes.c_uint32",
                Type::UInt64 => "ctypes.c_uint64",
                Type::Void => panic!("void is not supported in argument"),
                Type::Char => "ctypes.c_char",
                Type::Float32 => "ctypes.c_float",
                Type::Float64 => "ctypes.c_double",
                Type::Bool => "ctypes.c_bool",
                Type::VoidPtr => "ctypes.c_void_p",
                Type::Custom(ref s) => s,
            },
            1 => match arg.ty {
                Type::Int8 => "ctypes.POINTER(ctypes.c_int8)",
                Type::Int16 => "ctypes.POINTER(ctypes.c_int16)",
                Type::Int32 => "ctypes.POINTER(ctypes.c_int32)",
                Type::Int64 => "ctypes.POINTER(ctypes.c_int64)",
                Type::UInt8 => "ctypes.POINTER(ctypes.c_uint8)",
                Type::UInt16 => "ctypes.POINTER(ctypes.c_uint16)",
                Type::UInt32 => "ctypes.POINTER(ctypes.c_uint32)",
                Type::UInt64 => "ctypes.POINTER(ctypes.c_uint64)",
                Type::Void => unimplemented!(),
                Type::Char => "ctypes.c_char_p",
                Type::Float32 => "ctypes.POINTER(ctypes.c_float)",
                Type::Float64 => "ctypes.POINTER(ctypes.c_double)",
                Type::Bool => "ctypes.POINTER(ctypes.c_bool)",
                Type::VoidPtr => "ctypes.POINTER(ctypes.c_void_p)",
                _ => unimplemented!(),
            },
            _ => {
                panic!("double or more pointer is not supported")
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
                _ => unimplemented!(),
            },
            _ => {
                panic!("double or more pointer is not supported")
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

        if crate_name != "autd3capi-def"
            && self
                .functions
                .iter()
                .any(|f| f.args.iter().any(|arg| matches!(arg.ty, Type::Custom(_))))
        {
            writeln!(w, r"from .autd3capi_def import *")?;
        }

        if !self.enums.is_empty() {
            writeln!(w, r"from enum import IntEnum")?;
        }

        for e in self.enums {
            writeln!(
                w,
                r"
class {}(IntEnum):",
                e.name
            )?;

            for (i, v) in &e.values {
                writeln!(w, r"    {} = {}", i, v)?;
            }

            writeln!(
                w,
                r"
    @classmethod
    def from_param(cls, obj):
        return int(obj)
"
            )?;
        }

        for constant in self.constants {
            write!(
                w,
                r"
{}: {} = {}",
                constant.name,
                Self::to_python_ty(&constant.ty),
                constant.value
            )?;
        }

        if crate_name == "autd3capi-def" {
            writeln!(w)?;
            return Ok(());
        }

        write!(
            w,
            r"


class Singleton(type):
    _instances = {{}} # type: ignore
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
        except FileNotFoundError:
            return",
            crate_name.replace('-', "_")
        )?;

        for function in self.functions.iter() {
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
                    " # type: ignore"
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
            )?;
        }

        for function in self.functions.iter() {
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
                    "None"
                } else {
                    Self::to_return_ty(&function.return_ty)
                },
                function.name,
                arg_names
            )?;
        }

        writeln!(w)?;

        Ok(())
    }
}
