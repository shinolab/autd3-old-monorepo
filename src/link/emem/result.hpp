// File: result.hpp
// Project: emem
// Created Date: 06/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <string>

namespace autd3::link {

enum class EmemResult {
  Ok,
  SendFrame,
  ReceiveFrame,
  NoFrame,
  UnknownFrame,
  UndefinedBehavior,
  SlaveNotFound,
  TooManySlaves,
  Recover,
};

inline std::string to_string(const EmemResult r) {
  switch (r) {
    case EmemResult::Ok:
      return "No error";
    case EmemResult::SendFrame:
      return "Failed to send frame";
    case EmemResult::ReceiveFrame:
      return "Failed to receive frame";
    case EmemResult::NoFrame:
      return "No frame available";
    case EmemResult::UnknownFrame:
      return "Unknown frame";
    case EmemResult::UndefinedBehavior:
      return "Undefined behavior";
    case EmemResult::SlaveNotFound:
      return "No slave found";
    case EmemResult::TooManySlaves:
      return "Too many slaves";
    case EmemResult::Recover:
      return "Filed to recover";
  }
  return "Unknown error";
}

inline std::ostream& operator<<(std::ostream& os, const EmemResult r) { return os << to_string(r); }

#define EMEM_CHECK_RESULT(action) \
  if (const auto check_result_res = action; check_result_res != EmemResult::Ok) return check_result_res

}  // namespace autd3::link
