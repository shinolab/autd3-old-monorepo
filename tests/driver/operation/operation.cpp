// File: operation.cpp
// Project: operation
// Created Date: 07/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 18/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
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

#include "autd3/driver/fpga/defined.hpp"
#include "autd3/driver/operation/clear.hpp"
#include "autd3/driver/operation/focus_stm.hpp"
#include "autd3/driver/operation/force_fan.hpp"
#include "autd3/driver/operation/gain.hpp"
#include "autd3/driver/operation/gain_stm.hpp"
#include "autd3/driver/operation/info.hpp"
#include "autd3/driver/operation/modulation.hpp"
#include "autd3/driver/operation/null.hpp"
#include "autd3/driver/operation/reads_fpga_info.hpp"
#include "autd3/driver/operation/silencer.hpp"
#include "autd3/driver/operation/sync.hpp"

using autd3::driver::CPUControlFlags;
using autd3::driver::FPGAControlFlags;

constexpr size_t NUM_TRANS_IN_UNIT = 249;

TEST(Driver_Driver, clear) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::Clear().pack(tx);

  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_CLEAR);
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(Driver_Driver, null_header) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::NullHeader().pack(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::Mod));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSilencer));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSync));
  ASSERT_EQ(tx.header().size, 0);
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(Driver_Driver, null_body) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::NullBody().pack(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(Driver_Driver, sync_legacy) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<uint16_t> cycles;
  cycles.resize(NUM_TRANS_IN_UNIT * 10, 4096);
  autd3::driver::Sync<autd3::driver::Legacy>().pack(tx);
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::Mod));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSilencer));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSync));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], cycles[i]);
  ASSERT_EQ(tx.num_bodies, 10);

  autd3::driver::Sync<autd3::driver::Legacy>().pack(tx);
}

TEST(Driver_Driver, sync_normal) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<uint16_t> cycles;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_int_distribution dist(0, 0xFFFF);
  cycles.reserve(NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) cycles.emplace_back(dist(engine));

  autd3::driver::Sync<autd3::driver::Normal> op(cycles);

  op.init();
  op.pack(tx);

  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::Mod));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSilencer));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSync));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], cycles[i]);
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(Driver_Driver, modulation) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<uint8_t> mod_data;
  for (size_t i = 0; i < autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE + autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE + 1; i++)
    mod_data.emplace_back(static_cast<uint8_t>(i));

  {
    autd3::driver::Modulation op(mod_data, 1160);

    op.pack(tx);
    ASSERT_FALSE(op.is_finished());
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::Mod));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::ModBegin));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ModEnd));
    ASSERT_EQ(tx.header().size, static_cast<uint16_t>(autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE));
    ASSERT_EQ(tx.header().mod_initial().freq_div, 1160);
    for (size_t i = 0; i < autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE; i++) ASSERT_EQ(tx.header().mod_initial().data[i], static_cast<uint8_t>(i));

    op.pack(tx);
    ASSERT_FALSE(op.is_finished());
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::Mod));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ModBegin));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ModEnd));
    ASSERT_EQ(tx.header().size, static_cast<uint16_t>(autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE));
    for (size_t i = autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE;
         i < autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE + autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE; i++)
      ASSERT_EQ(tx.header().mod_subsequent().data[i - autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE], static_cast<uint8_t>(i));

    op.pack(tx);
    ASSERT_TRUE(op.is_finished());
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::Mod));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ModBegin));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::ModEnd));
    ASSERT_EQ(tx.header().size, 1);
    for (size_t i = autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE + autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE;
         i < autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE + autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE + 1; i++)
      ASSERT_EQ(tx.header().mod_subsequent().data[i - (autd3::driver::MOD_HEADER_INITIAL_DATA_SIZE + autd3::driver::MOD_HEADER_SUBSEQUENT_DATA_SIZE)],
                static_cast<uint8_t>(i));
  }

  {
    autd3::driver::Modulation op(mod_data, 1159);
    ASSERT_THROW(op.pack(tx), std::runtime_error);
  }
}

