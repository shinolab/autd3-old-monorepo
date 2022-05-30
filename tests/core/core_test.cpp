// File: core_test.cpp
// Project: core
// Created Date: 24/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
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

#include <autd3/core/geometry/device.hpp>
#include <autd3/core/geometry/dynamic_transducer.hpp>
#include <autd3/core/geometry/geometry.hpp>
#include <autd3/core/geometry/legacy_transducer.hpp>
#include <autd3/core/geometry/normal_transducer.hpp>
#include <random>

#include "test_utils.hpp"

using autd3::core::Quaternion;
using autd3::core::Vector3;
using autd3::driver::CPUControlFlags;
using autd3::driver::FPGAControlFlags;

TEST(TransducerTest, LegacyTransducer) {
  autd3::core::LegacyTransducer tr(1, Vector3(10, 20, 30), Vector3(1, 2, 3), Vector3(4, 5, 6), Vector3(7, 8, 9));

  ASSERT_NEAR_VECTOR3(tr.position(), Vector3(10, 20, 30), 1e-3);
  ASSERT_NEAR_VECTOR3(tr.x_direction(), Vector3(1, 2, 3), 1e-3);
  ASSERT_NEAR_VECTOR3(tr.y_direction(), Vector3(4, 5, 6), 1e-3);
  ASSERT_NEAR_VECTOR3(tr.z_direction(), Vector3(7, 8, 9), 1e-3);

  ASSERT_EQ(tr.id(), 1);

  ASSERT_EQ(tr.cycle(), 4096);
  ASSERT_NEAR(tr.frequency(), 40e3, 1e-3);

  ASSERT_NEAR(tr.wavelength(340), 8.5, 1e-3);
  ASSERT_NEAR(tr.wavenumber(340), 0.7391982714328924978818737573L, 1e-3);

  autd3::driver::TxDatagram tx(10);
  autd3::core::LegacyTransducer::pack_header(tx);
  ASSERT_EQ((tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY), 0);
  ASSERT_NE((tx.header().fpga_flag.value() & FPGAControlFlags::LEGACY_MODE), 0);
  ASSERT_EQ((tx.header().fpga_flag.value() & FPGAControlFlags::STM_MODE), 0);
  ASSERT_EQ(tx.num_bodies, 0);

  bool phase_sent = false;
  bool duty_sent = false;
  autd3::core::LegacyDriveData<autd3::core::LegacyTransducer> data;
  data.init(10 * autd3::driver::NUM_TRANS_IN_UNIT);
  autd3::core::LegacyTransducer::pack_body(phase_sent, duty_sent, data, tx);
  ASSERT_EQ(phase_sent, true);
  ASSERT_EQ(duty_sent, true);
}
TEST(TransducerTest, NormalTransducer) {
  autd3::core::NormalTransducer tr(1, Vector3(10, 20, 30), Vector3(1, 2, 3), Vector3(4, 5, 6), Vector3(7, 8, 9));

  ASSERT_NEAR_VECTOR3(tr.position(), Vector3(10, 20, 30), 1e-3);
  ASSERT_NEAR_VECTOR3(tr.x_direction(), Vector3(1, 2, 3), 1e-3);
  ASSERT_NEAR_VECTOR3(tr.y_direction(), Vector3(4, 5, 6), 1e-3);
  ASSERT_NEAR_VECTOR3(tr.z_direction(), Vector3(7, 8, 9), 1e-3);

  ASSERT_EQ(tr.id(), 1);

  tr.set_cycle(3000);
  ASSERT_EQ(tr.cycle(), 3000);
  tr.set_frequency(70e3);
  ASSERT_NEAR(tr.frequency(), 70e3, 15.0);

  ASSERT_NEAR(tr.wavelength(340), 4.857142857142857142857142857L, 1e-3);
  ASSERT_NEAR(tr.wavenumber(340), 1.293596975007561871293279075L, 1e-3);

  autd3::driver::TxDatagram tx(10);
  autd3::core::NormalTransducer::pack_header(tx);
  ASSERT_EQ((tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY), 0);
  ASSERT_EQ((tx.header().fpga_flag.value() & FPGAControlFlags::LEGACY_MODE), 0);
  ASSERT_EQ((tx.header().fpga_flag.value() & FPGAControlFlags::STM_MODE), 0);
  ASSERT_EQ(tx.num_bodies, 0);

  bool phase_sent = false;
  bool duty_sent = false;
  autd3::core::NormalDriveData<autd3::core::NormalTransducer> data;
  data.init(10 * autd3::driver::NUM_TRANS_IN_UNIT);
  autd3::core::NormalTransducer::pack_body(phase_sent, duty_sent, data, tx);
  ASSERT_EQ(phase_sent, true);
  ASSERT_EQ(duty_sent, false);
  autd3::core::NormalTransducer::pack_body(phase_sent, duty_sent, data, tx);
  ASSERT_EQ(phase_sent, true);
  ASSERT_EQ(duty_sent, true);
}
TEST(TransducerTest, DynamicTransducer) {
  autd3::core::DynamicTransducer tr(1, Vector3(10, 20, 30), Vector3(1, 2, 3), Vector3(4, 5, 6), Vector3(7, 8, 9));
  autd3::core::DynamicTransducer::legacy_mode() = true;

  ASSERT_NEAR_VECTOR3(tr.position(), Vector3(10, 20, 30), 1e-3);
  ASSERT_NEAR_VECTOR3(tr.x_direction(), Vector3(1, 2, 3), 1e-3);
  ASSERT_NEAR_VECTOR3(tr.y_direction(), Vector3(4, 5, 6), 1e-3);
  ASSERT_NEAR_VECTOR3(tr.z_direction(), Vector3(7, 8, 9), 1e-3);

  ASSERT_EQ(tr.id(), 1);

  ASSERT_EQ(tr.cycle(), 4096);
  ASSERT_NEAR(tr.frequency(), 40e3, 1e-3);

  ASSERT_NEAR(tr.wavelength(340), 8.5, 1e-3);
  ASSERT_NEAR(tr.wavenumber(340), 0.7391982714328924978818737573L, 1e-3);

  autd3::driver::TxDatagram tx(10);
  autd3::core::DynamicTransducer::pack_header(tx);
  ASSERT_EQ((tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY), 0);
  ASSERT_NE((tx.header().fpga_flag.value() & FPGAControlFlags::LEGACY_MODE), 0);
  ASSERT_EQ((tx.header().fpga_flag.value() & FPGAControlFlags::STM_MODE), 0);
  ASSERT_EQ(tx.num_bodies, 0);

  bool phase_sent = false;
  bool duty_sent = false;
  autd3::core::DynamicDriveData<autd3::core::DynamicTransducer> data;
  data.init(10 * autd3::driver::NUM_TRANS_IN_UNIT);
  autd3::core::DynamicTransducer::pack_body(phase_sent, duty_sent, data, tx);
  ASSERT_EQ(phase_sent, true);
  ASSERT_EQ(duty_sent, true);

  autd3::core::DynamicTransducer::legacy_mode() = false;
  ASSERT_NEAR_VECTOR3(tr.position(), Vector3(10, 20, 30), 1e-3);
  ASSERT_NEAR_VECTOR3(tr.x_direction(), Vector3(1, 2, 3), 1e-3);
  ASSERT_NEAR_VECTOR3(tr.y_direction(), Vector3(4, 5, 6), 1e-3);
  ASSERT_NEAR_VECTOR3(tr.z_direction(), Vector3(7, 8, 9), 1e-3);

  ASSERT_EQ(tr.id(), 1);

  tr.set_cycle(3000);
  ASSERT_EQ(tr.cycle(), 3000);
  tr.set_frequency(70e3);
  ASSERT_NEAR(tr.frequency(), 70e3, 15.0);

  ASSERT_NEAR(tr.wavelength(340), 4.857142857142857142857142857L, 1e-3);
  ASSERT_NEAR(tr.wavenumber(340), 1.293596975007561871293279075L, 1e-3);

  autd3::core::NormalTransducer::pack_header(tx);
  ASSERT_EQ((tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY), 0);
  ASSERT_EQ((tx.header().fpga_flag.value() & FPGAControlFlags::LEGACY_MODE), 0);
  ASSERT_EQ((tx.header().fpga_flag.value() & FPGAControlFlags::STM_MODE), 0);
  ASSERT_EQ(tx.num_bodies, 0);

  phase_sent = false;
  duty_sent = false;
  data.init(10 * autd3::driver::NUM_TRANS_IN_UNIT);
  autd3::core::DynamicTransducer::pack_body(phase_sent, duty_sent, data, tx);
  ASSERT_EQ(phase_sent, true);
  ASSERT_EQ(duty_sent, false);
  autd3::core::DynamicTransducer::pack_body(phase_sent, duty_sent, data, tx);
  ASSERT_EQ(phase_sent, true);
  ASSERT_EQ(duty_sent, true);
}

