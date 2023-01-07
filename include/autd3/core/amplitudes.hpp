// File: amplitudes.hpp
// Project: core
// Created Date: 28/06/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3/core/datagram.hpp"
#include "autd3/core/geometry.hpp"
#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/operation/gain.hpp"

namespace autd3::core {
/**
 * @brief Amplitude configuration for NormalPhaseMode
 */
class Amplitudes final : public DatagramBody {
 public:
  explicit Amplitudes(const driver::autd3_float_t amp = 1.0) : _amp(amp) {}
  ~Amplitudes() override = default;
  Amplitudes(const Amplitudes& v) = default;
  Amplitudes& operator=(const Amplitudes& obj) = default;
  Amplitudes(Amplitudes&& obj) = default;
  Amplitudes& operator=(Amplitudes&& obj) = default;

  void init(const Mode, const Geometry& geometry) override {
    _op.init();
    _op.cycles = geometry.cycles();
    _op.drives.resize(geometry.num_transducers(), driver::Drive{0, _amp});
  }

  void pack(driver::TxDatagram& tx) override { _op.pack(tx); }

  [[nodiscard]] bool is_finished() const noexcept override { return _op.is_finished(); }

 private:
  driver::autd3_float_t _amp;
  driver::Gain<driver::Normal> _op;
};

}  // namespace autd3::core
