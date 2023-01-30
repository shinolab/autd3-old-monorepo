// File: test.cpp
// Project: tests
// Created Date: 14/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 31/01/2023
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

  auto geometry = autd3::Geometry::Builder().add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero())).sound_speed(340.0e3).build();

  auto autd = autd3::Controller::open(std::move(geometry), autd3::test::NullLink().build());

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

TEST(ControllerTest, basic_usage) {
  auto geometry = autd3::Geometry::Builder().add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero())).sound_speed(340.0e3).build();

  auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  const auto firm_infos = autd.firmware_infos();
  ASSERT_EQ(firm_infos.size(), autd.geometry().num_devices());
  for (const auto& firm : firm_infos) {
    ASSERT_EQ(firm.cpu_version(), "v2.8");
    ASSERT_EQ(firm.fpga_version(), "v2.8");
  }

  autd << autd3::clear << autd3::synchronize;
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives(0);
    for (const auto& [duty] : duties) ASSERT_EQ(duty, 0x0000);
    for (const auto& [phase] : phases) ASSERT_EQ(phase, 0x0000);

    const auto cycles = cpu.fpga().cycles();
    for (const auto& cycle : cycles) ASSERT_EQ(cycle, 0x1000);

    ASSERT_EQ(cpu.fpga().modulation_cycle(), 2);
    ASSERT_EQ(cpu.fpga().modulation_frequency_division(), 40960);
    const auto mod = cpu.fpga().modulation();
    for (const auto& m : mod) ASSERT_EQ(m, 0x00);
  }

  autd3::SilencerConfig silencer;
  const autd3::Vector3 focus = autd.geometry().center() + autd3::Vector3(0, 0, 150);
  autd3::gain::Focus g(focus);
  autd3::modulation::Sine m(150);
  autd << silencer << m, g;
  for (const auto& cpu : *cpus) {
    ASSERT_TRUE(cpu.fpga_flags().contains(autd3::driver::FPGAControlFlags::LegacyMode));
    ASSERT_FALSE(cpu.fpga_flags().contains(autd3::driver::FPGAControlFlags::STMMode));
  }
  const auto& base_tr = autd.geometry()[0];
  const auto expect =
      std::arg(autd3::core::propagate(base_tr.position(), base_tr.z_direction(), 0, base_tr.wavenumber(autd.geometry().sound_speed), focus) *
               std::exp(std::complex<autd3::driver::autd3_float_t>(
                   0, 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(cpus->at(0).fpga().drives(0).second[0].phase) /
                          static_cast<autd3::driver::autd3_float_t>(cpus->at(0).fpga().cycles()[0]))));
  for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
    for (size_t j = 0; j < autd.geometry().device_map()[i]; j++) {
      const auto p =
          std::arg(autd3::core::propagate(autd.geometry()[i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j].position(),
                                          autd.geometry()[i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j].z_direction(), 0,
                                          autd.geometry()[i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j].wavenumber(autd.geometry().sound_speed), focus) *
                   std::exp(std::complex<autd3::driver::autd3_float_t>(
                       0, 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(cpus->at(i).fpga().drives(0).second[j].phase) /
                              static_cast<autd3::driver::autd3_float_t>(cpus->at(i).fpga().cycles()[j]))));
      ASSERT_EQ(cpus->at(i).fpga().drives(0).first[j].duty, cpus->at(i).fpga().cycles()[j] >> 1);
      ASSERT_NEAR(p, expect, 2 * autd3::pi / 256);
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
    const auto [duties, phases] = cpu.fpga().drives(0);
    for (const auto& [duty] : duties) ASSERT_EQ(duty, 0x0000);
  }

  autd << autd3::clear;
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives(0);
    for (const auto& [duty] : duties) ASSERT_EQ(duty, 0x0000);
    for (const auto& [phase] : phases) ASSERT_EQ(phase, 0x0000);

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
  auto geometry =
      autd3::Geometry::Builder().add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero())).sound_speed(340.0e3).normal_mode().build();

  const auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  for (auto& tr : autd.geometry()) tr.set_cycle(2341);

  autd.send(autd3::clear());
  autd.send(autd3::synchronize());

  for (const auto& cpu : *cpus)
    for (const auto& cycle : cpu.fpga().cycles()) ASSERT_EQ(cycle, 2341);

  autd.close();
}

