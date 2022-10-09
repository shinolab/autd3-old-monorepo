// File: debug.hpp
// Project: link
// Created Date: 26/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/core/link.hpp"

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 6285 6385 26437 26800 26498 26451 26495)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#ifdef __clang__
#pragma clang diagnostic push
#endif
#include "spdlog/spdlog.h"
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif
#ifdef __clang__
#pragma clang diagnostic pop
#endif

namespace autd3::link {
/**
 * @brief Link for debug
 */
class Debug {
 public:
  /**
   * @brief Create Bundle link
   */
  [[nodiscard]] core::LinkPtr build() const;

  /**
   * @brief Constructor
   */
  Debug() = default;

  ~Debug() = default;
  Debug(const Debug& v) noexcept = delete;
  Debug& operator=(const Debug& obj) = delete;
  Debug(Debug&& obj) = delete;
  Debug& operator=(Debug&& obj) = delete;
};
}  // namespace autd3::link
