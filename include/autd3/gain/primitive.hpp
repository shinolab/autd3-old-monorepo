// File: primitive.hpp
// Project: gain
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <utility>

#include "autd3/core/gain.hpp"

namespace autd3::gain {

/**
 * @brief Gain to produce nothing
 */
template <typename T = core::LegacyTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class Null final : public core::Gain<T> {
 public:
  Null() noexcept {}

  void calc(const core::Geometry<T>& geometry) override {
    for (const auto& dev : geometry)
      for (const auto& transducer : dev) {
        this->_props.drives.set_drive(transducer, 0.0, 0.0);
      }
  }

  ~Null() override = default;
  Null(const Null& v) noexcept = delete;
  Null& operator=(const Null& obj) = delete;
  Null(Null&& obj) = default;
  Null& operator=(Null&& obj) = default;
};

/**
 * @brief Gain to produce single focal point
 */
template <typename T = core::LegacyTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class Focus final : public core::Gain<T> {
 public:
  /**
   * @param[in] point focal point
   * @param[in] amp amplitude of the wave (from 0.0 to 1.0)
   */
  explicit Focus(core::Vector3 point, const double amp = 1.0) : _point(std::move(point)), _amp(amp) {}

  void calc(const core::Geometry<T>& geometry) override {
    for (const auto& dev : geometry)
      for (const auto& transducer : dev) {
        const auto dist = (_point - transducer.position()).norm();
        const auto phase = transducer.align_phase_at(dist, geometry.sound_speed);
        this->_props.drives.set_drive(transducer, phase, _amp);
      }
  }

  ~Focus() override = default;
  Focus(const Focus& v) noexcept = delete;
  Focus& operator=(const Focus& obj) = delete;
  Focus(Focus&& obj) = default;
  Focus& operator=(Focus&& obj) = default;

 private:
  core::Vector3 _point;
  double _amp;
};

/**
 * @brief Gain to produce Bessel Beam
 */
template <typename T = core::LegacyTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class BesselBeam final : public core::Gain<T> {
 public:
  /**
   * @param[in] apex apex of the conical wavefront of the beam
   * @param[in] vec_n direction of the beam
   * @param[in] theta_z angle between the side of the cone and the plane perpendicular to direction of the beam
   * @param[in] amp amplitude of the wave (from 0.0 to 1.0)
   */
  explicit BesselBeam(core::Vector3 apex, core::Vector3 vec_n, const double theta_z, const double amp = 1.0)
      : core::Gain<T>(), _apex(std::move(apex)), _vec_n(std::move(vec_n)), _theta_z(theta_z), _amp(amp) {}

  void calc(const core::Geometry<T>& geometry) override {
    _vec_n.normalize();
    core::Vector3 v = core::Vector3::UnitZ().cross(_vec_n);
    const auto theta_v = std::asin(v.norm());
    v.normalize();
    const Eigen::AngleAxisd rot(-theta_v, v);
    for (const auto& dev : geometry)
      for (const auto& transducer : dev) {
        const auto r = transducer.position() - this->_apex;
        const auto rr = rot * r;
        const auto d = std::sin(_theta_z) * std::sqrt(rr.x() * rr.x() + rr.y() * rr.y()) - std::cos(_theta_z) * rr.z();
        const auto phase = transducer.align_phase_at(d, geometry.sound_speed);
        this->_props.drives.set_drive(transducer, phase, _amp);
      }
  }

  ~BesselBeam() override = default;
  BesselBeam(const BesselBeam& v) noexcept = delete;
  BesselBeam& operator=(const BesselBeam& obj) = delete;
  BesselBeam(BesselBeam&& obj) = default;
  BesselBeam& operator=(BesselBeam&& obj) = default;

 private:
  core::Vector3 _apex;
  core::Vector3 _vec_n;
  double _theta_z;
  double _amp;
};

/**
 * @brief Gain to group some gains
 */
template <typename T = core::LegacyTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class Grouped final : public core::Gain<T> {
 public:
  /**
   * \brief Decide which device outputs which Gain
   * \param device_id device id
   * \param gain gain
   */
  template <class G>
  std::enable_if_t<std::is_base_of_v<core::Gain<T>, G>> add(const size_t device_id, G& gain) {
    gain.build(_geometry);
    if (device_id < _geometry.num_devices()) this->_props.drives.copy_from(device_id, gain.drives());
  }

  void calc(const core::Geometry<T>& geometry) override {}

  explicit Grouped(const core::Geometry<T>& geometry) : core::Gain<T>(), _geometry(geometry) {
    this->_props.init(_geometry.num_devices() * driver::NUM_TRANS_IN_UNIT);
  }
  ~Grouped() override = default;
  Grouped(const Grouped& v) noexcept = delete;
  Grouped& operator=(const Grouped& obj) = delete;
  Grouped(Grouped&& obj) = default;
  Grouped& operator=(Grouped&& obj) = delete;

 private:
  const core::Geometry<T>& _geometry;
};

}  // namespace autd3::gain