TEST(ControllerTest, simple_legacy) {
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(0, autd3::AUTD3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, autd3::AUTD3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero()))
                      .sound_speed(340.0e3)
                      .build();

  const auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  ASSERT_EQ(autd.geometry().num_devices(), 4);
  ASSERT_EQ(autd.geometry().num_transducers(), 4 * 249);

  autd << autd3::clear << autd3::synchronize;

  const autd3::Vector3 focus = autd.geometry().center() + autd3::Vector3(0, 0, 150);
  autd3::gain::Focus g(focus);
  autd << g;
  const auto& base_tr = autd.geometry()[0];
  const auto expect =
      std::arg(autd3::core::propagate(base_tr.position(), base_tr.z_direction(), 0, base_tr.wavenumber(autd.geometry().sound_speed), focus) *
               std::exp(std::complex<autd3::driver::autd3_float_t>(
                   0, 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(cpus->at(0).fpga().drives(0).second[0].phase) /
                          static_cast<autd3::driver::autd3_float_t>(cpus->at(0).fpga().cycles()[0]))));
  for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
    for (size_t j = 0; j < autd.geometry().device_map()[i]; j++) {
      const auto p =
          std::arg(autd3::core::propagate(autd.geometry()[i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j].position(),
                                          autd.geometry()[i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j].z_direction(), 0,
                                          autd.geometry()[i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j].wavenumber(autd.geometry().sound_speed), focus) *
                   std::exp(std::complex<autd3::driver::autd3_float_t>(
                       0, 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(cpus->at(i).fpga().drives(0).second[j].phase) /
                              static_cast<autd3::driver::autd3_float_t>(cpus->at(i).fpga().cycles()[j]))));
      ASSERT_EQ(cpus->at(i).fpga().drives(0).first[j].duty, cpus->at(i).fpga().cycles()[j] >> 1);
      ASSERT_NEAR(p, expect, 2 * autd3::pi / 256.0);
    }
  }
  autd.close();
}

TEST(ControllerTest, simple_normal) {
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(0, autd3::AUTD3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, autd3::AUTD3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero()))
                      .sound_speed(340.0e3)
                      .normal_mode()
                      .build();

  const auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  ASSERT_EQ(autd.geometry().num_devices(), 4);
  ASSERT_EQ(autd.geometry().num_transducers(), 4 * 249);

  constexpr uint16_t cycle = 2341;  // 70kHz
  for (auto& tr : autd.geometry()) tr.set_cycle(cycle);

  autd << autd3::clear << autd3::synchronize;

  const autd3::Vector3 focus = autd.geometry().center() + autd3::Vector3(0, 0, 150);
  autd3::gain::Focus g(focus);
  autd << g;
  const auto& base_tr = autd.geometry()[0];
  const auto expect =
      std::arg(autd3::core::propagate(base_tr.position(), base_tr.z_direction(), 0, base_tr.wavenumber(autd.geometry().sound_speed), focus) *
               std::exp(std::complex<autd3::driver::autd3_float_t>(
                   0, 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(cpus->at(0).fpga().drives(0).second[0].phase) /
                          static_cast<autd3::driver::autd3_float_t>(cpus->at(0).fpga().cycles()[0]))));
  for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
    for (size_t j = 0; j < autd.geometry().device_map()[i]; j++) {
      const auto p =
          std::arg(autd3::core::propagate(autd.geometry()[i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j].position(),
                                          autd.geometry()[i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j].z_direction(), 0,
                                          autd.geometry()[i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j].wavenumber(autd.geometry().sound_speed), focus) *
                   std::exp(std::complex<autd3::driver::autd3_float_t>(
                       0, 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(cpus->at(i).fpga().drives(0).second[j].phase) /
                              static_cast<autd3::driver::autd3_float_t>(cpus->at(i).fpga().cycles()[j]))));
      ASSERT_LE(std::abs(static_cast<int>(cpus->at(i).fpga().drives(0).first[j].duty) - (cpus->at(i).fpga().cycles()[j] >> 1)), 1);
      ASSERT_NEAR(p, expect, 2 * autd3::pi / static_cast<autd3::driver::autd3_float_t>(cycle));
    }
  }
  autd.close();
}

