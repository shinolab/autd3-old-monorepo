// File: holo.hpp
// Project: gain
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
#include <utility>
#include <vector>

#include "autd3/core/gain.hpp"
#include "autd3/gain/backend.hpp"

/**
 * @brief `holo` namespace provides Holo gains
 */
namespace autd3::gain::holo {

/**
 * @brief Amplitude constraint
 */
struct AmplitudeConstraint {
  AmplitudeConstraint() = default;
  virtual ~AmplitudeConstraint() = default;
  AmplitudeConstraint(const AmplitudeConstraint& v) noexcept = default;
  AmplitudeConstraint& operator=(const AmplitudeConstraint& obj) = default;
  AmplitudeConstraint(AmplitudeConstraint&& obj) = default;
  AmplitudeConstraint& operator=(AmplitudeConstraint&& obj) = default;
  [[nodiscard]] virtual driver::float_t convert(driver::float_t raw, driver::float_t max) const = 0;
};

using ConstraintPtr = std::unique_ptr<AmplitudeConstraint>;

/**
 * @brief AmplitudeConstraint to do nothing
 */
struct DontCare final : AmplitudeConstraint {
  DontCare() = default;
  ~DontCare() override = default;
  DontCare(const DontCare& v) noexcept = default;
  DontCare& operator=(const DontCare& obj) = default;
  DontCare(DontCare&& obj) = default;
  DontCare& operator=(DontCare&& obj) = default;

  [[nodiscard]] ConstraintPtr build() const { return std::make_unique<DontCare>(); }

  [[nodiscard]] driver::float_t convert(const driver::float_t raw, const driver::float_t) const override { return raw; }
};

/**
 * @brief AmplitudeConstraint to normalize to the largest amplitude
 */
struct Normalize final : AmplitudeConstraint {
  Normalize() = default;
  ~Normalize() override = default;
  Normalize(const Normalize& v) noexcept = default;
  Normalize& operator=(const Normalize& obj) = default;
  Normalize(Normalize&& obj) = default;
  Normalize& operator=(Normalize&& obj) = default;

  [[nodiscard]] ConstraintPtr build() const { return std::make_unique<Normalize>(); }

  [[nodiscard]] driver::float_t convert(const driver::float_t raw, const driver::float_t max) const override { return raw / max; }
};

/**
 * @brief AmplitudeConstraint to give the same amplitude to all transducers
 */
struct Uniform final : AmplitudeConstraint {
  explicit Uniform(const driver::float_t value) : AmplitudeConstraint(), _value(value) {}
  ~Uniform() override = default;
  Uniform(const Uniform& v) noexcept = default;
  Uniform& operator=(const Uniform& obj) = default;
  Uniform(Uniform&& obj) = default;
  Uniform& operator=(Uniform&& obj) = default;

  [[nodiscard]] ConstraintPtr build() const { return std::make_unique<Uniform>(_value); }

  [[nodiscard]] driver::float_t convert(const driver::float_t, const driver::float_t) const override { return _value; }

 private:
  driver::float_t _value;
};

/**
 * @brief AmplitudeConstraint to clamp amplitude in [0, 1]
 */
using Clamp = DontCare;

/**
 * @brief Gain to produce multiple focal points
 */
class Holo : public core::Gain {
 public:
  explicit Holo(BackendPtr backend, ConstraintPtr constraint = Normalize().build())
      : constraint(std::move(constraint)), _backend(std::move(backend)) {}
  ~Holo() override = default;
  Holo(const Holo& v) noexcept = delete;
  Holo& operator=(const Holo& obj) = delete;
  Holo(Holo&& obj) = default;
  Holo& operator=(Holo&& obj) = default;

  /**
   * @brief Add focus position and amplitude of focus
   */
  void add_focus(const core::Vector3& focus, const driver::float_t amp) {
    _foci.emplace_back(focus);
    _amps.emplace_back(amp, driver::float_t{0});
  }

  [[nodiscard]] const std::vector<core::Vector3>& foci() const { return this->_foci; }
  [[nodiscard]] const std::vector<complex>& amplitudes() const { return this->_amps; }

  ConstraintPtr constraint{std::make_unique<Normalize>()};

 protected:
  BackendPtr _backend;
  std::vector<core::Vector3> _foci;
  std::vector<complex> _amps;
};

/**
 * @brief Gain to produce multiple focal points with SDP method.
 * Refer to Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch
 * perception produced by airborne ultrasonic haptic hologram." 2015 IEEE
 * World Haptics Conference (WHC). IEEE, 2015.
 */
class SDP final : public Holo {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit SDP(BackendPtr backend)
      : Holo(std::move(backend), std::make_unique<Normalize>()),
        alpha(static_cast<driver::float_t>(1e-3)),
        lambda(static_cast<driver::float_t>(0.9)),
        repeat(100) {}

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override;

  driver::float_t alpha;
  driver::float_t lambda;
  size_t repeat;
};

/**
 * @brief Gain to produce multiple focal points with EVP method.
 * Refer to Long, Benjamin, et al. "Rendering volumetric haptic shapes in mid-air
 * using ultrasound." ACM Transactions on Graphics (TOG) 33.6 (2014): 1-10.
 */
class EVP final : public Holo {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit EVP(BackendPtr backend) : Holo(std::move(backend), std::make_unique<Uniform>(driver::float_t{1})), gamma(1) {}

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override;

