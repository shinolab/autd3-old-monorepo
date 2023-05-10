/*
 * File: emulator.rs
 * Project: src
 * Created Date: 06/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    Body, CPUControlFlags, FPGAControlFlags, GlobalHeader, TxDatagram, MSG_CLEAR, MSG_END,
    MSG_RD_CPU_VERSION, MSG_RD_CPU_VERSION_MINOR, MSG_RD_FPGA_FUNCTION, MSG_RD_FPGA_VERSION,
    MSG_RD_FPGA_VERSION_MINOR,
};

use crate::fpga::emulator::FPGAEmulator;

use super::params::*;

pub struct CPUEmulator {
    id: usize,
    msg_id: u8,
    ack: u8,
    read_fpga_info: bool,
    mod_cycle: u32,
    stm_cycle: u32,
    fpga: FPGAEmulator,
    gain_stm_mode: u16,
    cycles: Vec<u16>,
    synchronized: bool,
    num_transducers: usize,
    fpga_flags: FPGAControlFlags,
    cpu_flags: CPUControlFlags,
}

impl CPUEmulator {
    pub fn new(id: usize, num_transducers: usize) -> Self {
        let mut s = Self {
            id,
            msg_id: 0x00,
            ack: 0x0000,
            read_fpga_info: false,
            mod_cycle: 0,
            stm_cycle: 0,
            fpga: FPGAEmulator::new(num_transducers),
            gain_stm_mode: GAIN_STM_MODE_PHASE_DUTY_FULL,
            cycles: vec![0x0000; num_transducers],
            synchronized: false,
            num_transducers,
            fpga_flags: FPGAControlFlags::empty(),
            cpu_flags: CPUControlFlags::empty(),
        };
        s.init();
        s
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn msg_id(&self) -> u8 {
        self.msg_id
    }

    pub fn ack(&self) -> u8 {
        self.ack
    }

    pub fn fpga_flags(&self) -> FPGAControlFlags {
        self.fpga_flags
    }

    pub fn cpu_flags(&self) -> CPUControlFlags {
        self.cpu_flags
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
        self.ecat_recv(tx.header(), tx.body(self.id))
    }

    pub fn init(&mut self) {
        self.fpga.init();
        self.cycles.fill(0x1000);
        self.clear();
    }

    pub fn update(&mut self) {
        match self.msg_id {
            MSG_RD_CPU_VERSION
            | MSG_RD_CPU_VERSION_MINOR
            | MSG_RD_FPGA_VERSION
            | MSG_RD_FPGA_VERSION_MINOR
            | MSG_RD_FPGA_FUNCTION => {}
            _ => {
                if self.read_fpga_info {
                    self.ack = self.read_fpga_info() as _;
                }
            }
        }
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

    fn synchronize(&mut self, body: &Body<[u16]>) {
        let cycles = body.data();

        self.bram_cpy(
            BRAM_SELECT_CONTROLLER,
            BRAM_ADDR_CYCLE_BASE,
            cycles.as_ptr(),
            cycles.len(),
        );

        self.cycles.copy_from_slice(body.data());

        self.synchronized = true;

        // Do nothing to sync
    }

    fn write_mod(&mut self, header: &GlobalHeader) {
        let write = header.size;

        let data = if header.cpu_flag.contains(CPUControlFlags::MOD_BEGIN) {
            self.mod_cycle = 0;
            self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_ADDR_OFFSET, 0);
            let freq_div = header.mod_initial().freq_div;
            self.bram_cpy(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_MOD_FREQ_DIV_0,
                &freq_div as *const _ as _,
                std::mem::size_of::<u32>() >> 1,
            );
            header.mod_initial().data[..].as_ptr() as *const u16
        } else {
            header.mod_subsequent().data[..].as_ptr() as *const u16
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

        if header.cpu_flag.contains(CPUControlFlags::MOD_END) {
            self.bram_write(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_MOD_CYCLE,
                (self.mod_cycle.max(1) - 1) as _,
            );
        }
    }

    fn config_silencer(&mut self, header: &GlobalHeader) {
        let step = header.silencer().step;
        let cycle = header.silencer().cycle;
        self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_STEP, step);
        self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_CYCLE, cycle);
    }

    fn set_mod_delay(&mut self, body: &Body<[u16]>) {
        self.bram_cpy(
            BRAM_SELECT_CONTROLLER,
            BRAM_ADDR_MOD_DELAY_BASE,
            body.data().as_ptr(),
            body.data().len(),
        );
    }

    fn write_normal_op(&mut self, header: &GlobalHeader, body: &Body<[u16]>) {
        if header.fpga_flag.contains(FPGAControlFlags::LEGACY_MODE) {
            (0..self.num_transducers)
                .for_each(|i| self.bram_write(BRAM_SELECT_NORMAL, (i << 1) as _, body.data()[i]));
        } else if header.cpu_flag.contains(CPUControlFlags::IS_DUTY) {
            (0..self.num_transducers).for_each(|i| {
                self.bram_write(BRAM_SELECT_NORMAL, (i << 1) as u16 + 1, body.data()[i])
            });
        } else {
            (0..self.num_transducers)
                .for_each(|i| self.bram_write(BRAM_SELECT_NORMAL, (i << 1) as u16, body.data()[i]));
        }
    }

    fn write_focus_stm(&mut self, header: &GlobalHeader, body: &Body<[u16]>) {
        let size: u32;

        let mut src = if header.cpu_flag.contains(CPUControlFlags::STM_BEGIN) {
            self.stm_cycle = 0;
            self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET, 0);
            size = body.focus_stm_initial().data()[0] as _;
            let freq_div = ((body.focus_stm_initial().data()[2] as u32) << 16)
                | body.focus_stm_initial().data()[1] as u32;
            let sound_speed = ((body.focus_stm_initial().data()[4] as u32) << 16)
                | body.focus_stm_initial().data()[3] as u32;
            let start_idx = body.focus_stm_initial().data()[5];
            let finish_idx = body.focus_stm_initial().data()[6];

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

            unsafe { body.focus_stm_initial().data().as_ptr().add(7) }
        } else {
            size = body.focus_stm_subsequent().data()[0] as _;
            unsafe { body.focus_stm_subsequent().data().as_ptr().add(1) }
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

        if header.cpu_flag.contains(CPUControlFlags::STM_END) {
            self.bram_write(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_STM_CYCLE,
                (self.stm_cycle.max(1) - 1) as _,
            );
            self.bram_write(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_CTL_REG,
                header.fpga_flag.bits() as u16 | CTL_FLAG_OP_MODE,
            );
        }
    }

    fn write_gain_stm(&mut self, header: &GlobalHeader, body: &Body<[u16]>) {
        if header.cpu_flag.contains(CPUControlFlags::STM_BEGIN) {
            self.stm_cycle = 0;
            self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET, 0);
            let freq_div = ((body.gain_stm_initial().data()[1] as u32) << 16)
                | body.gain_stm_initial().data()[0] as u32;
            self.bram_cpy(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_STM_FREQ_DIV_0,
                &freq_div as *const _ as _,
                std::mem::size_of::<u32>() >> 1,
            );
            self.gain_stm_mode = body.gain_stm_initial().data()[2];

            let start_idx = body.gain_stm_initial().data()[4];
            let finish_idx = body.gain_stm_initial().data()[5];

            self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_START_IDX, start_idx);
            self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FINISH_IDX, finish_idx);

            return;
        }

        let mut src = body.gain_stm_subsequent().data().as_ptr();
        let mut dst = ((self.stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9) as u16;

        match self.gain_stm_mode {
            GAIN_STM_MODE_PHASE_DUTY_FULL => {
                if header.fpga_flag.contains(FPGAControlFlags::LEGACY_MODE) {
                    self.stm_cycle += 1;
                } else if header.cpu_flag.contains(CPUControlFlags::IS_DUTY) {
                    dst += 1;
                    self.stm_cycle += 1;
                }
                (0..self.num_transducers).for_each(|_| unsafe {
                    self.bram_write(BRAM_SELECT_STM, dst, src.read());
                    dst += 2;
                    src = src.add(1);
                });
            }
            GAIN_STM_MODE_PHASE_FULL => {
                if header.fpga_flag.contains(FPGAControlFlags::LEGACY_MODE) {
                    (0..self.num_transducers).for_each(|_| unsafe {
                        self.bram_write(BRAM_SELECT_STM, dst, 0xFF00 | (src.read() & 0x00FF));
                        dst += 2;
                        src = src.add(1);
                    });
                    self.stm_cycle += 1;

                    let mut src = body.gain_stm_subsequent().data().as_ptr();
                    let mut dst = ((self.stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9) as u16;
                    (0..self.num_transducers).for_each(|_| unsafe {
                        self.bram_write(
                            BRAM_SELECT_STM,
                            dst,
                            0xFF00 | ((src.read() >> 8) & 0x00FF),
                        );
                        dst += 2;
                        src = src.add(1);
                    });
                    self.stm_cycle += 1;
                } else if !header.cpu_flag.contains(CPUControlFlags::IS_DUTY) {
                    (0..self.num_transducers).for_each(|i| unsafe {
                        self.bram_write(BRAM_SELECT_STM, dst, src.read());
                        dst += 1;
                        self.bram_write(BRAM_SELECT_STM, dst, self.cycles[i] >> 1);
                        dst += 1;
                        src = src.add(1);
                    });
                    self.stm_cycle += 1;
                }
            }
            GAIN_STM_MODE_PHASE_HALF => {
                if header.fpga_flag.contains(FPGAControlFlags::LEGACY_MODE) {
                    (0..self.num_transducers).for_each(|_| unsafe {
                        let phase = src.read() & 0x000F;
                        self.bram_write(BRAM_SELECT_STM, dst, 0xFF00 | (phase << 4) | phase);
                        dst += 2;
                        src = src.add(1);
                    });
                    self.stm_cycle += 1;

                    let mut src = body.gain_stm_subsequent().data().as_ptr();
                    let mut dst = ((self.stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9) as u16;
                    (0..self.num_transducers).for_each(|_| unsafe {
                        let phase = (src.read() >> 4) & 0x000F;
                        self.bram_write(BRAM_SELECT_STM, dst, 0xFF00 | (phase << 4) | phase);
                        dst += 2;
                        src = src.add(1);
                    });
                    self.stm_cycle += 1;

                    let mut src = body.gain_stm_subsequent().data().as_ptr();
                    let mut dst = ((self.stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9) as u16;
                    (0..self.num_transducers).for_each(|_| unsafe {
                        let phase = (src.read() >> 8) & 0x000F;
                        self.bram_write(BRAM_SELECT_STM, dst, 0xFF00 | (phase << 4) | phase);
                        dst += 2;
                        src = src.add(1);
                    });
                    self.stm_cycle += 1;

                    let mut src = body.gain_stm_subsequent().data().as_ptr();
                    let mut dst = ((self.stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9) as u16;
                    (0..self.num_transducers).for_each(|_| unsafe {
                        let phase = (src.read() >> 12) & 0x000F;
                        self.bram_write(BRAM_SELECT_STM, dst, 0xFF00 | (phase << 4) | phase);
                        dst += 2;
                        src = src.add(1);
                    });
                    self.stm_cycle += 1;
                }
            }
            _ => {
                if header.fpga_flag.contains(FPGAControlFlags::LEGACY_MODE) {
                    self.stm_cycle += 1;
                } else if header.cpu_flag.contains(CPUControlFlags::IS_DUTY) {
                    dst += 1;
                    self.stm_cycle += 1;
                }
                (0..self.num_transducers).for_each(|_| unsafe {
                    self.bram_write(BRAM_SELECT_STM, dst, src.read());
                    dst += 2;
                    src = src.add(1);
                });
            }
        }

        if self.stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK == 0 {
            self.bram_write(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_STM_ADDR_OFFSET,
                ((self.stm_cycle & !GAIN_STM_BUF_SEGMENT_SIZE_MASK)
                    >> GAIN_STM_BUF_SEGMENT_SIZE_WIDTH) as _,
            );
        }

        if header.cpu_flag.contains(CPUControlFlags::STM_END) {
            self.bram_write(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_STM_CYCLE,
                (self.stm_cycle.max(1) - 1) as _,
            );

            self.bram_write(
                BRAM_SELECT_CONTROLLER,
                BRAM_ADDR_CTL_REG,
                header.fpga_flag.bits() as u16 | CTL_FLAG_OP_MODE,
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

        let ctl_reg = FPGAControlFlags::LEGACY_MODE;
        self.bram_write(
            BRAM_SELECT_CONTROLLER,
            BRAM_ADDR_CTL_REG,
            ctl_reg.bits() as _,
        );

        self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_STEP, 10);
        self.bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_CYCLE, 4096);

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

    fn ecat_recv(&mut self, header: &GlobalHeader, body: &Body<[u16]>) {
        if self.msg_id == header.msg_id {
            return;
        }

        self.msg_id = header.msg_id;
        self.read_fpga_info = header.fpga_flag.contains(FPGAControlFlags::READS_FPGA_INFO);
        if self.read_fpga_info {
            self.ack = self.read_fpga_info() as _;
        }

        match self.msg_id {
            MSG_CLEAR => {
                self.clear();
            }
            MSG_RD_CPU_VERSION => {
                self.ack = (self.get_cpu_version() & 0xFF) as _;
            }
            MSG_RD_CPU_VERSION_MINOR => {
                self.ack = (self.get_cpu_version_minor() & 0xFF) as _;
            }
            MSG_RD_FPGA_VERSION => {
                self.ack = (self.get_fpga_version() & 0xFF) as _;
            }
            MSG_RD_FPGA_VERSION_MINOR => {
                self.ack = (self.get_fpga_version_minor() & 0xFF) as _;
            }
            MSG_RD_FPGA_FUNCTION => {
                self.ack = ((self.get_fpga_version() >> 8) & 0xFF) as _;
            }
            _ => {
                if self.msg_id > MSG_END {
                    return;
                }

                let ctl_reg = header.fpga_flag;
                self.bram_write(
                    BRAM_SELECT_CONTROLLER,
                    BRAM_ADDR_CTL_REG,
                    ctl_reg.bits() as _,
                );

                if header.cpu_flag.contains(CPUControlFlags::MOD) {
                    self.write_mod(header);
                } else if header.cpu_flag.contains(CPUControlFlags::CONFIG_SILENCER) {
                    self.config_silencer(header);
                } else if header.cpu_flag.contains(CPUControlFlags::CONFIG_SYNC) {
                    self.synchronize(body);
                    return;
                }

                if !header.cpu_flag.contains(CPUControlFlags::WRITE_BODY) {
                    return;
                }

                if header.cpu_flag.contains(CPUControlFlags::MOD_DELAY) {
                    self.set_mod_delay(body);
                    return;
                }

                if !ctl_reg.contains(FPGAControlFlags::STM_MODE) {
                    self.write_normal_op(header, body);
                    return;
                }

                if !ctl_reg.contains(FPGAControlFlags::STM_GAIN_MODE) {
                    self.write_focus_stm(header, body);
                } else {
                    self.write_gain_stm(header, body);
                }
            }
        }
    }
}