TEST(ControllerTest, simple_normal_phase) {
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(0, autd3::AUTD3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, autd3::AUTD3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero()))
                      .sound_speed(340.0e3)
                      .normal_phase_mode()
                      .build();

  const auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  ASSERT_EQ(autd.geometry().num_devices(), 4);
  ASSERT_EQ(autd.geometry().num_transducers(), 4 * 249);

  constexpr uint16_t cycle = 2341;  // 70kHz
  for (auto& tr : autd.geometry()) tr.set_cycle(cycle);

  autd << autd3::clear << autd3::synchronize;

  autd3::Amplitudes amp(1.0);
  autd << amp;

  const autd3::Vector3 focus = autd.geometry().center() + autd3::Vector3(0, 0, 150);
  autd3::gain::Focus g(focus);
  autd << g;
  const auto& base_tr = autd.geometry()[0];
  const auto expect =
      std::arg(autd3::core::propagate(base_tr.position(), base_tr.z_direction(), 0, base_tr.wavenumber(autd.geometry().sound_speed), focus) *
               std::exp(std::complex<autd3::driver::autd3_float_t>(
                   0, 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(cpus->at(0).fpga().drives(0).second[0].phase) /
                          static_cast<autd3::driver::autd3_float_t>(cpus->at(0).fpga().cycles()[0]))));
  for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
    for (size_t j = 0; j < autd.geometry().device_map()[i]; j++) {
      const auto p =
          std::arg(autd3::core::propagate(autd.geometry()[i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j].position(),
                                          autd.geometry()[i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j].z_direction(), 0,
                                          autd.geometry()[i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j].wavenumber(autd.geometry().sound_speed), focus) *
                   std::exp(std::complex<autd3::driver::autd3_float_t>(
                       0, 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(cpus->at(i).fpga().drives(0).second[j].phase) /
                              static_cast<autd3::driver::autd3_float_t>(cpus->at(i).fpga().cycles()[j]))));
      ASSERT_LE(std::abs(static_cast<int>(cpus->at(i).fpga().drives(0).first[j].duty) - (cpus->at(i).fpga().cycles()[j] >> 1)), 1);
      ASSERT_NEAR(p, expect, 2 * autd3::pi / static_cast<autd3::driver::autd3_float_t>(cycle));
    }
  }
  autd.close();
}

