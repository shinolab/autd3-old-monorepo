// File: core_test.cpp
// Project: core
// Created Date: 24/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2022
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
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY) == 0);
  ASSERT_TRUE((tx.header().fpga_flag.value() & FPGAControlFlags::LEGACY_MODE) != 0);
  ASSERT_TRUE((tx.header().fpga_flag.value() & FPGAControlFlags::STM_MODE) == 0);
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
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY) == 0);
  ASSERT_TRUE((tx.header().fpga_flag.value() & FPGAControlFlags::LEGACY_MODE) == 0);
  ASSERT_TRUE((tx.header().fpga_flag.value() & FPGAControlFlags::STM_MODE) == 0);
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
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY) == 0);
  ASSERT_TRUE((tx.header().fpga_flag.value() & FPGAControlFlags::LEGACY_MODE) != 0);
  ASSERT_TRUE((tx.header().fpga_flag.value() & FPGAControlFlags::STM_MODE) == 0);
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
  ASSERT_TRUE((tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY) == 0);
  ASSERT_TRUE((tx.header().fpga_flag.value() & FPGAControlFlags::LEGACY_MODE) == 0);
  ASSERT_TRUE((tx.header().fpga_flag.value() & FPGAControlFlags::STM_MODE) == 0);
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
      expect += 10.16 * Vector3(i, j, 0.0) + Vector3(10, 20, 30);
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
      expect += 10.16 * Vector3(i, j, 0.0) + Vector3(10, 20, 30);
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
