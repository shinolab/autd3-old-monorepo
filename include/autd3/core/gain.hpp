// File: gain.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 12/05/2022
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

template <typename T, std::enable_if_t<std::is_base_of_v<Transducer<typename T::D>, T>, nullptr_t> = nullptr>
struct GainProps {
  bool built;
  bool phase_sent;
  bool duty_sent;
  typename T::D drives;

  GainProps() noexcept : built(false), phase_sent(false), duty_sent(false), drives() {}

  void init(size_t size) { drives.init(size); }

  void pack_header(const uint8_t msg_id, driver::TxDatagram& tx) { T::pack_header(msg_id, tx); }
  void pack_body(driver::TxDatagram& tx) { T::pack_body(phase_sent, duty_sent, drives, tx); }
};

template <typename T, std::enable_if_t<std::is_base_of_v<Transducer<typename T::D>, T>, nullptr_t> = nullptr>
struct Gain : DatagramBody<T> {
  Gain() : _props() {}
  ~Gain() override = default;
  Gain(const Gain& v) = default;
  Gain& operator=(const Gain& obj) = default;
  Gain(Gain&& obj) = default;
  Gain& operator=(Gain&& obj) = default;

  virtual void calc(const Geometry<T>& geometry) = 0;

  void build(const Geometry<T>& geometry) {
    if (_props.built) return;
    _props.init(geometry.num_devices() * driver::NUM_TRANS_IN_UNIT);
    calc(geometry);
    _props.built = true;
  }

  void rebuild(const Geometry<T>& geometry) {
    _props.built = false;
    build(geometry);
  }

  typename T::D& drives() { return _props.drives; }

  bool built() { return _props.built; }

  void init() override {
    _props.phase_sent = false;
    _props.duty_sent = false;
  }

  void pack(uint8_t msg_id, Geometry<T>& geometry, driver::TxDatagram& tx) override {
    _props.pack_header(msg_id, tx);
    if (is_finished()) return;
    build(geometry);
    _props.pack_body(tx);
  }

  [[nodiscard]] bool is_finished() const noexcept override { return _props.phase_sent && _props.duty_sent; }

 protected:
  GainProps<T> _props;
};

}  // namespace autd3::core