TEST(ControllerTest, focus_stm) {
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(0, autd3::AUTD3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, autd3::AUTD3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero()))
                      .sound_speed(340.0e3)
                      .build();

  const auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  autd << autd3::clear << autd3::synchronize;

  const autd3::Vector3 center = autd.geometry().center();

  constexpr size_t size = 200;
  std::vector<autd3::FocusSTM::Focus> points;
  constexpr autd3::driver::autd3_float_t radius = 30;
  std::vector<size_t> iota(size);
  std::iota(iota.begin(), iota.end(), 0);
  std::transform(iota.begin(), iota.end(), std::back_inserter(points), [&](const size_t i) {
    const auto theta = 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(i) / static_cast<autd3::driver::autd3_float_t>(size);
    return autd3::FocusSTM::Focus(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
  });

  autd3::FocusSTM stm;
  std::copy(points.begin(), points.end(), std::back_inserter(stm));
  stm.set_frequency(1);

  autd << stm;
  for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

  for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
    ASSERT_FALSE(cpus->at(i).fpga().stm_start_idx().has_value());
    ASSERT_FALSE(cpus->at(i).fpga().stm_finish_idx().has_value());
  }

  const auto cycle = cpus->at(0).fpga().cycles()[0];
  const auto wavenumber = autd.geometry()[0].wavenumber(autd.geometry().sound_speed);
  const auto& base_tr = autd.geometry()[0];
  const auto& z_dir = base_tr.z_direction();
  constexpr autd3::driver::autd3_float_t criteria = 2 * autd3::pi / 100;
  for (size_t k = 0; k < size; k++) {
    const auto& focus = points[k].point;
    const auto expect = std::arg(autd3::core::propagate(base_tr.position(), z_dir, 0, wavenumber, focus)) +
                        2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(cpus->at(0).fpga().drives(k).second[0].phase) /
                            static_cast<autd3::driver::autd3_float_t>(cycle);
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
      const auto [duties, phases] = cpus->at(i).fpga().drives(k);
      for (size_t j = 0; j < autd.geometry().device_map()[i]; j++) {
        const auto p =
            std::arg(autd3::core::propagate(autd.geometry()[i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j].position(), z_dir, 0, wavenumber, focus)) +
            2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(phases[j].phase) / static_cast<autd3::driver::autd3_float_t>(cycle);
        ASSERT_EQ(duties[j].duty, cycle >> 1);
        if (autd3::driver::rem_euclid(p - expect, 2 * autd3::pi) > autd3::pi)
          ASSERT_NEAR(autd3::driver::rem_euclid(p - expect, 2 * autd3::pi), 2 * autd3::pi, criteria);
        else
          ASSERT_NEAR(autd3::driver::rem_euclid(p - expect, 2 * autd3::pi), 0, criteria);
      }
    }
  }

  stm.start_idx() = 1;
  stm.finish_idx() = 2;
  autd << stm;
  for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
    ASSERT_TRUE(cpus->at(i).fpga().stm_start_idx().has_value());
    ASSERT_EQ(cpus->at(i).fpga().stm_start_idx().value_or(0), 1);
    ASSERT_TRUE(cpus->at(i).fpga().stm_finish_idx().has_value());
    ASSERT_EQ(cpus->at(i).fpga().stm_finish_idx().value_or(0), 2);
  }

  autd << autd3::stop;
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives(0);
    for (const auto& [duty] : duties) ASSERT_EQ(duty, 0x0000);
  }

  autd.close();
}

