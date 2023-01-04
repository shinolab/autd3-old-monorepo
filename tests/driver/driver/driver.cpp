// File: autd3::driver::cpp
// Project: v2_7
// Created Date: 15/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 04/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26439 26495 26812)
#endif
#include <gtest/gtest.h>
#if _MSC_VER
#pragma warning(pop)
#endif

#include <autd3/driver/cpu/body.hpp>
#include <autd3/driver/cpu/datagram.hpp>
#include <random>

#include "autd3/driver/driver.hpp"
#include "autd3/driver/fpga/defined.hpp"

using autd3::driver::CPUControlFlags;
using autd3::driver::FPGAControlFlags;

constexpr size_t NUM_TRANS_IN_UNIT = 249;

TEST(Driver_Driver, operation_clear) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::Clear().pack(tx);

  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_CLEAR);
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(Driver_Driver, operation_null_header) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::NullHeader().msg_id(1).pack(tx);

  ASSERT_EQ(tx.header().msg_id, 1);
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::Mod));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSilencer));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSync));
  ASSERT_EQ(tx.header().size, 0);
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(Driver_Driver, operation_null_body) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::NullBody().pack(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(Driver_Driver, operation_sync_legacy) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<uint16_t> cycles;
  cycles.resize(NUM_TRANS_IN_UNIT * 10, 4096);
  ASSERT_TRUE(autd3::driver::Sync<autd3::driver::Legacy>().cycles(cycles).pack(tx));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::Mod));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSilencer));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSync));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], cycles[i]);
  ASSERT_EQ(tx.num_bodies, 10);

  cycles[0] = 0;
  ASSERT_FALSE(autd3::driver::Sync<autd3::driver::Legacy>().cycles(cycles).pack(tx));
}

TEST(Driver_Driver, operation_sync_normal) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<uint16_t> cycles;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_int_distribution dist(0, 0xFFFF);
  cycles.reserve(NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) cycles.emplace_back(dist(engine));

  ASSERT_TRUE(autd3::driver::Sync<autd3::driver::Normal>().cycles(cycles).pack(tx));

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::Mod));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSilencer));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSync));

  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], cycles[i]);

  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(Driver_Driver, operation_modulation) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<uint8_t> mod_data;
  for (size_t i = 0; i < autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE + autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE + 1; i++)
    mod_data.emplace_back(static_cast<uint8_t>(i));

  size_t sent = 0;

  ASSERT_TRUE(autd3::driver::Modulation().msg_id(1).mod_data(mod_data).sent(&sent).freq_div(1160).pack(tx));
  ASSERT_EQ(sent, autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE);
  ASSERT_EQ(tx.header().msg_id, 1);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::Mod));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::ModBegin));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ModEnd));
  ASSERT_EQ(tx.header().size, static_cast<uint16_t>(autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE));
  ASSERT_EQ(tx.header().mod_initial().freq_div, 1160);
  for (size_t i = 0; i < sent; i++) ASSERT_EQ(tx.header().mod_initial().data[i], static_cast<uint8_t>(i));

  ASSERT_TRUE(autd3::driver::Modulation().msg_id(0xFF).mod_data(mod_data).sent(&sent).freq_div(1160).pack(tx));
  ASSERT_EQ(sent, autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE + autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE);
  ASSERT_EQ(tx.header().msg_id, 0xFF);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::Mod));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ModBegin));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ModEnd));
  ASSERT_EQ(tx.header().size, static_cast<uint16_t>(autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE));
  for (size_t i = autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE; i < sent; i++)
    ASSERT_EQ(tx.header().mod_subsequent().data[i - autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE], static_cast<uint8_t>(i));

  ASSERT_TRUE(autd3::driver::Modulation().msg_id(0xF0).mod_data(mod_data).sent(&sent).freq_div(1160).pack(tx));
  ASSERT_EQ(sent, autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE + autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE + 1);
  ASSERT_EQ(tx.header().msg_id, 0xF0);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::Mod));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ModBegin));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::ModEnd));
  ASSERT_EQ(tx.header().size, 1);
  for (size_t i = autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE + autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE; i < sent; i++)
    ASSERT_EQ(tx.header().mod_subsequent().data[i - (autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE + autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE)],
              static_cast<uint8_t>(i));

  sent = 0;
  ASSERT_FALSE(autd3::driver::Modulation().msg_id(0xFF).mod_data(mod_data).sent(&sent).freq_div(1159).pack(tx));
}

