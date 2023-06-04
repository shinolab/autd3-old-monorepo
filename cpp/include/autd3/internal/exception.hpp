// File: exception.hpp
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

#include <stdexcept>

#ifdef WIN32
#include <Windows.h>
#endif

namespace autd3::internal {

class AUTDException final : public std::runtime_error {
 public:
  explicit AUTDException(const char* message) : runtime_error(message) {}
};

#define FORCE_CODEPAGE_UTF8_WIN                       \
  struct AUTD3Utf8Console final {                     \
    inline static UINT old_cp = 0;                    \
    AUTD3Utf8Console() {                              \
      old_cp = GetConsoleOutputCP();                  \
      SetConsoleOutputCP(CP_UTF8);                    \
      std::set_terminate(terminate);                  \
    }                                                 \
    ~AUTD3Utf8Console() { pop(); }                    \
    static void pop() { SetConsoleOutputCP(old_cp); } \
    static void terminate() {                         \
      pop();                                          \
      std::abort();                                   \
    }                                                 \
  } utf8_console;

}  // namespace autd3::internal