TEST(Driver_Driver, config_silencer) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  {
    autd3::driver::ConfigSilencer op(1044, 4);

    op.init();
    op.pack(tx);
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::Mod));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSync));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::ConfigSilencer));
    ASSERT_EQ(tx.header().silencer().cycle, 1044);
    ASSERT_EQ(tx.header().silencer().step, 4);
  }

  {
    autd3::driver::ConfigSilencer op(1043, 4);
    op.init();
    ASSERT_THROW(op.pack(tx), std::runtime_error);
  }
}

TEST(Driver_Driver, normal_legacy_gain) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<autd3::driver::Drive> drives;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution<autd3::driver::autd3_float_t> dist(0, 1);
  drives.reserve(NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine)});

  autd3::driver::Gain<autd3::driver::Legacy> op(drives);
  op.pack(tx);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) {
    ASSERT_EQ(tx.bodies_raw_ptr()[i] & 0xFF, autd3::driver::LegacyDrive::to_phase(drives[i]));
    ASSERT_EQ(tx.bodies_raw_ptr()[i] >> 8, autd3::driver::LegacyDrive::to_duty(drives[i]));
  }
  ASSERT_EQ(tx.num_bodies, 10);

  op.pack(tx);
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(Driver_Driver, normal_gain) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<autd3::driver::Drive> drives;
  std::vector<uint16_t> cycles;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution<autd3::driver::autd3_float_t> dist(0, 1);
  drives.reserve(NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine)});
  cycles.resize(NUM_TRANS_IN_UNIT * 10, 4096);

  autd3::driver::Gain<autd3::driver::Normal> op(drives, cycles);
  op.init();
  op.pack(tx);

  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Phase::to_phase(drives[i], 4096));
  ASSERT_EQ(tx.num_bodies, 10);

  op.pack(tx);
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Duty::to_duty(drives[i], 4096));
  ASSERT_EQ(tx.num_bodies, 10);

  op.pack(tx);
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(Driver_Driver, normal_phase_gain) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<autd3::driver::Drive> drives;
  std::vector<uint16_t> cycles;

  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution<autd3::driver::autd3_float_t> dist(0, 1);
  drives.reserve(NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine)});
  cycles.resize(NUM_TRANS_IN_UNIT * 10, 4096);

  autd3::driver::Gain<autd3::driver::NormalPhase> op(drives, cycles);

  op.init();
  op.pack(tx);
  ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
  ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));

  for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Phase::to_phase(drives[i], 4096));

  ASSERT_EQ(tx.num_bodies, 10);

  op.pack(tx);
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(Driver_Driver, focus_stm) {
  std::vector device_map = {NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                            NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT};

  autd3::driver::TxDatagram tx(device_map);

  constexpr size_t size = 150;
  std::vector<autd3::driver::STMFocus> p;
  {
    std::random_device seed_gen;
    std::mt19937 engine(seed_gen());
    std::uniform_real_distribution<autd3::driver::autd3_float_t> dist(-1000, 1000);
    std::uniform_int_distribution dist_u8(0, 0xFF);
    p.reserve(size);
    for (size_t i = 0; i < size; i++) p.emplace_back(dist(engine), dist(engine), dist(engine), static_cast<uint8_t>(dist_u8(engine)));
  }

  constexpr double sound_speed = 340e3;
  constexpr uint32_t sp = 340 * 1024;

  autd3::driver::FocusSTMProps props;
  props.start_idx = 1;
  props.finish_idx = 1;
  props.freq_div = 3224;

  {
    std::vector<std::vector<autd3::driver::STMFocus>> points;
    points.reserve(10);
    for (int i = 0; i < 10; i++) points.emplace_back(p);

    autd3::driver::FocusSTM op(points, *std::min_element(device_map.begin(), device_map.end()), sound_speed, props);

    op.init();
    op.pack(tx);
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().size, 60);
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().freq_div, 3224);
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().sound_speed, sp);
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().stm_start_idx, 1);
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_initial().stm_finish_idx, 1);
    ASSERT_EQ(tx.num_bodies, 10);

    op.pack(tx);
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_subsequent().size, 62);
    ASSERT_EQ(tx.num_bodies, 10);

    op.pack(tx);
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).focus_stm_subsequent().size, 28);
    ASSERT_EQ(tx.num_bodies, 10);

    op.pack(tx);
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    ASSERT_EQ(tx.num_bodies, 0);
  }

  {
    std::vector<std::vector<autd3::driver::STMFocus>> points;
    points.reserve(10);
    for (int i = 0; i < 10; i++) points.emplace_back(p);

    props.start_idx = static_cast<uint16_t>(size);
    props.finish_idx = 0;

    autd3::driver::FocusSTM op(points, *std::min_element(device_map.begin(), device_map.end()), sound_speed, props);

    op.init();
    ASSERT_THROW(op.pack(tx), std::runtime_error);
  }

  {
    std::vector<std::vector<autd3::driver::STMFocus>> points;
    points.reserve(10);
    for (int i = 0; i < 10; i++) points.emplace_back(p);

    props.start_idx = 0;
    props.finish_idx = static_cast<uint16_t>(size);

    autd3::driver::FocusSTM op(points, *std::min_element(device_map.begin(), device_map.end()), sound_speed, props);

    op.init();
    ASSERT_THROW(op.pack(tx), std::runtime_error);
  }
}