TEST(Driver_Driver, operation_config_silencer) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  ASSERT_TRUE(autd3::driver::ConfigSilencer().msg_id(1).cycle(1044).step(4).pack(tx));
  ASSERT_EQ(tx.header().msg_id, 1);
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::Mod));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSync));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSilencer));
  ASSERT_EQ(tx.header().silencer().cycle, 1044);
  ASSERT_EQ(tx.header().silencer().step, 4);

  ASSERT_FALSE(autd3::driver::ConfigSilencer().msg_id(1).cycle(1043).step(4).pack(tx));
}

TEST(Driver_Driver, normal_legacy_header) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::GainHeader<autd3::driver::Legacy>().pack(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));

  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(Driver_Driver, operation_normal_legacy_body) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<autd3::driver::Drive> drives;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution<autd3::driver::autd3_float_t> dist(0, 1);
  drives.reserve(NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine)});

  autd3::driver::GainBody<autd3::driver::Legacy>().drives(drives).pack(tx);

  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));

  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) {
    ASSERT_EQ(tx.bodies_raw_ptr()[i] & 0xFF, autd3::driver::LegacyDrive::to_phase(drives[i]));
    ASSERT_EQ(tx.bodies_raw_ptr()[i] >> 8, autd3::driver::LegacyDrive::to_duty(drives[i]));
  }

  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(Driver_Driver, operation_normal_header) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::GainHeader<autd3::driver::Normal>().pack(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));

  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(Driver_Driver, operation_normal_duty_body) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<autd3::driver::Drive> drives;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution<autd3::driver::autd3_float_t> dist(0, 1);
  drives.reserve(NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine)});

  std::vector<uint16_t> cycles;
  cycles.resize(NUM_TRANS_IN_UNIT * 10, 4096);
  autd3::driver::GainBody<autd3::driver::NormalDuty>().drives(drives).cycles(cycles).pack(tx);

  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));

  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Duty::to_duty(drives[i], 4096));

  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(Driver_Driver, operation_normal_phase_body) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<autd3::driver::Drive> drives;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution<autd3::driver::autd3_float_t> dist(0, 1);
  drives.reserve(NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine)});

  std::vector<uint16_t> cycles;
  cycles.resize(NUM_TRANS_IN_UNIT * 10, 4096);
  autd3::driver::GainBody<autd3::driver::NormalPhase>().drives(drives).cycles(cycles).pack(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));

  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Phase::to_phase(drives[i], 4096));

  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(Driver_Driver, operation_focus_stm_header) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::FocusSTMHeader().pack(tx);
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(Driver_Driver, operation_focus_stm_subsequent) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  constexpr size_t size = 30;

  std::vector<autd3::driver::STMFocus> points_30;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution<autd3::driver::autd3_float_t> dist(-1000, 1000);
  std::uniform_int_distribution dist_u8(0, 0xFF);
  points_30.reserve(30);
  for (size_t i = 0; i < size; i++) points_30.emplace_back(dist(engine), dist(engine), dist(engine), static_cast<uint8_t>(dist_u8(engine)));

  std::vector<std::vector<autd3::driver::STMFocus>> points;
  points.reserve(10);
  for (int i = 0; i < 10; i++) points.emplace_back(points_30);

  constexpr double sound_speed = 340e3;
  constexpr uint32_t sp = 340 * 1024;

  autd3::driver::FocusSTMHeader().pack(tx);
  size_t sent = 0;
  ASSERT_TRUE(autd3::driver::FocusSTMBody()
                  .points(points)
                  .sent(&sent)
                  .total_size(size)
                  .freq_div(3224)
                  .sound_speed(sound_speed)
                  .start_idx(1)
                  .finish_idx(1)
                  .pack(tx));
  ASSERT_EQ(sent, size);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));

  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().data()[0], 30);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().data()[1], 3224);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().data()[2], 0);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().data()[3], sp & 0xFFFF);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().data()[4], sp >> 16);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().data()[5], 1);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().data()[6], 1);
  ASSERT_EQ(tx.num_bodies, 10);

  autd3::driver::FocusSTMHeader().pack(tx);
  sent = 0;
  ASSERT_TRUE(autd3::driver::FocusSTMBody()
                  .points(points)
                  .sent(&sent)
                  .total_size(500)
                  .freq_div(3234)
                  .sound_speed(sound_speed)
                  .start_idx(std::nullopt)
                  .finish_idx(std::nullopt)
                  .pack(tx));
  ASSERT_EQ(sent, size);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));

  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().data()[0], 30);
  ASSERT_EQ(tx.num_bodies, 10);

  autd3::driver::FocusSTMHeader().pack(tx);
  sent = 1;
  ASSERT_TRUE(autd3::driver::FocusSTMBody()
                  .points(points)
                  .sent(&sent)
                  .total_size(500)
                  .freq_div(3234)
                  .sound_speed(sound_speed)
                  .start_idx(29)
                  .finish_idx(0)
                  .pack(tx));
  ASSERT_EQ(sent, size + 1);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));

  autd3::driver::FocusSTMHeader().pack(tx);
  ASSERT_TRUE(autd3::driver::FocusSTMBody().points({}).sent(&sent).total_size(0).freq_div(3234).sound_speed(sound_speed).pack(tx));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
  ASSERT_EQ(tx.num_bodies, 0);

  sent = 0;
  ASSERT_FALSE(autd3::driver::FocusSTMBody()
                   .points(points)
                   .sent(&sent)
                   .total_size(0)
                   .freq_div(3224)
                   .sound_speed(sound_speed)
                   .start_idx(30)
                   .finish_idx(0)
                   .pack(tx));
  ASSERT_FALSE(autd3::driver::FocusSTMBody()
                   .points(points)
                   .sent(&sent)
                   .total_size(0)
                   .freq_div(3224)
                   .sound_speed(sound_speed)
                   .start_idx(0)
                   .finish_idx(30)
                   .pack(tx));
}

