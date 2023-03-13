// File: geometry.cpp
// Project: core
// Created Date: 29/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 14/03/2023
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

#include "autd3/autd3_device.hpp"
#include "test_utils.hpp"

using Vector3 = autd3::core::Vector3;
using Quaternion = autd3::core::Quaternion;
using Affine3 = autd3::core::Affine3;
using Matrix3X3 = autd3::core::Matrix3X3;

TEST(GeometryTest, num_transducers) {
  {
    const auto geometry = autd3::core::Geometry::Builder().add_device(autd3::AUTD3(Vector3::Zero(), Vector3::Zero())).build();
    ASSERT_EQ(geometry.num_transducers(), 249);
  }

  {
    const auto geometry = autd3::core::Geometry::Builder()
                              .add_device(autd3::AUTD3(Vector3::Zero(), Vector3::Zero()))
                              .add_device(autd3::AUTD3(Vector3::Zero(), Vector3::Zero()))
                              .build();
    ASSERT_EQ(geometry.num_transducers(), 249 * 2);
  }
}

TEST(GeometryTest, center) {
  const auto geometry = autd3::core::Geometry::Builder().add_device(autd3::AUTD3(Vector3(10, 20, 30), Vector3::Zero())).build();

  Vector3 expect = Vector3::Zero();
  for (size_t i = 0; i < 18; i++) {
    for (size_t j = 0; j < 14; j++) {
      if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
      expect += 10.16 * Vector3(static_cast<double>(i), static_cast<double>(j), 0.0) + Vector3(10, 20, 30);
    }
  }
  expect /= 249;
  ASSERT_NEAR_VECTOR3(geometry.center(), expect, 1e-3);
}

TEST(GeometryTest, center_of) {
  const auto geometry = autd3::core::Geometry::Builder()
                            .add_device(autd3::AUTD3(Vector3(10, 20, 30), Vector3::Zero()))
                            .add_device(autd3::AUTD3(Vector3(40, 50, 60), Vector3::Zero()))
                            .build();

  Vector3 expect_0 = Vector3::Zero();
  Vector3 expect_1 = Vector3::Zero();
  for (size_t i = 0; i < 18; i++) {
    for (size_t j = 0; j < 14; j++) {
      if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
      expect_0 += 10.16 * Vector3(static_cast<double>(i), static_cast<double>(j), 0.0) + Vector3(10, 20, 30);
    }
  }
  expect_0 /= 249;
  for (size_t i = 0; i < 18; i++) {
    for (size_t j = 0; j < 14; j++) {
      if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
      expect_1 += 10.16 * Vector3(static_cast<double>(i), static_cast<double>(j), 0.0) + Vector3(40, 50, 60);
    }
  }
  expect_1 /= 249;
  ASSERT_NEAR_VECTOR3(geometry.center_of(0), expect_0, 1e-3);
  ASSERT_NEAR_VECTOR3(geometry.center_of(1), expect_1, 1e-3);
}

