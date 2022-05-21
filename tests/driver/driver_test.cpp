// File: driver_test.cpp
// Project: driver
// Created Date: 20/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 21/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
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
#include <autd3/driver/cpu/operation.hpp>
#include <autd3/driver/hardware.hpp>
#include <random>

#include "autd3/driver/fpga/defined.hpp"

using autd3::driver::CPUControlFlags;
using autd3::driver::FPGAControlFlags;

TEST(FPGATest, FPGAControlFlagsTest) {
  FPGAControlFlags flag(FPGAControlFlags::NONE);

  ASSERT_TRUE(flag == FPGAControlFlags::NONE);

  flag.set(FPGAControlFlags::LEGACY_MODE);

  ASSERT_TRUE(flag != FPGAControlFlags::NONE);
  ASSERT_TRUE(flag == FPGAControlFlags::LEGACY_MODE);

  flag.set(FPGAControlFlags::STM_MODE);
  flag.remove(FPGAControlFlags::LEGACY_MODE);

  ASSERT_TRUE(flag != FPGAControlFlags::LEGACY_MODE);
  ASSERT_TRUE(flag == FPGAControlFlags::STM_MODE);
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

TEST(HARDTest, is_missing_transducer) {
  using autd3::driver::is_missing_transducer;

  ASSERT_TRUE(is_missing_transducer(1, 1));
  ASSERT_TRUE(is_missing_transducer(2, 1));
  ASSERT_TRUE(is_missing_transducer(16, 1));

  ASSERT_FALSE(is_missing_transducer(0, 0));
  ASSERT_FALSE(is_missing_transducer(17, 0));
  ASSERT_FALSE(is_missing_transducer(17, 13));
}

TEST(CPUTest, STMFocus) {
  ASSERT_EQ(sizeof(autd3::driver::STMFocus), 8);

  constexpr auto max = static_cast<double>((1 << 17) - 1) * autd3::driver::POINT_STM_FIXED_NUM_UNIT;
  constexpr auto min = static_cast<double>(-(1 << 17)) * autd3::driver::POINT_STM_FIXED_NUM_UNIT;

  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution dist(min, max);
  std::uniform_int_distribution dist_u8(0, 0xFF);

  const auto to = [](const uint64_t v) -> double {
    auto b = static_cast<uint32_t>(v & 0x0003fffful);
    b = (v & 0x20000) == 0 ? b : (b | 0xfffc0000u);
    const auto xi = *reinterpret_cast<int32_t*>(&b);
    return static_cast<double>(xi) * autd3::driver::POINT_STM_FIXED_NUM_UNIT;
  };

  for (auto i = 0; i < 10000; i++) {
    const auto x = dist(engine);
    const auto y = dist(engine);
    const auto z = dist(engine);
    const uint8_t shift = dist_u8(engine);

    const autd3::driver::STMFocus focus(x, y, z, shift);

    uint64_t v = 0;
    std::memcpy(&v, &focus, sizeof(autd3::driver::STMFocus));

    const auto xx = to(v);
    ASSERT_NEAR(xx, x, autd3::driver::POINT_STM_FIXED_NUM_UNIT / 2);

    v >>= 18;
    const auto yy = to(v);
    ASSERT_NEAR(yy, y, autd3::driver::POINT_STM_FIXED_NUM_UNIT / 2);

    v >>= 18;
    const auto zz = to(v);
    ASSERT_NEAR(zz, z, autd3::driver::POINT_STM_FIXED_NUM_UNIT / 2);

    v >>= 18;
    const auto s = static_cast<uint8_t>(v & 0xFF);
    ASSERT_EQ(s, shift);
  }
}

TEST(CPUTest, Body) {
  ASSERT_EQ(sizeof(autd3::driver::PointSTMBodyHead), 498);
  ASSERT_EQ(sizeof(autd3::driver::PointSTMBodyBody), 498);
  ASSERT_EQ(sizeof(autd3::driver::GainSTMBodyHead), 498);
  ASSERT_EQ(sizeof(autd3::driver::GainSTMBodyBody), 498);
  ASSERT_EQ(sizeof(autd3::driver::Body), 498);
}

TEST(CPUTest, TxDatagram) {
  autd3::driver::TxDatagram tx(10);

  ASSERT_EQ(tx.size(), 10);
  ASSERT_EQ(tx.effective_size(), 128 + 498 * 10);

  tx.num_bodies = 5;
  ASSERT_EQ(tx.effective_size(), 128 + 498 * 5);
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

  ASSERT_TRUE(flag == CPUControlFlags::NONE);

  flag.set(CPUControlFlags::MOD);

  ASSERT_TRUE(flag != CPUControlFlags::NONE);
  ASSERT_TRUE(flag == CPUControlFlags::MOD);

  flag.set(CPUControlFlags::MOD_BEGIN);
  flag.remove(CPUControlFlags::MOD);

  ASSERT_TRUE(flag != CPUControlFlags::MOD);
  ASSERT_TRUE(flag == CPUControlFlags::MOD_BEGIN);
}

TEST(CPUTest, Header) {
  ASSERT_EQ(sizeof(autd3::driver::SyncHeader), 124);
  ASSERT_EQ(sizeof(autd3::driver::ModHead), 124);
  ASSERT_EQ(sizeof(autd3::driver::ModBody), 124);
  ASSERT_EQ(sizeof(autd3::driver::SilencerHeader), 124);
  ASSERT_EQ(sizeof(autd3::driver::GlobalHeader), 128);
}

TEST(CPUTest, operation_clear) {
  autd3::driver::TxDatagram tx(10);

  clear(tx);

  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_CLEAR);
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(CPUTest, operation_null_header) {
  autd3::driver::TxDatagram tx(10);

  null_header(1, tx);

  ASSERT_EQ(tx.header().msg_id, 1);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::MOD) == 0);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::CONFIG_SILENCER) == 0);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::CONFIG_SYNC) == 0);
  ASSERT_EQ(tx.header().size, 0);
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(CPUTest, operation_null_body) {
  autd3::driver::TxDatagram tx(10);

  null_body(tx);

  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY) == 0);
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(CPUTest, operation_sync) {
  autd3::driver::TxDatagram tx(10);

  std::vector<uint16_t> cycle;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_int_distribution dist(0, 0xFFFF);
  for (int i = 0; i < 249 * 10; i++) cycle.emplace_back(dist(engine));

  sync(1, 2, cycle.data(), tx);

  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::MOD) == 0);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::CONFIG_SILENCER) == 0);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::CONFIG_SYNC) != 0);
  ASSERT_TRUE(tx.header().sync_header().ecat_sync_cycle_ticks == 2);

  for (int i = 0; i < 249 * 10; i++) ASSERT_EQ(tx.bodies()[i / 249].data[i % 249], cycle[i]);

  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(CPUTest, operation_modulation) {
  autd3::driver::TxDatagram tx(10);

  uint8_t mod_data[4] = {0x00, 0x01, 0x02, 0x03};

  modulation(1, mod_data, 4, true, 2320, false, tx);

  ASSERT_EQ(tx.header().msg_id, 1);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::MOD) != 0);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::MOD_BEGIN) != 0);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::MOD_END) == 0);
  ASSERT_EQ(tx.header().size, 4);
  ASSERT_TRUE(tx.header().mod_head().freq_div == 2320);
  ASSERT_TRUE(tx.header().mod_head().data[0] == 0x00);
  ASSERT_TRUE(tx.header().mod_head().data[1] == 0x01);
  ASSERT_TRUE(tx.header().mod_head().data[2] == 0x02);
  ASSERT_TRUE(tx.header().mod_head().data[3] == 0x03);

  mod_data[0] = 0x04;
  mod_data[1] = 0x05;
  mod_data[2] = 0x06;
  mod_data[3] = 0x07;

  modulation(0xFF, mod_data, 4, false, 5, true, tx);

  ASSERT_EQ(tx.header().msg_id, 0xFF);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::MOD) != 0);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::MOD_BEGIN) == 0);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::MOD_END) != 0);
  ASSERT_EQ(tx.header().size, 4);
  ASSERT_TRUE(tx.header().mod_body().data[0] == 0x04);
  ASSERT_TRUE(tx.header().mod_body().data[1] == 0x05);
  ASSERT_TRUE(tx.header().mod_body().data[2] == 0x06);
  ASSERT_TRUE(tx.header().mod_body().data[3] == 0x07);

  modulation(0xF0, nullptr, 0, false, 5, true, tx);

  ASSERT_EQ(tx.header().msg_id, 0xF0);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::MOD) == 0);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::MOD_BEGIN) == 0);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::MOD_END) == 0);
  ASSERT_EQ(tx.header().size, 0);

  ASSERT_THROW(modulation(1, mod_data, 2319, true, 5, false, tx), std::runtime_error);
}

