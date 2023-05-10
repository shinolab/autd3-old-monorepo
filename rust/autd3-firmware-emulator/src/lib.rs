/*
 * File: lib.rs
 * Project: src
 * Created Date: 06/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

pub mod cpu;
pub mod error;
pub mod fpga;

pub use cpu::emulator::CPUEmulator;
pub use fpga::emulator::FPGAEmulator;
