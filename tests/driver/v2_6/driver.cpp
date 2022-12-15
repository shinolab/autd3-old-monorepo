// File: driver.cpp
// Project: v2_6
// Created Date: 02/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 15/12/2022
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

#include <autd3/driver/common/cpu/body.hpp>
#include <autd3/driver/common/cpu/datagram.hpp>
#include <random>

#include "autd3/driver/common/fpga/defined.hpp"
#include "autd3/driver/v2_6/driver.hpp"

using autd3::driver::CPUControlFlags;
using autd3::driver::FPGAControlFlags;

constexpr size_t NUM_TRANS_IN_UNIT = 249;

TEST(DriverV2_6Driver, operation_clear_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.clear(tx);

  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_CLEAR);
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(DriverV2_6Driver, operation_null_header_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.null_header(1, tx);

  ASSERT_EQ(tx.header().msg_id, 1);
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::MOD));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_SILENCER));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_SYNC));
  ASSERT_EQ(tx.header().size, 0);
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(DriverV2_6Driver, operation_null_body_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.null_body(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(DriverV2_6Driver, operation_sync_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<uint16_t> cycle;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_int_distribution dist(0, 0xFFFF);
  cycle.reserve(NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) cycle.emplace_back(dist(engine));

  driver.sync(cycle, tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::MOD));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_SILENCER));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_SYNC));

  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], cycle[i]);

  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(DriverV2_6Driver, operation_modulation_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<uint8_t> mod_data;
  for (size_t i = 0; i < autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE + autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE + 1; i++)
    mod_data.emplace_back(static_cast<uint8_t>(i));

  size_t sent = 0;

  driver.modulation(1, mod_data, sent, 580, tx);
  ASSERT_EQ(sent, autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE);
  ASSERT_EQ(tx.header().msg_id, 1);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::MOD));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::MOD_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::MOD_END));
  ASSERT_EQ(tx.header().size, static_cast<uint16_t>(autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE));
  ASSERT_EQ(tx.header().mod_initial().freq_div, 580);
  for (size_t i = 0; i < sent; i++) ASSERT_EQ(tx.header().mod_initial().data[i], static_cast<uint8_t>(i));

  driver.modulation(0xFF, mod_data, sent, 580, tx);
  ASSERT_EQ(sent, autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE + autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE);
  ASSERT_EQ(tx.header().msg_id, 0xFF);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::MOD));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::MOD_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::MOD_END));
  ASSERT_EQ(tx.header().size, static_cast<uint16_t>(autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE));
  for (size_t i = autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE; i < sent; i++)
    ASSERT_EQ(tx.header().mod_subsequent().data[i - autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE], static_cast<uint8_t>(i));

  driver.modulation(0xF0, mod_data, sent, 580, tx);
  ASSERT_EQ(sent, autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE + autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE + 1);
  ASSERT_EQ(tx.header().msg_id, 0xF0);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::MOD));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::MOD_BEGIN));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::MOD_END));
  ASSERT_EQ(tx.header().size, 1);
  for (size_t i = autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE + autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE; i < sent; i++)
    ASSERT_EQ(tx.header().mod_subsequent().data[i - (autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE + autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE)],
              static_cast<uint8_t>(i));

  sent = 0;
  ASSERT_FALSE(driver.modulation(0xFF, mod_data, sent, 579, tx));
}

TEST(DriverV2_6Driver, operation_config_silencer_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.config_silencer(1, 522, 4, tx);
  ASSERT_EQ(tx.header().msg_id, 1);
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::MOD));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_SYNC));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_SILENCER));
  ASSERT_EQ(tx.header().silencer().cycle, 522);
  ASSERT_EQ(tx.header().silencer().step, 4);

  ASSERT_FALSE(driver.config_silencer(1, 521, 4, tx));
}

