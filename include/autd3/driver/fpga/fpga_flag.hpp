// File: defined.hpp
// Project: fpga
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
#include <sstream>
#include <string>
#include <vector>

#include "autd3/driver/bitflags.hpp"

namespace autd3::driver {

/**
 * @brief Flags to control FPGA firmware
 */
enum class FPGAControlFlags : uint8_t {
  None = 0,
  /**
   * @brief Set when legacy mode
   */
  LegacyMode = 1 << 0,
  /**
   * @brief Set when using STM finish idx
   */
  UseSTMFinishIdx = 1 << 2,
  /**
   * @brief Set when using STM start idx
   */
  UseSTMStartIdx = 1 << 3,
  /**
   * @brief Set when forcing fan
   */
  ForceFan = 1 << 4,
  /**
   * @brief Set when STM
   */
  STMMode = 1 << 5,
  /**
   * @brief Set when GainSTM
   */
  STMGainMode = 1 << 6,
  /**
   * @brief Set when returning FPGA information
   */
  ReadsFPGAInfo = 1 << 7,
};

inline std::string to_string(const BitFlags<FPGAControlFlags> v) noexcept {
  std::vector<std::string> flags;
  if (v.contains(FPGAControlFlags::LegacyMode)) flags.emplace_back("LegacyMode");
  if (v.contains(FPGAControlFlags::UseSTMStartIdx)) flags.emplace_back("UseSTMStartIdx");
  if (v.contains(FPGAControlFlags::UseSTMFinishIdx)) flags.emplace_back("UseSTMFinishIdx");
  if (v.contains(FPGAControlFlags::ForceFan)) flags.emplace_back("ForceFan");
  if (v.contains(FPGAControlFlags::STMMode)) flags.emplace_back("STMMode");
  if (v.contains(FPGAControlFlags::STMGainMode)) flags.emplace_back("STMGainMode");
  if (v.contains(FPGAControlFlags::ReadsFPGAInfo)) flags.emplace_back("ReadsFPGAInfo");
  if (flags.empty()) flags.emplace_back("None");

  constexpr auto delim = " | ";
  std::ostringstream os;
  std::copy(flags.begin(), flags.end(), std::ostream_iterator<std::string>(os, delim));
  std::string s = os.str();
  s.erase(s.size() - std::char_traits<char>::length(delim));
  return s;
}

inline std::ostream& operator<<(std::ostream& os, const FPGAControlFlags& flag) { return os << to_string(flag); }

}  // namespace autd3::driver
