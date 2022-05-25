/*
 * File: python.rs
 * Project: src
 * Created Date: 25/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
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

pub struct PythonGenerator {}

impl PythonGenerator {
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
        }
    }

    fn to_arg(arg: &Arg) -> &str {
        match arg.pointer() {
            0 => match arg.ty() {
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
            },
            1 => match arg.ty() {
                Type::Int8 => "ctypes.POINTER(ctypes.c_int8)",
                Type::Int16 => "ctypes.POINTER(ctypes.c_int16)",
                Type::Int32 => "ctypes.POINTER(ctypes.c_int32)",
                Type::Int64 => "ctypes.POINTER(ctypes.c_int64)",
                Type::UInt8 => "ctypes.POINTER(ctypes.c_uint8)",
                Type::UInt16 => "ctypes.POINTER(ctypes.c_uint16)",
                Type::UInt32 => "ctypes.POINTER(ctypes.c_uint32)",
                Type::UInt64 => "ctypes.POINTER(ctypes.c_uint64)",
                Type::Void => "ctypes.c_void_p",
                Type::Char => "ctypes.c_char_p",
                Type::Float32 => "ctypes.POINTER(ctypes.c_float)",
                Type::Float64 => "ctypes.POINTER(ctypes.c_double)",
                Type::Bool => "ctypes.POINTER(ctypes.c_bool)",
            },
            2 => match arg.ty() {
                Type::Void => "ctypes.POINTER(ctypes.c_void_p)",
                _ => panic!("double pointer is not supported, but void**"),
            },
            _ => {
                panic!("triple or more pointer is not supported")
            }
        }
    }
}

impl Generator for PythonGenerator {
    fn print_header<W: Write>(w: &mut W, bin_name: &str) -> Result<()> {
        write!(
            w,
            r"# This file was automatically generated from header file
import threading
import ctypes
import os


class Singleton(type):
    _instances = {{}}
    _lock = threading.Lock()

    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            with cls._lock:
                if cls not in cls._instances:
                    cls._instances[cls] = super(Singleton, cls).__call__(*args, **kwargs)
        return cls._instances[cls]


class NativeMethods(metaclass=Singleton):
    def init_dll(self, bin_location: str, bin_prefix: str, version_triple: str, bin_ext: str):
        self.dll = ctypes.CDLL(os.path.join(bin_location, f'{{bin_prefix}}{}-{{version_triple}}{{bin_ext}}'))",
            bin_name
        )?;
        Ok(())
    }

    fn register_func<W: Write>(w: &mut W, function: &Function) -> Result<()> {
        writeln!(w)?;
        let args = function.args().iter().map(Self::to_arg).join(", ");
        write!(
            w,
            r"
        self.dll.{}.argtypes = [{}]",
            function.name(),
            args
        )?;
        write!(
            w,
            r" 
        self.dll.{}.restype = {}",
            function.name(),
            Self::to_return_ty(&function.return_ty())
        )?;
        Ok(())
    }

    fn print_footer<W: Write>(w: &mut W) -> Result<()> {
        writeln!(w)?;
        Ok(())
    }

    fn get_filename(name: &str) -> String {
        format!("{}.py", name)
    }
}