TEST(ControllerTest, gain_stm_legacy) {
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(0, autd3::AUTD3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, autd3::AUTD3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero()))
                      .sound_speed(340.0e3)
                      .build();

  auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  autd << autd3::clear << autd3::synchronize;

  const autd3::Vector3 center = autd.geometry().center();

  {
    autd3::GainSTM stm;
    constexpr size_t size = 50;
    std::vector<std::vector<autd3::driver::Drive>> drives;
    constexpr autd3::driver::autd3_float_t radius = 30;
    std::vector<size_t> iota(size);
    std::iota(iota.begin(), iota.end(), 0);
    std::for_each(iota.begin(), iota.end(), [&](const size_t i) {
      const auto theta = 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(i) / static_cast<autd3::driver::autd3_float_t>(size);
      autd3::gain::Focus f(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
      drives.emplace_back(f.calc(autd.geometry()));
      stm.add(f);
    });

    autd << stm;
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

    for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
      ASSERT_FALSE(cpus->at(i).fpga().stm_start_idx().has_value());
      ASSERT_FALSE(cpus->at(i).fpga().stm_finish_idx().has_value());
    }

    for (size_t k = 0; k < size; k++) {
      for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
        const auto [duties, phases] = cpus->at(i).fpga().drives(k);
        for (size_t j = 0; j < autd.geometry().device_map()[i]; j++) {
          ASSERT_EQ(duties[j].duty, (autd3::driver::LegacyDrive::to_duty(drives[k][i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j]) << 3) + 0x08);
          ASSERT_EQ(phases[j].phase, autd3::driver::LegacyDrive::to_phase(drives[k][i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j]) << 4);
        }
      }
    }
  }

  {
    autd3::GainSTM stm;
    stm.mode() = autd3::GainSTMMode::PhaseFull;
    constexpr size_t size = 50;
    std::vector<std::vector<autd3::driver::Drive>> drives;
    constexpr autd3::driver::autd3_float_t radius = 40;
    std::vector<size_t> iota(size);
    std::iota(iota.begin(), iota.end(), 0);
    std::for_each(iota.begin(), iota.end(), [&](const size_t i) {
      const auto theta = 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(i) / static_cast<autd3::driver::autd3_float_t>(size);
      autd3::gain::Focus f(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
      drives.emplace_back(f.calc(autd.geometry()));
      stm.add(f);
    });

    stm.start_idx() = 1;
    stm.finish_idx() = 2;

    autd << stm;
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

    for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
      ASSERT_TRUE(cpus->at(i).fpga().stm_start_idx().has_value());
      ASSERT_EQ(cpus->at(i).fpga().stm_start_idx().value_or(0), 1);
      ASSERT_TRUE(cpus->at(i).fpga().stm_finish_idx().has_value());
      ASSERT_EQ(cpus->at(i).fpga().stm_finish_idx().value_or(0), 2);
    }

    const uint16_t cycle = autd.geometry()[0].cycle();
    for (size_t k = 0; k < size; k++) {
      for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
        const auto [duties, phases] = cpus->at(i).fpga().drives(k);
        for (size_t j = 0; j < autd.geometry().device_map()[i]; j++) {
          ASSERT_EQ(duties[j].duty, cycle >> 1);
          ASSERT_EQ(phases[j].phase, autd3::driver::LegacyDrive::to_phase(drives[k][i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j]) << 4);
        }
      }
    }
  }

  {
    autd3::GainSTM stm;
    stm.mode() = autd3::GainSTMMode::PhaseHalf;
    constexpr size_t size = 50;
    std::vector<std::vector<autd3::driver::Drive>> drives;
    constexpr autd3::driver::autd3_float_t radius = 40;
    std::vector<size_t> iota(size);
    std::iota(iota.begin(), iota.end(), 0);
    std::for_each(iota.begin(), iota.end(), [&](const size_t i) {
      const auto theta = 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(i) / static_cast<autd3::driver::autd3_float_t>(size);
      autd3::gain::Focus f(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
      drives.emplace_back(f.calc(autd.geometry()));
      stm.add(f);
    });

    autd << stm;
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

    for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
      ASSERT_FALSE(cpus->at(i).fpga().stm_start_idx().has_value());
      ASSERT_FALSE(cpus->at(i).fpga().stm_finish_idx().has_value());
    }

    const uint16_t cycle = autd.geometry()[0].cycle();
    for (size_t k = 0; k < size; k++) {
      for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
        const auto [duties, phases] = cpus->at(i).fpga().drives(k);
        for (size_t j = 0; j < autd.geometry().device_map()[i]; j++) {
          ASSERT_EQ(duties[j].duty, cycle >> 1);
          const auto legacy_phase = autd3::driver::LegacyDrive::to_phase(drives[k][i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j]) >> 4;
          auto phase = legacy_phase << 4;
          phase += legacy_phase;
          phase <<= 4;
          ASSERT_EQ(phases[j].phase, phase);
        }
      }
    }
  }

  autd << autd3::stop;
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives(0);
    for (const auto& [duty] : duties) ASSERT_EQ(duty, 0x0000);
  }

  autd.close();
}

