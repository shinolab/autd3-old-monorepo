// File: simulator.cpp
// Project: simulator
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 05/10/2022
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
  explicit SimulatorImpl(const int32_t width, const int32_t height, const bool vsync, std::string shader, std::string texture, std::string font,
                         const size_t gpu_idx, std::function<void()> callback)
      : Link(),
        _is_open(false),
        _simulator(extra::simulator::Simulator::create(width, height, vsync, std::move(shader), std::move(texture), std::move(font), gpu_idx,
                                                       std::move(callback))) {}
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
  core::LinkPtr link = std::make_unique<SimulatorImpl>(_width, _height, _vsync, _shader, _texture, _font, _gpu_idx, _callback);
  return link;
}

}  // namespace autd3::link
