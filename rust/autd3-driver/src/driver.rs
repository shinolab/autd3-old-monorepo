/*
 * File: operation.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{cpu::*, defined::*, error::*, fpga::*};

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

pub fn sync(cycles: &[u16], tx: &mut TxDatagram) -> Result<()> {
    if cycles.len() != tx.num_transducers() {
        return Err(DriverError::NumberOfTransducerMismatch {
            a: tx.num_transducers(),
            b: cycles.len(),
        }
        .into());
    }

    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
    tx.header_mut()
        .cpu_flag
        .remove(CPUControlFlags::CONFIG_SILENCER);
    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::CONFIG_SYNC, true);
    tx.num_bodies = tx.num_devices();

    tx.body_data_mut().clone_from_slice(cycles);

    Ok(())
}

pub fn mod_delay(delays: &[u16], tx: &mut TxDatagram) -> Result<()> {
    if delays.len() != tx.num_transducers() {
        return Err(DriverError::NumberOfTransducerMismatch {
            a: tx.num_transducers(),
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
    tx.num_bodies = tx.num_devices();

    tx.body_data_mut().clone_from_slice(delays);

    Ok(())
}

pub fn modulation(
    msg_id: u8,
    mod_data: &[u8],
    sent: &mut usize,
    freq_div: u32,
    tx: &mut TxDatagram,
) -> Result<()> {
    if mod_data.len() > MOD_BUF_SIZE_MAX {
        return Err(DriverError::ModulationSizeOutOfRange(mod_data.len()).into());
    }

    let is_first_frame = *sent == 0;
    let max_size = if is_first_frame {
        MOD_HEADER_INITIAL_DATA_SIZE
    } else {
        MOD_HEADER_SUBSEQUENT_DATA_SIZE
    };
    let mod_size = (mod_data.len() - *sent).min(max_size);
    let is_last_frame = *sent + mod_size == mod_data.len();

    tx.header_mut().msg_id = msg_id;
    tx.header_mut().cpu_flag.set(CPUControlFlags::MOD, true);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_BEGIN);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_END);
    tx.header_mut().size = mod_size as _;

    if mod_data.is_empty() {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
        return Ok(());
    }

    if is_first_frame {
        if freq_div < MOD_SAMPLING_FREQ_DIV_MIN {
            return Err(DriverError::ModFreqDivOutOfRange(freq_div).into());
        }
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::MOD_BEGIN, true);
        tx.header_mut().mod_initial_mut().freq_div = freq_div;
        tx.header_mut().mod_initial_mut().data[0..mod_size].copy_from_slice(&mod_data[*sent..]);
    } else {
        tx.header_mut().mod_subsequent_mut().data[0..mod_size].copy_from_slice(&mod_data[*sent..]);
    }

    if is_last_frame {
        tx.header_mut().cpu_flag.set(CPUControlFlags::MOD_END, true);
    }

    *sent += mod_size;

    Ok(())
}

pub fn config_silencer(msg_id: u8, cycle: u16, step: u16, tx: &mut TxDatagram) -> Result<()> {
    if cycle < SILENCER_CYCLE_MIN {
        return Err(DriverError::SilencerCycleOutOfRange(cycle).into());
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

pub fn normal_legacy_header(tx: &mut TxDatagram) {
    tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

    tx.header_mut()
        .fpga_flag
        .set(FPGAControlFlags::LEGACY_MODE, true);
    tx.header_mut().fpga_flag.remove(FPGAControlFlags::STM_MODE);

    tx.num_bodies = 0;
}

pub fn normal_legacy_body(drive: &[Drive], tx: &mut TxDatagram) -> Result<()> {
    if drive.len() != tx.num_transducers() {
        return Err(DriverError::NumberOfTransducerMismatch {
            a: tx.num_transducers(),
            b: drive.len(),
        }
        .into());
    }

    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::WRITE_BODY, true);

    tx.legacy_drives_mut()
        .iter_mut()
        .zip(drive)
        .for_each(|(d, s)| d.set(s));

    tx.num_bodies = tx.num_devices();

    Ok(())
}

pub fn normal_header(tx: &mut TxDatagram) {
    tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

    tx.header_mut()
        .fpga_flag
        .remove(FPGAControlFlags::LEGACY_MODE);
    tx.header_mut().fpga_flag.remove(FPGAControlFlags::STM_MODE);

    tx.num_bodies = 0;
}

pub fn normal_duty_body(drive: &[Drive], tx: &mut TxDatagram) -> Result<()> {
    if drive.len() != tx.num_transducers() {
        return Err(DriverError::NumberOfTransducerMismatch {
            a: tx.num_transducers(),
            b: drive.len(),
        }
        .into());
    }

    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::WRITE_BODY, true);
    tx.header_mut().cpu_flag.set(CPUControlFlags::IS_DUTY, true);

    tx.duties_mut()
        .iter_mut()
        .zip(drive)
        .for_each(|(d, s)| d.set(s));

    tx.num_bodies = tx.num_devices();

    Ok(())
}

pub fn normal_phase_body(drive: &[Drive], tx: &mut TxDatagram) -> Result<()> {
    if drive.len() != tx.num_transducers() {
        return Err(DriverError::NumberOfTransducerMismatch {
            a: tx.num_transducers(),
            b: drive.len(),
        }
        .into());
    }

    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::WRITE_BODY, true);
    tx.header_mut().cpu_flag.remove(CPUControlFlags::IS_DUTY);

    tx.phases_mut()
        .iter_mut()
        .zip(drive)
        .for_each(|(d, s)| d.set(s));

    tx.num_bodies = tx.num_devices();

    Ok(())
}

pub fn focus_stm_header(tx: &mut TxDatagram) {
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

pub fn focus_stm_send_size(total_size: usize, sent: usize, device_map: &[usize]) -> usize {
    let tr_num = device_map.iter().min().unwrap();
    let data_len = tr_num * std::mem::size_of::<u16>();
    let max_size = if sent == 0 {
        (data_len
            - std::mem::size_of::<u16>()
            - std::mem::size_of::<u32>()
            - std::mem::size_of::<u32>())
            / std::mem::size_of::<STMFocus>()
    } else {
        (data_len - std::mem::size_of::<u16>()) / std::mem::size_of::<STMFocus>()
    };
    (total_size - sent).min(max_size)
}

pub fn focus_stm_body(
    points: &[Vec<STMFocus>],
    sent: &mut usize,
    total_size: usize,
    freq_div: u32,
    sound_speed: f64,
    tx: &mut TxDatagram,
) -> Result<()> {
    if total_size > FOCUS_STM_BUF_SIZE_MAX {
        return Err(DriverError::FocusSTMPointSizeOutOfRange(total_size).into());
    }

    if points.is_empty() || points[0].is_empty() {
        return Ok(());
    }

    if *sent == 0 {
        if freq_div < FOCUS_STM_SAMPLING_FREQ_DIV_MIN {
            return Err(DriverError::FocusSTMFreqDivOutOfRange(freq_div).into());
        }
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::STM_BEGIN, true);
        let sound_speed = (sound_speed / 1e3 * 1024.0).round() as u32;
        (0..tx.num_devices()).for_each(|idx| {
            let d = tx.body_mut(idx);
            let s = &points[idx];
            d.focus_stm_initial_mut().set_size(s.len() as _);
            d.focus_stm_initial_mut().set_freq_div(freq_div);
            d.focus_stm_initial_mut().set_sound_speed(sound_speed);
            d.focus_stm_initial_mut().set_points(s);
        });
    } else {
        (0..tx.num_devices()).for_each(|idx| {
            let d = tx.body_mut(idx);
            let s = &points[idx];
            d.focus_stm_initial_mut().set_size(s.len() as _);
            d.focus_stm_initial_mut().set_points(s);
        });
    }

    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::WRITE_BODY, true);

    let send_size = points[0].len();
    if *sent + send_size == total_size {
        tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
    }

    tx.num_bodies = tx.num_devices();
    *sent += send_size;

    Ok(())
}

pub fn gain_stm_legacy_header(tx: &mut TxDatagram) {
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
    sent: &mut usize,
    freq_div: u32,
    mode: Mode,
    tx: &mut TxDatagram,
) -> Result<()> {
    if drives.len() > GAIN_STM_LEGACY_BUF_SIZE_MAX {
        return Err(DriverError::GainSTMLegacySizeOutOfRange(drives.len()).into());
    }

    let mut is_last_frame = false;

    if *sent == 0 {
        if freq_div < GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN {
            return Err(DriverError::GainSTMLegacyFreqDivOutOfRange(freq_div).into());
        }
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::STM_BEGIN, true);
        (0..tx.num_devices()).for_each(|idx| {
            let d = tx.body_mut(idx);
            d.gain_stm_initial_mut().set_freq_div(freq_div);
            d.gain_stm_initial_mut().set_mode(mode);
            d.gain_stm_initial_mut().set_cycle(drives.len());
        });
        *sent += 1;
    } else {
        match mode {
            Mode::PhaseDutyFull => {
                is_last_frame = *sent + 1 >= drives.len() + 1;
                tx.legacy_drives_mut()
                    .iter_mut()
                    .zip(drives[*sent - 1])
                    .for_each(|(d, s)| d.set(s));
                *sent += 1;
            }
            Mode::PhaseFull => {
                is_last_frame = *sent + 2 >= drives.len() + 1;
                tx.legacy_phase_full_mut()
                    .iter_mut()
                    .zip(drives[*sent - 1])
                    .for_each(|(d, s)| d.set(0, s));
                *sent += 1;
                if *sent - 1 < drives.len() {
                    tx.legacy_phase_full_mut()
                        .iter_mut()
                        .zip(drives[*sent - 1])
                        .for_each(|(d, s)| d.set(1, s));
                    *sent += 1;
                }
            }
            Mode::PhaseHalf => {
                is_last_frame = *sent + 4 >= drives.len() + 1;
                tx.legacy_phase_half_mut()
                    .iter_mut()
                    .zip(drives[*sent - 1])
                    .for_each(|(d, s)| d.set(0, s));
                *sent += 1;
                if *sent - 1 < drives.len() {
                    tx.legacy_phase_half_mut()
                        .iter_mut()
                        .zip(drives[*sent - 1])
                        .for_each(|(d, s)| d.set(1, s));
                    *sent += 1;
                }
                if *sent - 1 < drives.len() {
                    tx.legacy_phase_half_mut()
                        .iter_mut()
                        .zip(drives[*sent - 1])
                        .for_each(|(d, s)| d.set(2, s));
                    *sent += 1;
                }
                if *sent - 1 < drives.len() {
                    tx.legacy_phase_half_mut()
                        .iter_mut()
                        .zip(drives[*sent - 1])
                        .for_each(|(d, s)| d.set(3, s));
                    *sent += 1;
                }
            }
        }
    }

    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::WRITE_BODY, true);

    if is_last_frame {
        tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
    }

    tx.num_bodies = tx.num_devices();

    Ok(())
}

pub fn gain_stm_normal_header(tx: &mut TxDatagram) {
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
    drives: &[Vec<Drive>],
    sent: usize,
    freq_div: u32,
    mode: Mode,
    tx: &mut TxDatagram,
) -> Result<()> {
    if drives.len() > GAIN_STM_BUF_SIZE_MAX {
        return Err(DriverError::GainSTMSizeOutOfRange(drives.len()).into());
    }

    if mode == Mode::PhaseHalf {
        return Err(DriverError::PhaseHalfNotSupported.into());
    }

    tx.header_mut().cpu_flag.remove(CPUControlFlags::IS_DUTY);

    if sent == 0 {
        if freq_div < GAIN_STM_SAMPLING_FREQ_DIV_MIN {
            return Err(DriverError::GainSTMFreqDivOutOfRange(freq_div).into());
        }
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::STM_BEGIN, true);
        (0..tx.num_devices()).for_each(|idx| {
            let d = tx.body_mut(idx);
            d.gain_stm_initial_mut().set_freq_div(freq_div);
            d.gain_stm_initial_mut().set_mode(mode);
            d.gain_stm_initial_mut().set_cycle(drives.len());
        });
    } else {
        tx.phases_mut()
            .iter_mut()
            .zip(&drives[sent - 1])
            .for_each(|(d, s)| d.set(s));
    }

    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::WRITE_BODY, true);

    if sent + 1 == drives.len() + 1 {
        tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
    }

    tx.num_bodies = tx.num_devices();

    Ok(())
}

pub fn gain_stm_normal_duty_body(
    drives: &[Vec<Drive>],
    sent: usize,
    tx: &mut TxDatagram,
) -> Result<()> {
    if drives.len() > GAIN_STM_BUF_SIZE_MAX {
        return Err(DriverError::GainSTMSizeOutOfRange(drives.len()).into());
    }

    tx.header_mut().cpu_flag.set(CPUControlFlags::IS_DUTY, true);

    tx.duties_mut()
        .iter_mut()
        .zip(&drives[sent - 1])
        .for_each(|(d, s)| d.set(s));

    tx.header_mut()
        .cpu_flag
        .set(CPUControlFlags::WRITE_BODY, true);

    if sent + 1 == drives.len() + 1 {
        tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
    }

    tx.num_bodies = tx.num_devices();

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