TEST(DeviceTest, center) {
  const autd3::core::Device<autd3::core::LegacyTransducer> device(0, Vector3(10, 20, 30), Quaternion::Identity());

  Vector3 expect = Vector3::Zero();
  for (size_t i = 0; i < 18; i++) {
    for (size_t j = 0; j < 14; j++) {
      if (autd3::driver::is_missing_transducer(i, j)) continue;
      expect += 10.16 * Vector3(static_cast<double>(i), static_cast<double>(j), 0.0) + Vector3(10, 20, 30);
    }
  }
  expect /= 249;
  ASSERT_NEAR_VECTOR3(device.center(), expect, 1e-3);
}

TEST(DeviceTest, to_local) {
  const autd3::core::Device<autd3::core::LegacyTransducer> device0(0, Vector3(10, 20, 30), Quaternion::Identity());
  ASSERT_NEAR_VECTOR3(device0.to_local_position(Vector3(10, 20, 30)), Vector3::Zero(), 1e-3);

  const autd3::core::Device<autd3::core::LegacyTransducer> device1(1, Vector3(0, 0, 0),
                                                                   Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi, Vector3::UnitX()));
  ASSERT_NEAR_VECTOR3(device1.to_local_position(Vector3(10, 20, 30)), Vector3(10, -20, -30), 1e-3);

  const autd3::core::Device<autd3::core::LegacyTransducer> device2(1, Vector3(0, 0, 0),
                                                                   Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi, Vector3::UnitY()));
  ASSERT_NEAR_VECTOR3(device2.to_local_position(Vector3(10, 20, 30)), Vector3(-10, 20, -30), 1e-3);

  const autd3::core::Device<autd3::core::LegacyTransducer> device3(1, Vector3(0, 0, 0),
                                                                   Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi, Vector3::UnitZ()));
  ASSERT_NEAR_VECTOR3(device3.to_local_position(Vector3(10, 20, 30)), Vector3(-10, -20, 30), 1e-3);

  const autd3::core::Device<autd3::core::LegacyTransducer> device4(
      1, Vector3(40, 60, 50), Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi / 2, Vector3::UnitZ()));
  ASSERT_NEAR_VECTOR3(device4.to_local_position(Vector3(10, 20, 30)), Vector3(-40, 30, -20), 1e-3);
}

