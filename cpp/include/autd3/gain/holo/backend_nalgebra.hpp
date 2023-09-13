// File: backend_nalgebra.hpp
// Project: holo
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/gain/holo/backend.hpp"
#include "autd3/gain/holo/constraint.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain::holo {

/**
 * @brief Backend using [Nalgebra](https://nalgebra.org/)
 */
class NalgebraBackend final : public Backend {
 public:
  NalgebraBackend() : Backend(internal::native_methods::AUTDNalgebraBackend()) {}
  ~NalgebraBackend() override {
    if (_ptr._0 != nullptr) {
      AUTDDeleteNalgebraBackend(_ptr);
      _ptr._0 = nullptr;
    }
  }
  NalgebraBackend(const NalgebraBackend& v) noexcept = delete;
  NalgebraBackend& operator=(const NalgebraBackend& obj) = delete;
  NalgebraBackend(NalgebraBackend&& obj) = default;
  NalgebraBackend& operator=(NalgebraBackend&& obj) = default;

  internal::native_methods::GainPtr sdp(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloSDP(this->_ptr, foci, amps, size);
  }

  [[nodiscard]] internal::native_methods::GainPtr sdp_with_alpha(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloSDPWithAlpha(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr sdp_with_repeat(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloSDPWithRepeat(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr sdp_with_lambda(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloSDPWithLambda(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr sdp_with_constraint(const internal::native_methods::GainPtr ptr,
                                                                      const AmplitudeConstraint v) const override {
    return AUTDGainHoloSDPWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr evp(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloEVP(this->_ptr, foci, amps, size);
  }

  [[nodiscard]] internal::native_methods::GainPtr evp_with_gamma(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloEVPWithGamma(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr evp_with_constraint(const internal::native_methods::GainPtr ptr,
                                                                      const AmplitudeConstraint v) const override {
    return AUTDGainHoloEVPWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr gs(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloGS(this->_ptr, foci, amps, size);
  }

  [[nodiscard]] internal::native_methods::GainPtr gs_with_repeat(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloGSWithRepeat(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr gs_with_constraint(const internal::native_methods::GainPtr ptr,
                                                                     const AmplitudeConstraint v) const override {
    return AUTDGainHoloGSWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr gspat(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloGS(this->_ptr, foci, amps, size);
  }

  [[nodiscard]] internal::native_methods::GainPtr gspat_with_repeat(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloGSWithRepeat(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr gspat_with_constraint(const internal::native_methods::GainPtr ptr,
                                                                        const AmplitudeConstraint v) const override {
    return AUTDGainHoloGSWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr naive(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloNaive(this->_ptr, foci, amps, size);
  }

  [[nodiscard]] internal::native_methods::GainPtr naive_with_constraint(const internal::native_methods::GainPtr ptr,
                                                                        const AmplitudeConstraint v) const override {
    return AUTDGainHoloNaiveWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr lm(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloLM(this->_ptr, foci, amps, size);
  }

  [[nodiscard]] internal::native_methods::GainPtr lm_with_eps1(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloLMWithEps1(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr lm_with_eps2(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloLMWithEps2(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr lm_with_tau(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloLMWithTau(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr lm_with_k_max(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloLMWithKMax(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_initial(const internal::native_methods::GainPtr ptr, const double* v,
                                                    const uint64_t size) const override {
    return AUTDGainHoloLMWithInitial(ptr, v, size);
  }

  [[nodiscard]] internal::native_methods::GainPtr lm_with_constraint(const internal::native_methods::GainPtr ptr,
                                                                     const AmplitudeConstraint v) const override {
    return AUTDGainHoloLMWithConstraint(ptr, v.ptr());
  }
};
}  // namespace autd3::gain::holo
