// File: test.cpp
// Project: tests
// Created Date: 14/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/11/2022
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

#include <autd3.hpp>
#include <autd3/core/acoustics.hpp>

#include "emulator_link.hpp"
#include "null_link.hpp"

TEST(ControllerTest, stream) {
  using autd3::clear;
  using autd3::gain::Null;
  using autd3::modulation::Sine;

  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());
  autd.open(autd3::test::NullLink().build());

  autd << clear;
  autd << Sine(150);
  autd << Null();
  autd << Sine(150), Null();

  autd << clear << clear                             //
       << clear << Sine(150)                         //
       << clear << Null();                           //
  autd << clear << Sine(150), Null();                //
  autd << Sine(150) << clear                         //
       << Sine(150) << Sine(150)                     //
       << Sine(150) << Null();                       //
  autd << Sine(150) << Sine(150), Null();            //
  autd << Null() << clear                            //
       << Null() << Sine(150)                        //
       << Null() << Null();                          //
  autd << Null() << (Sine(150), Null());             //
  autd << (Sine(150), Null()) << clear               //
       << (Sine(150), Null()) << Sine(150)           //
       << (Sine(150), Null()) << Null();             //
  autd << (Sine(150), Null()) << Sine(150), Null();  //

  auto s = Sine(150);
  auto n = Null();

  autd << s;
  autd << n;
  autd << s, n;

  autd << clear << clear   //
       << clear << s       //
       << clear << n;      //
  autd << clear << s, n;   //
  autd << s << clear       //
       << s << s           //
       << s << n;          //
  autd << s << s, n;       //
  autd << n << clear       //
       << n << s           //
       << n << n;          //
  autd << n << (s, n);     //
  autd << (s, n) << clear  //
       << (s, n) << s      //
       << (s, n) << n;     //
  autd << (s, n) << s, n;  //
}

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 4834)
#endif
TEST(ControllerTest, stream_async) {
  using autd3::async;
  using autd3::clear;
  using autd3::gain::Null;
  using autd3::modulation::Sine;

  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());
  autd.open(autd3::test::NullLink().build());

  autd << async << clear;
  autd << async << Sine(150);
  autd << async << Null();
  autd << async << Sine(150), Null();

  autd << async                                               //
       << clear << clear                                      //
       << clear << Sine(150)                                  //
       << clear << Null();                                    //
  autd << async << clear << Sine(150), Null();                //
  autd << async                                               //
       << Sine(150) << clear                                  //
       << Sine(150) << Sine(150)                              //
       << Sine(150) << Null();                                //
  autd << async << Sine(150) << Sine(150), Null();            //
  autd << async                                               //
       << Null() << clear                                     //
       << Null() << Sine(150)                                 //
       << Null() << Null();                                   //
  autd << async << Null() << (Sine(150), Null());             //
  autd << async                                               //
       << (Sine(150), Null()) << clear                        //
       << (Sine(150), Null()) << Sine(150)                    //
       << (Sine(150), Null()) << Null();                      //
  autd << async << (Sine(150), Null()) << Sine(150), Null();  //

  auto s = Sine(150);
  auto n = Null();

  autd << async << std::move(s);
  autd << async << std::move(n);

  s = Sine(150);
  n = Null();
  autd << async << std::move(s), std::move(n);

  {
    auto s1 = Sine(150);
    auto s2 = Sine(150);
    auto n1 = Null();
    auto n2 = Null();
    autd << async                                            //
         << clear << clear                                   //
         << clear << std::move(s1)                           //
         << clear << std::move(n1);                          //
    autd << async << clear << std::move(s2), std::move(n2);  //
  }

  {
    auto s1 = Sine(150);
    auto s2 = Sine(150);
    auto s3 = Sine(150);
    auto s4 = Sine(150);
    auto s5 = Sine(150);
    auto s6 = Sine(150);
    auto n1 = Null();
    auto n2 = Null();
    autd << async                                                    //
         << std::move(s1) << clear                                   //
         << std::move(s2) << std::move(s3)                           //
         << std::move(s4) << std::move(n1);                          //
    autd << async << std::move(s5) << std::move(s6), std::move(n2);  //
  }

  {
    auto s1 = Sine(150);
    auto s2 = Sine(150);
    auto n1 = Null();
    auto n2 = Null();
    auto n3 = Null();
    auto n4 = Null();
    auto n5 = Null();
    auto n6 = Null();
    autd << async                                                      //
         << std::move(n1) << clear                                     //
         << std::move(n2) << std::move(s1)                             //
         << std::move(n3) << std::move(n4);                            //
    autd << async << std::move(n5) << (std::move(s2), std::move(n6));  //
  }

  {
    auto s1 = Sine(150);
    auto s2 = Sine(150);
    auto s3 = Sine(150);
    auto s4 = Sine(150);
    auto s5 = Sine(150);
    auto s6 = Sine(150);
    auto n1 = Null();
    auto n2 = Null();
    auto n3 = Null();
    auto n4 = Null();
    auto n5 = Null();
    auto n6 = Null();
    autd << async                                                                     //
         << (std::move(s1), std::move(n1)) << clear                                   //
         << (std::move(s2), std::move(n2)) << std::move(s3)                           //
         << (std::move(s4), std::move(n3)) << std::move(n4);                          //
    autd << async << (std::move(s5), std::move(n5)) << std::move(s6), std::move(n6);  //
  }
}
#if _MSC_VER
#pragma warning(pop)
#endif