  driver::float_t gamma;
};

/**
 * @brief Gain to produce multiple focal points with Linear Synthesis Scheme.
 */
class LSS final : public Holo {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit LSS(BackendPtr backend) : Holo(std::move(backend), std::make_unique<Normalize>()) {}

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override;
};

/**
 * @brief Alias of LSS
 */
using Naive = LSS;

/**
 * @brief Gain to produce multiple focal points with GS method.
 * Refer to Asier Marzo and Bruce W Drinkwater, "Holographic acoustic
 * tweezers," Proceedings of theNational Academy of Sciences, 116(1):84–89, 2019.
 */
class GS final : public Holo {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit GS(BackendPtr backend) : Holo(std::move(backend), std::make_unique<Normalize>()), repeat(100) {}

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override;

  size_t repeat;
};

/**
 * @brief Gain to produce multiple focal points with GS-PAT method (not yet been implemented with GPU).
 * Refer to Diego Martinez Plasencia et al. "Gs-pat: high-speed multi-point
 * sound-fields for phased arrays of transducers," ACMTrans-actions on
 * Graphics (TOG), 39(4):138–1, 2020.
 */
class GSPAT final : public Holo {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit GSPAT(BackendPtr backend) : Holo(std::move(backend), std::make_unique<Normalize>()), repeat(100) {}

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override;

  size_t repeat;
};

/**
 * @brief Gain to produce multiple focal points with Levenberg-Marquardt method.
 * Refer to K.Levenberg, “A method for the solution of certain non-linear problems in
 * least squares,” Quarterly of applied mathematics, vol.2, no.2, pp.164–168, 1944.
 * D.W.Marquardt, “An algorithm for least-squares estimation of non-linear parameters,” Journal of the society for Industrial and
 * AppliedMathematics, vol.11, no.2, pp.431–441, 1963.
 * K.Madsen, H.Nielsen, and O.Tingleff, “Methods for non-linear least squares problems (2nd ed.),” 2004.
 */
class LM final : public Holo {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit LM(BackendPtr backend)
      : Holo(std::move(backend), std::make_unique<Normalize>()),
        eps_1(static_cast<driver::float_t>(1e-8)),
        eps_2(static_cast<driver::float_t>(1e-8)),
        tau(static_cast<driver::float_t>(1e-3)),
        k_max(5) {}

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override;

  driver::float_t eps_1;
  driver::float_t eps_2;
  driver::float_t tau;
  size_t k_max;
  std::vector<driver::float_t> initial;
};

/**
 * @brief Gain to produce multiple focal points with Greedy algorithm.
 * Refer to Shun suzuki, et al. “Radiation Pressure Field Reconstruction for Ultrasound Midair Haptics by Greedy Algorithm with Brute-Force Search,”
 * in IEEE Transactions on Haptics, doi: 10.1109/TOH.STM.3076489
 * @details This method is computed on the CPU.
 */
class Greedy final : public Holo {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit Greedy(BackendPtr backend)
      : Holo(std::move(backend), std::make_unique<Uniform>(driver::float_t{1})),
        phase_div(16),
        objective([](const VectorXd& target, const VectorXc& p) { return (target - p.cwiseAbs()).cwiseAbs().sum(); }) {}

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override;

  size_t phase_div;
  std::function<driver::float_t(const VectorXd&, const VectorXc&)> objective;
};

/**
 * @brief Gain to produce multiple focal points, applying greedy algorithm on linear synthesis scheme.
 * Refer to Jianyu Chen, et al. Sound Pressure Field Reconstruction for Ultrasound Phased Array by Linear Synthesis Scheme Optimization,”
 * in Haptics: Science, Technology, Applications. EuroHaptics 2022. https://doi.org/10.1007/978-3-031-06249-0_17
 */
class LSSGreedy final : public Holo {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit LSSGreedy(BackendPtr backend)
      : Holo(std::move(backend), std::make_unique<Uniform>(driver::float_t{1})),
        phase_div(16),
        objective([](const VectorXd& target, const VectorXc& p) { return (target - p.cwiseAbs()).cwiseAbs().sum(); }) {}

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override;

  size_t phase_div;
  std::function<driver::float_t(const VectorXd&, const VectorXc&)> objective;
};

/**
 * @brief Gain to produce multiple focal points with Acoustic Power Optimization method.
 * Refer to Keisuke Hasegawa, Hiroyuki Shinoda, and Takaaki Nara. Volumetric acoustic holography and its application to self-positioning by single
 * channel measurement.Journal of Applied Physics,127(24):244904, 2020.7
 */
class APO final : public Holo {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit APO(BackendPtr backend)
      : Holo(std::move(backend), std::make_unique<Normalize>()),
        eps(static_cast<driver::float_t>(1e-8)),
        lambda(1.0),
        k_max(200),
        line_search_max(100) {}

  std::vector<driver::Drive> calc(const core::Geometry& geometry) override;

  driver::float_t eps;
  driver::float_t lambda;
  size_t k_max;
  size_t line_search_max;
};

}  // namespace autd3::gain::holo
