// File: driver_test.cpp
// Project: driver
// Created Date: 20/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/11/2022
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
#include <autd3/driver/firmware_version.hpp>
#include <random>

#include "autd3/driver/common/fpga/defined.hpp"
#include "autd3/driver/utils.hpp"
#include "autd3/driver/v2_6/driver.hpp"

using autd3::driver::CPUControlFlags;
using autd3::driver::FPGAControlFlags;

constexpr size_t NUM_TRANS_IN_UNIT = 249;

TEST(FPGATest, FPGAControlFlagsTest) {
  FPGAControlFlags flag(FPGAControlFlags::NONE);
  ASSERT_EQ(flag, FPGAControlFlags::NONE);

  flag.set(FPGAControlFlags::LEGACY_MODE);
  ASSERT_TRUE(flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::FORCE_FAN));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::READS_FPGA_INFO));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_GAIN_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_MODE));

  flag.set(FPGAControlFlags::STM_MODE);
  ASSERT_TRUE(flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::FORCE_FAN));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::READS_FPGA_INFO));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_GAIN_MODE));

  flag.remove(FPGAControlFlags::LEGACY_MODE);
  ASSERT_FALSE(flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::FORCE_FAN));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::READS_FPGA_INFO));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_GAIN_MODE));
}

TEST(FPGATest, FPGAInfo) {
  using autd3::driver::FPGAInfo;

  FPGAInfo info(0);
  ASSERT_FALSE(info.is_thermal_assert());

  info = FPGAInfo(1);
  ASSERT_TRUE(info.is_thermal_assert());

  info = FPGAInfo(2);
  ASSERT_FALSE(info.is_thermal_assert());
}

TEST(VersionTest, FirmwareInfo) {
  {
    const autd3::driver::FirmwareInfo info(0, 0, 0, 0);
    EXPECT_EQ("older than v0.4", info.cpu_version());
    EXPECT_EQ("older than v0.4", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 1, 1, 0);
    EXPECT_EQ("v0.4", info.cpu_version());
    EXPECT_EQ("v0.4", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 2, 2, 0);
    EXPECT_EQ("v0.5", info.cpu_version());
    EXPECT_EQ("v0.5", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 3, 3, 0);
    EXPECT_EQ("v0.6", info.cpu_version());
    EXPECT_EQ("v0.6", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 4, 4, 0);
    EXPECT_EQ("v0.7", info.cpu_version());
    EXPECT_EQ("v0.7", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 5, 5, 0);
    EXPECT_EQ("v0.8", info.cpu_version());
    EXPECT_EQ("v0.8", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 6, 6, 0);
    EXPECT_EQ("v0.9", info.cpu_version());
    EXPECT_EQ("v0.9", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 7, 7, 0);
    EXPECT_EQ("unknown (7)", info.cpu_version());
    EXPECT_EQ("unknown (7)", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 8, 8, 0);
    EXPECT_EQ("unknown (8)", info.cpu_version());
    EXPECT_EQ("unknown (8)", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 9, 9, 0);
    EXPECT_EQ("unknown (9)", info.cpu_version());
    EXPECT_EQ("unknown (9)", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 10, 10, 0);
    EXPECT_EQ("v1.0", info.cpu_version());
    EXPECT_EQ("v1.0", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 11, 11, 0);
    EXPECT_EQ("v1.1", info.cpu_version());
    EXPECT_EQ("v1.1", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 12, 12, 0);
    EXPECT_EQ("v1.2", info.cpu_version());
    EXPECT_EQ("v1.2", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 13, 13, 0);
    EXPECT_EQ("v1.3", info.cpu_version());
    EXPECT_EQ("v1.3", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 14, 14, 0);
    EXPECT_EQ("v1.4", info.cpu_version());
    EXPECT_EQ("v1.4", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 15, 15, 0);
    EXPECT_EQ("v1.5", info.cpu_version());
    EXPECT_EQ("v1.5", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 16, 16, 0);
    EXPECT_EQ("v1.6", info.cpu_version());
    EXPECT_EQ("v1.6", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 17, 17, 0);
    EXPECT_EQ("v1.7", info.cpu_version());
    EXPECT_EQ("v1.7", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 18, 18, 0);
    EXPECT_EQ("v1.8", info.cpu_version());
    EXPECT_EQ("v1.8", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 19, 19, 0);
    EXPECT_EQ("v1.9", info.cpu_version());
    EXPECT_EQ("v1.9", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 20, 20, 0);
    EXPECT_EQ("v1.10", info.cpu_version());
    EXPECT_EQ("v1.10", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 21, 21, 0);
    EXPECT_EQ("v1.11", info.cpu_version());
    EXPECT_EQ("v1.11", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 128, 128, 0);
    EXPECT_EQ("v2.0", info.cpu_version());
    EXPECT_EQ("v2.0", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 129, 129, 0);
    EXPECT_EQ("v2.1", info.cpu_version());
    EXPECT_EQ("v2.1", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 130, 130, 0);
    EXPECT_EQ("v2.2", info.cpu_version());
    EXPECT_EQ("v2.2", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 131, 131, 0);
    EXPECT_EQ("v2.3", info.cpu_version());
    EXPECT_EQ("v2.3", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 132, 132, 0);
    EXPECT_EQ("v2.4", info.cpu_version());
    EXPECT_EQ("v2.4", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 133, 133, 0);
    EXPECT_EQ("v2.5", info.cpu_version());
    EXPECT_EQ("v2.5", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 134, 134, 0);
    EXPECT_EQ("v2.6", info.cpu_version());
    EXPECT_EQ("v2.6", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 135, 135, 0);
    EXPECT_EQ("unknown (135)", info.cpu_version());
    EXPECT_EQ("unknown (135)", info.fpga_version());
  }
}