TEST(ControllerTest, basic_usage) {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());

  auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  autd.open(std::move(link));

  const auto firm_infos = autd.firmware_infos();
  ASSERT_EQ(firm_infos.size(), autd.geometry().num_devices());
  for (const auto& firm : firm_infos) {
    ASSERT_EQ(firm.cpu_version(), "v2.6");
    ASSERT_EQ(firm.fpga_version(), "v2.6");
  }

  autd << autd3::clear << autd3::synchronize;
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives();
    for (const auto& duty_pat : duties)
      for (const auto& [duty] : duty_pat) ASSERT_EQ(duty, 0x0000);
    for (const auto& phase_pat : phases)
      for (const auto& [phase] : phase_pat) ASSERT_EQ(phase, 0x0000);

    const auto cycles = cpu.fpga().cycles();
    for (const auto& cycle : cycles) ASSERT_EQ(cycle, 0x1000);

    ASSERT_EQ(cpu.fpga().modulation_cycle(), 2);
    ASSERT_EQ(cpu.fpga().modulation_frequency_division(), 40960);
    const auto mod = cpu.fpga().modulation();
    for (const auto& m : mod) ASSERT_EQ(m, 0x00);
  }

  autd3::SilencerConfig silencer;
  const autd3::Vector3 focus = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g(focus);
  autd3::modulation::Sine m(150);
  autd << silencer << m, g;
  for (const auto& cpu : *cpus) {
    ASSERT_TRUE(cpu.fpga_flags().contains(autd3::driver::FPGAControlFlags::LEGACY_MODE));
    ASSERT_FALSE(cpu.fpga_flags().contains(autd3::driver::FPGAControlFlags::STM_MODE));
  }
  const auto& base_tr = autd.geometry()[0][0];
  ASSERT_EQ(cpus->at(0).fpga().drives().first.size(), 1);
  ASSERT_EQ(cpus->at(0).fpga().drives().second.size(), 1);
  const auto expect =
      std::arg(autd3::core::propagate(base_tr.position(), base_tr.z_direction(), 0.0, base_tr.wavenumber(autd.geometry().sound_speed), focus) *
               std::exp(std::complex(0.0, 2.0 * autd3::pi * static_cast<double>(cpus->at(0).fpga().drives().second[0][0].phase) /
                                              static_cast<double>(cpus->at(0).fpga().cycles()[0]))));
  for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
    for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT; j++) {
      const auto p = std::arg(autd3::core::propagate(autd.geometry()[i][j].position(), autd.geometry()[i][j].z_direction(), 0.0,
                                                     autd.geometry()[i][j].wavenumber(autd.geometry().sound_speed), focus) *
                              std::exp(std::complex(0.0, 2.0 * autd3::pi * static_cast<double>(cpus->at(i).fpga().drives().second[0][j].phase) /
                                                             static_cast<double>(cpus->at(i).fpga().cycles()[j]))));
      ASSERT_EQ(cpus->at(i).fpga().drives().first[0][j].duty, cpus->at(i).fpga().cycles()[j] >> 1);
      ASSERT_NEAR(p, expect, 2.0 * autd3::pi / 256.0);
    }
  }
  const std::vector<uint8_t> expect_mod = {85,  108, 132, 157, 183, 210, 237, 246, 219, 192, 166, 140, 116, 92,  71,  51,  34,  19,  9,   2,
                                           0,   2,   9,   19,  34,  51,  71,  92,  116, 140, 166, 192, 219, 246, 237, 210, 183, 157, 132, 108,
                                           85,  64,  45,  29,  16,  6,   1,   0,   4,   12,  24,  39,  57,  78,  100, 124, 149, 175, 201, 228,
                                           255, 228, 201, 175, 149, 124, 100, 78,  57,  39,  24,  12,  4,   0,   1,   6,   16,  29,  45,  64};
  for (const auto& cpu : *cpus) {
    ASSERT_EQ(cpu.fpga().modulation_cycle(), expect_mod.size());
    for (size_t i = 0; i < std::min(cpu.fpga().modulation_cycle(), expect_mod.size()); i++) ASSERT_EQ(cpu.fpga().modulation()[i], expect_mod[i]);
  }
  for (const auto& cpu : *cpus) {
    ASSERT_EQ(cpu.fpga().silencer_cycle(), 4096);
    ASSERT_EQ(cpu.fpga().silencer_step(), 10);
  }

  autd << autd3::stop;
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives();
    for (const auto& duty_pat : duties)
      for (const auto& [duty] : duty_pat) ASSERT_EQ(duty, 0x0000);
  }

  autd << autd3::clear;
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives();
    for (const auto& duty_pat : duties)
      for (const auto& [duty] : duty_pat) ASSERT_EQ(duty, 0x0000);
    for (const auto& phase_pat : phases)
      for (const auto& [phase] : phase_pat) ASSERT_EQ(phase, 0x0000);

    const auto cycles = cpu.fpga().cycles();
    for (const auto& cycle : cycles) ASSERT_EQ(cycle, 0x1000);

    ASSERT_EQ(cpu.fpga().modulation_cycle(), 2);
    ASSERT_EQ(cpu.fpga().modulation_frequency_division(), 40960);
    const auto mod = cpu.fpga().modulation();
    for (const auto& mv : mod) ASSERT_EQ(mv, 0x00);
  }

  autd.close();
}

