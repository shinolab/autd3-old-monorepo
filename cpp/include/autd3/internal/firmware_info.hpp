// File: firmware_info.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 03/06/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <string>

#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class FirmwareInfo {
 public:
  FirmwareInfo(std::string info, const bool is_valid, const bool is_supported) noexcept
      : _info(std::move(info)), _is_valid(is_valid), _is_supported(is_supported) {}

  [[nodiscard]] std::string info() const { return _info; }

  [[nodiscard]] static bool is_valid(const FirmwareInfo& info) noexcept { return info._is_valid; }
  [[nodiscard]] static bool is_supported(const FirmwareInfo& info) noexcept { return info._is_supported; }
  [[nodiscard]] static std::string latest_version() {
    char info[256];
    native_methods::AUTDGetLatestFirmware(info);
    return {info};
  }

 private:
  std::string _info;
  bool _is_valid;
  bool _is_supported;
};

inline std::ostream& operator<<(std::ostream& os, const FirmwareInfo& obj) {
  os << obj.info();
  return os;
}

}  // namespace autd3::internal
