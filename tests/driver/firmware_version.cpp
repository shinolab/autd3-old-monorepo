// File: firmware_version.cpp
// Project: driver
// Created Date: 02/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/01/2023
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

#include <autd3/driver/firmware_version.hpp>

TEST(DriverFirmwareVersion, FirmwareInfo) {
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
    EXPECT_EQ("v2.7", info.cpu_version());
    EXPECT_EQ("v2.7", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 136, 136, 0);
    EXPECT_EQ("v2.8", info.cpu_version());
    EXPECT_EQ("v2.8", info.fpga_version());
  }
  {
    const autd3::driver::FirmwareInfo info(0, 137, 137, 0);
    EXPECT_EQ("unknown (137)", info.cpu_version());
    EXPECT_EQ("unknown (137)", info.fpga_version());
  }
}
