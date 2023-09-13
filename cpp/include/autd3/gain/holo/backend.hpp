// File: backend.hpp
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

#include "autd3/gain/holo/constraint.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain::holo {

/**
 * @brief Calculation backend
 */
class Backend {
 public:
  Backend() noexcept : _ptr(internal::native_methods::BackendPtr{nullptr}) {}
  explicit Backend(const internal::native_methods::BackendPtr ptr) noexcept : _ptr(ptr) {}
  Backend(const Backend& obj) = default;
  Backend& operator=(const Backend& obj) = default;
  Backend(Backend&& obj) = default;
  Backend& operator=(Backend&& obj) = default;
  virtual ~Backend() = default;

  [[nodiscard]] internal::native_methods::BackendPtr ptr() const { return _ptr; }

  [[nodiscard]] virtual internal::native_methods::GainPtr sdp(const double* foci, const double* amps, uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr sdp_with_alpha(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr sdp_with_repeat(internal::native_methods::GainPtr ptr, uint32_t v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr sdp_with_lambda(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr sdp_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const = 0;

  [[nodiscard]] virtual internal::native_methods::GainPtr evp(const double* foci, const double* amps, uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr evp_with_gamma(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr evp_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const = 0;

  [[nodiscard]] virtual internal::native_methods::GainPtr gs(const double* foci, const double* amps, uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr gs_with_repeat(internal::native_methods::GainPtr ptr, uint32_t v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr gs_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const = 0;

  [[nodiscard]] virtual internal::native_methods::GainPtr gspat(const double* foci, const double* amps, uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr gspat_with_repeat(internal::native_methods::GainPtr ptr, uint32_t v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr gspat_with_constraint(internal::native_methods::GainPtr ptr,
                                                                                AmplitudeConstraint v) const = 0;

  [[nodiscard]] virtual internal::native_methods::GainPtr naive(const double* foci, const double* amps, uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr naive_with_constraint(internal::native_methods::GainPtr ptr,
                                                                                AmplitudeConstraint v) const = 0;

  [[nodiscard]] virtual internal::native_methods::GainPtr lm(const double* foci, const double* amps, uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_eps1(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_eps2(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_tau(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_k_max(internal::native_methods::GainPtr ptr, uint32_t v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_initial(internal::native_methods::GainPtr ptr, const double* v,
                                                                          uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const = 0;

 protected:
  internal::native_methods::BackendPtr _ptr;
};

}  // namespace autd3::gain::holo
