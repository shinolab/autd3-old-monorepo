// File: link_test.cpp
// Project: link
// Created Date: 29/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 10/06/2022
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

#ifdef TEST_LINK_SOEM
#include "autd3/link/soem.hpp"
TEST(LinkSOEMTest, basic) { auto link = autd3::link::SOEM("", 0).build(); }
#endif

#ifdef TEST_LINK_TWINCAT
#include "autd3/link/twincat.hpp"
TEST(LinkTwinCATTest, basic) { auto link = autd3::link::TwinCAT().build(); }
#endif

#ifdef TEST_LINK_REMOTE_TWINCAT
#include "autd3/link/remote_twincat.hpp"
TEST(LinkRemoteTwinCATTest, basic) { auto link = autd3::link::RemoteTwinCAT("", "").build(); }
#endif

#ifdef TEST_LINK_EMULATOR
#include "autd3/link/emulator.hpp"
TEST(LinkEmulatorTest, basic) {
  const autd3::core::Geometry geometry;
  const auto link = autd3::link::Emulator(geometry).port(10).build();
}
#endif