TEST(GeometryTest, num_devices) {
  autd3::core::Geometry<autd3::core::LegacyTransducer> geometry;
  ASSERT_EQ(geometry.num_devices(), 0);
  geometry.add_device(Vector3::Zero(), Vector3::Zero());
  ASSERT_EQ(geometry.num_devices(), 1);
  geometry.add_device(Vector3::Zero(), Vector3::Zero());
  ASSERT_EQ(geometry.num_devices(), 2);
}

TEST(GeometryTest, num_transducers) {
  autd3::core::Geometry<autd3::core::LegacyTransducer> geometry;
  ASSERT_EQ(geometry.num_transducers(), 0);
  geometry.add_device(Vector3::Zero(), Vector3::Zero());
  ASSERT_EQ(geometry.num_transducers(), 249);
  geometry.add_device(Vector3::Zero(), Vector3::Zero());
  ASSERT_EQ(geometry.num_transducers(), 249 * 2);
}

TEST(GeometryTest, center) {
  autd3::core::Geometry<autd3::core::LegacyTransducer> geometry;
  Vector3 expect = Vector3::Zero();
  ASSERT_NEAR_VECTOR3(geometry.center(), expect, 1e-3);

  geometry.add_device(Vector3(10, 20, 30), Vector3::Zero());
  for (size_t i = 0; i < 18; i++) {
    for (size_t j = 0; j < 14; j++) {
      if (autd3::driver::is_missing_transducer(i, j)) continue;
      expect += 10.16 * Vector3(static_cast<double>(i), static_cast<double>(j), 0.0) + Vector3(10, 20, 30);
    }
  }
  expect /= 249;
  ASSERT_NEAR_VECTOR3(geometry.center(), expect, 1e-3);
}

