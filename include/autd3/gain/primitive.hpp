// File: primitive.hpp
// Project: gain
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <initializer_list>
#include <memory>
#include <unordered_map>
#include <utility>
#include <vector>

#include "autd3/core/gain.hpp"

namespace autd3::gain {

/**
 * @brief Gain to produce nothing
 */
class Null final : public core::Gain {
 public:
  Null() noexcept {}

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override { return {geometry.num_transducers(), driver::Drive{0, 0}}; }

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

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override {
    const auto sound_speed = geometry.sound_speed;
    return core::Gain::transform(geometry, [&](const auto& transducer) {
      const auto phase = transducer.align_phase_at(_point, sound_speed);
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

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override {
    _vec_n.normalize();
    core::Vector3 v = core::Vector3::UnitZ().cross(_vec_n);
    const auto theta_v = std::asin(v.norm());
    v.normalize();
    const Eigen::AngleAxis<driver::autd3_float_t> rot(-theta_v, v);

    const auto sound_speed = geometry.sound_speed;
    return core::Gain::transform(geometry, [&](const auto& transducer) {
      const auto r = transducer.position() - this->_apex;
      const auto rr = rot * r;
      const auto d = std::sin(_theta_z) * std::sqrt(rr.x() * rr.x() + rr.y() * rr.y()) - std::cos(_theta_z) * rr.z();
      const auto phase = d * transducer.wavenumber(sound_speed);
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

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override {
    const auto sound_speed = geometry.sound_speed;
    return core::Gain::transform(geometry, [&](const auto& transducer) {
      const auto dist = transducer.position().dot(_direction);
      const auto phase = dist * transducer.wavenumber(sound_speed);
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
#ifdef AUTD3_CAPI
  void add(const size_t device_idx, core::Gain* b) { _gains.insert_or_assign(device_idx, b); }
#else
  /**
   * \brief Decide which device outputs which Gain
   * \param device_idx device index
   * \param gain gain
   */
  template <class G>
  void add(const size_t device_idx, G&& gain) {
    static_assert(std::is_base_of_v<core::Gain, std::remove_reference_t<G>>, "This is not Gain");
    _gains.insert_or_assign(device_idx, std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain)));
  }

  /**
   * \brief Decide which device outputs which Gain
   * \param idx_list device index list
   * \param gain gain
   */
  template <class G>
  void add(const std::initializer_list<size_t> idx_list, G&& gain) {
    for (const auto idx : idx_list) add<G>(idx, gain);
  }

  /**
   * \brief Decide which device outputs which Gain
   * \param device_idx device index
   * \param gain gain
   */
  void add(const size_t device_idx, std::shared_ptr<core::Gain> b) { _gains.insert_or_assign(device_idx, std::move(b)); }

  /**
   * \brief Decide which device outputs which Gain
   * \param idx_list device index list
   * \param gain gain
   */
  void add(const std::initializer_list<size_t> idx_list, std::shared_ptr<core::Gain> b) {
    for (const auto idx : idx_list) add(idx, b);
  }
#endif

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override {
    std::vector<driver::Drive> drives(geometry.num_transducers(), driver::Drive{0, 0});
    std::for_each(_gains.begin(), _gains.end(), [&drives, geometry](const auto& g) {
      const auto& [device_id, gain] = g;
      const auto d = gain->calc(geometry);
      const auto start = device_id == 0 ? 0 : geometry.device_map()[device_id - 1];
      std::memcpy(&drives[start], &d[start], sizeof(autd3::driver::Drive) * geometry.device_map()[device_id]);
    });
    return drives;
  }

  Grouped() : core::Gain() {}
  ~Grouped() override = default;
  Grouped(const Grouped& v) noexcept = delete;
  Grouped& operator=(const Grouped& obj) = delete;
  Grouped(Grouped&& obj) = delete;
  Grouped& operator=(Grouped&& obj) = delete;

 private:
#ifdef AUTD3_CAPI
  std::unordered_map<size_t, Gain*> _gains{};
#else
  std::unordered_map<size_t, std::shared_ptr<Gain>> _gains{};
#endif
};

/**
 * @brief Gain to drive a tranducer
 */
class TransducerTest final : public core::Gain {
 public:
  TransducerTest() noexcept : _map(){};

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override {
    std::vector<driver::Drive> drives(geometry.num_transducers(), driver::Drive{0, 0});
    std::for_each(_map.begin(), _map.end(), [&drives](const auto& v) {
      const auto& [id, value] = v;
      drives[id].amp = value.first;
      drives[id].phase = value.second;
    });
    return drives;
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

template <typename T>
class Cache final : public core::Gain {
 public:
  template <typename... Args>
  explicit Cache(Args&&... args) : gain(std::forward<Args>(args)...) {}

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override {
    if (!_built) {
      _drives = gain.calc(geometry);
      _built = true;
    }
    std::vector<driver::Drive> drives;
    drives.reserve(_drives.size());
    std::copy(_drives.begin(), _drives.end(), std::back_inserter(drives));
    return drives;
  }

  std::vector<driver::Drive> recalc(const core::Geometry& geometry) {
    _built = false;
    return calc(geometry);
  }

  /**
   * @brief Getter function for the data of duty ratio and phase of each transducers
   */
  [[nodiscard]] const std::vector<driver::Drive>& drives() const { return _drives; }

  /**
   * @brief [Advanced] Getter function for the data of duty ratio and phase of each transducers
   * @details Call calc before using this function to initialize drive data.
   */
  std::vector<driver::Drive>& drives() { return _drives; }

  [[nodiscard]] std::vector<driver::Drive>::const_iterator begin() const noexcept { return _drives.begin(); }
  [[nodiscard]] std::vector<driver::Drive>::const_iterator end() const noexcept { return _drives.end(); }
  [[nodiscard]] std::vector<driver::Drive>::iterator begin() noexcept { return _drives.begin(); }
  [[nodiscard]] std::vector<driver::Drive>::iterator end() noexcept { return _drives.end(); }
  [[nodiscard]] const driver::Drive& operator[](const size_t i) const { return _drives[i]; }
  [[nodiscard]] driver::Drive& operator[](const size_t i) { return _drives[i]; }

  T gain;

 private:
  bool _built{false};
  std::vector<driver::Drive> _drives;
};

}  // namespace autd3::gain
