// File: simulator.cpp
// Project: simulator
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 06/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/extra/simulator/simulator.hpp"

#include "autd3/core/interface.hpp"
#include "autd3/core/link.hpp"
#include "autd3/link/simulator.hpp"

namespace autd3::link {

class SimulatorImpl final : public core::Link {
 public:
  explicit SimulatorImpl(extra::simulator::Settings settings, std::function<void(extra::simulator::Settings)> callback)
      : Link(), _is_open(false), _simulator(extra::simulator::Simulator::create(std::move(settings), std::move(callback))) {}
  ~SimulatorImpl() override = default;
  SimulatorImpl(const SimulatorImpl& v) noexcept = delete;
  SimulatorImpl& operator=(const SimulatorImpl& obj) = delete;
  SimulatorImpl(SimulatorImpl&& obj) = default;
  SimulatorImpl& operator=(SimulatorImpl&& obj) = default;

  void open(const core::Geometry& geometry) override {
    if (is_open()) return;

    _simulator->start(geometry);

    _is_open = true;
  }

  void close() override {
    if (!is_open()) return;

    _simulator->exit();

    _is_open = false;
  }

  bool send(const driver::TxDatagram& tx) override { return _simulator->send(tx); }
  bool receive(driver::RxDatagram& rx) override { return _simulator->receive(rx); }
  bool is_open() override { return _is_open; }

 private:
  bool _is_open;

  std::unique_ptr<extra::simulator::Simulator> _simulator;
};

core::LinkPtr Simulator::build() const {
  core::LinkPtr link = std::make_unique<SimulatorImpl>(_settings, _callback);
  return link;
}

}  // namespace autd3::link
