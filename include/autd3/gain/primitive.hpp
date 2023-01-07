// File: primitive.hpp
// Project: gain
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <unordered_map>
#include <utility>
#include <vector>

#ifdef AUTD3_PARALLEL_FOR
#include <execution>
#endif

#include "autd3/core/gain.hpp"

namespace autd3::gain {

/**
 * @brief Gain to produce nothing
 */
class Null final : public core::Gain {
 public:
  Null() noexcept {}

  void calc(const core::Geometry& geometry) override {
#ifdef AUTD3_PARALLEL_FOR
    std::transform(std::execution::par_unseq, geometry.begin(), geometry.end(), this->begin(), [&](const auto& transducer) {
#else
    std::transform(geometry.begin(), geometry.end(), this->begin(), [&](const auto& transducer) {
#endif
      return driver::Drive{0, 0};
    });
  }

  ~Null() override = default;
  Null(const Null& v) noexcept = default;
  Null& operator=(const Null& obj) = default;
  Null(Null&& obj) = default;
  Null& operator=(Null&& obj) = default;
};

/**
 * @brief Gain to produce single focal point
 */
class Focus final : public core::Gain {
 public:
  /**
   * @param[in] point focal point
   * @param[in] amp amplitude of the focus (from 0.0 to 1.0)
   */
  explicit Focus(core::Vector3 point, const driver::autd3_float_t amp = 1) : _point(std::move(point)), _amp(amp) {}

  void calc(const core::Geometry& geometry) override {
    const auto sound_speed = geometry.sound_speed;
#ifdef AUTD3_PARALLEL_FOR
    std::transform(std::execution::par_unseq, geometry.begin(), geometry.end(), this->begin(), [&](const auto& transducer) {
#else
    std::transform(geometry.begin(), geometry.end(), this->begin(), [&](const auto& transducer) {
#endif
      const auto dist = (_point - transducer.position()).norm();
      const auto phase = transducer.align_phase_at(dist, sound_speed);
      return driver::Drive{phase, _amp};
    });
  }

  ~Focus() override = default;
  Focus(const Focus& v) noexcept = default;
  Focus& operator=(const Focus& obj) = default;
  Focus(Focus&& obj) = default;
  Focus& operator=(Focus&& obj) = default;

 private:
  core::Vector3 _point;
  driver::autd3_float_t _amp;
};

/**
 * @brief Gain to produce Bessel Beam
 */
class BesselBeam final : public core::Gain {
 public:
  /**
   * @param[in] apex apex of the conical wavefront of the beam
   * @param[in] vec_n direction of the beam
   * @param[in] theta_z angle between the side of the cone and the plane perpendicular to direction of the beam
   * @param[in] amp amplitude of the wave (from 0.0 to 1.0)
   */
  explicit BesselBeam(core::Vector3 apex, core::Vector3 vec_n, const driver::autd3_float_t theta_z, const driver::autd3_float_t amp = 1)
      : core::Gain(), _apex(std::move(apex)), _vec_n(std::move(vec_n)), _theta_z(theta_z), _amp(amp) {}

  void calc(const core::Geometry& geometry) override {
    _vec_n.normalize();
    core::Vector3 v = core::Vector3::UnitZ().cross(_vec_n);
    const auto theta_v = std::asin(v.norm());
    v.normalize();
    const Eigen::AngleAxis<driver::autd3_float_t> rot(-theta_v, v);

    const auto sound_speed = geometry.sound_speed;
#ifdef AUTD3_PARALLEL_FOR
    std::transform(std::execution::par_unseq, geometry.begin(), geometry.end(), this->begin(), [&](const auto& transducer) {
#else
    std::transform(geometry.begin(), geometry.end(), this->begin(), [&](const auto& transducer) {
#endif
      const auto r = transducer.position() - this->_apex;
      const auto rr = rot * r;
      const auto d = std::sin(_theta_z) * std::sqrt(rr.x() * rr.x() + rr.y() * rr.y()) - std::cos(_theta_z) * rr.z();
      const auto phase = transducer.align_phase_at(d, sound_speed);
      return driver::Drive{phase, _amp};
    });
  }

  ~BesselBeam() override = default;
  BesselBeam(const BesselBeam& v) noexcept = default;
  BesselBeam& operator=(const BesselBeam& obj) = default;
  BesselBeam(BesselBeam&& obj) = default;
  BesselBeam& operator=(BesselBeam&& obj) = default;

 private:
  core::Vector3 _apex;
  core::Vector3 _vec_n;
  driver::autd3_float_t _theta_z;
  driver::autd3_float_t _amp;
};

/**
 * @brief Gain to create plane wave
 */
class PlaneWave final : public core::Gain {
 public:
  /**
   * @param[in] direction wave direction
   * @param[in] amp amplitude of the wave (from 0.0 to 1.0)
   */
  explicit PlaneWave(core::Vector3 direction, const driver::autd3_float_t amp = 1) noexcept : _direction(std::move(direction)), _amp(amp) {}

  void calc(const core::Geometry& geometry) override {
    const auto sound_speed = geometry.sound_speed;
#ifdef AUTD3_PARALLEL_FOR
    std::transform(std::execution::par_unseq, geometry.begin(), geometry.end(), this->begin(), [&](const auto& transducer) {
#else
    std::transform(geometry.begin(), geometry.end(), this->begin(), [&](const auto& transducer) {
#endif
      const auto dist = transducer.position().dot(_direction);
      const auto phase = transducer.align_phase_at(dist, sound_speed);
      return driver::Drive{phase, _amp};
    });
  }

  ~PlaneWave() override = default;
  PlaneWave(const PlaneWave& v) noexcept = default;
  PlaneWave& operator=(const PlaneWave& obj) = default;
  PlaneWave(PlaneWave&& obj) = default;
  PlaneWave& operator=(PlaneWave&& obj) = default;

 private:
  core::Vector3 _direction;
  driver::autd3_float_t _amp;
};

/**
 * @brief Gain to group some gains
 */
class Grouped final : public core::Gain {
 public:
  /**
   * \brief Decide which device outputs which Gain
   * \param device_id device id
   * \param gain gain
   */
  template <class G>
  void add(const size_t device_id, G&& gain) {
    static_assert(std::is_base_of_v<core::Gain, std::remove_reference_t<G>>, "This is not Gain");
    _gains.insert_or_assign(device_id, std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain)));
  }

  void calc(const core::Geometry& geometry) override {
    for (const auto& [device_id, gain] : _gains) {
      gain->init(_mode, geometry);
      const auto start = device_id == 0 ? 0 : geometry.device_map()[device_id - 1];
      std::memcpy(_op->drives.data() + start, gain->drives().data() + start, sizeof(driver::Drive) * geometry.device_map()[device_id]);
    }
  }

  Grouped() : core::Gain() {}
  ~Grouped() override = default;
  Grouped(const Grouped& v) noexcept = delete;
  Grouped& operator=(const Grouped& obj) = delete;
  Grouped(Grouped&& obj) = delete;
  Grouped& operator=(Grouped&& obj) = delete;

 private:
  std::unordered_map<size_t, std::shared_ptr<Gain>> _gains{};
};

/**
 * @brief Gain to drive a tranducer
 */
class TransducerTest final : public core::Gain {
 public:
  TransducerTest() noexcept : _map(){};

  void calc(const core::Geometry& geometry) override {
    for (const auto& [id, value] : _map) {
      _op->drives[id].amp = value.first;
      _op->drives[id].phase = value.second;
    }
  }

  /**
   * @param[in] tr_idx transducer index
   * @param[in] amp amplitude (from 0.0 to 1.0)
   * @param[in] phase phase in radian
   */
  void set(const size_t tr_idx, const driver::autd3_float_t amp, const driver::autd3_float_t phase) {
    _map.insert_or_assign(tr_idx, std::make_pair(amp, phase));
  }

  ~TransducerTest() override = default;
  TransducerTest(const TransducerTest& v) noexcept = default;
  TransducerTest& operator=(const TransducerTest& obj) = default;
  TransducerTest(TransducerTest&& obj) = default;
  TransducerTest& operator=(TransducerTest&& obj) = default;

 private:
  std::unordered_map<size_t, std::pair<driver::autd3_float_t, driver::autd3_float_t>> _map;
};

}  // namespace autd3::gain
