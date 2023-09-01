/*
 * File: modulation.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{collections::HashMap, fmt};

use crate::{
    defined::{float, PI},
    error::AUTDInternalError,
    fpga::MOD_BUF_SIZE_MAX,
    geometry::{Device, Transducer},
    operation::{Operation, TypeTag},
};

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct ModulationControlFlags : u8 {
        const NONE      = 0;
        const MOD_BEGIN = 1 << 0;
        const MOD_END   = 1 << 1;
    }
}

impl fmt::Display for ModulationControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();
        if self.contains(ModulationControlFlags::MOD_BEGIN) {
            flags.push("MOD_BEGIN")
        }
        if self.contains(ModulationControlFlags::MOD_END) {
            flags.push("MOD_END")
        }
        if self.is_empty() {
            flags.push("NONE")
        }
        write!(
            f,
            "{}",
            flags
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" | ")
        )
    }
}

#[derive(Default)]
pub struct ModulationOp {
    buf: Vec<u8>,
    freq_div: u32,
    remains: HashMap<usize, usize>,
    sent: HashMap<usize, usize>,
}

impl ModulationOp {
    pub fn to_duty(amp: &float) -> u8 {
        (amp.clamp(0., 1.).asin() * 2.0 / PI * 255.0) as u8
    }

    pub fn new(buf: Vec<float>, freq_div: u32) -> Self {
        Self {
            buf: buf.iter().map(Self::to_duty).collect(),
            freq_div,
            remains: HashMap::new(),
            sent: HashMap::new(),
        }
    }
}

impl<T: Transducer> Operation<T> for ModulationOp {
    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        assert!(self.remains[&device.idx()] > 0);

        if self.buf.len() < 2 || self.buf.len() > MOD_BUF_SIZE_MAX {
            return Err(AUTDInternalError::ModulationSizeOutOfRange(self.buf.len()));
        }

        tx[0] = TypeTag::Modulation as u8;

        let sent = self.sent[&device.idx()];
        let mut offset = 4;
        if sent == 0 {
            offset += 4;
        }
        let mod_size = (self.buf.len() - sent).min(tx.len() - offset);
        assert!(mod_size > 0);

        let mut f = ModulationControlFlags::NONE;
        f.set(ModulationControlFlags::MOD_BEGIN, sent == 0);
        f.set(
            ModulationControlFlags::MOD_END,
            sent + mod_size == self.buf.len(),
        );
        tx[1] = f.bits();

        tx[2] = (mod_size & 0xFF) as u8;
        tx[3] = (mod_size >> 8) as u8;

        if sent == 0 {
            tx[4] = (self.freq_div & 0xFF) as u8;
            tx[5] = ((self.freq_div >> 8) & 0xFF) as u8;
            tx[6] = ((self.freq_div >> 16) & 0xFF) as u8;
            tx[7] = ((self.freq_div >> 24) & 0xFF) as u8;
        }

        unsafe {
            std::ptr::copy_nonoverlapping(
                self.buf[sent..].as_ptr(),
                (&mut tx[offset..]).as_mut_ptr() as *mut u8,
                mod_size,
            )
        }

        self.sent.insert(device.idx(), sent + mod_size);

        Ok(mod_size)
    }

    fn required_size(&self, device: &Device<T>) -> usize {
        if self.sent[&device.idx()] == 0 {
            7
        } else {
            5
        }
    }

    fn init(&mut self, device: &Device<T>) {
        self.remains.insert(device.idx(), self.buf.len());
        self.sent.insert(device.idx(), 0);
    }

    fn remains(&self, device: &Device<T>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<T>) {
        self.remains
            .insert(device.idx(), self.buf.len() - self.sent[&device.idx()]);
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::float;

//     const NUM_TRANS_IN_UNIT: usize = 249;

//     #[test]
//     fn modulation() {
//         let mut tx = TxDatagram::new(&[
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//         ]);

//         let size = MOD_HEADER_INITIAL_DATA_SIZE + MOD_HEADER_SUBSEQUENT_DATA_SIZE + 1;
//         let mod_data = (0..size)
//             .map(|i| (i as float) / (size as float))
//             .collect::<Vec<_>>();

//         let mut op = Modulation::new(mod_data.clone(), 512);
//         op.init();
//         assert!(!op.is_finished());

//         op.pack(&mut tx).unwrap();
//         assert!(!op.is_finished());
//         assert!(tx.header().cpu_flag.contains(CPUControlFlags::MOD));
//         assert!(tx.header().cpu_flag.contains(CPUControlFlags::MOD_BEGIN));
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD_END));
//         assert_eq!(tx.header().size as usize, MOD_HEADER_INITIAL_DATA_SIZE);
//         assert_eq!(tx.header().mod_initial().freq_div, 4096);
//         tx.header()
//             .mod_initial()
//             .data
//             .iter()
//             .zip(mod_data.iter())
//             .for_each(|(&h, &d)| {
//                 assert_eq!(h, Modulation::to_duty(d));
//             });

//         op.pack(&mut tx).unwrap();
//         assert!(!op.is_finished());
//         assert!(tx.header().cpu_flag.contains(CPUControlFlags::MOD));
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD_BEGIN));
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD_END));
//         assert_eq!(tx.header().size as usize, MOD_HEADER_SUBSEQUENT_DATA_SIZE);
//         (0..MOD_HEADER_SUBSEQUENT_DATA_SIZE).for_each(|i| {
//             assert_eq!(
//                 tx.header().mod_subsequent().data[i],
//                 Modulation::to_duty(mod_data[MOD_HEADER_INITIAL_DATA_SIZE + i])
//             );
//         });

//         op.pack(&mut tx).unwrap();
//         assert!(op.is_finished());
//         assert!(tx.header().cpu_flag.contains(CPUControlFlags::MOD));
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD_BEGIN));
//         assert!(tx.header().cpu_flag.contains(CPUControlFlags::MOD_END));
//         assert_eq!(tx.header().size as usize, 1);
//         (0..1).for_each(|i| {
//             assert_eq!(
//                 tx.header().mod_subsequent().data[i],
//                 Modulation::to_duty(
//                     mod_data[MOD_HEADER_INITIAL_DATA_SIZE + MOD_HEADER_SUBSEQUENT_DATA_SIZE + i]
//                 )
//             );
//         });

//         op.init();
//         assert!(!op.is_finished());

//         let mut op = Modulation::new(mod_data, 511);
//         op.init();
//         assert!(op.pack(&mut tx).is_err());

//         let mut op = Modulation::new(vec![0.0; MOD_BUF_SIZE_MAX + 1], 512);
//         op.init();
//         assert!(op.pack(&mut tx).is_err());
//     }
// }
