// File: amplitudes.hpp
// Project: core
// Created Date: 28/06/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

namespace autd3::core {
/**
 * @brief Amplitude configuration for DynamicTransducer.
 */
class Amplitudes final : DatagramBody {
 public:
  explicit Amplitudes(const double amp = 1.0) : _amp(amp), _phase_sent(false), _duty_sent(false), _drives() {}
  ~Amplitudes() override = default;
  Amplitudes(const Amplitudes& v) = default;
  Amplitudes& operator=(const Amplitudes& obj) = default;
  Amplitudes(Amplitudes&& obj) = default;
  Amplitudes& operator=(Amplitudes&& obj) = default;

  void init() override {
    _phase_sent = false;
    _duty_sent = false;
  }

  void pack(const Geometry& geometry, driver::TxDatagram& tx) override {
    geometry.mode()->pack_gain_header(tx);
    if (is_finished()) return;
    _drives.resize(geometry.num_transducers(), driver::Drive{0.0, _amp, 4096});
    geometry.mode()->pack_gain_body(_phase_sent, _duty_sent, _drives, tx);
  }

  [[nodiscard]] bool is_finished() const noexcept override { return _phase_sent && _duty_sent; }

 private:
  double _amp;
  bool _phase_sent;
  bool _duty_sent;
  std::vector<driver::Drive> _drives;
};

}  // namespace autd3::core
