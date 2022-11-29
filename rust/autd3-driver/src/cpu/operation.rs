/*
 * File: operation.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    cpu::{
        error::CPUError, CPUControlFlags, TxDatagram, MOD_HEADER_INITIAL_DATA_SIZE,
        MOD_HEADER_SUBSEQUENT_DATA_SIZE, MSG_CLEAR, MSG_RD_CPU_VERSION, MSG_RD_FPGA_FUNCTION,
        MSG_RD_FPGA_VERSION,
    },
    fpga::{FPGAControlFlags, FPGAError, MOD_SAMPLING_FREQ_DIV_MIN, SILENCER_CYCLE_MIN},
    hardware::NUM_TRANS_IN_UNIT,
    Drive, Mode, SeqFocus, FOCUS_STM_BODY_DATA_SIZE, FOCUS_STM_HEAD_DATA_SIZE,
    STM_SAMPLING_FREQ_DIV_MIN,
};

use anyhow::Result;

pub fn clear(tx: &mut TxDatagram) {
    tx.header_mut().msg_id = MSG_CLEAR;
    tx.num_bodies = 0;
}

pub fn null_header(msg_id: u8, tx: &mut TxDatagram) {
    tx.header_mut().msg_id = msg_id;
    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
    tx.header_mut()
        .cpu_flag
        .remove(CPUControlFlags::CONFIG_SILENCER);
    tx.header_mut()
        .cpu_flag
        .remove(CPUControlFlags::CONFIG_SYNC);

    tx.header_mut().size = 0;
}

pub fn null_body(tx: &mut TxDatagram) {
    tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
    tx.num_bodies = 0;
}

pub fn sync(msg_id: u8, cycles: &[[u16; NUM_TRANS_IN_UNIT]], tx: &mut TxDatagram) -> Result<()> {
    if cycles.len() != tx.body().len() {
        return Err(CPUError::DeviceNumberNotCorrect {
            a: tx.body().len(),
            b: cycles.len(),
        }
        .into());
    }

    tx.header_mut().msg_id = msg_id;
    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
    tx.header_mut()
        .cpu_flag
        .remove(CPUControlFlags::CONFIG_SILENCER);
    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::CONFIG_SYNC, true);

    tx.body_mut()
        .iter_mut()
        .zip(cycles.iter())
        .for_each(|(d, s)| d.data.clone_from(s));

    tx.num_bodies = tx.body().len();

    Ok(())
}

pub fn modulation(
    msg_id: u8,
    mod_data: &[u8],
    is_first_frame: bool,
    freq_div: u32,
    is_last_frame: bool,
    tx: &mut TxDatagram,
) -> Result<()> {
    tx.header_mut().msg_id = msg_id;
    tx.header_mut().cpu_flag.set(CPUControlFlags::MOD, true);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_BEGIN);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_END);
    tx.header_mut().size = mod_data.len() as _;

    if mod_data.is_empty() {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
        return Ok(());
    }

    if is_first_frame && mod_data.len() > MOD_HEADER_INITIAL_DATA_SIZE {
        return Err(CPUError::ModulationHeadDataSizeOutOfRange(mod_data.len()).into());
    }

    if !is_first_frame && mod_data.len() > MOD_HEADER_SUBSEQUENT_DATA_SIZE {
        return Err(CPUError::ModulationBodyDataSizeOutOfRange(mod_data.len()).into());
    }

    if is_first_frame {
        if freq_div < MOD_SAMPLING_FREQ_DIV_MIN {
            return Err(FPGAError::ModFreqDivOutOfRange(freq_div).into());
        }
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::MOD_BEGIN, true);
        tx.header_mut().mod_initial_mut().freq_div = freq_div;
        tx.header_mut().mod_initial_mut().data[0..mod_data.len()].copy_from_slice(mod_data);
    } else {
        tx.header_mut().mod_subsequent_mut().data[0..mod_data.len()].copy_from_slice(mod_data);
    }

    if is_last_frame {
        tx.header_mut().cpu_flag.set(CPUControlFlags::MOD_END, true);
    }

    Ok(())
}

pub fn config_silencer(msg_id: u8, cycle: u16, step: u16, tx: &mut TxDatagram) -> Result<()> {
    if cycle < SILENCER_CYCLE_MIN {
        return Err(FPGAError::SilencerCycleOutOfRange(cycle).into());
    }

    tx.header_mut().msg_id = msg_id;
    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
    tx.header_mut()
        .cpu_flag
        .remove(CPUControlFlags::CONFIG_SYNC);
    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::CONFIG_SILENCER, true);

    tx.header_mut().silencer_mut().cycle = cycle;
    tx.header_mut().silencer_mut().step = step;

    Ok(())
}

pub fn mod_delay(delays: &[[u16; NUM_TRANS_IN_UNIT]], tx: &mut TxDatagram) -> Result<()> {
    if delays.len() != tx.body().len() {
        return Err(CPUError::DeviceNumberNotCorrect {
            a: tx.body().len(),
            b: delays.len(),
        }
        .into());
    }

    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::WRITE_BODY, true);
    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::MOD_DELAY, true);

    tx.body_mut()
        .iter_mut()
        .zip(delays.iter())
        .for_each(|(d, s)| d.data.clone_from(s));

    tx.num_bodies = tx.body().len();

    Ok(())
}

pub fn normal_legacy_head(tx: &mut TxDatagram) {
    tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

    tx.header_mut()
        .fpga_flag
        .set(FPGAControlFlags::LEGACY_MODE, true);
    tx.header_mut().fpga_flag.remove(FPGAControlFlags::STM_MODE);

    tx.num_bodies = 0;
}

pub fn normal_legacy_body(drive: &[Drive], tx: &mut TxDatagram) -> Result<()> {
    if drive.len() / NUM_TRANS_IN_UNIT != tx.body().len() {
        return Err(CPUError::DeviceNumberNotCorrect {
            a: tx.body().len(),
            b: drive.len() / NUM_TRANS_IN_UNIT,
        }
        .into());
    }

    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::WRITE_BODY, true);

    tx.body_mut()
        .iter_mut()
        .zip(drive.chunks(NUM_TRANS_IN_UNIT))
        .for_each(|(dd, ss)| {
            dd.legacy_drives_mut()
                .iter_mut()
                .zip(ss.iter())
                .for_each(|(d, s)| d.set(s))
        });

    tx.num_bodies = tx.body().len();

    Ok(())
}

pub fn normal_head(tx: &mut TxDatagram) {
    tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

    tx.header_mut()
        .fpga_flag
        .remove(FPGAControlFlags::LEGACY_MODE);
    tx.header_mut().fpga_flag.remove(FPGAControlFlags::STM_MODE);

    tx.num_bodies = 0;
}

pub fn normal_duty_body(drive: &[Drive], tx: &mut TxDatagram) -> Result<()> {
    if drive.len() / NUM_TRANS_IN_UNIT != tx.body().len() {
        return Err(CPUError::DeviceNumberNotCorrect {
            a: tx.body().len(),
            b: drive.len() / NUM_TRANS_IN_UNIT,
        }
        .into());
    }

    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::WRITE_BODY, true);
    tx.header_mut().cpu_flag.set(CPUControlFlags::IS_DUTY, true);

    tx.body_mut()
        .iter_mut()
        .zip(drive.chunks(NUM_TRANS_IN_UNIT))
        .for_each(|(dd, ss)| {
            dd.duties_mut()
                .iter_mut()
                .zip(ss.iter())
                .for_each(|(d, s)| d.set(s))
        });

    tx.num_bodies = tx.body().len();

    Ok(())
}

pub fn normal_phase_body(drive: &[Drive], tx: &mut TxDatagram) -> Result<()> {
    if drive.len() / NUM_TRANS_IN_UNIT != tx.body().len() {
        return Err(CPUError::DeviceNumberNotCorrect {
            a: tx.body().len(),
            b: drive.len() / NUM_TRANS_IN_UNIT,
        }
        .into());
    }

    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::WRITE_BODY, true);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::IS_DUTY);

    tx.body_mut()
        .iter_mut()
        .zip(drive.chunks(NUM_TRANS_IN_UNIT))
        .for_each(|(dd, ss)| {
            dd.phases_mut()
                .iter_mut()
                .zip(ss.iter())
                .for_each(|(d, s)| d.set(s))
        });

    tx.num_bodies = tx.body().len();

    Ok(())
}

pub fn focus_stm_initial(tx: &mut TxDatagram) {
    tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_END);

    tx.header_mut()
        .fpga_flag
        .set(FPGAControlFlags::STM_MODE, true);
    tx.header_mut()
        .fpga_flag
        .remove(FPGAControlFlags::STM_GAIN_MODE);

    tx.num_bodies = 0;
}

pub fn focus_stm_body(
    points: &[Vec<SeqFocus>],
    is_first_frame: bool,
    freq_div: u32,
    sound_speed: f64,
    is_last_frame: bool,
    tx: &mut TxDatagram,
) -> Result<()> {
    if points.is_empty() || points[0].is_empty() {
        return Ok(());
    }

    if is_first_frame {
        for s in points {
            if s.len() > FOCUS_STM_HEAD_DATA_SIZE {
                return Err(CPUError::FocusSTMHeadDataSizeOutOfRange(s.len()).into());
            }
        }
    } else {
        for s in points {
            if s.len() > FOCUS_STM_BODY_DATA_SIZE {
                return Err(CPUError::FocusSTMBodyDataSizeOutOfRange(s.len()).into());
            }
        }
    }

    if is_first_frame {
        if freq_div < STM_SAMPLING_FREQ_DIV_MIN {
            return Err(FPGAError::STMFreqDivOutOfRange(freq_div).into());
        }
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::STM_BEGIN, true);
        let sound_speed = (sound_speed / 1e3 * 1024.0).round() as u32;
        tx.body_mut().iter_mut().zip(points).for_each(|(d, s)| {
            d.focus_stm_initial_mut().set_size(s.len() as _);
            d.focus_stm_initial_mut().set_freq_div(freq_div);
            d.focus_stm_initial_mut().set_sound_speed(sound_speed);
            d.focus_stm_initial_mut().set_points(s);
        });
    } else {
        tx.body_mut().iter_mut().zip(points).for_each(|(d, s)| {
            d.focus_stm_subsequent_mut().set_size(s.len() as _);
            d.focus_stm_subsequent_mut().set_points(s);
        });
    }

    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::WRITE_BODY, true);

    if is_last_frame {
        tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
    }

    tx.num_bodies = tx.body().len();

    Ok(())
}

pub fn gain_stm_legacy_head(tx: &mut TxDatagram) {
    tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_END);

    tx.header_mut()
        .fpga_flag
        .set(FPGAControlFlags::LEGACY_MODE, true);
    tx.header_mut()
        .fpga_flag
        .set(FPGAControlFlags::STM_MODE, true);
    tx.header_mut()
        .fpga_flag
        .set(FPGAControlFlags::STM_GAIN_MODE, true);

    tx.num_bodies = 0;
}

pub fn gain_stm_legacy_body(
    drives: &[&[Drive]],
    size: usize,
    is_first_frame: bool,
    freq_div: u32,
    is_last_frame: bool,
    mode: Mode,
    tx: &mut TxDatagram,
) -> Result<()> {
    if is_first_frame {
        if freq_div < STM_SAMPLING_FREQ_DIV_MIN {
            return Err(FPGAError::STMFreqDivOutOfRange(freq_div).into());
        }
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::STM_BEGIN, true);
        tx.body_mut().iter_mut().for_each(|d| {
            d.gain_stm_initial_mut().set_freq_div(freq_div);
            d.gain_stm_initial_mut().set_mode(mode);
            d.gain_stm_initial_mut().set_cycle(size);
        });
    } else {
        match mode {
            Mode::PhaseDutyFull => {
                tx.body_mut()
                    .iter_mut()
                    .zip(drives[0].chunks(NUM_TRANS_IN_UNIT))
                    .for_each(|(dd, ss)| {
                        dd.gain_stm_subsequent_mut()
                            .legacy_drives_mut()
                            .iter_mut()
                            .zip(ss.iter())
                            .for_each(|(d, s)| d.set(s))
                    });
            }
            Mode::PhaseFull => {
                tx.body_mut()
                    .iter_mut()
                    .zip(drives[0].chunks(NUM_TRANS_IN_UNIT))
                    .for_each(|(dd, ss)| {
                        dd.gain_stm_subsequent_mut()
                            .legacy_phase_full_mut()
                            .iter_mut()
                            .zip(ss.iter())
                            .for_each(|(d, s)| d.set(0, s))
                    });
                tx.body_mut()
                    .iter_mut()
                    .zip(drives[1].chunks(NUM_TRANS_IN_UNIT))
                    .for_each(|(dd, ss)| {
                        dd.gain_stm_subsequent_mut()
                            .legacy_phase_full_mut()
                            .iter_mut()
                            .zip(ss.iter())
                            .for_each(|(d, s)| d.set(1, s))
                    });
            }
            Mode::PhaseHalf => {
                tx.body_mut()
                    .iter_mut()
                    .zip(drives[0].chunks(NUM_TRANS_IN_UNIT))
                    .for_each(|(dd, ss)| {
                        dd.gain_stm_subsequent_mut()
                            .legacy_phase_half_mut()
                            .iter_mut()
                            .zip(ss.iter())
                            .for_each(|(d, s)| d.set(0, s))
                    });
                tx.body_mut()
                    .iter_mut()
                    .zip(drives[1].chunks(NUM_TRANS_IN_UNIT))
                    .for_each(|(dd, ss)| {
                        dd.gain_stm_subsequent_mut()
                            .legacy_phase_half_mut()
                            .iter_mut()
                            .zip(ss.iter())
                            .for_each(|(d, s)| d.set(1, s))
                    });
                tx.body_mut()
                    .iter_mut()
                    .zip(drives[2].chunks(NUM_TRANS_IN_UNIT))
                    .for_each(|(dd, ss)| {
                        dd.gain_stm_subsequent_mut()
                            .legacy_phase_half_mut()
                            .iter_mut()
                            .zip(ss.iter())
                            .for_each(|(d, s)| d.set(2, s))
                    });
                tx.body_mut()
                    .iter_mut()
                    .zip(drives[3].chunks(NUM_TRANS_IN_UNIT))
                    .for_each(|(dd, ss)| {
                        dd.gain_stm_subsequent_mut()
                            .legacy_phase_half_mut()
                            .iter_mut()
                            .zip(ss.iter())
                            .for_each(|(d, s)| d.set(2, s))
                    });
            }
        }
    }

    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::WRITE_BODY, true);

    if is_last_frame {
        tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
    }

    tx.num_bodies = tx.body().len();

    Ok(())
}

pub fn gain_stm_normal_head(tx: &mut TxDatagram) {
    tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_END);

    tx.header_mut()
        .fpga_flag
        .remove(FPGAControlFlags::LEGACY_MODE);
    tx.header_mut()
        .fpga_flag
        .set(FPGAControlFlags::STM_MODE, true);
    tx.header_mut()
        .fpga_flag
        .set(FPGAControlFlags::STM_GAIN_MODE, true);

    tx.num_bodies = 0;
}

pub fn gain_stm_normal_phase_body(
    drives: &[Drive],
    size: usize,
    is_first_frame: bool,
    freq_div: u32,
    mode: Mode,
    is_last_frame: bool,
    tx: &mut TxDatagram,
) -> Result<()> {
    if mode == Mode::PhaseHalf {
        return Err(CPUError::PhaseHalfNotSupported.into());
    }

    tx.header_mut().cpu_flag.remove(CPUControlFlags::IS_DUTY);

    if is_first_frame {
        if freq_div < STM_SAMPLING_FREQ_DIV_MIN {
            return Err(FPGAError::STMFreqDivOutOfRange(freq_div).into());
        }
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::STM_BEGIN, true);
        tx.body_mut().iter_mut().for_each(|d| {
            d.gain_stm_initial_mut().set_freq_div(freq_div);
            d.gain_stm_initial_mut().set_mode(mode);
            d.gain_stm_initial_mut().set_cycle(size);
        });
    } else {
        tx.body_mut()
            .iter_mut()
            .zip(drives.chunks(NUM_TRANS_IN_UNIT))
            .for_each(|(dd, ss)| {
                dd.gain_stm_subsequent_mut()
                    .phases_mut()
                    .iter_mut()
                    .zip(ss.iter())
                    .for_each(|(d, s)| d.set(s));
            });
    }

    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::WRITE_BODY, true);

    if is_last_frame {
        tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
    }

    tx.num_bodies = tx.body().len();

    Ok(())
}

pub fn gain_stm_normal_duty_body(
    drives: &[Drive],
    is_last_frame: bool,
    tx: &mut TxDatagram,
) -> Result<()> {
    tx.header_mut().cpu_flag.set(CPUControlFlags::IS_DUTY, true);

    tx.body_mut()
        .iter_mut()
        .zip(drives.chunks(NUM_TRANS_IN_UNIT))
        .for_each(|(dd, ss)| {
            dd.gain_stm_subsequent_mut()
                .duties_mut()
                .iter_mut()
                .zip(ss.iter())
                .for_each(|(d, s)| d.set(s))
        });

    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::WRITE_BODY, true);

    if is_last_frame {
        tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
    }

    tx.num_bodies = tx.body().len();

    Ok(())
}

pub fn force_fan(tx: &mut TxDatagram, value: bool) {
    tx.header_mut()
        .fpga_flag
        .set(FPGAControlFlags::FORCE_FAN, value);
}

pub fn reads_fpga_info(tx: &mut TxDatagram, value: bool) {
    tx.header_mut()
        .fpga_flag
        .set(FPGAControlFlags::READS_FPGA_INFO, value);
}

pub fn cpu_version(tx: &mut TxDatagram) {
    tx.header_mut().msg_id = MSG_RD_CPU_VERSION;
    tx.header_mut().cpu_flag = CPUControlFlags::from_bits(0x02).unwrap(); // For backward compatibility before 1.9
    tx.num_bodies = 0;
}

pub fn fpga_version(tx: &mut TxDatagram) {
    tx.header_mut().msg_id = MSG_RD_FPGA_VERSION;
    tx.header_mut().cpu_flag = CPUControlFlags::from_bits(0x04).unwrap(); // For backward compatibility before 1.9
    tx.num_bodies = 0;
}

pub fn fpga_functions(tx: &mut TxDatagram) {
    tx.header_mut().msg_id = MSG_RD_FPGA_FUNCTION;
    tx.header_mut().cpu_flag = CPUControlFlags::from_bits(0x05).unwrap(); // For backward compatibility before 1.9
    tx.num_bodies = 0;
}