TEST(GeometryTest, add_device) {
  const auto geometry = autd3::core::Geometry::Builder()
                            .add_device(autd3::AUTD3(Vector3(10, 20, 30), Vector3::Zero()))
                            .add_device(autd3::AUTD3(Vector3(0, 0, 0), Vector3(autd3::driver::pi, autd3::driver::pi, 0)))
                            .add_device(autd3::AUTD3(Vector3(0, 0, 0), Vector3(0, autd3::driver::pi, 0)))
                            .add_device(autd3::AUTD3(Vector3(0, 0, 0), Vector3(autd3::driver::pi, 0, 0)))
                            .add_device(autd3::AUTD3(Vector3(40, 60, 50), Vector3(0, 0, autd3::driver::pi / 2)))
                            .build();

  const Vector3 origin(0, 0, 0);
  const Vector3 right_bottom(10.16 * 17, 0, 0);
  const Vector3 left_top(0, 10.16 * 13, 0);

  ASSERT_NEAR_VECTOR3(geometry[0].position(), (Vector3(10, 20, 30) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[17].position(), (Vector3(10, 20, 30) + right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[231].position(), (Vector3(10, 20, 30) + left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[248].position(), (Vector3(10, 20, 30) + right_bottom + left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(0, 0, 0) + right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(0, 0, 0) - left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(), (Vector3(0, 0, 0) + right_bottom - left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(0, 0, 0) - right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(0, 0, 0) + left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(), (Vector3(0, 0, 0) - right_bottom + left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(0, 0, 0) - right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(0, 0, 0) - left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(), (Vector3(0, 0, 0) - right_bottom - left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(40, 60, 50) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(40, 60, 50) + Vector3(0, 10.16 * 17, 0)), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(40, 60, 50) - Vector3(10.16 * 13, 0, 0)), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(),
                      (Vector3(40, 60, 50) + Vector3(0, 10.16 * 17, 0) - Vector3(10.16 * 13, 0, 0)), 1e-3);
}

TEST(GeometryTest, add_device_quaternion) {
  const auto geometry =
      autd3::core::Geometry::Builder()
          .add_device(autd3::AUTD3(Vector3(10, 20, 30), Quaternion::Identity()))
          .add_device(autd3::AUTD3(Vector3(0, 0, 0), Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi, Vector3::UnitX())))
          .add_device(autd3::AUTD3(Vector3(0, 0, 0), Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi, Vector3::UnitY())))
          .add_device(autd3::AUTD3(Vector3(0, 0, 0), Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi, Vector3::UnitZ())))
          .add_device(autd3::AUTD3(Vector3(40, 60, 50), Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi / 2, Vector3::UnitZ())))
          .build();

  const Vector3 origin(0, 0, 0);
  const Vector3 right_bottom(10.16 * 17, 0, 0);
  const Vector3 left_top(0, 10.16 * 13, 0);

  ASSERT_NEAR_VECTOR3(geometry[0 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(10, 20, 30) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[0 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(10, 20, 30) + right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[0 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(10, 20, 30) + left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[0 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(), (Vector3(10, 20, 30) + right_bottom + left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(0, 0, 0) + right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(0, 0, 0) - left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(), (Vector3(0, 0, 0) + right_bottom - left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(0, 0, 0) - right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(0, 0, 0) + left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(), (Vector3(0, 0, 0) - right_bottom + left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(0, 0, 0) - right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(0, 0, 0) - left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(), (Vector3(0, 0, 0) - right_bottom - left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(40, 60, 50) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(40, 60, 50) + Vector3(0, 10.16 * 17, 0)), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(40, 60, 50) - Vector3(10.16 * 13, 0, 0)), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(),
                      (Vector3(40, 60, 50) + Vector3(0, 10.16 * 17, 0) - Vector3(10.16 * 13, 0, 0)), 1e-3);
}

TEST(GeometryTest, translate) {
  auto geometry = autd3::core::Geometry::Builder()
                      .add_device(autd3::AUTD3(Vector3(0, 0, 0), Quaternion::Identity()))
                      .add_device(autd3::AUTD3(Vector3(10, 20, 30), Quaternion::Identity()))
                      .build();

  const Vector3 t(40, 50, 60);
  geometry.translate(t);

  size_t tr_idx = 0;
  for (size_t j = 0; j < 14; j++) {
    for (size_t i = 0; i < 18; i++) {
      if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
      const Vector3 expect = 10.16 * Vector3(static_cast<double>(i), static_cast<double>(j), 0.0) + t;
      ASSERT_NEAR_VECTOR3(expect, geometry[tr_idx].position(), 1e-3);
      tr_idx++;
    }
  }
  for (size_t j = 0; j < 14; j++) {
    for (size_t i = 0; i < 18; i++) {
      if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
      const Vector3 expect = 10.16 * Vector3(static_cast<double>(i), static_cast<double>(j), 0.0) + Vector3(10, 20, 30) + t;
      ASSERT_NEAR_VECTOR3(expect, geometry[tr_idx].position(), 1e-3);
      tr_idx++;
    }
  }
}

TEST(GeometryTest, translate_of) {
  auto geometry = autd3::core::Geometry::Builder()
                      .add_device(autd3::AUTD3(Vector3(0, 0, 0), Quaternion::Identity()))
                      .add_device(autd3::AUTD3(Vector3(10, 20, 30), Quaternion::Identity()))
                      .build();

  const Vector3 t(40, 50, 60);
  geometry.translate(0, t);

  size_t tr_idx = 0;
  for (size_t j = 0; j < 14; j++) {
    for (size_t i = 0; i < 18; i++) {
      if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
      const Vector3 expect = 10.16 * Vector3(static_cast<double>(i), static_cast<double>(j), 0.0) + t;
      ASSERT_NEAR_VECTOR3(expect, geometry[tr_idx].position(), 1e-3);
      tr_idx++;
    }
  }
  for (size_t j = 0; j < 14; j++) {
    for (size_t i = 0; i < 18; i++) {
      if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
      const Vector3 expect = 10.16 * Vector3(static_cast<double>(i), static_cast<double>(j), 0.0) + Vector3(10, 20, 30);
      ASSERT_NEAR_VECTOR3(expect, geometry[tr_idx].position(), 1e-3);
      tr_idx++;
    }
  }

  geometry.translate(1, t);

  tr_idx = 0;
  for (size_t j = 0; j < 14; j++) {
    for (size_t i = 0; i < 18; i++) {
      if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
      const auto expect = 10.16 * Vector3(static_cast<double>(i), static_cast<double>(j), 0.0) + t;
      ASSERT_NEAR_VECTOR3(expect, geometry[tr_idx].position(), 1e-3);
      tr_idx++;
    }
  }
  for (size_t j = 0; j < 14; j++) {
    for (size_t i = 0; i < 18; i++) {
      if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
      const auto expect = 10.16 * Vector3(static_cast<double>(i), static_cast<double>(j), 0.0) + Vector3(10, 20, 30) + t;
      ASSERT_NEAR_VECTOR3(expect, geometry[tr_idx].position(), 1e-3);
      tr_idx++;
    }
  }
}

TEST(GeometryTest, rotate) {
  auto geometry = autd3::core::Geometry::Builder()
                      .add_device(autd3::AUTD3(Vector3(0, 0, 0), Quaternion::Identity()))
                      .add_device(autd3::AUTD3(Vector3(0, 0, 0), Quaternion::Identity()))
                      .build();

  const Quaternion rot = Eigen::AngleAxisd(0, Eigen::Vector3d::UnitX()) * Eigen::AngleAxisd(0, Eigen::Vector3d::UnitY()) *
                         Eigen::AngleAxisd(autd3::driver::pi / 2, Eigen::Vector3d::UnitZ());
  geometry.rotate(rot);

  const Vector3 expect_x(0, 1, 0);
  const Vector3 expect_y(-1, 0, 0);
  const Vector3 expect_z(0, 0, 1);

  for (const auto& tr : geometry) {
    ASSERT_NEAR_VECTOR3(expect_x, tr.x_direction(), 1e-3);
    ASSERT_NEAR_VECTOR3(expect_y, tr.y_direction(), 1e-3);
    ASSERT_NEAR_VECTOR3(expect_z, tr.z_direction(), 1e-3);
  }
}

TEST(GeometryTest, rotate_of) {
  auto geometry = autd3::core::Geometry::Builder()
                      .add_device(autd3::AUTD3(Vector3(0, 0, 0), Quaternion::Identity()))
                      .add_device(autd3::AUTD3(Vector3(0, 0, 0), Quaternion::Identity()))
                      .build();

  {
    const Quaternion rot = Eigen::AngleAxisd(0, Eigen::Vector3d::UnitX()) * Eigen::AngleAxisd(autd3::driver::pi / 2, Eigen::Vector3d::UnitY()) *
                           Eigen::AngleAxisd(0, Eigen::Vector3d::UnitZ());
    geometry.rotate(0, rot);

    std::for_each(geometry.begin(0), geometry.end(0), [](const auto& tr) {
      ASSERT_NEAR_VECTOR3(Vector3(0, 0, -1), tr.x_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(0, 1, 0), tr.y_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(1, 0, 0), tr.z_direction(), 1e-3);
    });

    std::for_each(geometry.begin(1), geometry.end(1), [](const auto& tr) {
      ASSERT_NEAR_VECTOR3(Vector3(1, 0, 0), tr.x_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(0, 1, 0), tr.y_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(0, 0, 1), tr.z_direction(), 1e-3);
    });
  }

  {
    const Quaternion rot = Eigen::AngleAxisd(autd3::driver::pi / 2, Eigen::Vector3d::UnitX()) * Eigen::AngleAxisd(0, Eigen::Vector3d::UnitY()) *
                           Eigen::AngleAxisd(0, Eigen::Vector3d::UnitZ());
    geometry.rotate(1, rot);

    std::for_each(geometry.begin(0), geometry.end(0), [](const auto& tr) {
      ASSERT_NEAR_VECTOR3(Vector3(0, 0, -1), tr.x_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(0, 1, 0), tr.y_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(1, 0, 0), tr.z_direction(), 1e-3);
    });

    std::for_each(geometry.begin(1), geometry.end(1), [](const auto& tr) {
      ASSERT_NEAR_VECTOR3(Vector3(1, 0, 0), tr.x_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(0, 0, 1), tr.y_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(0, -1, 0), tr.z_direction(), 1e-3);
    });
  }
}

TEST(GeometryTest, affine) {
  auto geometry = autd3::core::Geometry::Builder()
                      .add_device(autd3::AUTD3(Vector3(0, 0, 0), Quaternion::Identity()))
                      .add_device(autd3::AUTD3(Vector3(10, 20, 30), Quaternion::Identity()))
                      .build();

  const Vector3 t(40, 50, 60);

  const Quaternion rot = Eigen::AngleAxisd(0, Eigen::Vector3d::UnitX()) * Eigen::AngleAxisd(0, Eigen::Vector3d::UnitY()) *
                         Eigen::AngleAxisd(autd3::driver::pi / 2, Eigen::Vector3d::UnitZ());

  geometry.affine(t, rot);

  for (const auto& tr : geometry) {
    ASSERT_NEAR_VECTOR3(Vector3(0, 1, 0), tr.x_direction(), 1e-3);
    ASSERT_NEAR_VECTOR3(Vector3(-1, 0, 0), tr.y_direction(), 1e-3);
    ASSERT_NEAR_VECTOR3(Vector3(0, 0, 1), tr.z_direction(), 1e-3);
  }

  size_t tr_idx = 0;
  for (size_t j = 0; j < 14; j++) {
    for (size_t i = 0; i < 18; i++) {
      if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
      const Vector3 expect = 10.16 * Vector3(-static_cast<double>(j), static_cast<double>(i), 0.0) + t;
      ASSERT_NEAR_VECTOR3(expect, geometry[tr_idx].position(), 1e-3);
      tr_idx++;
    }
  }
  for (size_t j = 0; j < 14; j++) {
    for (size_t i = 0; i < 18; i++) {
      if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
      const Vector3 expect = 10.16 * Vector3(-static_cast<double>(j), static_cast<double>(i), 0.0) + Vector3(-20, 10, 30) + t;
      ASSERT_NEAR_VECTOR3(expect, geometry[tr_idx].position(), 1e-3);
      tr_idx++;
    }
  }
}

TEST(GeometryTest, affine_of) {
  auto geometry = autd3::core::Geometry::Builder()
                      .add_device(autd3::AUTD3(Vector3(0, 0, 0), Quaternion::Identity()))
                      .add_device(autd3::AUTD3(Vector3(10, 20, 30), Quaternion::Identity()))
                      .build();

  const Vector3 t(40, 50, 60);

  const Quaternion rot = Eigen::AngleAxisd(0, Eigen::Vector3d::UnitX()) * Eigen::AngleAxisd(0, Eigen::Vector3d::UnitY()) *
                         Eigen::AngleAxisd(autd3::driver::pi / 2, Eigen::Vector3d::UnitZ());

  {
    geometry.affine(0, t, rot);
    std::for_each(geometry.begin(0), geometry.end(0), [](const auto& tr) {
      ASSERT_NEAR_VECTOR3(Vector3(0, 1, 0), tr.x_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(-1, 0, 0), tr.y_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(0, 0, 1), tr.z_direction(), 1e-3);
    });
    std::for_each(geometry.begin(1), geometry.end(1), [](const auto& tr) {
      ASSERT_NEAR_VECTOR3(Vector3(1, 0, 0), tr.x_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(0, 1, 0), tr.y_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(0, 0, 1), tr.z_direction(), 1e-3);
    });
    size_t tr_idx = 0;
    for (size_t j = 0; j < 14; j++) {
      for (size_t i = 0; i < 18; i++) {
        if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
        const Vector3 expect = 10.16 * Vector3(-static_cast<double>(j), static_cast<double>(i), 0.0) + t;
        ASSERT_NEAR_VECTOR3(expect, geometry[tr_idx].position(), 1e-3);
        tr_idx++;
      }
    }
    for (size_t j = 0; j < 14; j++) {
      for (size_t i = 0; i < 18; i++) {
        if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
        const Vector3 expect = 10.16 * Vector3(static_cast<double>(i), static_cast<double>(j), 0.0) + Vector3(10, 20, 30);
        ASSERT_NEAR_VECTOR3(expect, geometry[tr_idx].position(), 1e-3);
        tr_idx++;
      }
    }
  }
  {
    geometry.affine(1, t, rot);
    std::for_each(geometry.begin(0), geometry.end(0), [](const auto& tr) {
      ASSERT_NEAR_VECTOR3(Vector3(0, 1, 0), tr.x_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(-1, 0, 0), tr.y_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(0, 0, 1), tr.z_direction(), 1e-3);
    });
    std::for_each(geometry.begin(1), geometry.end(1), [](const auto& tr) {
      ASSERT_NEAR_VECTOR3(Vector3(0, 1, 0), tr.x_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(-1, 0, 0), tr.y_direction(), 1e-3);
      ASSERT_NEAR_VECTOR3(Vector3(0, 0, 1), tr.z_direction(), 1e-3);
    });
    size_t tr_idx = 0;
    for (size_t j = 0; j < 14; j++) {
      for (size_t i = 0; i < 18; i++) {
        if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
        const Vector3 expect = 10.16 * Vector3(-static_cast<double>(j), static_cast<double>(i), 0.0) + t;
        ASSERT_NEAR_VECTOR3(expect, geometry[tr_idx].position(), 1e-3);
        tr_idx++;
      }
    }
    for (size_t j = 0; j < 14; j++) {
      for (size_t i = 0; i < 18; i++) {
        if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
        const Vector3 expect = 10.16 * Vector3(-static_cast<double>(j), static_cast<double>(i), 0.0) + Vector3(-20, 10, 30) + t;
        ASSERT_NEAR_VECTOR3(expect, geometry[tr_idx].position(), 1e-3);
        tr_idx++;
      }
    }
  }
}

TEST(GeometryTest, cycle) {
  auto geometry = autd3::core::Geometry::Builder().advanced_mode().add_device(autd3::AUTD3(Vector3::Zero(), Vector3::Zero())).build();
  {
    const auto& cycles = geometry.cycles();
    ASSERT_EQ(cycles.size(), 249);
    for (const auto cycle : cycles) ASSERT_EQ(cycle, 4096);
  }

  for (auto& tr : geometry) tr.cycle = 2000;
  {
    const auto& cycles = geometry.cycles();
    ASSERT_EQ(cycles.size(), 249);
    for (const auto cycle : cycles) ASSERT_EQ(cycle, 2000);
  }
}