TEST(ControllerTest, gain_stm_normal) {
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(0, autd3::AUTD3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, autd3::AUTD3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero()))
                      .sound_speed(340.0e3)
                      .normal_mode()
                      .build();

  auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  autd << autd3::clear << autd3::synchronize;

  const autd3::Vector3 center = autd.geometry().center();

  {
    autd3::GainSTM stm;
    constexpr size_t size = 50;
    std::vector<std::vector<autd3::driver::Drive>> drives;
    constexpr autd3::driver::autd3_float_t radius = 30;
    std::vector<size_t> iota(size);
    std::iota(iota.begin(), iota.end(), 0);
    std::for_each(iota.begin(), iota.end(), [&](const size_t i) {
      const auto theta = 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(i) / static_cast<autd3::driver::autd3_float_t>(size);
      autd3::gain::Focus f(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
      drives.emplace_back(f.calc(autd.geometry()));
      stm.add(f);
    });

    autd << stm;
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

    for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
      ASSERT_FALSE(cpus->at(i).fpga().stm_start_idx().has_value());
      ASSERT_FALSE(cpus->at(i).fpga().stm_finish_idx().has_value());
    }

    const uint16_t cycle = autd.geometry()[0].cycle();
    for (size_t k = 0; k < size; k++) {
      for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
        const auto [duties, phases] = cpus->at(i).fpga().drives(k);
        for (size_t j = 0; j < autd.geometry().device_map()[i]; j++) {
          ASSERT_EQ(duties[j].duty, autd3::driver::NormalDriveDuty::to_duty(drives[k][i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j], cycle));
          ASSERT_EQ(phases[j].phase, autd3::driver::NormalDrivePhase::to_phase(drives[k][i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j], cycle));
        }
      }
    }
  }

  {
    autd3::GainSTM stm;
    stm.mode() = autd3::GainSTMMode::PhaseFull;
    constexpr size_t size = 50;
    std::vector<std::vector<autd3::driver::Drive>> drives;
    constexpr autd3::driver::autd3_float_t radius = 40;
    std::vector<size_t> iota(size);
    std::iota(iota.begin(), iota.end(), 0);
    std::for_each(iota.begin(), iota.end(), [&](const size_t i) {
      const auto theta = 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(i) / static_cast<autd3::driver::autd3_float_t>(size);
      autd3::gain::Focus f(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
      drives.emplace_back(f.calc(autd.geometry()));
      stm.add(f);
    });

    stm.start_idx() = 2;
    stm.finish_idx() = 1;

    autd << stm;
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

    for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
      ASSERT_TRUE(cpus->at(i).fpga().stm_start_idx().has_value());
      ASSERT_EQ(cpus->at(i).fpga().stm_start_idx().value_or(0), 2);
      ASSERT_TRUE(cpus->at(i).fpga().stm_finish_idx().has_value());
      ASSERT_EQ(cpus->at(i).fpga().stm_finish_idx().value_or(0), 1);
    }

    const uint16_t cycle = autd.geometry()[0].cycle();
    for (size_t k = 0; k < size; k++) {
      for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
        const auto [duties, phases] = cpus->at(i).fpga().drives(k);
        for (size_t j = 0; j < autd.geometry().device_map()[i]; j++) {
          ASSERT_EQ(duties[j].duty, cycle >> 1);
          ASSERT_EQ(phases[j].phase, autd3::driver::NormalDrivePhase::to_phase(drives[k][i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j], cycle));
        }
      }
    }
  }

  autd << autd3::stop;
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives(0);
    for (const auto& [duty] : duties) ASSERT_EQ(duty, 0x0000);
  }

  autd.close();
}

