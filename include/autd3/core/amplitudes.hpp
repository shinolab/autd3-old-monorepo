// File: amplitudes.hpp
// Project: core
// Created Date: 28/06/2022
// Author: Shun Suzuki
// -----
// Last Modified: 21/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <vector>

#include "autd3/core/datagram.hpp"
#include "autd3/core/geometry.hpp"
#include "autd3/driver/common/cpu/datagram.hpp"

namespace autd3::core {
/**
 * @brief Amplitude configuration for NormalPhaseMode
 */
class Amplitudes final : public DatagramBody {
 public:
  explicit Amplitudes(const driver::autd3_float_t amp = 1.0) : _amp(amp), _sent(false) {}
  ~Amplitudes() override = default;
  Amplitudes(const Amplitudes& v) = default;
  Amplitudes& operator=(const Amplitudes& obj) = default;
  Amplitudes(Amplitudes&& obj) = default;
  Amplitudes& operator=(Amplitudes&& obj) = default;

  bool init() override {
    _sent = false;
    return true;
  }

  bool pack(const std::unique_ptr<const driver::Driver>& driver, const std::unique_ptr<const Mode>&, const Geometry& geometry,
            driver::TxDatagram& tx) override {
    driver->normal_header(tx);
    if (is_finished()) return true;

    std::vector<driver::Drive> drives;
    drives.reserve(geometry.num_transducers());
    std::transform(geometry.begin(), geometry.end(), std::back_inserter(drives), [&](const auto& tr) { return driver::Drive{0, _amp, tr.cycle()}; });

    driver->normal_duty_body(drives, tx);
    _sent = true;
    return true;
  }

  [[nodiscard]] bool is_finished() const noexcept override { return _sent; }

 private:
  driver::autd3_float_t _amp;
  bool _sent;
};

}  // namespace autd3::core
