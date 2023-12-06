// File: fpga_info.cpp
// Project: internal
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 06/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/datagram/reads_fpga_info.hpp>
#include <autd3/internal/datagram.hpp>
#include <autd3/internal/exception.hpp>
#include <autd3/internal/fpga_info.hpp>

#include "utils.hpp"

TEST(Internal, FPGAInfo) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send_async(autd3::datagram::ConfigureReadsFPGAInfo([](const auto&) { return true; })).get());
  {
    for (const auto infos = autd.fpga_info_async().get(); auto info : infos) ASSERT_FALSE(info.is_thermal_assert());
  }

  {
    autd.link().assert_thermal_sensor(0);
    autd.link().update(0);
    autd.link().update(1);

    const auto infos = autd.fpga_info_async().get();
    ASSERT_TRUE(infos[0].is_thermal_assert());
    ASSERT_FALSE(infos[1].is_thermal_assert());
  }

  {
    autd.link().deassert_thermal_sensor(0);
    autd.link().assert_thermal_sensor(1);
    autd.link().update(0);
    autd.link().update(1);

    const auto infos = autd.fpga_info_async().get();
    ASSERT_FALSE(infos[0].is_thermal_assert());
    ASSERT_TRUE(infos[1].is_thermal_assert());
  }

  {
    autd.link().break_down();
    ASSERT_THROW((void)autd.fpga_info_async().get(), autd3::internal::AUTDException);
  }
}
