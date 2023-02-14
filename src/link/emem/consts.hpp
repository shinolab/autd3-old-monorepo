// File: consts.hpp
// Project: emem
// Created Date: 05/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <cstdint>

namespace autd3::link {

constexpr uint32_t EC_MAX_FRAME_SIZE = 1518;
constexpr uint32_t EC_MAX_LRW_DATA = EC_MAX_FRAME_SIZE - 14 - 2 - 10 - 2 - 4;
constexpr uint32_t EC_FIRST_DC_DATAGRAM = 20;
constexpr size_t EC_BUF_SIZE = 16;
constexpr uint16_t EC_SLAVE_MAX = 200;
constexpr int32_t EC_DEFAULT_RETRIES = 3;

constexpr uint16_t MAX_IO_SEGMENT = 64;

constexpr uint64_t EC_TIMEOUT_US = 2000;
constexpr std::chrono::nanoseconds EC_TIMEOUT = std::chrono::nanoseconds(EC_TIMEOUT_US * 1000);
constexpr std::chrono::nanoseconds EC_TIMEOUT3 = 3 * EC_TIMEOUT;
constexpr std::chrono::nanoseconds EC_TIMEOUT_SAFE = 10 * EC_TIMEOUT;
constexpr std::chrono::nanoseconds EC_TIMEOUT_STATE = std::chrono::nanoseconds(2000000000UL);
constexpr uint16_t EC_NODE_OFFSET = 0x1000;

}  // namespace autd3::link