TEST(CPUTest, STMFocus) {
  ASSERT_EQ(sizeof(autd3::driver::STMFocus), 8);

  constexpr auto max = static_cast<double>((1 << 17) - 1) * autd3::driver::FOCUS_STM_FIXED_NUM_UNIT;
  constexpr auto min = static_cast<double>(-(1 << 17)) * autd3::driver::FOCUS_STM_FIXED_NUM_UNIT;

  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution dist(min, max);
  std::uniform_int_distribution dist_u8(0, 0xFF);

  const auto to = [](const uint64_t v) -> double {
    auto b = static_cast<uint32_t>(v & 0x0003fffful);
    b = (v & 0x20000) == 0 ? b : b | 0xfffc0000u;
    const auto xi = *reinterpret_cast<int32_t*>(&b);
    return static_cast<double>(xi) * autd3::driver::FOCUS_STM_FIXED_NUM_UNIT;
  };

  for (auto i = 0; i < 10000; i++) {
    const auto x = dist(engine);
    const auto y = dist(engine);
    const auto z = dist(engine);
    const auto shift = static_cast<uint8_t>(dist_u8(engine));

    const autd3::driver::STMFocus focus(x, y, z, shift);

    uint64_t v = 0;
    std::memcpy(&v, &focus, sizeof(autd3::driver::STMFocus));

    const auto xx = to(v);
    ASSERT_NEAR(xx, x, autd3::driver::FOCUS_STM_FIXED_NUM_UNIT / 2);

    v >>= 18;
    const auto yy = to(v);
    ASSERT_NEAR(yy, y, autd3::driver::FOCUS_STM_FIXED_NUM_UNIT / 2);

    v >>= 18;
    const auto zz = to(v);
    ASSERT_NEAR(zz, z, autd3::driver::FOCUS_STM_FIXED_NUM_UNIT / 2);

    v >>= 18;
    const auto s = static_cast<uint8_t>(v & 0xFF);
    ASSERT_EQ(s, shift);
  }
}

TEST(CPUTest, TxDatagram) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  ASSERT_EQ(tx.num_devices(), 10);
  ASSERT_EQ(tx.transmitting_size(), 128 + 498 * 10);

  tx.num_bodies = 5;
  ASSERT_EQ(tx.transmitting_size(), 128 + 498 * 5);
}

TEST(CPUTest, RxDatagram) {
  autd3::driver::RxDatagram rx(10);

  ASSERT_FALSE(rx.is_msg_processed(1));

  rx[0].msg_id = 1;
  ASSERT_FALSE(rx.is_msg_processed(1));

  for (auto& msg : rx) msg.msg_id = 1;
  ASSERT_TRUE(rx.is_msg_processed(1));
  ASSERT_FALSE(rx.is_msg_processed(2));
}