TEST(ControllerTest, basic_usage_async) {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());

  auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  autd.open(std::move(link));

  const auto firm_infos = autd.firmware_infos();
  ASSERT_EQ(firm_infos.size(), autd.geometry().num_devices());
  for (const auto& firm : firm_infos) {
    ASSERT_EQ(firm.cpu_version(), "v2.6");
    ASSERT_EQ(firm.fpga_version(), "v2.6");
  }

  autd << autd3::async << autd3::clear << autd3::synchronize;
  autd.wait();
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives();
    for (const auto& duty_pat : duties)
      for (const auto& [duty] : duty_pat) ASSERT_EQ(duty, 0x0000);
    for (const auto& phase_pat : phases)
      for (const auto& [phase] : phase_pat) ASSERT_EQ(phase, 0x0000);

    const auto cycles = cpu.fpga().cycles();
    for (const auto& cycle : cycles) ASSERT_EQ(cycle, 0x1000);

    ASSERT_EQ(cpu.fpga().modulation_cycle(), 2);
    ASSERT_EQ(cpu.fpga().modulation_frequency_division(), 40960);
    const auto mod = cpu.fpga().modulation();
    for (const auto& m : mod) ASSERT_EQ(m, 0x00);
  }

  autd3::SilencerConfig silencer;
  const autd3::Vector3 focus = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g(focus);
  autd3::modulation::Sine m(150);
  autd << autd3::async << silencer << std::move(m), std::move(g);
  autd.wait();
  for (const auto& cpu : *cpus) {
    ASSERT_TRUE(cpu.fpga_flags().contains(autd3::driver::FPGAControlFlags::LEGACY_MODE));
    ASSERT_FALSE(cpu.fpga_flags().contains(autd3::driver::FPGAControlFlags::STM_MODE));
  }
  const auto& base_tr = autd.geometry()[0][0];
  ASSERT_EQ(cpus->at(0).fpga().drives().first.size(), 1);
  ASSERT_EQ(cpus->at(0).fpga().drives().second.size(), 1);
  const auto expect =
      std::arg(autd3::core::propagate(base_tr.position(), base_tr.z_direction(), 0.0, base_tr.wavenumber(autd.geometry().sound_speed), focus) *
               std::exp(std::complex(0.0, 2.0 * autd3::pi * static_cast<double>(cpus->at(0).fpga().drives().second[0][0].phase) /
                                              static_cast<double>(cpus->at(0).fpga().cycles()[0]))));
  for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
    for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT; j++) {
      const auto p = std::arg(autd3::core::propagate(autd.geometry()[i][j].position(), autd.geometry()[i][j].z_direction(), 0.0,
                                                     autd.geometry()[i][j].wavenumber(autd.geometry().sound_speed), focus) *
                              std::exp(std::complex(0.0, 2.0 * autd3::pi * static_cast<double>(cpus->at(i).fpga().drives().second[0][j].phase) /
                                                             static_cast<double>(cpus->at(i).fpga().cycles()[j]))));
      ASSERT_EQ(cpus->at(i).fpga().drives().first[0][j].duty, cpus->at(i).fpga().cycles()[j] >> 1);
      ASSERT_NEAR(p, expect, 2.0 * autd3::pi / 256.0);
    }
  }
  const std::vector<uint8_t> expect_mod = {85,  108, 132, 157, 183, 210, 237, 246, 219, 192, 166, 140, 116, 92,  71,  51,  34,  19,  9,   2,
                                           0,   2,   9,   19,  34,  51,  71,  92,  116, 140, 166, 192, 219, 246, 237, 210, 183, 157, 132, 108,
                                           85,  64,  45,  29,  16,  6,   1,   0,   4,   12,  24,  39,  57,  78,  100, 124, 149, 175, 201, 228,
                                           255, 228, 201, 175, 149, 124, 100, 78,  57,  39,  24,  12,  4,   0,   1,   6,   16,  29,  45,  64};
  for (const auto& cpu : *cpus) {
    ASSERT_EQ(cpu.fpga().modulation_cycle(), expect_mod.size());
    for (size_t i = 0; i < std::min(cpu.fpga().modulation_cycle(), expect_mod.size()); i++) ASSERT_EQ(cpu.fpga().modulation()[i], expect_mod[i]);
  }
  for (const auto& cpu : *cpus) {
    ASSERT_EQ(cpu.fpga().silencer_cycle(), 4096);
    ASSERT_EQ(cpu.fpga().silencer_step(), 10);
  }

  autd << autd3::async << autd3::stop;
  autd.wait();
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives();
    for (const auto& duty_pat : duties)
      for (const auto& [duty] : duty_pat) ASSERT_EQ(duty, 0x0000);
  }

  autd << autd3::async << autd3::clear;
  autd.wait();
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives();
    for (const auto& duty_pat : duties)
      for (const auto& [duty] : duty_pat) ASSERT_EQ(duty, 0x0000);
    for (const auto& phase_pat : phases)
      for (const auto& [phase] : phase_pat) ASSERT_EQ(phase, 0x0000);

    const auto cycles = cpu.fpga().cycles();
    for (const auto& cycle : cycles) ASSERT_EQ(cycle, 0x1000);

    ASSERT_EQ(cpu.fpga().modulation_cycle(), 2);
    ASSERT_EQ(cpu.fpga().modulation_frequency_division(), 40960);
    const auto mod = cpu.fpga().modulation();
    for (const auto& mv : mod) ASSERT_EQ(mv, 0x00);
  }

  autd.close();
}

