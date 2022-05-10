// File: operation.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 10/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <sstream>

#include "datagram.hpp"

namespace autd3::driver {

inline void clear(TxDatagram& tx) noexcept {
  tx.header().msg_id = MSG_CLEAR;
  tx.num_bodies = 0;
}

inline void sync(const uint8_t msg_id, const uint16_t sync_cycle_ticks, const std::span<uint16_t> cycles, TxDatagram& tx) noexcept {
  tx.header().msg_id = msg_id;
  tx.header().cpu_flag.set(CPUControlFlags::DO_SYNC);
  tx.header().sync_header().ecat_sync_cycle_ticks = sync_cycle_ticks;

  for (size_t i = 0; i < tx.bodies().size(); i++) {
    auto& dst = tx.bodies()[i];
    const auto src = cycles.subspan(i * NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT);
    std::memcpy(dst.data, src.data(), src.size_bytes());
  }

  tx.num_bodies = tx.bodies().size();
}

inline void modulation(const uint8_t msg_id, const std::span<uint8_t> mod_data, const bool is_first_frame, const uint32_t freq_div,
                       const bool is_last_frame, TxDatagram& tx) noexcept(false) {
  tx.header().msg_id = msg_id;
  tx.header().cpu_flag.remove(CPUControlFlags::DO_SYNC);
  tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SILENCER);

  if (is_first_frame) {
    if (freq_div < MOD_SAMPLING_FREQ_DIV_MIN) {
      std::stringstream ss;
      ss << "Modulation frequency division is oud of range. Minimum is " << MOD_SAMPLING_FREQ_DIV_MIN << ", but you use " << freq_div;
      throw std::runtime_error(ss.str());
    }

    tx.header().cpu_flag.set(CPUControlFlags::MOD_BEGIN);
    tx.header().mod_head().freq_div = freq_div;
    std::memcpy(tx.header().mod_head().data, mod_data.data(), mod_data.size_bytes());
  } else {
    std::memcpy(tx.header().mod_body().data, mod_data.data(), mod_data.size_bytes());
  }
  tx.header().size = static_cast<uint8_t>(mod_data.size());

  if (is_last_frame) {
    tx.header().cpu_flag.set(CPUControlFlags::MOD_END);
  }
}

// pub fn config_silencer(msg_id : u8, cycle : u16, step : u16, tx : &mut TxDatagram)->Result<()> {
//   if cycle
//     < SILENCER_CYCLE_MIN { return Err(FPGAError::SilencerCycleOutOfRange(cycle).into()); }

//   tx.header_mut().msg_id = msg_id;
//   tx.header_mut().cpu_flag.remove(CPUControlFlags::DO_SYNC);
//   tx.header_mut().cpu_flag.set(CPUControlFlags::CONFIG_SILENCER, true);

//   tx.header_mut().silencer_header_mut().cycle = cycle;
//   tx.header_mut().silencer_header_mut().step = step;

//   Ok(())
// }

// pub fn normal_legacy(msg_id : u8, drive : &[LegacyDrive], tx : &mut TxDatagram)->Result<()> {
//   if drive
//     .len() / NUM_TRANS_IN_UNIT != tx.body().len() {
//       return Err(CPUError::DeviceNumberNotCorrect{
//         a : tx.body().len(),
//         b : drive.len() / NUM_TRANS_IN_UNIT,
//       }
//                      .into());
//     }

//   tx.header_mut().msg_id = msg_id;

//   tx.header_mut().fpga_flag.set(FPGAControlFlags::LEGACY_MODE, true);
//   tx.header_mut().fpga_flag.remove(FPGAControlFlags::STM_MODE);

//   tx.body_mut().iter_mut().zip(drive.chunks(NUM_TRANS_IN_UNIT)).for_each(| (d, s) | d.legacy_drives_mut().copy_from_slice(s));

//   tx.num_bodies = tx.body().len();

//   Ok(())
// }

// pub fn normal_duty(msg_id : u8, drive : &[Duty], tx : &mut TxDatagram)->Result<()> {
//   if drive
//     .len() / NUM_TRANS_IN_UNIT != tx.body().len() {
//       return Err(CPUError::DeviceNumberNotCorrect{
//         a : tx.body().len(),
//         b : drive.len() / NUM_TRANS_IN_UNIT,
//       }
//                      .into());
//     }

//   tx.header_mut().msg_id = msg_id;

//   tx.header_mut().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
//   tx.header_mut().fpga_flag.remove(FPGAControlFlags::STM_MODE);

//   tx.header_mut().cpu_flag.set(CPUControlFlags::IS_DUTY, true);

