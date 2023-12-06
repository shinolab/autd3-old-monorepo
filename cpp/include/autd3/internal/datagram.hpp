// File: datagram.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 06/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"

namespace autd3::internal {

template <class S>
concept special_datagram = requires(S s) {
  { s.ptr() } -> std::same_as<native_methods::DatagramSpecialPtr>;
};

template <class D>
concept datagram = requires(D d, const geometry::Geometry& g) {
  { d.ptr(g) } -> std::same_as<native_methods::DatagramPtr>;
};

class NullDatagram final {
 public:
  NullDatagram() = default;
  ~NullDatagram() = default;
  NullDatagram(const NullDatagram& v) noexcept = default;
  NullDatagram& operator=(const NullDatagram& obj) = default;
  NullDatagram(NullDatagram&& obj) = default;
  NullDatagram& operator=(NullDatagram&& obj) = default;

  [[nodiscard]] static native_methods::DatagramPtr ptr(const geometry::Geometry&) { return native_methods::DatagramPtr{nullptr}; }
};

/**
 * @brief Datagram to configure silencer
 */
class Silencer final {
 public:
  Silencer() noexcept : Silencer(256, 256) {}
  /**
   * @brief Constructor
   *
   * @param step_intensity Intensity update step of silencer. The smaller `step`
   * is, the quieter the output is.
   * @param step_phase Phase update step of silencer. The smaller `step` is, the
   * quieter the output is.
   */
  explicit Silencer(const uint16_t step_intensity, const uint16_t step_phase) noexcept : _step_intensity(step_intensity), _step_phase(step_phase) {}

  /**
   * @brief Disable silencer
   */
  static Silencer disable() noexcept { return Silencer(0xFFFF, 0xFFFF); }

  [[nodiscard]] native_methods::DatagramPtr ptr(const geometry::Geometry&) const {
    return validate(native_methods::AUTDDatagramSilencer(_step_intensity, _step_phase));
  }

 private:
  uint16_t _step_intensity;
  uint16_t _step_phase;
};

/**
 * @brief Datagram for clear all data in devices
 */
class Clear final {
 public:
  Clear() = default;

  [[nodiscard]] static native_methods::DatagramPtr ptr(const geometry::Geometry&) { return native_methods::AUTDDatagramClear(); }
};

/**
 * @brief Datagram to synchronize devices
 */
class Synchronize final {
 public:
  Synchronize() = default;

  [[nodiscard]] static native_methods::DatagramPtr ptr(const geometry::Geometry&) { return native_methods::AUTDDatagramSynchronize(); }
};

}  // namespace autd3::internal