TEST(ControllerTest, freq_config) {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());

  const auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  autd.open(std::move(link));

  autd.geometry().mode() = std::make_unique<autd3::NormalMode>();
  for (auto& dev : autd.geometry())
    for (auto& tr : dev) tr.set_cycle(2341);

  autd.send(autd3::clear());
  autd.send(autd3::synchronize());

  for (const auto& cpu : *cpus)
    for (const auto& cycle : cpu.fpga().cycles()) ASSERT_EQ(cycle, 2341);

  autd.close();
}

TEST(ControllerTest, simple_legacy) {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(0, autd3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, autd3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero());

  ASSERT_EQ(autd.geometry().num_devices(), 4);
  ASSERT_EQ(autd.geometry().num_transducers(), 4 * 249);

  const auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  autd.open(std::move(link));

  autd << autd3::clear << autd3::synchronize;

  const autd3::Vector3 focus = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g(focus);
  autd << g;
  const auto& base_tr = autd.geometry()[0][0];
  const auto expect =
      std::arg(autd3::core::propagate(base_tr.position(), base_tr.z_direction(), 0.0, base_tr.wavenumber(autd.geometry().sound_speed), focus) *
               std::exp(std::complex(0.0, 2.0 * autd3::pi * static_cast<double>(cpus->at(0).fpga().drives().second[0][0].phase) /
                                              static_cast<double>(cpus->at(0).fpga().cycles()[0]))));
  for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
    for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT; j++) {
      const auto p = std::arg(autd3::core::propagate(autd.geometry()[i][j].position(), autd.geometry()[i][j].z_direction(), 0.0,
                                                     autd.geometry()[i][j].wavenumber(autd.geometry().sound_speed), focus) *
                              std::exp(std::complex(0.0, 2.0 * autd3::pi * static_cast<double>(cpus->at(i).fpga().drives().second[0][j].phase) /
                                                             static_cast<double>(cpus->at(i).fpga().cycles()[j]))));
      ASSERT_EQ(cpus->at(i).fpga().drives().first[0][j].duty, cpus->at(i).fpga().cycles()[j] >> 1);
      ASSERT_NEAR(p, expect, 2.0 * autd3::pi / 256.0);
    }
  }
  autd.close();
}

