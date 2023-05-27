#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::{c_char, CStr},
    fs::{self, File, OpenOptions},
    io::{BufReader, Write},
    path::Path,
};

pub const ERR: i32 = -1;

use autd3capi_common::*;

use autd3_simulator::{Simulator, ViewerSettings};

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSimulator() -> ConstPtr {
    Box::into_raw(Box::new(Simulator::new())) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSimulatorPort(simulator: ConstPtr, port: u16) -> ConstPtr {
    let simulator = Box::from_raw(simulator as *mut Simulator).port(port);
    Box::into_raw(Box::new(simulator)) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSimulatorWindowSize(
    simulator: ConstPtr,
    width: u32,
    height: u32,
) -> ConstPtr {
    let simulator = Box::from_raw(simulator as *mut Simulator).window_size(width, height);
    Box::into_raw(Box::new(simulator)) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSimulatorVsync(simulator: ConstPtr, vsync: bool) -> ConstPtr {
    let simulator = Box::from_raw(simulator as *mut Simulator).vsync(vsync);
    Box::into_raw(Box::new(simulator)) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSimulatorGpuIdx(simulator: ConstPtr, idx: i32) -> ConstPtr {
    let simulator = Box::from_raw(simulator as *mut Simulator).gpu_idx(idx);
    Box::into_raw(Box::new(simulator)) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSimulatorSettingsPath(
    simulator: ConstPtr,
    path: *const c_char,
    err: *mut c_char,
) -> ConstPtr {
    let file = try_or_return!(
        File::open(CStr::from_ptr(path).to_str().unwrap()),
        err,
        NULL
    );
    let reader = BufReader::new(file);
    let settings: ViewerSettings = try_or_return!(serde_json::from_reader(reader), err, NULL);
    let simulator = Box::from_raw(simulator as *mut Simulator).settings(settings);
    Box::into_raw(Box::new(simulator)) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSimulatorRun(
    simulator: ConstPtr,
    path: *const c_char,
    err: *mut c_char,
) -> i32 {
    let mut simulator = Box::from_raw(simulator as *mut Simulator);
    let res = simulator.run();

    let settings_str = try_or_return!(
        serde_json::to_string_pretty(simulator.get_settings()),
        err,
        ERR
    );

    let path = CStr::from_ptr(path).to_str().unwrap();

    if Path::new(path).exists() {
        fs::remove_file(path).unwrap();
    }

    let mut file = try_or_return!(
        OpenOptions::new()
            .create_new(true)
            .write(true)
            .append(false)
            .open(path),
        err,
        ERR
    );

    try_or_return!(write!(file, "{}", settings_str), err, ERR);

    res
}

#[cfg(test)]
mod tests {
    use std::ffi::{c_char, CStr};

    use super::*;

    #[test]
    fn run_simulator() {
        unsafe {
            let simulator = AUTDLinkSimulator();
            let simulator = AUTDLinkSimulatorPort(simulator, 8080);
            let simulator = AUTDLinkSimulatorWindowSize(simulator, 800, 600);
            let simulator = AUTDLinkSimulatorVsync(simulator, true);
            let simulator = AUTDLinkSimulatorGpuIdx(simulator, -1);
            let mut err = vec![c_char::default(); 256];
            let simulator_ = AUTDLinkSimulatorSettingsPath(
                simulator,
                CStr::from_bytes_with_nul(b"settings.json\0")
                    .unwrap()
                    .as_ptr(),
                err.as_mut_ptr(),
            );

            #[allow(unused_variables)]
            let simulator = if simulator_ == NULL {
                eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                simulator
            } else {
                simulator_
            };

            // let res = AUTDLinkSimulatorRun(
            //     simulator,
            //     CStr::from_bytes_with_nul(b"settings.json\0")
            //         .unwrap()
            //         .as_ptr(),
            //     err.as_mut_ptr(),
            // );
            // if res == -1 {
            //     eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
            // }
        }
    }
}