TEST(Driver_Driver, operation_gain_stm_legacy_header) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::GainSTMHeader<autd3::driver::Legacy>().pack(tx);
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(Driver_Driver, operation_gain_stm_legacy_body) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<std::vector<autd3::driver::Drive>> drives_list;
  drives_list.reserve(5);
  for (int i = 0; i < 5; i++) {
    std::vector<autd3::driver::Drive> drives;
    std::random_device seed_gen;
    std::mt19937 engine(seed_gen());
    std::uniform_real_distribution<autd3::driver::autd3_float_t> dist(0, 1);
    drives.reserve(NUM_TRANS_IN_UNIT * 10);
    for (size_t j = 0; j < NUM_TRANS_IN_UNIT * 10; j++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine)});
    drives_list.emplace_back(drives);
  }

  autd3::driver::GainSTMHeader<autd3::driver::Legacy>().pack(tx);
  size_t sent = 0;
  ASSERT_TRUE(autd3::driver::GainSTMBody<autd3::driver::Legacy>()
                  .drives(drives_list)
                  .sent(&sent)
                  .freq_div(3224)
                  .mode(autd3::driver::GainSTMMode::PhaseDutyFull)
                  .start_idx(4)
                  .finish_idx(4)
                  .pack(tx));
  ASSERT_EQ(sent, 1);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[0], 3224);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[1], 0);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[3], 5);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[4], 4);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[5], 4);
  ASSERT_EQ(tx.num_bodies, 10);

  autd3::driver::GainSTMHeader<autd3::driver::Legacy>().pack(tx);
  ASSERT_TRUE(autd3::driver::GainSTMBody<autd3::driver::Legacy>()
                  .drives(drives_list)
                  .sent(&sent)
                  .freq_div(3224)
                  .mode(autd3::driver::GainSTMMode::PhaseDutyFull)
                  .start_idx(std::nullopt)
                  .finish_idx(std::nullopt)
                  .pack(tx));
  ASSERT_EQ(sent, 2);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) {
    ASSERT_EQ(tx.bodies_raw_ptr()[i] & 0xFF, autd3::driver::LegacyDrive::to_phase(drives_list[0][i]));
    ASSERT_EQ(tx.bodies_raw_ptr()[i] >> 8, autd3::driver::LegacyDrive::to_duty(drives_list[0][i]));
  }
  ASSERT_EQ(tx.num_bodies, 10);

  autd3::driver::GainSTMHeader<autd3::driver::Legacy>().pack(tx);
  sent = 5;
  ASSERT_TRUE(autd3::driver::GainSTMBody<autd3::driver::Legacy>()
                  .drives(drives_list)
                  .sent(&sent)
                  .freq_div(3224)
                  .mode(autd3::driver::GainSTMMode::PhaseDutyFull)
                  .start_idx(std::nullopt)
                  .finish_idx(std::nullopt)
                  .pack(tx));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) {
    ASSERT_EQ(tx.bodies_raw_ptr()[i] & 0xFF, autd3::driver::LegacyDrive::to_phase(drives_list[4][i]));
    ASSERT_EQ(tx.bodies_raw_ptr()[i] >> 8, autd3::driver::LegacyDrive::to_duty(drives_list[4][i]));
  }
  ASSERT_EQ(tx.num_bodies, 10);

  sent = 0;
  ASSERT_FALSE(autd3::driver::GainSTMBody<autd3::driver::Legacy>()
                   .drives(drives_list)
                   .sent(&sent)
                   .freq_div(3224)
                   .mode(autd3::driver::GainSTMMode::PhaseDutyFull)
                   .start_idx(5)
                   .finish_idx(0)
                   .pack(tx));
  ASSERT_FALSE(autd3::driver::GainSTMBody<autd3::driver::Legacy>()
                   .drives(drives_list)
                   .sent(&sent)
                   .freq_div(3224)
                   .mode(autd3::driver::GainSTMMode::PhaseDutyFull)
                   .start_idx(0)
                   .finish_idx(5)
                   .pack(tx));
}