TEST(ControllerTest, simple_normal) {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(0, autd3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, autd3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero());

  ASSERT_EQ(autd.geometry().num_devices(), 4);
  ASSERT_EQ(autd.geometry().num_transducers(), 4 * 249);

  const auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  autd.open(std::move(link));

  autd.geometry().mode() = std::make_unique<autd3::NormalMode>();
  constexpr uint16_t cycle = 2341;  // 70kHz
  for (auto& dev : autd.geometry())
    for (auto& tr : dev) tr.set_cycle(cycle);

  autd << autd3::clear << autd3::synchronize;

  const autd3::Vector3 focus = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g(focus);
  autd << g;
  const auto& base_tr = autd.geometry()[0][0];
  const auto expect =
      std::arg(autd3::core::propagate(base_tr.position(), base_tr.z_direction(), 0.0, base_tr.wavenumber(autd.geometry().sound_speed), focus) *
               std::exp(std::complex(0.0, 2.0 * autd3::pi * static_cast<double>(cpus->at(0).fpga().drives().second[0][0].phase) /
                                              static_cast<double>(cpus->at(0).fpga().cycles()[0]))));
  for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
    for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT; j++) {
      const auto p = std::arg(autd3::core::propagate(autd.geometry()[i][j].position(), autd.geometry()[i][j].z_direction(), 0.0,
                                                     autd.geometry()[i][j].wavenumber(autd.geometry().sound_speed), focus) *
                              std::exp(std::complex(0.0, 2.0 * autd3::pi * static_cast<double>(cpus->at(i).fpga().drives().second[0][j].phase) /
                                                             static_cast<double>(cpus->at(i).fpga().cycles()[j]))));
      ASSERT_LE(std::abs(static_cast<int>(cpus->at(i).fpga().drives().first[0][j].duty) - (cpus->at(i).fpga().cycles()[j] >> 1)), 1);
      ASSERT_NEAR(p, expect, 2.0 * autd3::pi / static_cast<double>(cycle));
    }
  }
  autd.close();
}

TEST(ControllerTest, simple_normal_phase) {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(0, autd3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, autd3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero());

  ASSERT_EQ(autd.geometry().num_devices(), 4);
  ASSERT_EQ(autd.geometry().num_transducers(), 4 * 249);

  const auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  autd.open(std::move(link));

  autd.geometry().mode() = std::make_unique<autd3::NormalPhaseMode>();
  constexpr uint16_t cycle = 2341;  // 70kHz
  for (auto& dev : autd.geometry())
    for (auto& tr : dev) tr.set_cycle(cycle);

  autd << autd3::clear << autd3::synchronize;

  autd3::Amplitudes amp(1.0);
  autd << amp;

  const autd3::Vector3 focus = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g(focus);
  autd << g;
  const auto& base_tr = autd.geometry()[0][0];
  const auto expect =
      std::arg(autd3::core::propagate(base_tr.position(), base_tr.z_direction(), 0.0, base_tr.wavenumber(autd.geometry().sound_speed), focus) *
               std::exp(std::complex(0.0, 2.0 * autd3::pi * static_cast<double>(cpus->at(0).fpga().drives().second[0][0].phase) /
                                              static_cast<double>(cpus->at(0).fpga().cycles()[0]))));
  for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
    for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT; j++) {
      const auto p = std::arg(autd3::core::propagate(autd.geometry()[i][j].position(), autd.geometry()[i][j].z_direction(), 0.0,
                                                     autd.geometry()[i][j].wavenumber(autd.geometry().sound_speed), focus) *
                              std::exp(std::complex(0.0, 2.0 * autd3::pi * static_cast<double>(cpus->at(i).fpga().drives().second[0][j].phase) /
                                                             static_cast<double>(cpus->at(i).fpga().cycles()[j]))));
      ASSERT_LE(std::abs(static_cast<int>(cpus->at(i).fpga().drives().first[0][j].duty) - (cpus->at(i).fpga().cycles()[j] >> 1)), 1);
      ASSERT_NEAR(p, expect, 2.0 * autd3::pi / static_cast<double>(cycle));
    }
  }
  autd.close();
}