//   tx.body_mut().iter_mut().zip(drive.chunks(NUM_TRANS_IN_UNIT)).for_each(| (d, s) | d.duties_mut().copy_from_slice(s));

//   tx.num_bodies = tx.body().len();

//   Ok(())
// }

// pub fn normal_phase(msg_id : u8, drive : &[Phase], tx : &mut TxDatagram)->Result<()> {
//   if drive
//     .len() / NUM_TRANS_IN_UNIT != tx.body().len() {
//       return Err(CPUError::DeviceNumberNotCorrect{
//         a : tx.body().len(),
//         b : drive.len() / NUM_TRANS_IN_UNIT,
//       }
//                      .into());
//     }

//   tx.header_mut().msg_id = msg_id;

//   tx.header_mut().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
//   tx.header_mut().fpga_flag.remove(FPGAControlFlags::STM_MODE);

//   tx.header_mut().cpu_flag.remove(CPUControlFlags::IS_DUTY);

//   tx.body_mut().iter_mut().zip(drive.chunks(NUM_TRANS_IN_UNIT)).for_each(| (d, s) | d.phases_mut().copy_from_slice(s));

//   tx.num_bodies = tx.body().len();

//   Ok(())
// }

// pub fn point_stm(msg_id
//                  : u8, points
//                  : &[Vec<SeqFocus>], is_first_frame
//                  : bool, freq_div
//                  : u32, sound_speed
//                  : f64, is_last_frame
//                  : bool, tx
//                  : &mut TxDatagram, )
//     ->Result<()> {
//   tx.header_mut().msg_id = msg_id;

//   tx.header_mut().fpga_flag.set(FPGAControlFlags::STM_MODE, true);
//   tx.header_mut().fpga_flag.remove(FPGAControlFlags::STM_GAIN_MODE);

//   if is_first_frame {
//         for
//           s in points {
//             if s
//               .len() > POINT_STM_HEAD_DATA_SIZE { return Err(CPUError::PointSTMHeadDataSizeOutOfRange(s.len()).into()); }
//           }
//   }

//   if
//     !is_first_frame {
//         for
//           s in points {
//             if s
//               .len() > POINT_STM_BODY_DATA_SIZE { return Err(CPUError::PointSTMBodyDataSizeOutOfRange(s.len()).into()); }
//           }
//     }

//   if is_first_frame {
//     if freq_div
//       < STM_SAMPLING_FREQ_DIV_MIN { return Err(FPGAError::STMFreqDivOutOfRange(freq_div).into()); }
//     tx.header_mut().cpu_flag.set(CPUControlFlags::STM_BEGIN, true);
//     let sound_speed = (sound_speed * 1024.0).round() as u32;
//     tx.body_mut().iter_mut().zip(points).for_each(| (d, s) | {
//       d.point_stm_head_mut().set_size(s.len() as _);
//       d.point_stm_head_mut().set_freq_div(freq_div);
//       d.point_stm_head_mut().set_sound_speed(sound_speed);
//       d.point_stm_head_mut().set_points(s);
//     });
//   } else {
//     tx.body_mut().iter_mut().zip(points).for_each(| (d, s) | {
//       d.point_stm_body_mut().set_size(s.len() as _);
//       d.point_stm_body_mut().set_points(s);
//     });
//   }

//   if is_last_frame {
//     tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
//   }

//   tx.num_bodies = tx.body().len();

//   Ok(())
// }

// pub fn gain_stm_legacy(msg_id
//                        : u8, gain
//                        : &[LegacyDrive], is_first_frame
//                        : bool, freq_div
//                        : u32, is_last_frame
//                        : bool, tx
//                        : &mut TxDatagram, )
//     ->Result<()> {
//   tx.header_mut().msg_id = msg_id;

//   tx.header_mut().fpga_flag.set(FPGAControlFlags::LEGACY_MODE, true);
//   tx.header_mut().fpga_flag.set(FPGAControlFlags::STM_MODE, true);
//   tx.header_mut().fpga_flag.set(FPGAControlFlags::STM_GAIN_MODE, true);

//   if is_first_frame {
//     if freq_div
//       < STM_SAMPLING_FREQ_DIV_MIN { return Err(FPGAError::STMFreqDivOutOfRange(freq_div).into()); }
//     tx.header_mut().cpu_flag.set(CPUControlFlags::STM_BEGIN, true);
//     tx.body_mut().iter_mut().for_each(| d | { d.gain_stm_head_mut().set_freq_div(freq_div); });
//   } else {
//     tx.body_mut().iter_mut().zip(gain.chunks(NUM_TRANS_IN_UNIT)).for_each(| (d, s) | {
//       d.gain_stm_body_mut().legacy_drives_mut().clone_from_slice(s);
//     });
//   }