TEST(Driver_Driver, operation_gain_stm_normal_header) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::GainSTMHeader<autd3::driver::Normal>().pack(tx);
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(Driver_Driver, operation_gain_stm_normal_phase) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<std::vector<autd3::driver::Drive>> drives_list;
  drives_list.reserve(5);
  for (int i = 0; i < 5; i++) {
    std::vector<autd3::driver::Drive> drives;
    std::random_device seed_gen;
    std::mt19937 engine(seed_gen());
    std::uniform_real_distribution<autd3::driver::autd3_float_t> dist(0, 1);
    drives.reserve(NUM_TRANS_IN_UNIT * 10);
    for (size_t j = 0; j < NUM_TRANS_IN_UNIT * 10; j++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine)});
    drives_list.emplace_back(drives);
  }

  std::vector<uint16_t> cycles;
  cycles.resize(NUM_TRANS_IN_UNIT * 10, 4096);

  autd3::driver::GainSTMHeader<autd3::driver::Normal>().pack(tx);
  size_t sent = 0;
  ASSERT_TRUE(autd3::driver::GainSTMBody<autd3::driver::NormalPhase>()
                  .drives(drives_list)
                  .cycles(cycles)
                  .sent(&sent)
                  .freq_div(3224)
                  .mode(autd3::driver::GainSTMMode::PhaseDutyFull)
                  .start_idx(4)
                  .finish_idx(4)
                  .pack(tx));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[0], 3224);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[1], 0);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[4], 4);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[5], 4);
  ASSERT_EQ(tx.num_bodies, 10);

  autd3::driver::GainSTMHeader<autd3::driver::Normal>().pack(tx);
  sent = 1;
  ASSERT_TRUE(autd3::driver::GainSTMBody<autd3::driver::NormalPhase>()
                  .drives(drives_list)
                  .cycles(cycles)
                  .sent(&sent)
                  .freq_div(3224)
                  .mode(autd3::driver::GainSTMMode::PhaseDutyFull)
                  .start_idx(std::nullopt)
                  .finish_idx(std::nullopt)
                  .pack(tx));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Phase::to_phase(drives_list[0][i], 4096));
  ASSERT_EQ(tx.num_bodies, 10);

  autd3::driver::GainSTMHeader<autd3::driver::Normal>().pack(tx);
  sent = 5;
  ASSERT_TRUE(autd3::driver::GainSTMBody<autd3::driver::NormalPhase>()
                  .drives(drives_list)
                  .cycles(cycles)
                  .sent(&sent)
                  .freq_div(3224)
                  .mode(autd3::driver::GainSTMMode::PhaseDutyFull)
                  .start_idx(std::nullopt)
                  .finish_idx(std::nullopt)
                  .pack(tx));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Phase::to_phase(drives_list[4][i], 4096));
  ASSERT_EQ(tx.num_bodies, 10);

  sent = 0;
  ASSERT_FALSE(autd3::driver::GainSTMBody<autd3::driver::NormalPhase>()
                   .drives(drives_list)
                   .cycles(cycles)
                   .sent(&sent)
                   .freq_div(3224)
                   .mode(autd3::driver::GainSTMMode::PhaseDutyFull)
                   .start_idx(5)
                   .finish_idx(0)
                   .pack(tx));
  sent = 0;
  ASSERT_FALSE(autd3::driver::GainSTMBody<autd3::driver::NormalPhase>()
                   .drives(drives_list)
                   .cycles(cycles)
                   .sent(&sent)
                   .freq_div(3224)
                   .mode(autd3::driver::GainSTMMode::PhaseDutyFull)
                   .start_idx(0)
                   .finish_idx(5)
                   .pack(tx));
}

