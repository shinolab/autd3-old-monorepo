// File: adapter.hpp
// Project: pcap
// Created Date: 07/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 08/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <pcap.h>

namespace autd3::link::pcap {

struct Adapter {
  [[nodiscard]] const std::string& name() const noexcept { return _name; }
  [[nodiscard]] const std::string& desc() const noexcept { return _desc; }

  static std::vector<Adapter> enumerate_adapters() {
    pcap_if* all_devices = nullptr;
    char errbuf[256] = {};

    if (const auto res = pcap_findalldevs(&all_devices, errbuf); res == PCAP_ERROR) throw std::runtime_error(errbuf);

    std::vector<Adapter> adapters;
    for (const pcap_if* cursor = all_devices; cursor != nullptr; cursor = cursor->next)
      adapters.emplace_back(Adapter{std::string(cursor->name), std::string(cursor->description)});

    pcap_freealldevs(all_devices);

    return adapters;
  }

 private:
  explicit Adapter(std::string name, std::string desc) : _name(std::move(name)), _desc(std::move(desc)) {}

  std::string _name;
  std::string _desc;
};

}  // namespace autd3::link::pcap