TEST(CPUTest, CPUControlFlags) {
  CPUControlFlags flag(CPUControlFlags::NONE);

  ASSERT_EQ(flag, CPUControlFlags::NONE);

  flag.set(CPUControlFlags::MOD);

  ASSERT_TRUE(flag != CPUControlFlags::NONE);
  ASSERT_EQ(flag, CPUControlFlags::MOD);

  flag.set(CPUControlFlags::MOD_BEGIN);
  flag.remove(CPUControlFlags::MOD);

  ASSERT_TRUE(flag != CPUControlFlags::MOD);
  ASSERT_EQ(flag, CPUControlFlags::MOD_BEGIN);
}

TEST(CPUTest, Header) {
  ASSERT_EQ(sizeof(autd3::driver::ModHeaderInitial), 124);
  ASSERT_EQ(sizeof(autd3::driver::ModHeaderSubsequent), 124);
  ASSERT_EQ(sizeof(autd3::driver::SilencerHeader), 124);
  ASSERT_EQ(sizeof(autd3::driver::GlobalHeader), 128);
}

TEST(UtilitiesTest, rem_euclid) {
  ASSERT_EQ(autd3::driver::rem_euclid(0, 256), 0);
  ASSERT_EQ(autd3::driver::rem_euclid(10, 256), 10);
  ASSERT_EQ(autd3::driver::rem_euclid(255, 256), 255);
  ASSERT_EQ(autd3::driver::rem_euclid(256, 256), 0);
  ASSERT_EQ(autd3::driver::rem_euclid(266, 256), 10);
  ASSERT_EQ(autd3::driver::rem_euclid(-10, 256), 246);
  ASSERT_EQ(autd3::driver::rem_euclid(-266, 256), 246);
}

