/*
 * File: rx_message.rs
 * Project: cpu
 * Created Date: 29/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::cpu::{Header, EC_OUTPUT_FRAME_SIZE};

#[derive(Clone)]
pub struct TxDatagram {
    data: Vec<u8>,
    num_devices: usize,
}

impl TxDatagram {
    pub fn new(num_devices: usize) -> Self {
        Self {
            data: vec![0x00; (EC_OUTPUT_FRAME_SIZE) * num_devices],
            num_devices,
        }
    }

    pub fn num_devices(&self) -> usize {
        self.num_devices
    }

    pub fn all_data(&self) -> &[u8] {
        &self.data
    }

    pub fn all_data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn data(&self, i: usize) -> &[u8] {
        &self.data[i * EC_OUTPUT_FRAME_SIZE..(i + 1) * EC_OUTPUT_FRAME_SIZE]
    }

    pub fn headers(&self) -> impl Iterator<Item = &Header> {
        (0..self.num_devices).map(|i| self.header(i))
    }

    pub fn header_mut(&mut self, i: usize) -> &mut Header {
        unsafe {
            (self.data[i * EC_OUTPUT_FRAME_SIZE..].as_mut_ptr() as *mut Header)
                .as_mut()
                .unwrap()
        }
    }

    pub fn payload_mut(&mut self, i: usize) -> &mut [u8] {
        &mut self.data[i * EC_OUTPUT_FRAME_SIZE + std::mem::size_of::<Header>()
            ..(i + 1) * EC_OUTPUT_FRAME_SIZE]
    }

    pub fn payloads(&self) -> impl Iterator<Item = &[u8]> {
        (0..self.num_devices).map(|i| self.payload(i))
    }

    fn header(&self, i: usize) -> &Header {
        unsafe {
            (self.data[i * EC_OUTPUT_FRAME_SIZE..].as_ptr() as *const Header)
                .as_ref()
                .unwrap()
        }
    }

    fn payload(&self, i: usize) -> &[u8] {
        &self.data[i * EC_OUTPUT_FRAME_SIZE + std::mem::size_of::<Header>()
            ..(i + 1) * EC_OUTPUT_FRAME_SIZE]
    }
}

#[cfg(test)]
mod tests {
    use crate::fpga::FPGAControlFlags;

    use super::*;

    #[test]
    fn num_devices() {
        let tx = TxDatagram::new(2);
        assert_eq!(2, tx.num_devices());
    }

    #[test]
    fn all_data() {
        let tx = TxDatagram::new(2);
        assert_eq!(2 * EC_OUTPUT_FRAME_SIZE, tx.all_data().len());
    }

    #[test]
    fn clone() {
        let mut tx = TxDatagram::new(2);

        tx.all_data_mut().iter_mut().enumerate().for_each(|(i, d)| {
            *d = i as u8;
        });

        let tx2 = tx.clone();

        assert_eq!(tx.num_devices(), tx2.num_devices());
        assert_eq!(tx.all_data(), tx2.all_data());
    }

    #[test]
    fn data() {
        let mut tx = TxDatagram::new(2);

        tx.all_data_mut().iter_mut().enumerate().for_each(|(i, d)| {
            *d = i as u8;
        });

        assert_eq!(EC_OUTPUT_FRAME_SIZE, tx.data(0).len());
        assert_eq!(0, *tx.data(0).first().unwrap());
        assert_eq!(
            (EC_OUTPUT_FRAME_SIZE - 1) as u8,
            *tx.data(0).last().unwrap()
        );

        assert_eq!(EC_OUTPUT_FRAME_SIZE, tx.data(1).len());
        assert_eq!(EC_OUTPUT_FRAME_SIZE as u8, *tx.data(1).first().unwrap());
        assert_eq!(
            (2 * EC_OUTPUT_FRAME_SIZE - 1) as u8,
            *tx.data(1).last().unwrap()
        );
    }

    #[test]
    fn header() {
        let mut tx = TxDatagram::new(2);

        tx.header_mut(0).msg_id = 0x01;
        tx.header_mut(0).fpga_flag = FPGAControlFlags::FORCE_FAN;
        tx.header_mut(0).slot_2_offset = 0x02;

        tx.header_mut(1).msg_id = 0x03;
        tx.header_mut(1).fpga_flag = FPGAControlFlags::READS_FPGA_INFO;
        tx.header_mut(1).slot_2_offset = 0x04;

        let headers = tx.headers().collect::<Vec<_>>();
        assert_eq!(2, headers.len());
        assert_eq!(0x01, headers[0].msg_id);
        assert_eq!(
            FPGAControlFlags::FORCE_FAN.bits(),
            headers[0].fpga_flag.bits()
        );
        assert_eq!(0x02, headers[0].slot_2_offset);
        assert_eq!(0x03, headers[1].msg_id);
        assert_eq!(
            FPGAControlFlags::READS_FPGA_INFO.bits(),
            headers[1].fpga_flag.bits()
        );
        assert_eq!(0x04, headers[1].slot_2_offset);

        assert_eq!(0x01, tx.all_data()[memoffset::offset_of!(Header, msg_id)]);
        assert_eq!(
            FPGAControlFlags::FORCE_FAN.bits(),
            tx.all_data()[memoffset::offset_of!(Header, fpga_flag)]
        );
        assert_eq!(
            0x02,
            tx.all_data()[memoffset::offset_of!(Header, slot_2_offset)]
        );
        assert_eq!(
            0x03,
            tx.all_data()[EC_OUTPUT_FRAME_SIZE + memoffset::offset_of!(Header, msg_id)]
        );
        assert_eq!(
            FPGAControlFlags::READS_FPGA_INFO.bits(),
            tx.all_data()[EC_OUTPUT_FRAME_SIZE + memoffset::offset_of!(Header, fpga_flag)]
        );
        assert_eq!(
            0x04,
            tx.all_data()[EC_OUTPUT_FRAME_SIZE + memoffset::offset_of!(Header, slot_2_offset)]
        );
    }

    #[test]
    fn payload() {
        let mut tx = TxDatagram::new(2);

        assert_eq!(
            EC_OUTPUT_FRAME_SIZE - std::mem::size_of::<Header>(),
            tx.payload_mut(0).len()
        );
        *tx.payload_mut(0).first_mut().unwrap() = 0x01;
        *tx.payload_mut(0).last_mut().unwrap() = 0x02;

        assert_eq!(
            EC_OUTPUT_FRAME_SIZE - std::mem::size_of::<Header>(),
            tx.payload_mut(1).len()
        );
        *tx.payload_mut(1).first_mut().unwrap() = 0x03;
        *tx.payload_mut(1).last_mut().unwrap() = 0x04;

        let payloads = tx.payloads().collect::<Vec<_>>();
        assert_eq!(2, payloads.len());
        assert_eq!(
            EC_OUTPUT_FRAME_SIZE - std::mem::size_of::<Header>(),
            payloads[0].len()
        );
        assert_eq!(
            EC_OUTPUT_FRAME_SIZE - std::mem::size_of::<Header>(),
            payloads[1].len()
        );
        assert_eq!(0x01, *payloads[0].first().unwrap());
        assert_eq!(0x02, *payloads[0].last().unwrap());
        assert_eq!(0x03, *payloads[1].first().unwrap());
        assert_eq!(0x04, *payloads[1].last().unwrap());

        assert_eq!(0x01, tx.all_data()[std::mem::size_of::<Header>()]);
        assert_eq!(0x02, tx.all_data()[EC_OUTPUT_FRAME_SIZE - 1]);
        assert_eq!(
            0x03,
            tx.all_data()[EC_OUTPUT_FRAME_SIZE + std::mem::size_of::<Header>()]
        );
        assert_eq!(0x04, tx.all_data()[2 * EC_OUTPUT_FRAME_SIZE - 1]);
    }
}
