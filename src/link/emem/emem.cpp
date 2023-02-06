// File: emem.cpp
// Project: emem
// Created Date: 04/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 07/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include "buffer.hpp"
#include "ethercat_driver.hpp"
#include "idx_stack.hpp"
#include "interface.hpp"
#include "master.hpp"
#include "network_driver.hpp"

namespace autd3::link {
struct DInterface : Interface {
  DInterface() : Interface() {}
  ~DInterface() override {}

  Result<std::nullptr_t> send(const uint8_t* data, size_t size) override { return Result<std::nullptr_t>(EmemError::NoFrame); }
  Result<std::nullptr_t> read(uint8_t* data) override { return Result<std::nullptr_t>(EmemError::NoFrame); }
  void close() override {}
};

void a() {
  DInterface i;
  auto d = Master<DInterface>(i);

  d.send_process_data();
  const auto res = d.receive_process_data(std::chrono::nanoseconds(1));

  d.close();
}

}  // namespace autd3::link