TEST(GeometryTest, add_device) {
  autd3::core::Geometry geometry;

  geometry.add_device(Vector3(10, 20, 30), Vector3::Zero());
  geometry.add_device(Vector3(0, 0, 0), Vector3(autd3::driver::pi, autd3::driver::pi, 0));
  geometry.add_device(Vector3(0, 0, 0), Vector3(0, autd3::driver::pi, 0));
  geometry.add_device(Vector3(0, 0, 0), Vector3(autd3::driver::pi, 0, 0));
  geometry.add_device(Vector3(40, 60, 50), Vector3(0, 0, autd3::driver::pi / 2));

  const Vector3 origin(0, 0, 0);
  const Vector3 right_bottom(10.16 * 17, 0, 0);
  const Vector3 left_top(0, 10.16 * 13, 0);

  ASSERT_NEAR_VECTOR3(geometry[0][0].position(), (Vector3(10, 20, 30) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[0][17].position(), (Vector3(10, 20, 30) + right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[0][231].position(), (Vector3(10, 20, 30) + left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[0][248].position(), (Vector3(10, 20, 30) + right_bottom + left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[1][0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1][17].position(), (Vector3(0, 0, 0) + right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1][231].position(), (Vector3(0, 0, 0) - left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1][248].position(), (Vector3(0, 0, 0) + right_bottom - left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[2][0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2][17].position(), (Vector3(0, 0, 0) - right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2][231].position(), (Vector3(0, 0, 0) + left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2][248].position(), (Vector3(0, 0, 0) - right_bottom + left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[3][0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3][17].position(), (Vector3(0, 0, 0) - right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3][231].position(), (Vector3(0, 0, 0) - left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3][248].position(), (Vector3(0, 0, 0) - right_bottom - left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[4][0].position(), (Vector3(40, 60, 50) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4][17].position(), (Vector3(40, 60, 50) + Vector3(0, 10.16 * 17, 0)), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4][231].position(), (Vector3(40, 60, 50) - Vector3(10.16 * 13, 0, 0)), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4][248].position(), (Vector3(40, 60, 50) + Vector3(0, 10.16 * 17, 0) - Vector3(10.16 * 13, 0, 0)), 1e-3);
}

TEST(GeometryTest, add_device_quaternion) {
  autd3::core::Geometry geometry;

  geometry.add_device(Vector3(10, 20, 30), Quaternion::Identity());
  geometry.add_device(Vector3(0, 0, 0), Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi, Vector3::UnitX()));
  geometry.add_device(Vector3(0, 0, 0), Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi, Vector3::UnitY()));
  geometry.add_device(Vector3(0, 0, 0), Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi, Vector3::UnitZ()));
  geometry.add_device(Vector3(40, 60, 50), Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi / 2, Vector3::UnitZ()));

  const Vector3 origin(0, 0, 0);
  const Vector3 right_bottom(10.16 * 17, 0, 0);
  const Vector3 left_top(0, 10.16 * 13, 0);

  ASSERT_NEAR_VECTOR3(geometry[0][0].position(), (Vector3(10, 20, 30) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[0][17].position(), (Vector3(10, 20, 30) + right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[0][231].position(), (Vector3(10, 20, 30) + left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[0][248].position(), (Vector3(10, 20, 30) + right_bottom + left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[1][0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1][17].position(), (Vector3(0, 0, 0) + right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1][231].position(), (Vector3(0, 0, 0) - left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1][248].position(), (Vector3(0, 0, 0) + right_bottom - left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[2][0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2][17].position(), (Vector3(0, 0, 0) - right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2][231].position(), (Vector3(0, 0, 0) + left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2][248].position(), (Vector3(0, 0, 0) - right_bottom + left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[3][0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3][17].position(), (Vector3(0, 0, 0) - right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3][231].position(), (Vector3(0, 0, 0) - left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3][248].position(), (Vector3(0, 0, 0) - right_bottom - left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[4][0].position(), (Vector3(40, 60, 50) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4][17].position(), (Vector3(40, 60, 50) + Vector3(0, 10.16 * 17, 0)), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4][231].position(), (Vector3(40, 60, 50) - Vector3(10.16 * 13, 0, 0)), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4][248].position(), (Vector3(40, 60, 50) + Vector3(0, 10.16 * 17, 0) - Vector3(10.16 * 13, 0, 0)), 1e-3);
}

TEST(UtilitiesTest, Directivity) {
  constexpr double expects[91] = {
      1,        1,        1,        1,        1,        1,        1,        1,        1,        1,        1,        1,        1,
      1,        1,        1,        1,        1,        1,        1,        1,        0.994632, 0.987783, 0.979551, 0.970031, 0.95932,
      0.947513, 0.934707, 0.920997, 0.906479, 0.891251, 0.875394, 0.85894,  0.841907, 0.824312, 0.806173, 0.787508, 0.768335, 0.748672,
      0.728536, 0.707946, 0.686939, 0.665635, 0.644172, 0.622691, 0.601329, 0.580226, 0.559521, 0.539353, 0.519863, 0.501187, 0.483432,
      0.466559, 0.450499, 0.435179, 0.420529, 0.406476, 0.392949, 0.379878, 0.367189, 0.354813, 0.342697, 0.330862, 0.319348, 0.308198,
      0.297451, 0.287148, 0.277329, 0.268036, 0.259309, 0.251189, 0.243703, 0.236828, 0.230529, 0.22477,  0.219514, 0.214725, 0.210368,
      0.206407, 0.202805, 0.199526, 0.196537, 0.193806, 0.191306, 0.189007, 0.18688,  0.184898, 0.183031, 0.18125,  0.179526, 0.177831};

  for (size_t i = 0; i < 91; i++) ASSERT_NEAR(autd3::core::Directivity::t4010a1(static_cast<double>(i)), expects[i], 1e-3);
}

TEST(UtilitiesTest, propagate) {
  constexpr auto wavenumber = 2.0 * autd3::driver::pi / 2.0;  // lambda = 2.0

  ASSERT_NEAR_COMPLEX(autd3::core::propagate(Vector3::Zero(), Vector3::UnitZ(), 0.0, wavenumber, Vector3(0.0, 0.0, 1.0)), std::complex(-1.0, 0.0),
                      1e-3);

  ASSERT_NEAR_COMPLEX(autd3::core::propagate(Vector3::Zero(), Vector3::UnitZ(), 0.0, wavenumber, Vector3(0.0, 0.0, 2.0)), std::complex(0.5, 0.0),
                      1e-3);

  ASSERT_NEAR_COMPLEX(autd3::core::propagate(Vector3::Zero(), Vector3::UnitZ(), 0.0, wavenumber, Vector3(1.0, 0.0, 0.0)),
                      std::complex(-0.177831, 0.0), 1e-3);

  ASSERT_NEAR_COMPLEX(autd3::core::propagate(Vector3::Zero(), Vector3::UnitZ(), 0.0, wavenumber, Vector3(0.0, 1.0, 0.0)),
                      std::complex(-0.177831, 0.0), 1e-3);

  ASSERT_NEAR_COMPLEX(autd3::core::propagate(Vector3::Zero(), Vector3::UnitX(), 0.0, wavenumber, Vector3(1.0, 0.0, 0.0)), std::complex(-1.0, 0.0),
                      1e-3);
}