TEST(ControllerTest, point_stm) {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(0, autd3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, autd3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero());

  const auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  autd.open(std::move(link));

  autd << autd3::clear << autd3::synchronize;

  const autd3::Vector3 center = autd.geometry().center();

  constexpr size_t size = 200;
  std::vector<autd3::Point> points;
  constexpr auto radius = 30.0;
  std::vector<size_t> iota(size);
  std::iota(iota.begin(), iota.end(), 0);
  std::transform(iota.begin(), iota.end(), std::back_inserter(points), [&](const size_t i) {
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(size);
    return autd3::Point(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
  });

  autd3::PointSTM stm;
  std::copy(points.begin(), points.end(), std::back_inserter(stm));

  autd << stm;
  for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

  const auto cycle = cpus->at(0).fpga().cycles()[0];
  const auto wavenumber = autd.geometry()[0][0].wavenumber(autd.geometry().sound_speed);
  const auto& base_tr = autd.geometry()[0][0];
  const auto& z_dir = base_tr.z_direction();
  constexpr double criteria = 2.0 * autd3::pi / 100.0;
  for (size_t k = 0; k < size; k++) {
    const auto& focus = points[k].point;
    const auto expect = std::arg(autd3::core::propagate(base_tr.position(), z_dir, 0.0, wavenumber, focus)) +
                        2.0 * autd3::pi * static_cast<double>(cpus->at(0).fpga().drives().second[k][0].phase) / static_cast<double>(cycle);
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
      for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT; j++) {
        const auto p = std::arg(autd3::core::propagate(autd.geometry()[i][j].position(), z_dir, 0.0, wavenumber, focus)) +
                       2.0 * autd3::pi * static_cast<double>(cpus->at(i).fpga().drives().second[k][j].phase) / static_cast<double>(cycle);
        ASSERT_EQ(cpus->at(i).fpga().drives().first[k][j].duty, cycle >> 1);
        if (autd3::driver::rem_euclid(p - expect, 2.0 * autd3::pi) > autd3::pi)
          ASSERT_NEAR(autd3::driver::rem_euclid(p - expect, 2.0 * autd3::pi), 2.0 * autd3::pi, criteria);
        else
          ASSERT_NEAR(autd3::driver::rem_euclid(p - expect, 2.0 * autd3::pi), 0.0, criteria);
      }
    }
  }

  autd << autd3::stop;
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives();
    for (const auto& duty_pat : duties)
      for (const auto& [duty] : duty_pat) ASSERT_EQ(duty, 0x0000);
  }

  autd.close();
}