TEST(Driver_Driver, operation_gain_stm_normal_duty) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<std::vector<autd3::driver::Drive>> drives_list;
  drives_list.reserve(5);
  for (int i = 0; i < 5; i++) {
    std::vector<autd3::driver::Drive> drives;
    std::random_device seed_gen;
    std::mt19937 engine(seed_gen());
    std::uniform_real_distribution<autd3::driver::autd3_float_t> dist(0, 1);
    drives.reserve(NUM_TRANS_IN_UNIT * 10);
    for (size_t j = 0; j < NUM_TRANS_IN_UNIT * 10; j++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine)});
    drives_list.emplace_back(drives);
  }

  std::vector<uint16_t> cycles;
  cycles.resize(NUM_TRANS_IN_UNIT * 10, 4096);

  autd3::driver::GainSTMHeader<autd3::driver::Normal>().pack(tx);
  size_t sent = 1;
  ASSERT_TRUE(autd3::driver::GainSTMBody<autd3::driver::NormalDuty>()
                  .drives(drives_list)
                  .cycles(cycles)
                  .sent(&sent)
                  .freq_div(3224)
                  .mode(autd3::driver::GainSTMMode::PhaseDutyFull)
                  .start_idx(4)
                  .finish_idx(4)
                  .pack(tx));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Duty::to_duty(drives_list[0][i], 4096));
  ASSERT_EQ(tx.num_bodies, 10);

  autd3::driver::GainSTMHeader<autd3::driver::Normal>().pack(tx);
  sent = 5;
  ASSERT_TRUE(autd3::driver::GainSTMBody<autd3::driver::NormalDuty>()
                  .drives(drives_list)
                  .cycles(cycles)
                  .sent(&sent)
                  .freq_div(3224)
                  .mode(autd3::driver::GainSTMMode::PhaseDutyFull)
                  .pack(tx));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Duty::to_duty(drives_list[4][i], 4096));
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(Driver_Driver, operation_force_fan) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::ForceFan(true).pack(tx);
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::ForceFan));

  autd3::driver::ForceFan(false).pack(tx);
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::ForceFan));
}

TEST(Driver_Driver, operation_reads_fpga_info) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::ReadsFPGAInfo(true).pack(tx);
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::ReadsFPGAInfo));

  autd3::driver::ReadsFPGAInfo(false).pack(tx);
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::ReadsFPGAInfo));
}

TEST(Driver_Driver, operation_cpu_version) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::CPUVersion().pack(tx);
  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_RD_CPU_VERSION);
  ASSERT_EQ(static_cast<uint8_t>(tx.header().cpu_flag.value()), autd3::driver::MSG_RD_CPU_VERSION);
}

TEST(Driver_Driver, operation_fpga_version) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::FPGAVersion().pack(tx);
  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_RD_FPGA_VERSION);
  ASSERT_EQ(static_cast<uint8_t>(tx.header().cpu_flag.value()), autd3::driver::MSG_RD_FPGA_VERSION);
}

TEST(Driver_Driver, operation_fpga_functions) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::FPGAFunctions().pack(tx);
  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_RD_FPGA_FUNCTION);
  ASSERT_EQ(static_cast<uint8_t>(tx.header().cpu_flag.value()), autd3::driver::MSG_RD_FPGA_FUNCTION);
}
