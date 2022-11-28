// File: emulator_link.hpp
// Project: tests
// Created Date: 09/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 26/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3/core/link.hpp>
#include <autd3/extra/cpu_emulator.hpp>

namespace autd3::test {

class EmulatorLink : public core::Link {
 public:
  core::LinkPtr build() {
    core::LinkPtr link = std::make_unique<EmulatorLink>(_cpus);
    return link;
  }

  explicit EmulatorLink(std::shared_ptr<std::vector<extra::CPU>> cpus) noexcept : Link(), _cpus(cpus) {}
  ~EmulatorLink() override = default;
  EmulatorLink(const EmulatorLink& v) noexcept = delete;
  EmulatorLink& operator=(const EmulatorLink& obj) = delete;
  EmulatorLink(EmulatorLink&& obj) = delete;
  EmulatorLink& operator=(EmulatorLink&& obj) = delete;

  bool open(const core::Geometry& geometry) override {
    if (is_open()) return true;

    _cpus->clear();
    _cpus->reserve(geometry.num_devices());
    for (size_t i = 0; i < geometry.num_devices(); i++) {
      extra::CPU cpu(i, geometry.device_map()[i]);
      cpu.init();
      _cpus->emplace_back(cpu);
    }

    _is_open = true;
    return true;
  }

  bool close() override {
    _is_open = false;
    return true;
  }

  bool send(const driver::TxDatagram& tx) override {
    for (auto& cpu : *_cpus) cpu.send(tx);
    return true;
  }

  bool receive(driver::RxDatagram& rx) override {
    for (size_t i = 0; i < _cpus->size(); i++) {
      rx.messages()[i].msg_id = _cpus->at(i).msg_id();
      rx.messages()[i].ack = _cpus->at(i).ack();
    }
    return true;
  }

  bool is_open() override { return _is_open; }

 private:
  bool _is_open{false};
  std::shared_ptr<std::vector<extra::CPU>> _cpus;
};

}  // namespace autd3::test
