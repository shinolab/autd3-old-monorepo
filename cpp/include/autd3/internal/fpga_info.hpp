// File: fpga_info.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

namespace autd3::internal {

class FPGAInfo {
 public:
  explicit FPGAInfo(const uint8_t info) noexcept : _info(info) {}
  ~FPGAInfo() = default;
  FPGAInfo(const FPGAInfo& v) = default;
  FPGAInfo& operator=(const FPGAInfo& obj) = default;
  FPGAInfo(FPGAInfo&& obj) = default;
  FPGAInfo& operator=(FPGAInfo&& obj) = default;

  [[nodiscard]] bool is_thermal_assert() const noexcept { return (_info & 0x01) != 0; }

  [[nodiscard]] std::string to_string() const { return "Thermal assert = " + std::to_string(is_thermal_assert()); }

 private:
  uint8_t _info;
};

inline std::ostream& operator<<(std::ostream& os, const FPGAInfo& obj) {
  os << obj.to_string();
  return os;
}

}  // namespace autd3::internal
