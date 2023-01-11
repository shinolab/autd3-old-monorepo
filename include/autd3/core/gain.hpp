// File: gain.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/01/2023
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
   * @brief Getter function for the data of duty ratio and phase of each transducers
   */
  [[nodiscard]] const std::vector<driver::Drive>& drives() const { return _props.drives; }

  /**
   * @brief [Advanced] Getter function for the data of duty ratio and phase of each transducers
   * @details Call Gain::build before using this function to initialize drive data.
   */
  std::vector<driver::Drive>& drives() { return _props.drives; }

  void init(const Mode mode, const Geometry& geometry) override {
    _mode = mode;
    switch (mode) {
      case Mode::Legacy: {
        auto op = std::make_shared<driver::Gain<driver::Legacy>>(_props);
        op->init();
        _props.drives.resize(geometry.num_transducers());
        _op = std::move(op);
      } break;
      case Mode::Normal: {
        auto op = std::make_shared<driver::Gain<driver::Normal>>(_props);
        op->init();
        op->cycles = geometry.cycles();
        _props.drives.resize(geometry.num_transducers());
        _op = std::move(op);
      } break;
      case Mode::NormalPhase: {
        auto op = std::make_shared<driver::Gain<driver::NormalPhase>>(_props);
        op->init();
        op->cycles = geometry.cycles();
        _props.drives.resize(geometry.num_transducers());
        _op = std::move(op);
      } break;
    }
    calc(geometry);
  }

  void pack(driver::TxDatagram& tx) override { _op->pack(tx); }

  [[nodiscard]] bool is_finished() const noexcept override { return _op->is_finished(); }

  [[nodiscard]] std::vector<driver::Drive>::const_iterator begin() const noexcept { return _props.drives.begin(); }
  [[nodiscard]] std::vector<driver::Drive>::const_iterator end() const noexcept { return _props.drives.end(); }
  [[nodiscard]] std::vector<driver::Drive>::iterator begin() noexcept { return _props.drives.begin(); }
  [[nodiscard]] std::vector<driver::Drive>::iterator end() noexcept { return _props.drives.end(); }
  [[nodiscard]] const driver::Drive& operator[](const size_t i) const { return _props.drives[i]; }
  [[nodiscard]] driver::Drive& operator[](const size_t i) { return _props.drives[i]; }

 protected:
  Mode _mode{Mode::Legacy};
  bool _built{false};
  driver::GainProps _props;
  std::shared_ptr<driver::GainBase> _op{nullptr};
};

}  // namespace autd3::core