TEST(CPUTest, operation_config_silencer) {
  autd3::driver::TxDatagram tx(10);

  config_silencer(1, 2088, 4, tx);

  ASSERT_EQ(tx.header().msg_id, 1);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::MOD) == 0);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::CONFIG_SYNC) == 0);
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::CONFIG_SILENCER) != 0);
  ASSERT_EQ(tx.header().silencer_header().cycle, 2088);
  ASSERT_EQ(tx.header().silencer_header().step, 4);

  ASSERT_THROW(config_silencer(1, 2087, 4, tx), std::runtime_error);
}

TEST(CPUTest, operation_normal_legacy_header) {}
TEST(CPUTest, operation_normal_legacy_body) {}
TEST(CPUTest, operation_normal_header) {}
TEST(CPUTest, operation_normal_duty_body) {}
TEST(CPUTest, operation_normal_phase_body) {}
TEST(CPUTest, operation_point_stm_header) {}
TEST(CPUTest, operation_point_stm_body) {}
TEST(CPUTest, operation_gain_stm_legacy_header) {}
TEST(CPUTest, operation_gain_stm_legacy_body) {}
TEST(CPUTest, operation_gain_stm_normal_header) {}
TEST(CPUTest, operation_gain_stm_normal_phase) {}
TEST(CPUTest, operation_gain_stm_normal_duty) {}
TEST(CPUTest, operation_force_fan) {}
TEST(CPUTest, operation_reads_fpga_info) {}
TEST(CPUTest, operation_cpu_version) {}
TEST(CPUTest, operation_fpga_version) {}
TEST(CPUTest, operation_fpga_functions) {}