//   if is_last_frame {
//     tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
//   }

//   tx.num_bodies = tx.body().len();

//   Ok(())
// }

// pub fn gain_stm_normal_phase(msg_id : u8, phase : &[Phase], is_first_frame : bool, freq_div : u32, tx : &mut TxDatagram, )->Result<()> {
//   tx.header_mut().msg_id = msg_id;

//   tx.header_mut().cpu_flag.remove(CPUControlFlags::IS_DUTY);

//   tx.header_mut().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
//   tx.header_mut().fpga_flag.set(FPGAControlFlags::STM_MODE, true);
//   tx.header_mut().fpga_flag.set(FPGAControlFlags::STM_GAIN_MODE, true);

//   if is_first_frame {
//     if freq_div
//       < STM_SAMPLING_FREQ_DIV_MIN { return Err(FPGAError::STMFreqDivOutOfRange(freq_div).into()); }
//     tx.header_mut().cpu_flag.set(CPUControlFlags::STM_BEGIN, true);
//     tx.body_mut().iter_mut().for_each(| d | { d.gain_stm_head_mut().set_freq_div(freq_div); });
//   } else {
//     tx.body_mut().iter_mut().zip(phase.chunks(NUM_TRANS_IN_UNIT)).for_each(| (d, s) | { d.gain_stm_body_mut().phases_mut().clone_from_slice(s);
//     });
//   }

//   tx.num_bodies = tx.body().len();

//   Ok(())
// }

// pub fn gain_stm_normal_duty(msg_id
//                             : u8, duty
//                             : &[Duty], is_first_frame
//                             : bool, freq_div
//                             : u32, is_last_frame
//                             : bool, tx
//                             : &mut TxDatagram, )
//     ->Result<()> {
//   tx.header_mut().msg_id = msg_id;

//   tx.header_mut().cpu_flag.set(CPUControlFlags::IS_DUTY, true);

//   tx.header_mut().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
//   tx.header_mut().fpga_flag.set(FPGAControlFlags::STM_MODE, true);
//   tx.header_mut().fpga_flag.set(FPGAControlFlags::STM_GAIN_MODE, true);

//   if is_first_frame {
//     if freq_div
//       < STM_SAMPLING_FREQ_DIV_MIN { return Err(FPGAError::STMFreqDivOutOfRange(freq_div).into()); }
//     tx.header_mut().cpu_flag.set(CPUControlFlags::STM_BEGIN, true);
//     tx.body_mut().iter_mut().for_each(| d | { d.gain_stm_head_mut().set_freq_div(freq_div); });
//   } else {
//     tx.body_mut().iter_mut().zip(duty.chunks(NUM_TRANS_IN_UNIT)).for_each(| (d, s) | { d.gain_stm_body_mut().duties_mut().clone_from_slice(s);
//     });
//   }

//   if is_last_frame {
//     tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
//   }

//   tx.num_bodies = tx.body().len();

//   Ok(())
// }

// pub fn force_fan(tx : &mut TxDatagram, value : bool) { tx.header_mut().fpga_flag.set(FPGAControlFlags::FORCE_FAN, value); }

// pub fn reads_fpga_info(tx : &mut TxDatagram, value : bool) { tx.header_mut().cpu_flag.set(CPUControlFlags::READS_FPGA_INFO, value); }

// pub fn cpu_version(tx : &mut TxDatagram) {
//   tx.header_mut().msg_id = MSG_RD_CPU_VERSION;
//   tx.header_mut().cpu_flag = CPUControlFlags::from_bits(0x02).unwrap();  // For backward compatibility before 1.9
//   tx.num_bodies = 0;
// }

// pub fn fpga_version(tx : &mut TxDatagram) {
//   tx.header_mut().msg_id = MSG_RD_FPGA_VERSION;
//   tx.header_mut().cpu_flag = CPUControlFlags::from_bits(0x04).unwrap();  // For backward compatibility before 1.9
//   tx.num_bodies = 0;
// }

// pub fn fpga_functions(tx : &mut TxDatagram) {
//   tx.header_mut().msg_id = MSG_RD_FPGA_FUNCTION;
//   tx.header_mut().cpu_flag = CPUControlFlags::from_bits(0x05).unwrap();  // For backward compatibility before 1.9
//   tx.num_bodies = 0;
// }

}  // namespace autd3::driver
