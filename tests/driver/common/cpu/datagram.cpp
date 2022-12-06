// File: datagram.cpp
// Project: cpu
// Created Date: 01/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 06/12/2022
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

#include <random>

#include "autd3/driver/common/cpu/datagram.hpp"

TEST(DriverCommonCPUTest, TxDatagram) {
  const std::vector<size_t> device_map = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
  autd3::driver::TxDatagram tx(device_map);

  ASSERT_EQ(tx.num_devices(), 10);
  ASSERT_EQ(tx.bodies_size(), sizeof(uint16_t) * 55);
  ASSERT_EQ(tx.transmitting_size(), 128 + sizeof(uint16_t) * 55);

  tx.num_bodies = 5;
  ASSERT_EQ(tx.num_devices(), 10);
  ASSERT_EQ(tx.bodies_size(), sizeof(uint16_t) * 15);
  ASSERT_EQ(tx.transmitting_size(), 128 + sizeof(uint16_t) * 15);

  ASSERT_EQ(tx.data().data(), reinterpret_cast<uint8_t*>(&tx.header()));
  ASSERT_EQ(tx.data().data() + 128, reinterpret_cast<uint8_t*>(tx.bodies_raw_ptr()));
  auto* cursor = tx.data().data() + 128;
  for (size_t i = 0; i < device_map.size(); i++) {
    ASSERT_EQ(cursor, reinterpret_cast<uint8_t*>(&tx.body(i)));
    cursor += sizeof(uint16_t) * device_map[i];
  }
}

TEST(DriverCommonCPUTest, RxDatagram) {
  autd3::driver::RxDatagram rx(10);

  ASSERT_FALSE(rx.is_msg_processed(1));

  rx[0].msg_id = 1;
  ASSERT_FALSE(rx.is_msg_processed(1));

  for (auto& msg : rx) msg.msg_id = 1;
  ASSERT_TRUE(rx.is_msg_processed(1));
  ASSERT_FALSE(rx.is_msg_processed(2));
}