TEST(ControllerTest, gain_stm_legacy) {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(0, autd3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, autd3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero());

  auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  autd.open(std::move(link));

  autd << autd3::clear << autd3::synchronize;

  const autd3::Vector3 center = autd.geometry().center();

  {
    autd3::GainSTM stm(autd.geometry());
    constexpr size_t size = 50;
    std::vector<std::vector<autd3::driver::Drive>> drives;
    constexpr auto radius = 30.0;
    std::vector<size_t> iota(size);
    std::iota(iota.begin(), iota.end(), 0);
    std::for_each(iota.begin(), iota.end(), [&](const size_t i) {
      const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(size);
      autd3::gain::Focus f(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
      f.build(autd.geometry());
      drives.emplace_back(f.drives());
      stm.add(f);
    });

    autd << stm;
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

    for (size_t k = 0; k < size; k++) {
      for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
        for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT; j++) {
          ASSERT_EQ(cpus->at(i).fpga().drives().first[k][j].duty,
                    (autd3::driver::LegacyDrive::to_duty(drives[k][i * autd3::driver::NUM_TRANS_IN_UNIT + j]) << 3) + 0x08);
          ASSERT_EQ(cpus->at(i).fpga().drives().second[k][j].phase,
                    autd3::driver::LegacyDrive::to_phase(drives[k][i * autd3::driver::NUM_TRANS_IN_UNIT + j]) << 4);
        }
      }
    }
  }

  {
    autd3::GainSTM stm(autd.geometry());
    stm.mode() = autd3::GainSTMMode::PhaseFull;
    constexpr size_t size = 50;
    std::vector<std::vector<autd3::driver::Drive>> drives;
    constexpr auto radius = 40.0;
    std::vector<size_t> iota(size);
    std::iota(iota.begin(), iota.end(), 0);
    std::for_each(iota.begin(), iota.end(), [&](const size_t i) {
      const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(size);
      autd3::gain::Focus f(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
      f.build(autd.geometry());
      drives.emplace_back(f.drives());
      stm.add(f);
    });

    autd << stm;
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

    const uint16_t cycle = autd.geometry()[0][0].cycle();
    for (size_t k = 0; k < size; k++) {
      for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
        for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT; j++) {
          ASSERT_EQ(cpus->at(i).fpga().drives().first[k][j].duty, cycle >> 1);
          ASSERT_EQ(cpus->at(i).fpga().drives().second[k][j].phase,
                    autd3::driver::LegacyDrive::to_phase(drives[k][i * autd3::driver::NUM_TRANS_IN_UNIT + j]) << 4);
        }
      }
    }
  }

  {
    autd3::GainSTM stm(autd.geometry());
    stm.mode() = autd3::GainSTMMode::PhaseHalf;
    constexpr size_t size = 50;
    std::vector<std::vector<autd3::driver::Drive>> drives;
    constexpr auto radius = 40.0;
    std::vector<size_t> iota(size);
    std::iota(iota.begin(), iota.end(), 0);
    std::for_each(iota.begin(), iota.end(), [&](const size_t i) {
      const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(size);
      autd3::gain::Focus f(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
      f.build(autd.geometry());
      drives.emplace_back(f.drives());
      stm.add(f);
    });

    autd << stm;
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

    const uint16_t cycle = autd.geometry()[0][0].cycle();
    for (size_t k = 0; k < size; k++) {
      for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
        for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT; j++) {
          ASSERT_EQ(cpus->at(i).fpga().drives().first[k][j].duty, cycle >> 1);
          const auto phase = autd3::driver::LegacyDrive::to_phase(drives[k][i * autd3::driver::NUM_TRANS_IN_UNIT + j]) >> 4;
          ASSERT_EQ(cpus->at(i).fpga().drives().second[k][j].phase, ((phase << 4) + phase) << 4);
        }
      }
    }
  }

  autd << autd3::stop;
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives();
    for (const auto& duty_pat : duties)
      for (const auto& [duty] : duty_pat) ASSERT_EQ(duty, 0x0000);
  }

  autd.close();
}