TEST(Driver_Driver, gain_stm_legacy) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<std::vector<autd3::driver::Drive>> drives;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution<autd3::driver::autd3_float_t> dist(0, 1);
  for (size_t i = 0; i < 2; i++) {
    std::vector<autd3::driver::Drive> d;
    d.reserve(NUM_TRANS_IN_UNIT * 10);
    for (size_t j = 0; j < NUM_TRANS_IN_UNIT * 10; j++) d.emplace_back(autd3::driver::Drive{dist(engine), dist(engine)});
    drives.emplace_back(d);
  }

  autd3::driver::GainSTMProps props;
  props.freq_div = 3224;
  props.start_idx = 1;
  props.finish_idx = 1;

  {
    autd3::driver::GainSTM<autd3::driver::Legacy> op(drives, props);
    op.init();
    op.pack(tx);
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().freq_div, 3224);
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().mode, autd3::driver::GainSTMMode::PhaseDutyFull);
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().stm_start_idx, 1);
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().stm_finish_idx, 1);
    ASSERT_EQ(tx.num_bodies, 10);

    op.pack(tx);
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) {
      ASSERT_EQ(tx.bodies_raw_ptr()[i] & 0xFF, autd3::driver::LegacyDrive::to_phase(drives[0][i]));
      ASSERT_EQ(tx.bodies_raw_ptr()[i] >> 8, autd3::driver::LegacyDrive::to_duty(drives[0][i]));
    }
    ASSERT_EQ(tx.num_bodies, 10);

    op.pack(tx);
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) {
      ASSERT_EQ(tx.bodies_raw_ptr()[i] & 0xFF, autd3::driver::LegacyDrive::to_phase(drives[1][i]));
      ASSERT_EQ(tx.bodies_raw_ptr()[i] >> 8, autd3::driver::LegacyDrive::to_duty(drives[1][i]));
    }
    ASSERT_EQ(tx.num_bodies, 10);

    op.pack(tx);
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    ASSERT_EQ(tx.num_bodies, 0);
  }

  {
    props.start_idx = static_cast<uint16_t>(2);
    props.finish_idx = 0;
    autd3::driver::GainSTM<autd3::driver::Legacy> op(drives, props);
    op.init();
    ASSERT_THROW(op.pack(tx), std::runtime_error);
  }

  {
    props.start_idx = 0;
    props.finish_idx = static_cast<uint16_t>(2);
    autd3::driver::GainSTM<autd3::driver::Legacy> op(drives, props);
    op.init();
    ASSERT_THROW(op.pack(tx), std::runtime_error);
  }
}

