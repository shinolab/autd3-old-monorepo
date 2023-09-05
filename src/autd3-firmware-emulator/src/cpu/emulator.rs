/*
 * File: emulator.rs
 * Project: src
 * Created Date: 06/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{
    cpu::{Header, TxDatagram},
    fpga::FPGAControlFlags,
};

use crate::fpga::emulator::FPGAEmulator;

use super::params::*;

pub struct CPUEmulator {
    id: usize,
    ack: u8,
    rx_data: u8,
    read_fpga_info: bool,
    mod_cycle: u32,
    stm_cycle: u32,
    fpga: FPGAEmulator,
    cycles: Vec<u16>,
    synchronized: bool,
    num_transducers: usize,
    fpga_flags: FPGAControlFlags,
    fpga_flags_internal: u16,
}

impl CPUEmulator {
    pub fn new(id: usize, num_transducers: usize) -> Self {
        let mut s = Self {
            id,
            ack: 0x00,
            rx_data: 0x00,
            read_fpga_info: false,
            mod_cycle: 0,
            stm_cycle: 0,
            fpga: FPGAEmulator::new(num_transducers),
            cycles: vec![0x0000; num_transducers],
            synchronized: false,
            num_transducers,
            fpga_flags: FPGAControlFlags::empty(),
            fpga_flags_internal: 0x0000,
        };
        s.init();
        s
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn num_transducers(&self) -> usize {
        self.num_transducers
    }

    pub fn msg_id(&self) -> u8 {
        self.ack
    }

    pub fn ack(&self) -> u8 {
        self.rx_data
    }

    pub fn fpga_flags(&self) -> FPGAControlFlags {
        self.fpga_flags
    }

    pub fn fpga(&self) -> &FPGAEmulator {
        &self.fpga
    }

    pub fn fpga_mut(&mut self) -> &mut FPGAEmulator {
        &mut self.fpga
    }

    pub fn synchronized(&self) -> bool {
        self.synchronized
    }

    pub fn send(&mut self, tx: &TxDatagram) {
        self.ecat_recv(tx.data());
    }

    pub fn init(&mut self) {
        self.fpga.init();
        self.cycles.fill(0x1000);
        self.clear();
    }

    pub fn update(&mut self) {
        if self.should_update() {
            self.rx_data = self.read_fpga_info() as _;
        }
    }

    pub fn should_update(&self) -> bool {
        self.read_fpga_info
    }
}

impl CPUEmulator {
    fn get_addr(select: u8, addr: u16) -> u16 {
        ((select as u16 & 0x0003) << 14) | (addr & 0x3FFF)
    }

    fn bram_read(&self, select: u8, addr: u16) -> u16 {
        let addr = Self::get_addr(select, addr);
        self.fpga.read(addr)
    }

    fn bram_write(&mut self, select: u8, addr: u16, data: u16) {
        let addr = Self::get_addr(select, addr);
        self.fpga.write(addr, data)
    }

    fn bram_cpy(&mut self, select: u8, addr_base: u16, data: *const u16, size: usize) {
        let mut addr = Self::get_addr(select, addr_base);
        let mut src = data;
        (0..size).for_each(|_| unsafe {
            self.fpga.write(addr, src.read());
            addr += 1;
            src = src.add(1);
        })
    }

    fn bram_set(&mut self, select: u8, addr_base: u16, value: u16, size: usize) {
        let mut addr = Self::get_addr(select, addr_base);
        (0..size).for_each(|_| {
            self.fpga.write(addr, value);
            addr += 1;
        })
    }

    fn synchronize(&mut self, data: &[u8]) {
        let cycles = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u16, self.num_transducers)
        };

        self.bram_cpy(
            BRAM_SELECT_CONTROLLER,
            BRAM_ADDR_CYCLE_BASE,
            cycles.as_ptr(),
            cycles.len(),
        );

        self.cycles.copy_from_slice(cycles);

        self.synchronized = true;

        // Do nothing to sync
    }

    fn write_mod(&mut self, data: &[u8]) {
        let flag = data[1];

        let write = ((data[3] as u16) << 8) | data[2] as u16;
        let data = if (flag & MODULATION_FLAG_BEGIN) == MODULATION_FLAG_BEGIN {
            self.mod_cycle = 0;
            self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_ADDR_OFFSET, 0);
            let freq_div = ((data[7] as u32) << 24)
                | ((data[6] as u32) << 16)
                | ((data[5] as u32) << 8)
                | data[4] as u32;
            self.bram_cpy(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_MOD_FREQ_DIV_0,
                &freq_div as *const _ as _,
                std::mem::size_of::<u32>() >> 1,
            );
            data[8..].as_ptr() as *const u16
        } else {
            data[4..].as_ptr() as *const u16
        };

        let segment_capacity =
            (self.mod_cycle & !MOD_BUF_SEGMENT_SIZE_MASK) + MOD_BUF_SEGMENT_SIZE - self.mod_cycle;

        if write as u32 <= segment_capacity {
            self.bram_cpy(
                BRAM_SELECT_MOD,
                ((self.mod_cycle & MOD_BUF_SEGMENT_SIZE_MASK) >> 1) as u16,
                data,
                ((write + 1) >> 1) as usize,
            );
            self.mod_cycle += write as u32;
        } else {
            self.bram_cpy(
                BRAM_SELECT_MOD,
                ((self.mod_cycle & MOD_BUF_SEGMENT_SIZE_MASK) >> 1) as u16,
                data,
                (segment_capacity >> 1) as usize,
            );
            self.mod_cycle += segment_capacity;
            let data = unsafe { data.add(segment_capacity as _) };
            self.bram_write(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_MOD_ADDR_OFFSET,
                ((self.mod_cycle & !MOD_BUF_SEGMENT_SIZE_MASK) >> MOD_BUF_SEGMENT_SIZE_WIDTH)
                    as u16,
            );
            self.bram_cpy(
                BRAM_SELECT_MOD,
                ((self.mod_cycle & MOD_BUF_SEGMENT_SIZE_MASK) >> 1) as _,
                data,
                ((write as u32 - segment_capacity + 1) >> 1) as _,
            );
            self.mod_cycle += write as u32 - segment_capacity;
        }

        if (flag & MODULATION_FLAG_END) == MODULATION_FLAG_END {
            self.bram_write(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_MOD_CYCLE,
                (self.mod_cycle.max(1) - 1) as _,
            );
        }
    }

    fn config_silencer(&mut self, data: &[u8]) {
        let step = ((data[3] as u16) << 8) | data[2] as u16;
        self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_STEP, step);
    }

    fn write_mod_delay(&mut self, data: &[u8]) {
        let delays = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u16, self.num_transducers)
        };
        self.bram_cpy(
            BRAM_SELECT_CONTROLLER,
            BRAM_ADDR_MOD_DELAY_BASE,
            delays.as_ptr(),
            delays.len(),
        );
    }

    fn write_duty_filter(&mut self, data: &[u8]) {
        let filter = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u16, self.num_transducers)
        };
        self.bram_cpy(
            BRAM_SELECT_CONTROLLER,
            BRAM_ADDR_FILTER_DUTY_BASE,
            filter.as_ptr(),
            filter.len(),
        );
    }

    fn write_phase_filter(&mut self, data: &[u8]) {
        let filter = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u16, self.num_transducers)
        };
        self.bram_cpy(
            BRAM_SELECT_CONTROLLER,
            BRAM_ADDR_FILTER_PHASE_BASE,
            filter.as_ptr(),
            filter.len(),
        );
    }

    fn write_filter(&mut self, data: &[u8]) {
        let flag = data[1];
        if (flag & FILTER_ADD_DUTY) == FILTER_ADD_DUTY {
            self.write_duty_filter(&data[2..]);
        } else if (flag & FILTER_ADD_PHASE) == FILTER_ADD_PHASE {
            self.write_phase_filter(&data[2..]);
        } else {
            unimplemented!("unknown filter type: {flag}")
        }
    }

    fn write_gain(&mut self, data: &[u8]) {
        self.fpga_flags_internal &= !CTL_FLAG_OP_MODE;

        let flag = data[1];

        let data = unsafe {
            std::slice::from_raw_parts(data[2..].as_ptr() as *const u16, (data.len() - 2) >> 1)
        };

        if (flag & GAIN_FLAG_LEGACY) == GAIN_FLAG_LEGACY {
            (0..self.num_transducers)
                .for_each(|i| self.bram_write(BRAM_SELECT_NORMAL, (i << 1) as _, data[i]));

            self.fpga_flags_internal |= CTL_REG_LEGACY_MODE;
        } else if (flag & GAIN_FLAG_DUTY) == GAIN_FLAG_DUTY {
            (0..self.num_transducers)
                .for_each(|i| self.bram_write(BRAM_SELECT_NORMAL, (i << 1) as u16 + 1, data[i]));
            self.fpga_flags_internal &= !CTL_REG_LEGACY_MODE;
        } else {
            (0..self.num_transducers)
                .for_each(|i| self.bram_write(BRAM_SELECT_NORMAL, (i << 1) as u16, data[i]));
            self.fpga_flags_internal &= !CTL_REG_LEGACY_MODE;
        }
    }

    fn write_focus_stm(&mut self, data: &[u8]) {
        let flag = data[1];

        let size = (data[3] as u32) << 8 | data[2] as u32;

        let mut src = if (flag & FOCUS_STM_FLAG_BEGIN) == FOCUS_STM_FLAG_BEGIN {
            self.stm_cycle = 0;
            self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET, 0);
            let freq_div = ((data[7] as u32) << 24)
                | ((data[6] as u32) << 16)
                | ((data[5] as u32) << 8)
                | data[4] as u32;
            let sound_speed = ((data[11] as u32) << 24)
                | ((data[10] as u32) << 16)
                | ((data[9] as u32) << 8)
                | data[8] as u32;
            let start_idx = ((data[13] as u16) << 8) | data[12] as u16;
            let finish_idx = ((data[15] as u16) << 8) | data[14] as u16;

            self.bram_cpy(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_STM_FREQ_DIV_0,
                &freq_div as *const _ as _,
                std::mem::size_of::<u32>() >> 1,
            );
            self.bram_cpy(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_SOUND_SPEED_0,
                &sound_speed as *const _ as _,
                std::mem::size_of::<u32>() >> 1,
            );

            self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_START_IDX, start_idx);
            self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FINISH_IDX, finish_idx);

            unsafe { data.as_ptr().add(16) as *const u16 }
        } else {
            unsafe { data.as_ptr().add(4) as *const u16 }
        };

        let segment_capacity = (self.stm_cycle & !POINT_STM_BUF_SEGMENT_SIZE_MASK)
            + POINT_STM_BUF_SEGMENT_SIZE
            - self.stm_cycle;
        if size <= segment_capacity {
            let mut dst = ((self.stm_cycle & POINT_STM_BUF_SEGMENT_SIZE_MASK) << 3) as u16;
            (0..size as usize).for_each(|_| unsafe {
                self.bram_write(BRAM_SELECT_STM, dst, src.read());
                dst += 1;
                src = src.add(1);
                self.bram_write(BRAM_SELECT_STM, dst, src.read());
                dst += 1;
                src = src.add(1);
                self.bram_write(BRAM_SELECT_STM, dst, src.read());
                dst += 1;
                src = src.add(1);
                self.bram_write(BRAM_SELECT_STM, dst, src.read());
                dst += 1;
                src = src.add(1);
                dst += 4;
            });
            self.stm_cycle += size;
        } else {
            let mut dst = ((self.stm_cycle & POINT_STM_BUF_SEGMENT_SIZE_MASK) << 3) as u16;
            (0..segment_capacity as usize).for_each(|_| unsafe {
                self.bram_write(BRAM_SELECT_STM, dst, src.read());
                dst += 1;
                src = src.add(1);
                self.bram_write(BRAM_SELECT_STM, dst, src.read());
                dst += 1;
                src = src.add(1);
                self.bram_write(BRAM_SELECT_STM, dst, src.read());
                dst += 1;
                src = src.add(1);
                self.bram_write(BRAM_SELECT_STM, dst, src.read());
                dst += 1;
                src = src.add(1);
                dst += 4;
            });
            self.stm_cycle += segment_capacity;

            self.bram_write(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_STM_ADDR_OFFSET,
                ((self.stm_cycle & !POINT_STM_BUF_SEGMENT_SIZE_MASK)
                    >> POINT_STM_BUF_SEGMENT_SIZE_WIDTH) as _,
            );

            let mut dst = ((self.stm_cycle & POINT_STM_BUF_SEGMENT_SIZE_MASK) << 3) as u16;
            let cnt = size - segment_capacity;
            (0..cnt as usize).for_each(|_| unsafe {
                self.bram_write(BRAM_SELECT_STM, dst, src.read());
                dst += 1;
                src = src.add(1);
                self.bram_write(BRAM_SELECT_STM, dst, src.read());
                dst += 1;
                src = src.add(1);
                self.bram_write(BRAM_SELECT_STM, dst, src.read());
                dst += 1;
                src = src.add(1);
                self.bram_write(BRAM_SELECT_STM, dst, src.read());
                dst += 1;
                src = src.add(1);
                dst += 4;
            });
            self.stm_cycle += size - segment_capacity;
        }

        if (flag & FOCUS_STM_FLAG_END) == FOCUS_STM_FLAG_END {
            self.fpga_flags_internal |= CTL_FLAG_OP_MODE;
            self.fpga_flags_internal &= !CTL_REG_STM_GAIN_MODE;
            if (flag & FOCUS_STM_FLAG_USE_START_IDX) == FOCUS_STM_FLAG_USE_START_IDX {
                self.fpga_flags_internal |= CTL_FLAG_USE_STM_START_IDX;
            } else {
                self.fpga_flags_internal &= !CTL_FLAG_USE_STM_START_IDX;
            }
            if (flag & FOCUS_STM_FLAG_USE_FINISH_IDX) == FOCUS_STM_FLAG_USE_FINISH_IDX {
                self.fpga_flags_internal |= CTL_FLAG_USE_STM_FINISH_IDX;
            } else {
                self.fpga_flags_internal &= !CTL_FLAG_USE_STM_FINISH_IDX;
            }
            self.bram_write(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_STM_CYCLE,
                (self.stm_cycle.max(1) - 1) as _,
            );
        }
    }

    fn write_gain_stm(&mut self, data: &[u8]) {
        let flag = data[1];
        if (flag & GAIN_STM_FLAG_LEGACY) == GAIN_STM_FLAG_LEGACY {
            self.write_gain_stm_legacy(data);
        } else {
            self.write_gain_stm_advanced(data);
        }
    }

    fn write_gain_stm_legacy(&mut self, data: &[u8]) {
        let flag = data[1];

        let src_base = if (flag & GAIN_STM_FLAG_BEGIN) == GAIN_STM_FLAG_BEGIN {
            self.stm_cycle = 0;
            self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET, 0);
            let freq_div = ((data[5] as u32) << 24)
                | ((data[4] as u32) << 16)
                | ((data[3] as u32) << 8)
                | data[2] as u32;
            self.bram_cpy(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_STM_FREQ_DIV_0,
                &freq_div as *const _ as _,
                std::mem::size_of::<u32>() >> 1,
            );

            let start_idx = ((data[7] as u16) << 8) | data[6] as u16;
            let finish_idx = ((data[9] as u16) << 8) | data[8] as u16;

            self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_START_IDX, start_idx);
            self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FINISH_IDX, finish_idx);

            unsafe { data.as_ptr().add(10) as *const u16 }
        } else {
            unsafe { data.as_ptr().add(2) as *const u16 }
        };

        let mut src = src_base;
        let mut dst = ((self.stm_cycle & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8) as u16;

        let ignore_duty = (flag & GAIN_STM_FLAG_IGNORE_DUTY) == GAIN_STM_FLAG_IGNORE_DUTY;
        let phase_compress = (flag & GAIN_STM_FLAG_PHASE_COMPRESS) == GAIN_STM_FLAG_PHASE_COMPRESS;
        if ignore_duty {
            if phase_compress {
                (0..self.num_transducers).for_each(|_| unsafe {
                    let phase = src.read() & 0x000F;
                    self.bram_write(BRAM_SELECT_STM, dst, 0xFF00 | (phase << 4) | phase);
                    dst += 1;
                    src = src.add(1);
                });
                self.stm_cycle += 1;

                let mut src = src_base;
                let mut dst =
                    ((self.stm_cycle & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8) as u16;
                (0..self.num_transducers).for_each(|_| unsafe {
                    let phase = (src.read() >> 4) & 0x000F;
                    self.bram_write(BRAM_SELECT_STM, dst, 0xFF00 | (phase << 4) | phase);
                    dst += 1;
                    src = src.add(1);
                });
                self.stm_cycle += 1;

                let mut src = src_base;
                let mut dst =
                    ((self.stm_cycle & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8) as u16;
                (0..self.num_transducers).for_each(|_| unsafe {
                    let phase = (src.read() >> 8) & 0x000F;
                    self.bram_write(BRAM_SELECT_STM, dst, 0xFF00 | (phase << 4) | phase);
                    dst += 1;
                    src = src.add(1);
                });
                self.stm_cycle += 1;

                let mut src = src_base;
                let mut dst =
                    ((self.stm_cycle & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8) as u16;
                (0..self.num_transducers).for_each(|_| unsafe {
                    let phase = (src.read() >> 12) & 0x000F;
                    self.bram_write(BRAM_SELECT_STM, dst, 0xFF00 | (phase << 4) | phase);
                    dst += 1;
                    src = src.add(1);
                });
                self.stm_cycle += 1;
            } else {
                (0..self.num_transducers).for_each(|_| unsafe {
                    self.bram_write(BRAM_SELECT_STM, dst, 0xFF00 | (src.read() & 0x00FF));
                    dst += 1;
                    src = src.add(1);
                });
                self.stm_cycle += 1;

                let mut src = src_base;
                let mut dst =
                    ((self.stm_cycle & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8) as u16;
                (0..self.num_transducers).for_each(|_| unsafe {
                    self.bram_write(BRAM_SELECT_STM, dst, 0xFF00 | ((src.read() >> 8) & 0x00FF));
                    dst += 1;
                    src = src.add(1);
                });
                self.stm_cycle += 1;
            }
        } else {
            self.stm_cycle += 1;
            (0..self.num_transducers).for_each(|_| unsafe {
                self.bram_write(BRAM_SELECT_STM, dst, src.read());
                dst += 1;
                src = src.add(1);
            });
        }

        if self.stm_cycle & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK == 0 {
            self.bram_write(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_STM_ADDR_OFFSET,
                ((self.stm_cycle & !GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK)
                    >> GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_WIDTH) as _,
            );
        }

        if (flag & GAIN_STM_FLAG_END) == GAIN_STM_FLAG_END {
            self.fpga_flags_internal |= CTL_REG_LEGACY_MODE;
            self.fpga_flags_internal |= CTL_FLAG_OP_MODE;
            self.fpga_flags_internal |= CTL_REG_STM_GAIN_MODE;
            if (flag & GAIN_STM_FLAG_USE_START_IDX) == GAIN_STM_FLAG_USE_START_IDX {
                self.fpga_flags_internal |= CTL_FLAG_USE_STM_START_IDX;
            } else {
                self.fpga_flags_internal &= !CTL_FLAG_USE_STM_START_IDX;
            }
            if (flag & GAIN_STM_FLAG_USE_FINISH_IDX) == GAIN_STM_FLAG_USE_FINISH_IDX {
                self.fpga_flags_internal |= CTL_FLAG_USE_STM_FINISH_IDX;
            } else {
                self.fpga_flags_internal &= !CTL_FLAG_USE_STM_FINISH_IDX;
            }
            self.bram_write(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_STM_CYCLE,
                (self.stm_cycle.max(1) - 1) as _,
            );
        }
    }

    fn write_gain_stm_advanced(&mut self, data: &[u8]) {
        let flag = data[1];

        let src_base = if (flag & GAIN_STM_FLAG_BEGIN) == GAIN_STM_FLAG_BEGIN {
            self.stm_cycle = 0;
            self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET, 0);
            let freq_div = ((data[5] as u32) << 24)
                | ((data[4] as u32) << 16)
                | ((data[3] as u32) << 8)
                | data[2] as u32;
            self.bram_cpy(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_STM_FREQ_DIV_0,
                &freq_div as *const _ as _,
                std::mem::size_of::<u32>() >> 1,
            );

            let start_idx = ((data[7] as u16) << 8) | data[6] as u16;
            let finish_idx = ((data[9] as u16) << 8) | data[8] as u16;

            self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_START_IDX, start_idx);
            self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FINISH_IDX, finish_idx);

            unsafe { data.as_ptr().add(10) as *const u16 }
        } else {
            unsafe { data.as_ptr().add(2) as *const u16 }
        };

        let mut src = src_base;
        let mut dst = ((self.stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9) as u16;

        let ignore_duty = (flag & GAIN_STM_FLAG_IGNORE_DUTY) == GAIN_STM_FLAG_IGNORE_DUTY;
        let phase_compress = (flag & GAIN_STM_FLAG_PHASE_COMPRESS) == GAIN_STM_FLAG_PHASE_COMPRESS;

        if phase_compress {
            unimplemented!("Phase half mode is not supported in advanced mode")
        }

        if ignore_duty {
            if (flag & GAIN_STM_FLAG_DUTY) == GAIN_STM_FLAG_DUTY {
                (0..self.num_transducers).for_each(|i| unsafe {
                    self.bram_write(BRAM_SELECT_STM, dst, src.read());
                    dst += 1;
                    self.bram_write(BRAM_SELECT_STM, dst, self.cycles[i] >> 1);
                    dst += 1;
                    src = src.add(1);
                });
                self.stm_cycle += 1;
            } else {
                (0..self.num_transducers).for_each(|_| unsafe {
                    self.bram_write(BRAM_SELECT_STM, dst, src.read());
                    dst += 2;
                    src = src.add(1);
                });
                self.stm_cycle += 1;
            }
        } else {
            if (flag & GAIN_STM_FLAG_DUTY) == GAIN_STM_FLAG_DUTY {
                dst += 1;
                self.stm_cycle += 1;
            }
            (0..self.num_transducers).for_each(|_| unsafe {
                self.bram_write(BRAM_SELECT_STM, dst, src.read());
                dst += 2;
                src = src.add(1);
            });
        }

        if self.stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK == 0 {
            self.bram_write(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_STM_ADDR_OFFSET,
                ((self.stm_cycle & !GAIN_STM_BUF_SEGMENT_SIZE_MASK)
                    >> GAIN_STM_BUF_SEGMENT_SIZE_WIDTH) as _,
            );
        }

        if (flag & GAIN_STM_FLAG_END) == GAIN_STM_FLAG_END {
            self.fpga_flags_internal &= !CTL_REG_LEGACY_MODE;
            self.fpga_flags_internal |= CTL_FLAG_OP_MODE;
            self.fpga_flags_internal |= CTL_REG_STM_GAIN_MODE;
            if (flag & GAIN_STM_FLAG_USE_START_IDX) == GAIN_STM_FLAG_USE_START_IDX {
                self.fpga_flags_internal |= CTL_FLAG_USE_STM_START_IDX;
            } else {
                self.fpga_flags_internal &= !CTL_FLAG_USE_STM_START_IDX;
            }
            if (flag & GAIN_STM_FLAG_USE_FINISH_IDX) == GAIN_STM_FLAG_USE_FINISH_IDX {
                self.fpga_flags_internal |= CTL_FLAG_USE_STM_FINISH_IDX;
            } else {
                self.fpga_flags_internal &= !CTL_FLAG_USE_STM_FINISH_IDX;
            }
            self.bram_write(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_STM_CYCLE,
                (self.stm_cycle.max(1) - 1) as _,
            );
        }
    }

    fn get_cpu_version(&self) -> u16 {
        CPU_VERSION_MAJOR
    }

    fn get_cpu_version_minor(&self) -> u16 {
        CPU_VERSION_MINOR
    }

    fn get_fpga_version(&self) -> u16 {
        self.bram_read(BRAM_SELECT_CONTROLLER, BRAM_ADDR_VERSION_NUM)
    }

    fn get_fpga_version_minor(&self) -> u16 {
        self.bram_read(BRAM_SELECT_CONTROLLER, BRAM_ADDR_VERSION_NUM_MINOR)
    }

    fn read_fpga_info(&self) -> u16 {
        self.bram_read(BRAM_SELECT_CONTROLLER, BRAM_ADDR_FPGA_INFO)
    }

    fn clear(&mut self) {
        let freq_div_4k = 40960;

        self.fpga_flags_internal = 0x0000;
        self.fpga_flags = FPGAControlFlags::empty();
        self.bram_write(
            BRAM_SELECT_CONTROLLER,
            BRAM_ADDR_CTL_REG,
            self.fpga_flags_internal | self.fpga_flags.bits() as u16,
        );

        self.bram_cpy(
            BRAM_SELECT_CONTROLLER,
            BRAM_ADDR_CYCLE_BASE,
            self.cycles.as_ptr(),
            self.cycles.len(),
        );

        self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_STEP, 10);

        self.stm_cycle = 0;

        self.mod_cycle = 2;
        self.bram_write(
            BRAM_SELECT_CONTROLLER,
            BRAM_ADDR_MOD_CYCLE,
            (self.mod_cycle.max(1) - 1) as _,
        );
        self.bram_cpy(
            BRAM_SELECT_CONTROLLER,
            BRAM_ADDR_MOD_FREQ_DIV_0,
            &freq_div_4k as *const _ as _,
            std::mem::size_of::<u32>() >> 1,
        );
        self.bram_write(BRAM_SELECT_MOD, 0, 0x0000);

        self.bram_set(BRAM_SELECT_NORMAL, 0, 0x0000, self.num_transducers << 1);
    }

    fn handle_payload(&mut self, tag: u8, data: &[u8]) {
        match tag {
            TAG_NONE => {}
            TAG_CLEAR => self.clear(),
            TAG_SYNC => self.synchronize(&data[2..]),
            TAG_FIRM_INFO => {
                self.read_fpga_info = false;
                match data[1] {
                    TYPE_CPU_VERSION_MAJOR => {
                        self.rx_data = (self.get_cpu_version() & 0xFF) as _;
                    }
                    TYPE_CPU_VERSION_MINOR => {
                        self.rx_data = (self.get_cpu_version_minor() & 0xFF) as _;
                    }
                    TYPE_FPGA_VERSION_MAJOR => {
                        self.rx_data = (self.get_fpga_version() & 0xFF) as _;
                    }
                    TYPE_FPGA_VERSION_MINOR => {
                        self.rx_data = (self.get_fpga_version_minor() & 0xFF) as _;
                    }
                    TYPE_FPGA_FUNCTIONS => {
                        self.rx_data = ((self.get_fpga_version() >> 8) & 0xFF) as _;
                    }
                    _ => {
                        unimplemented!("Unsupported firmware info type")
                    }
                }
            }
            TAG_MODULATION => self.write_mod(data),
            TAG_MODULATION_DELAY => self.write_mod_delay(&data[2..]),
            TAG_SILENCER => self.config_silencer(data),
            TAG_GAIN => self.write_gain(data),
            TAG_FOCUS_STM => self.write_focus_stm(data),
            TAG_GAIN_STM => self.write_gain_stm(data),
            TAG_FILTER => self.write_filter(data),
            _ => {
                unimplemented!("Unsupported tag")
            }
        }
    }

    fn ecat_recv(&mut self, data: &[u8]) {
        let header = unsafe { &*(data.as_ptr() as *const Header) };
        if self.read_fpga_info {
            self.rx_data = self.read_fpga_info() as _;
        }

        if self.ack == header.msg_id {
            return;
        }

        if header.fpga_flag.contains(FPGAControlFlags::READS_FPGA_INFO) {
            self.read_fpga_info = true;
            self.rx_data = self.read_fpga_info() as _;
        }

        self.fpga_flags = header.fpga_flag;

        self.handle_payload(
            data[std::mem::size_of::<Header>()],
            &data[std::mem::size_of::<Header>()..],
        );

        if header.slot_2_offset != 0 {
            self.handle_payload(
                data[std::mem::size_of::<Header>() + header.slot_2_offset as usize],
                &data[std::mem::size_of::<Header>() + header.slot_2_offset as usize..],
            )
        }

        self.bram_write(
            BRAM_SELECT_CONTROLLER,
            BRAM_ADDR_CTL_REG,
            self.fpga_flags_internal | self.fpga_flags.bits() as u16,
        );

        self.ack = header.msg_id;
    }
}
