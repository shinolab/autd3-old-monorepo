// File: gain.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "datagram.hpp"
#include "geometry.hpp"

namespace autd3::core {

/**
 * @brief Gain controls the duty ratio and phase of each transducer in AUTD devices
 */
struct Gain : DatagramBody {
  Gain() : _built(false), _phase_sent(false), _duty_sent(false) {}
  ~Gain() override = default;
  Gain(const Gain& v) = default;
  Gain& operator=(const Gain& obj) = default;
  Gain(Gain&& obj) = default;
  Gain& operator=(Gain&& obj) = default;

  /**
   * \brief Calculate duty ratio and phase of each transducer
   * \param geometry Geometry
   */
  virtual void calc(const Geometry& geometry) = 0;

  /**
   * \brief Initialize data and call calc().
   * \param geometry Geometry
   */
  void build(const Geometry& geometry) {
    if (_built) return;
    _drives.resize(geometry.num_transducers());
    calc(geometry);
    _built = true;
  }

  /**
   * \brief Re-calculate duty ratio and phase of each transducer
   * \param geometry Geometry
   */
  void rebuild(const Geometry& geometry) {
    _built = false;
    build(geometry);
  }

  /**
   * @brief Getter function for the data of duty ratio and phase of each transducers
   */
  [[nodiscard]] const std::vector<driver::Drive>& drives() const noexcept { return _drives; }

  /**
   * @brief [Advanced] Getter function for the data of duty ratio and phase of each transducers
   * @details Call Gain::build before using this function to initialize drive data.
   */
  std::vector<driver::Drive>& drives() noexcept { return _drives; }

  [[nodiscard]] bool built() const { return _built; }

  bool init() override {
    _phase_sent = false;
    _duty_sent = false;
    return true;
  }

  bool pack(const std::unique_ptr<const driver::Driver>& driver, const std::unique_ptr<const Mode>& mode, const Geometry& geometry,
            driver::TxDatagram& tx) override {
    mode->pack_gain_header(driver, tx);
    if (is_finished()) return true;
    build(geometry);
    mode->pack_gain_body(driver, _phase_sent, _duty_sent, _drives, geometry, tx);
    return true;
  }

  [[nodiscard]] bool is_finished() const noexcept override { return _phase_sent && _duty_sent; }

 protected:
  bool _built;
  bool _phase_sent;
  bool _duty_sent;
  std::vector<driver::Drive> _drives;
};

}  // namespace autd3::core
