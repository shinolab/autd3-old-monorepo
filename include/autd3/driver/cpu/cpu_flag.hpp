// File: cpu_flag.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <cstdint>
#include <iterator>
#include <ostream>
#include <sstream>
#include <string>
#include <vector>

#include "autd3/driver/bitflags.hpp"

namespace autd3::driver {

enum class CPUControlFlags : uint8_t {
  None = 0,
  /**
   * @brief Set when Header contains modulation data
   */
  Mod = 1 << 0,
  /**
   * @brief Set when modulation data begins
   */
  ModBegin = 1 << 1,
  /**
   * @brief Set when modulation data ends
   */
  ModEnd = 1 << 2,
  /**
   * @brief Clear when Header contains silencer data or synchronization data
   */
  ConfigEnN = 1 << 0,
  /**
   * @brief Set when Header contains silencer
   */
  ConfigSilencer = 1 << 1,
  /**
   * @brief Set when Header synchronization data
   */
  ConfigSync = 1 << 2,
  /**
   * @brief Set when Body is valid
   */
  WriteBody = 1 << 3,
  /**
   * @brief Set when Body contains STM data and STM begins
   */
  STMBegin = 1 << 4,
  /**
   * @brief Set when Body contains STM data and STM ends
   */
  STMEnd = 1 << 5,
  /**
   * @brief Set when Body is duty data (used only in Advanced mode)
   */
  IsDuty = 1 << 6,
  /**
   * @brief Set when Body is modulation delay data
   */
  ModDelay = 1 << 7,
};

inline std::string to_string(const BitFlags<CPUControlFlags> v) noexcept {
  std::vector<std::string> flags;
  if (v.contains(CPUControlFlags::Mod)) {
    if (v.contains(CPUControlFlags::ModBegin)) flags.emplace_back("ModBegin");
    if (v.contains(CPUControlFlags::ModEnd)) flags.emplace_back("ModEnd");
  } else {
    if (v.contains(CPUControlFlags::ConfigSilencer)) flags.emplace_back("ConfigSilencer");
    if (v.contains(CPUControlFlags::ConfigSync)) flags.emplace_back("ConfigSync");
  }
  if (v.contains(CPUControlFlags::WriteBody)) flags.emplace_back("WriteBody");
  if (v.contains(CPUControlFlags::STMBegin)) flags.emplace_back("STMBegin");
  if (v.contains(CPUControlFlags::STMEnd)) flags.emplace_back("STMEnd");
  if (v.contains(CPUControlFlags::IsDuty)) flags.emplace_back("IsDuty");
  if (v.contains(CPUControlFlags::ModDelay)) flags.emplace_back("ModDelay");
  if (flags.empty()) flags.emplace_back("None");

  constexpr auto delim = " | ";
  std::ostringstream os;
  std::copy(flags.begin(), flags.end(), std::ostream_iterator<std::string>(os, delim));
  std::string s = os.str();
  s.erase(s.size() - std::char_traits<char>::length(delim));
  return s;
}

inline std::ostream& operator<<(std::ostream& os, const BitFlags<CPUControlFlags>& flag) { return os << to_string(flag); }

}  // namespace autd3::driver