TEST(ControllerTest, gain_stm_normal) {
  autd3::Controller autd;

  autd.geometry().mode() = std::make_unique<autd3::NormalMode>();

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(0, autd3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, autd3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero());

  auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  autd.open(std::move(link));

  autd << autd3::clear << autd3::synchronize;

  const autd3::Vector3 center = autd.geometry().center();

  {
    autd3::GainSTM stm(autd.geometry());
    constexpr size_t size = 50;
    std::vector<std::vector<autd3::driver::Drive>> drives;
    constexpr auto radius = 30.0;
    std::vector<size_t> iota(size);
    std::iota(iota.begin(), iota.end(), 0);
    std::for_each(iota.begin(), iota.end(), [&](const size_t i) {
      const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(size);
      autd3::gain::Focus f(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
      f.build(autd.geometry());
      drives.emplace_back(f.drives());
      stm.add(f);
    });

    autd << stm;
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

    for (size_t k = 0; k < size; k++) {
      for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
        for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT; j++) {
          ASSERT_EQ(cpus->at(i).fpga().drives().first[k][j].duty, autd3::driver::Duty::to_duty(drives[k][i * autd3::driver::NUM_TRANS_IN_UNIT + j]));
          ASSERT_EQ(cpus->at(i).fpga().drives().second[k][j].phase,
                    autd3::driver::Phase::to_phase(drives[k][i * autd3::driver::NUM_TRANS_IN_UNIT + j]));
        }
      }
    }
  }

  {
    autd3::GainSTM stm(autd.geometry());
    stm.mode() = autd3::GainSTMMode::PhaseFull;
    constexpr size_t size = 50;
    std::vector<std::vector<autd3::driver::Drive>> drives;
    constexpr auto radius = 40.0;
    std::vector<size_t> iota(size);
    std::iota(iota.begin(), iota.end(), 0);
    std::for_each(iota.begin(), iota.end(), [&](const size_t i) {
      const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(size);
      autd3::gain::Focus f(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
      f.build(autd.geometry());
      drives.emplace_back(f.drives());
      stm.add(f);
    });

    autd << stm;
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

    const uint16_t cycle = autd.geometry()[0][0].cycle();
    for (size_t k = 0; k < size; k++) {
      for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
        for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT; j++) {
          ASSERT_EQ(cpus->at(i).fpga().drives().first[k][j].duty, cycle >> 1);
          ASSERT_EQ(cpus->at(i).fpga().drives().second[k][j].phase,
                    autd3::driver::Phase::to_phase(drives[k][i * autd3::driver::NUM_TRANS_IN_UNIT + j]));
        }
      }
    }
  }

  autd << autd3::stop;
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives();
    for (const auto& duty_pat : duties)
      for (const auto& [duty] : duty_pat) ASSERT_EQ(duty, 0x0000);
  }

  autd.close();
}

TEST(ControllerTest, gain_stm_normal_phase) {
  autd3::Controller autd;

  autd.geometry().mode() = std::make_unique<autd3::NormalPhaseMode>();

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(0, autd3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, autd3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero());

  auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  autd.open(std::move(link));

  autd << autd3::clear << autd3::synchronize;

  autd << autd3::Amplitudes(1.0);

  const autd3::Vector3 center = autd.geometry().center();
  {
    autd3::GainSTM stm(autd.geometry());
    constexpr size_t size = 50;
    std::vector<std::vector<autd3::driver::Drive>> drives;
    constexpr auto radius = 30.0;
    std::vector<size_t> iota(size);
    std::iota(iota.begin(), iota.end(), 0);
    std::for_each(iota.begin(), iota.end(), [&](const size_t i) {
      const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(size);
      autd3::gain::Focus f(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
      f.build(autd.geometry());
      drives.emplace_back(f.drives());
      stm.add(f);
    });

    autd << stm;
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

    const uint16_t cycle = autd.geometry()[0][0].cycle();
    for (size_t k = 0; k < size; k++) {
      for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
        for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT; j++) {
          ASSERT_EQ(cpus->at(i).fpga().drives().first[k][j].duty, cycle >> 1);
          ASSERT_EQ(cpus->at(i).fpga().drives().second[k][j].phase,
                    autd3::driver::Phase::to_phase(drives[k][i * autd3::driver::NUM_TRANS_IN_UNIT + j]));
        }
      }
    }
  }

  {
    autd3::GainSTM stm(autd.geometry());
    stm.mode() = autd3::GainSTMMode::PhaseFull;
    constexpr size_t size = 50;
    std::vector<std::vector<autd3::driver::Drive>> drives;
    constexpr auto radius = 30.0;
    std::vector<size_t> iota(size);
    std::iota(iota.begin(), iota.end(), 0);
    std::for_each(iota.begin(), iota.end(), [&](const size_t i) {
      const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(size);
      autd3::gain::Focus f(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
      f.build(autd.geometry());
      drives.emplace_back(f.drives());
      stm.add(f);
    });

    autd << stm;
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

    const uint16_t cycle = autd.geometry()[0][0].cycle();
    for (size_t k = 0; k < size; k++) {
      for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
        for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT; j++) {
          ASSERT_EQ(cpus->at(i).fpga().drives().first[k][j].duty, cycle >> 1);
          ASSERT_EQ(cpus->at(i).fpga().drives().second[k][j].phase,
                    autd3::driver::Phase::to_phase(drives[k][i * autd3::driver::NUM_TRANS_IN_UNIT + j]));
        }
      }
    }
  }

  autd << autd3::stop;
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives();
    for (const auto& duty_pat : duties)
      for (const auto& [duty] : duty_pat) ASSERT_EQ(duty, 0x0000);
  }

  autd.close();
}

int main(int argc, char** argv) {
  testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