TEST(Driver_Driver, gain_stm_normal) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<std::vector<autd3::driver::Drive>> drives;
  {
    std::random_device seed_gen;
    std::mt19937 engine(seed_gen());
    std::uniform_real_distribution<autd3::driver::autd3_float_t> dist(0, 1);
    for (size_t i = 0; i < 2; i++) {
      std::vector<autd3::driver::Drive> d;
      d.reserve(NUM_TRANS_IN_UNIT * 10);
      for (size_t j = 0; j < NUM_TRANS_IN_UNIT * 10; j++) d.emplace_back(autd3::driver::Drive{dist(engine), dist(engine)});
      drives.emplace_back(d);
    }
  }
  std::vector<uint16_t> cycles;
  {
    std::random_device seed_gen;
    std::mt19937 engine(seed_gen());
    std::uniform_int_distribution<uint16_t> dist(2, 0xFFFF);
    for (size_t i = 0; i < 10 * NUM_TRANS_IN_UNIT; i++) {
      cycles.emplace_back(dist(engine));
    }
  }

  autd3::driver::GainSTMProps props;
  props.freq_div = 3224;
  props.start_idx = 1;
  props.finish_idx = 1;
  {
    autd3::driver::GainSTM<autd3::driver::Normal> op(drives, cycles, props);
    op.init();

    op.pack(tx);
    ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().freq_div, 3224);
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().mode, autd3::driver::GainSTMMode::PhaseDutyFull);
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().stm_finish_idx, 1);
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().stm_start_idx, 1);
    ASSERT_EQ(tx.num_bodies, 10);

    op.pack(tx);
    ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Phase::to_phase(drives[0][i], cycles[i]));
    ASSERT_EQ(tx.num_bodies, 10);

    op.pack(tx);
    ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Duty::to_duty(drives[0][i], cycles[i]));
    ASSERT_EQ(tx.num_bodies, 10);

    op.pack(tx);
    ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Phase::to_phase(drives[1][i], cycles[i]));
    ASSERT_EQ(tx.num_bodies, 10);

    op.pack(tx);
    ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Duty::to_duty(drives[1][i], cycles[i]));
    ASSERT_EQ(tx.num_bodies, 10);

    op.pack(tx);
    ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    ASSERT_EQ(tx.num_bodies, 0);
  }

  {
    props.start_idx = static_cast<uint16_t>(2);
    props.finish_idx = 0;
    autd3::driver::GainSTM<autd3::driver::Normal> op(drives, cycles, props);
    op.init();
    ASSERT_THROW(op.pack(tx), std::runtime_error);
  }

  {
    props.start_idx = 0;
    props.finish_idx = static_cast<uint16_t>(2);
    autd3::driver::GainSTM<autd3::driver::Normal> op(drives, cycles, props);
    op.init();
    ASSERT_THROW(op.pack(tx), std::runtime_error);
  }
}