TEST(CPUTest, operation_clear_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.clear(tx);

  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_CLEAR);
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(CPUTest, operation_null_header_v2_6) {
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

TEST(CPUTest, operation_null_body_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.null_body(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(CPUTest, operation_sync_v2_6) {
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

  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(reinterpret_cast<uint16_t*>(tx.bodies_ptr())[i], cycle[i]);

  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(CPUTest, operation_modulation_v2_6) {
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

TEST(CPUTest, operation_config_silencer_v2_6) {
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

TEST(CPUTest, normal_legacy_header_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.normal_legacy_header(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));

  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(CPUTest, operation_normal_legacy_body_v2_6) {
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
    ASSERT_EQ(reinterpret_cast<uint16_t*>(tx.bodies_ptr())[i] & 0xFF, autd3::driver::LegacyDrive::to_phase(drives[i]));
    ASSERT_EQ(reinterpret_cast<uint16_t*>(tx.bodies_ptr())[i] >> 8, autd3::driver::LegacyDrive::to_duty(drives[i]));
  }

  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(CPUTest, operation_normal_header_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.normal_header(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));

  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(CPUTest, operation_normal_duty_body_v2_6) {
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

  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++)
    ASSERT_EQ(reinterpret_cast<uint16_t*>(tx.bodies_ptr())[i], autd3::driver::Duty::to_duty(drives[i]));

  ASSERT_EQ(tx.num_bodies, 10);
}
TEST(CPUTest, operation_normal_phase_body_v2_6) {
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

  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++)
    ASSERT_EQ(reinterpret_cast<uint16_t*>(tx.bodies_ptr())[i], autd3::driver::Phase::to_phase(drives[i]));

  ASSERT_EQ(tx.num_bodies, 10);
}
TEST(CPUTest, operation_focus_stm_header_v2_6) {
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

TEST(CPUTest, operation_focus_stm_subsequent_v2_6) {
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
  driver.focus_stm_body(points, sent, size, 3224, sound_speed, tx);

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
  driver.focus_stm_body(points, sent, 500, 3224, sound_speed, tx);

  ASSERT_EQ(sent, size);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));

  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().data()[0], 30);
  ASSERT_EQ(tx.num_bodies, 10);

  driver.focus_stm_header(tx);
  sent = 1;
  driver.focus_stm_body(points, sent, 500, 3224, sound_speed, tx);
  ASSERT_EQ(sent, size + 1);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));

  driver.focus_stm_header(tx);
  driver.focus_stm_body({}, sent, 0, 3224, sound_speed, tx);
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STM_GAIN_MODE));
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(CPUTest, operation_gain_stm_legacy_header_v2_6) {
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

TEST(CPUTest, operation_gain_stm_legacy_body_v2_6) {
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
  driver.gain_stm_legacy_body(drives_list, sent, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, tx);
  ASSERT_EQ(sent, 1);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[0], 3224);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[1], 0);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[3], 5);
  ASSERT_EQ(tx.num_bodies, 10);

  driver.gain_stm_legacy_header(tx);
  driver.gain_stm_legacy_body(drives_list, sent, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, tx);
  ASSERT_EQ(sent, 2);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) {
    ASSERT_EQ(reinterpret_cast<uint16_t*>(tx.bodies_ptr())[i] & 0xFF, autd3::driver::LegacyDrive::to_phase(drives_list[0][i]));
    ASSERT_EQ(reinterpret_cast<uint16_t*>(tx.bodies_ptr())[i] >> 8, autd3::driver::LegacyDrive::to_duty(drives_list[0][i]));
  }
  ASSERT_EQ(tx.num_bodies, 10);

  driver.gain_stm_legacy_header(tx);
  sent = 5;
  driver.gain_stm_legacy_body(drives_list, sent, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, tx);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) {
    ASSERT_EQ(reinterpret_cast<uint16_t*>(tx.bodies_ptr())[i] & 0xFF, autd3::driver::LegacyDrive::to_phase(drives_list[4][i]));
    ASSERT_EQ(reinterpret_cast<uint16_t*>(tx.bodies_ptr())[i] >> 8, autd3::driver::LegacyDrive::to_duty(drives_list[4][i]));
  }
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(CPUTest, operation_gain_stm_normal_header_v2_6) {
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

TEST(CPUTest, operation_gain_stm_normal_phase_v2_6) {
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
  driver.gain_stm_normal_phase(drives_list, 0, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, tx);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[0], 3224);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().data()[1], 0);
  ASSERT_EQ(tx.num_bodies, 10);

  driver.gain_stm_normal_header(tx);
  driver.gain_stm_normal_phase(drives_list, 1, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, tx);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++)
    ASSERT_EQ(reinterpret_cast<uint16_t*>(tx.bodies_ptr())[i], autd3::driver::Phase::to_phase(drives_list[0][i]));
  ASSERT_EQ(tx.num_bodies, 10);

  driver.gain_stm_normal_header(tx);
  driver.gain_stm_normal_phase(drives_list, 5, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, tx);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++)
    ASSERT_EQ(reinterpret_cast<uint16_t*>(tx.bodies_ptr())[i], autd3::driver::Phase::to_phase(drives_list[4][i]));
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(CPUTest, operation_gain_stm_normal_duty_v2_6) {
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
  driver.gain_stm_normal_duty(drives_list, 1, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, tx);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++)
    ASSERT_EQ(reinterpret_cast<uint16_t*>(tx.bodies_ptr())[i], autd3::driver::Duty::to_duty(drives_list[0][i]));
  ASSERT_EQ(tx.num_bodies, 10);

  driver.gain_stm_normal_header(tx);
  driver.gain_stm_normal_duty(drives_list, 5, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, tx);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++)
    ASSERT_EQ(reinterpret_cast<uint16_t*>(tx.bodies_ptr())[i], autd3::driver::Duty::to_duty(drives_list[4][i]));
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(CPUTest, operation_force_fan_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.force_fan(tx, true);
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::FORCE_FAN));

  driver.force_fan(tx, false);
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::FORCE_FAN));
}

TEST(CPUTest, operation_reads_fpga_info_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.reads_fpga_info(tx, true);
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::READS_FPGA_INFO));

  driver.reads_fpga_info(tx, false);
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::READS_FPGA_INFO));
}

TEST(CPUTest, operation_cpu_version_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.cpu_version(tx);
  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_RD_CPU_VERSION);
  ASSERT_EQ(static_cast<uint8_t>(tx.header().cpu_flag.value()), autd3::driver::MSG_RD_CPU_VERSION);
}

TEST(CPUTest, operation_fpga_version_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.fpga_version(tx);
  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_RD_FPGA_VERSION);
  ASSERT_EQ(static_cast<uint8_t>(tx.header().cpu_flag.value()), autd3::driver::MSG_RD_FPGA_VERSION);
}

TEST(CPUTest, operation_fpga_functions_v2_6) {
  const auto driver = autd3::driver::DriverV2_6();

  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  driver.fpga_functions(tx);
  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_RD_FPGA_FUNCTION);
  ASSERT_EQ(static_cast<uint8_t>(tx.header().cpu_flag.value()), autd3::driver::MSG_RD_FPGA_FUNCTION);
}
