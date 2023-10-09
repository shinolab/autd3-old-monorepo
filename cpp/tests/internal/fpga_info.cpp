// File: fpga_info.cpp
// Project: internal
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 09/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/internal/datagram.hpp>
#include <autd3/internal/exception.hpp>
#include <autd3/internal/fpga_info.hpp>

#include "utils.hpp"

TEST(Internal, FPGAInfo) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) dev.reads_fpga_info(true);

  ASSERT_TRUE(autd.send(autd3::internal::UpdateFlags()));

  {
    for (const auto infos = autd.fpga_info(); auto info : infos) ASSERT_FALSE(info.is_thermal_assert());
  }

  {
    autd.link<autd3::link::Audit>().assert_thermal_sensor(0);
    autd.link<autd3::link::Audit>().update(0);
    autd.link<autd3::link::Audit>().update(1);

    const auto infos = autd.fpga_info();
    ASSERT_TRUE(infos[0].is_thermal_assert());
    ASSERT_FALSE(infos[1].is_thermal_assert());
  }

  {
    autd.link<autd3::link::Audit>().deassert_thermal_sensor(0);
    autd.link<autd3::link::Audit>().assert_thermal_sensor(1);
    autd.link<autd3::link::Audit>().update(0);
    autd.link<autd3::link::Audit>().update(1);

    const auto infos = autd.fpga_info();
    ASSERT_FALSE(infos[0].is_thermal_assert());
    ASSERT_TRUE(infos[1].is_thermal_assert());
  }

  {
    autd.link<autd3::link::Audit>().break_down();
    ASSERT_THROW((void)autd.fpga_info(), autd3::internal::AUTDException);
  }
}