TEST(Driver_Driver, gain_stm_normal_phase) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  std::vector<std::vector<autd3::driver::Drive>> drives;
  {
    std::random_device seed_gen;
    std::mt19937 engine(seed_gen());
    std::uniform_real_distribution<autd3::driver::autd3_float_t> dist(0, 1);
    for (size_t i = 0; i < 2; i++) {
      std::vector<autd3::driver::Drive> d;
      d.reserve(NUM_TRANS_IN_UNIT * 10);
      for (size_t j = 0; j < NUM_TRANS_IN_UNIT * 10; j++) d.emplace_back(autd3::driver::Drive{dist(engine), dist(engine)});
      drives.emplace_back(d);
    }
  }
  std::vector<uint16_t> cycles;
  {
    std::random_device seed_gen;
    std::mt19937 engine(seed_gen());
    std::uniform_int_distribution<uint16_t> dist(2, 0xFFFF);
    for (size_t i = 0; i < 10 * NUM_TRANS_IN_UNIT; i++) {
      cycles.emplace_back(dist(engine));
    }
  }

  autd3::driver::GainSTMProps props;
  props.freq_div = 3224;
  props.start_idx = 1;
  props.finish_idx = 1;
  {
    autd3::driver::GainSTM<autd3::driver::NormalPhase> op(drives, cycles, props);
    op.init();
    op.pack(tx);
    ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().freq_div, 3224);
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().mode, autd3::driver::GainSTMMode::PhaseFull);
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().stm_start_idx, 1);
    for (int i = 0; i < 10; i++) ASSERT_EQ(tx.body(i).gain_stm_initial().stm_finish_idx, 1);
    ASSERT_EQ(tx.num_bodies, 10);

    op.pack(tx);
    ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Phase::to_phase(drives[0][i], cycles[i]));
    ASSERT_EQ(tx.num_bodies, 10);

    op.pack(tx);
    ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::IsDuty));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_TRUE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    for (size_t i = 0; i < NUM_TRANS_IN_UNIT * 10; i++) ASSERT_EQ(tx.bodies_raw_ptr()[i], autd3::driver::Phase::to_phase(drives[1][i], cycles[i]));
    ASSERT_EQ(tx.num_bodies, 10);

    op.pack(tx);
    ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::LegacyMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMMode));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::STMGainMode));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::WriteBody));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMBegin));
    ASSERT_FALSE(tx.header().cpu_flag.contains(CPUControlFlags::STMEnd));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMStartIdx));
    ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::UseSTMFinishIdx));
    ASSERT_EQ(tx.num_bodies, 0);
  }

  {
    props.start_idx = static_cast<uint16_t>(2);
    props.finish_idx = 0;
    autd3::driver::GainSTM<autd3::driver::NormalPhase> op(drives, cycles, props);
    op.init();
    ASSERT_THROW(op.pack(tx), std::runtime_error);
  }

  {
    props.start_idx = 0;
    props.finish_idx = static_cast<uint16_t>(2);
    autd3::driver::GainSTM<autd3::driver::NormalPhase> op(drives, cycles, props);
    op.init();
    ASSERT_THROW(op.pack(tx), std::runtime_error);
  }
}

TEST(Driver_Driver, force_fan) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::ForceFan op;

  op.value = true;
  op.pack(tx);
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::ForceFan));

  op.value = false;
  op.pack(tx);
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::ForceFan));
}

TEST(Driver_Driver, reads_fpga_info) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::ReadsFPGAInfo op;

  op.value = true;
  op.pack(tx);
  ASSERT_TRUE(tx.header().fpga_flag.contains(FPGAControlFlags::ReadsFPGAInfo));

  op.value = false;
  op.pack(tx);
  ASSERT_FALSE(tx.header().fpga_flag.contains(FPGAControlFlags::ReadsFPGAInfo));
}

TEST(Driver_Driver, cpu_version) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::CPUVersion::pack(tx);
  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_RD_CPU_VERSION);
  ASSERT_EQ(static_cast<uint8_t>(tx.header().cpu_flag.value()), autd3::driver::MSG_RD_CPU_VERSION);
}

TEST(Driver_Driver, fpga_version) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::FPGAVersion::pack(tx);
  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_RD_FPGA_VERSION);
  ASSERT_EQ(static_cast<uint8_t>(tx.header().cpu_flag.value()), autd3::driver::MSG_RD_FPGA_VERSION);
}

TEST(Driver_Driver, fpga_functions) {
  autd3::driver::TxDatagram tx({NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT,
                                NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT});

  autd3::driver::FPGAFunctions::pack(tx);
  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_RD_FPGA_FUNCTION);
  ASSERT_EQ(static_cast<uint8_t>(tx.header().cpu_flag.value()), autd3::driver::MSG_RD_FPGA_FUNCTION);
}
