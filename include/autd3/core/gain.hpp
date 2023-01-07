// File: gain.hpp
// Project: core
// Created Date: 11/05/2022
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
#include "autd3/driver/operation/gain.hpp"

namespace autd3::core {

/**
 * @brief Gain controls the duty ratio and phase of each transducer in AUTD devices
 */
template <typename T>
struct Gain : DatagramBody {
  Gain() = default;
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
    _op._drives.resize(geometry.num_transducers());
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
  [[nodiscard]] const std::vector<driver::Drive>& drives() const noexcept { return _op.drives; }

  /**
   * @brief [Advanced] Getter function for the data of duty ratio and phase of each transducers
   * @details Call Gain::build before using this function to initialize drive data.
   */
  std::vector<driver::Drive>& drives() noexcept { return _op.drives; }

  [[nodiscard]] bool built() const { return _built; }

  bool init(const Geometry& geometry) override {
    _op.init();
    if constexpr (driver::uses_cycle_v<T>) _op.cycles = geometry.cycles();
    build(geometry);
    return true;
  }

  void pack(driver::TxDatagram& tx) override { _op.pack(tx); }

  [[nodiscard]] bool is_finished() const noexcept override { return _op.is_finished(); }

 protected:
  bool _built{false};
  driver::Gain<T> _op;
};

}  // namespace autd3::core