TEST(ControllerTest, gain_stm_normal_phase) {
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(0, autd3::AUTD3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, autd3::AUTD3::DEVICE_HEIGHT, 0), autd3::Vector3::Zero()))
                      .sound_speed(340.0e3)
                      .normal_phase_mode()
                      .build();

  auto cpus = std::make_shared<std::vector<autd3::extra::CPU>>();

  auto link = autd3::test::EmulatorLink(cpus).build();
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  autd << autd3::clear << autd3::synchronize;

  autd << autd3::Amplitudes(1.0);

  const autd3::Vector3 center = autd.geometry().center();
  {
    autd3::GainSTM stm;
    constexpr size_t size = 50;
    std::vector<std::vector<autd3::driver::Drive>> drives;
    constexpr autd3::driver::autd3_float_t radius = 30;
    std::vector<size_t> iota(size);
    std::iota(iota.begin(), iota.end(), 0);
    std::for_each(iota.begin(), iota.end(), [&](const size_t i) {
      const auto theta = 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(i) / static_cast<autd3::driver::autd3_float_t>(size);
      autd3::gain::Focus f(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
      drives.emplace_back(f.calc(autd.geometry()));
      stm.add(f);
    });

    autd << stm;
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

    for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
      ASSERT_FALSE(cpus->at(i).fpga().stm_start_idx().has_value());
      ASSERT_FALSE(cpus->at(i).fpga().stm_finish_idx().has_value());
    }

    const uint16_t cycle = autd.geometry()[0].cycle();
    for (size_t k = 0; k < size; k++) {
      for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
        const auto [duties, phases] = cpus->at(i).fpga().drives(k);
        for (size_t j = 0; j < autd.geometry().device_map()[i]; j++) {
          ASSERT_EQ(duties[j].duty, cycle >> 1);
          ASSERT_EQ(phases[j].phase, autd3::driver::NormalDrivePhase::to_phase(drives[k][i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j], cycle));
        }
      }
    }
  }

  {
    autd3::GainSTM stm;
    stm.mode() = autd3::GainSTMMode::PhaseFull;
    constexpr size_t size = 50;
    std::vector<std::vector<autd3::driver::Drive>> drives;
    constexpr autd3::driver::autd3_float_t radius = 30;
    std::vector<size_t> iota(size);
    std::iota(iota.begin(), iota.end(), 0);
    std::for_each(iota.begin(), iota.end(), [&](const size_t i) {
      const auto theta = 2 * autd3::pi * static_cast<autd3::driver::autd3_float_t>(i) / static_cast<autd3::driver::autd3_float_t>(size);
      autd3::gain::Focus f(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
      drives.emplace_back(f.calc(autd.geometry()));
      stm.add(f);
    });

    stm.start_idx() = 0;
    stm.finish_idx() = 0;

    autd << stm;
    for (size_t i = 0; i < autd.geometry().num_devices(); i++) ASSERT_EQ(cpus->at(i).fpga().stm_cycle(), size);

    for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
      ASSERT_TRUE(cpus->at(i).fpga().stm_start_idx().has_value());
      ASSERT_EQ(cpus->at(i).fpga().stm_start_idx().value_or(1), 0);
      ASSERT_TRUE(cpus->at(i).fpga().stm_finish_idx().has_value());
      ASSERT_EQ(cpus->at(i).fpga().stm_finish_idx().value_or(1), 0);
    }

    const uint16_t cycle = autd.geometry()[0].cycle();
    for (size_t k = 0; k < size; k++) {
      for (size_t i = 0; i < autd.geometry().num_devices(); i++) {
        const auto [duties, phases] = cpus->at(i).fpga().drives(k);
        for (size_t j = 0; j < autd.geometry().device_map()[i]; j++) {
          ASSERT_EQ(duties[j].duty, cycle >> 1);
          ASSERT_EQ(phases[j].phase, autd3::driver::NormalDrivePhase::to_phase(drives[k][i * autd3::AUTD3::NUM_TRANS_IN_UNIT + j], cycle));
        }
      }
    }
  }

  autd << autd3::stop;
  for (const auto& cpu : *cpus) {
    const auto [duties, phases] = cpu.fpga().drives(0);
    for (const auto& [duty] : duties) ASSERT_EQ(duty, 0x0000);
  }

  autd.close();
}

int main(int argc, char** argv) {
  testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
