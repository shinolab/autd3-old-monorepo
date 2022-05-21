// File: gain.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 21/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <type_traits>

#include "geometry/geometry.hpp"
#include "geometry/transducer.hpp"
#include "interface.hpp"

namespace autd3::core {

/**
 * @brief Properties of Gain
 */
template <typename T, std::enable_if_t<std::is_base_of_v<Transducer<typename T::D>, T>, nullptr_t> = nullptr>
struct GainProps {
  bool built;
  bool phase_sent;
  bool duty_sent;
  typename T::D drives;

  GainProps() noexcept : built(false), phase_sent(false), duty_sent(false), drives() {}

  void init(size_t size) { drives.init(size); }

  void pack_header(driver::TxDatagram& tx) { T::pack_header(tx); }
  void pack_body(driver::TxDatagram& tx) { T::pack_body(phase_sent, duty_sent, drives, tx); }
};

/**
 * @brief Gain controls the duty ratio and phase of each transducer in AUTD devices.
 */
template <typename T, std::enable_if_t<std::is_base_of_v<Transducer<typename T::D>, T>, nullptr_t> = nullptr>
struct Gain : DatagramBody<T> {
  Gain() : _props() {}
  ~Gain() override = default;
  Gain(const Gain& v) = default;
  Gain& operator=(const Gain& obj) = default;
  Gain(Gain&& obj) = default;
  Gain& operator=(Gain&& obj) = default;

  /**
   * \brief Calculate duty ratio and phase of each transducer
   * \param geometry Geometry
   */
  virtual void calc(const Geometry<T>& geometry) = 0;

  /**
   * \brief Initialize data and call calc().
   * \param geometry Geometry
   */
  void build(const Geometry<T>& geometry) {
    if (_props.built) return;
    _props.init(geometry.num_devices() * driver::NUM_TRANS_IN_UNIT);
    calc(geometry);
    _props.built = true;
  }

  /**
   * \brief Re-calculate duty ratio and phase of each transducer
   * \param geometry Geometry
   */
  void rebuild(const Geometry<T>& geometry) {
    _props.built = false;
    build(geometry);
  }

  /**
   * @brief Getter function for the data of duty ratio and phase of each transducers
   */
  typename T::D& drives() { return _props.drives; }

  bool built() { return _props.built; }

  void init() override {
    _props.phase_sent = false;
    _props.duty_sent = false;
  }

  void pack(const Geometry<T>& geometry, driver::TxDatagram& tx) override {
    _props.pack_header(tx);
    if (is_finished()) return;
    build(geometry);
    _props.pack_body(tx);
  }

  [[nodiscard]] bool is_finished() const noexcept override { return _props.phase_sent && _props.duty_sent; }

 protected:
  GainProps<T> _props;
};

}  // namespace autd3::core