TEST(DriverV2_6Driver, normal_legacy_header_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.normal_legacy_header(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));

  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(DriverV2_6Driver, operation_normal_legacy_body_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<autd3::driver::Drive> drives;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution dist(0.0, 1.0);
  drives.reserve(NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine), 4096});

  driver.normal_legacy_body(drives, tx);

  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));

  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) {
    ASSERT_EQ(tx.bodies_raw_ptr()[i] & 0xFF, autd3::driver::LegacyDrive::to_phase(drives[i]));
    ASSERT_EQ(tx.bodies_raw_ptr()[i] >> 8, autd3::driver::LegacyDrive::to_duty(drives[i]));
  }

  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(DriverV2_6Driver, operation_normal_header_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.normal_header(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));

  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(DriverV2_6Driver, operation_normal_duty_body_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<autd3::driver::Drive> drives;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution dist(0.0, 1.0);
  drives.reserve(NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine), 4096});

  driver.normal_duty_body(drives, tx);

  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));

  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Duty::to_duty(drives[i]));

  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(DriverV2_6Driver, operation_normal_phase_body_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<autd3::driver::Drive> drives;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution dist(0.0, 1.0);
  drives.reserve(NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine), 4096});

  driver.normal_phase_body(drives, tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));

  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Phase::to_phase(drives[i]));

  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(DriverV2_6Driver, operation_focus_stm_header_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.focus_stm_header(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STM_GAIN_MODE));

  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(DriverV2_6Driver, operation_focus_stm_subsequent_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  constexpr size_t size = 30;

  std::vector<autd3::driver::STMFocus> points_30;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution dist(-1000.0, 1000.0);
  std::uniform_int_distribution dist_u8(0, 0xFF);
  points_30.reserve(30);
  for (size_t i = 0; i < size; i++)
    points_30.emplace_back(autd3::driver::STMFocus(dist(engine), dist(engine), dist(engine), static_cast<uint8_t>(dist_u8(engine))));

  std::vector<std::vector<autd3::driver::STMFocus>> points;
  points.reserve(10);
  for (int i = 0; i < 10; i++) points.emplace_back(points_30);

  constexpr double sound_speed = 340e3;
  constexpr uint32_t sp = 340 * 1024;

  driver.focus_stm_header(tx);
  size_t sent = 0;
  driver.focus_stm_body(points, sent, size, 3224, sound_speed, std::nullopt, tx);

  ASSERT_EQ(sent, size);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));

  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().data()[0], 30);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().data()[1], 3224);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().data()[2], 0);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().data()[3], sp & 0xFFFF);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().data()[4], sp >> 16);
  ASSERT_EQ(tx.num_bodies, 10);

  driver.focus_stm_header(tx);
  sent = 0;
  driver.focus_stm_body(points, sent, 500, 3224, sound_speed, std::nullopt, tx);

  ASSERT_EQ(sent, size);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));

  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().data()[0], 30);
  ASSERT_EQ(tx.num_bodies, 10);

  driver.focus_stm_header(tx);
  sent = 1;
  driver.focus_stm_body(points, sent, 500, 3224, sound_speed, std::nullopt, tx);
  ASSERT_EQ(sent, size + 1);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));

  driver.focus_stm_header(tx);
  driver.focus_stm_body({}, sent, 0, 3224, sound_speed, std::nullopt, tx);
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STM_GAIN_MODE));
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(DriverV2_6Driver, operation_gain_stm_legacy_header_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.gain_stm_legacy_header(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STM_GAIN_MODE));

  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(DriverV2_6Driver, operation_gain_stm_legacy_body_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<std::vector<autd3::driver::Drive>> drives_list;
  drives_list.reserve(5);
  for (int i = 0; i < 5; i++) {
    std::vector<autd3::driver::Drive> drives;
    std::random_device seed_gen;
    std::mt19937 engine(seed_gen());
    std::uniform_real_distribution dist(0.0, 1.0);
    drives.reserve(NUM_TRANS_IN_UNIT * 10);
    for (size_t j = 0; j < NUM_TRANS_IN_UNIT * 10; j++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine), 4096});
    drives_list.emplace_back(drives);
  }

  driver.gain_stm_legacy_header(tx);
  size_t sent = 0;
  driver.gain_stm_legacy_body(drives_list, sent, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, std::nullopt, tx);
  ASSERT_EQ(sent, 1);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[0], 3224);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[1], 0);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[3], 5);
  ASSERT_EQ(tx.num_bodies, 10);

  driver.gain_stm_legacy_header(tx);
  driver.gain_stm_legacy_body(drives_list, sent, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, std::nullopt, tx);
  ASSERT_EQ(sent, 2);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) {
    ASSERT_EQ(tx.bodies_raw_ptr()[i] & 0xFF, autd3::driver::LegacyDrive::to_phase(drives_list[0][i]));
    ASSERT_EQ(tx.bodies_raw_ptr()[i] >> 8, autd3::driver::LegacyDrive::to_duty(drives_list[0][i]));
  }
  ASSERT_EQ(tx.num_bodies, 10);

  driver.gain_stm_legacy_header(tx);
  sent = 5;
  driver.gain_stm_legacy_body(drives_list, sent, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, std::nullopt, tx);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) {
    ASSERT_EQ(tx.bodies_raw_ptr()[i] & 0xFF, autd3::driver::LegacyDrive::to_phase(drives_list[4][i]));
    ASSERT_EQ(tx.bodies_raw_ptr()[i] >> 8, autd3::driver::LegacyDrive::to_duty(drives_list[4][i]));
  }
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(DriverV2_6Driver, operation_gain_stm_normal_header_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.gain_stm_normal_header(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STM_GAIN_MODE));

  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(DriverV2_6Driver, operation_gain_stm_normal_phase_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<std::vector<autd3::driver::Drive>> drives_list;
  drives_list.reserve(5);
  for (int i = 0; i < 5; i++) {
    std::vector<autd3::driver::Drive> drives;
    std::random_device seed_gen;
    std::mt19937 engine(seed_gen());
    std::uniform_real_distribution dist(0.0, 1.0);
    drives.reserve(NUM_TRANS_IN_UNIT * 10);
    for (size_t j = 0; j < NUM_TRANS_IN_UNIT * 10; j++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine), 4096});
    drives_list.emplace_back(drives);
  }

  driver.gain_stm_normal_header(tx);
  driver.gain_stm_normal_phase(drives_list, 0, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, std::nullopt, tx);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[0], 3224);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[1], 0);
  ASSERT_EQ(tx.num_bodies, 10);

  driver.gain_stm_normal_header(tx);
  driver.gain_stm_normal_phase(drives_list, 1, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, std::nullopt, tx);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Phase::to_phase(drives_list[0][i]));
  ASSERT_EQ(tx.num_bodies, 10);

  driver.gain_stm_normal_header(tx);
  driver.gain_stm_normal_phase(drives_list, 5, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, std::nullopt, tx);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Phase::to_phase(drives_list[4][i]));
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(DriverV2_6Driver, operation_gain_stm_normal_duty_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<std::vector<autd3::driver::Drive>> drives_list;
  drives_list.reserve(5);
  for (int i = 0; i < 5; i++) {
    std::vector<autd3::driver::Drive> drives;
    std::random_device seed_gen;
    std::mt19937 engine(seed_gen());
    std::uniform_real_distribution dist(0.0, 1.0);
    drives.reserve(NUM_TRANS_IN_UNIT * 10);
    for (size_t j = 0; j < NUM_TRANS_IN_UNIT * 10; j++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine), 4096});
    drives_list.emplace_back(drives);
  }

  driver.gain_stm_normal_header(tx);
  driver.gain_stm_normal_duty(drives_list, 1, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, std::nullopt, tx);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Duty::to_duty(drives_list[0][i]));
  ASSERT_EQ(tx.num_bodies, 10);

  driver.gain_stm_normal_header(tx);
  driver.gain_stm_normal_duty(drives_list, 5, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, std::nullopt, tx);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Duty::to_duty(drives_list[4][i]));
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(DriverV2_6Driver, operation_force_fan_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.force_fan(tx, true);
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::FORCE_FAN));

  driver.force_fan(tx, false);
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::FORCE_FAN));
}

TEST(DriverV2_6Driver, operation_reads_fpga_info_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.reads_fpga_info(tx, true);
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::READS_FPGA_INFO));

  driver.reads_fpga_info(tx, false);
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::READS_FPGA_INFO));
}

TEST(DriverV2_6Driver, operation_cpu_version_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.cpu_version(tx);
  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_RD_CPU_VERSION);
  ASSERT_EQ(static_cast<uint8_t>(tx.header().cpu_flag.value()), autd3::driver::MSG_RD_CPU_VERSION);
}

TEST(DriverV2_6Driver, operation_fpga_version_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.fpga_version(tx);
  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_RD_FPGA_VERSION);
  ASSERT_EQ(static_cast<uint8_t>(tx.header().cpu_flag.value()), autd3::driver::MSG_RD_FPGA_VERSION);
}

TEST(DriverV2_6Driver, operation_fpga_functions_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.fpga_functions(tx);
  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_RD_FPGA_FUNCTION);
  ASSERT_EQ(static_cast<uint8_t>(tx.header().cpu_flag.value()), autd3::driver::MSG_RD_FPGA_FUNCTION);
}
