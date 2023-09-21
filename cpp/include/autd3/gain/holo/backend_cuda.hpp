// File: backend_cuda.hpp
// Project: gain
// Created Date: 08/06/2023
// Author: Shun Suzuki
// -----
// Last Modified: 21/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/gain/holo/backend.hpp"
#include "autd3/gain/holo/constraint.hpp"
#include "autd3/internal/exception.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain::holo {

/**
 * @brief Backend using CUDA
 */
class CUDABackend final : public Backend {
 public:
  CUDABackend() {
    char err[256];
    _ptr = internal::native_methods::AUTDCUDABackend(err);
    if (_ptr._0 == nullptr) throw internal::AUTDException(err);
  }
  ~CUDABackend() override {
    if (_ptr._0 != nullptr) {
      AUTDCUDABackendDelete(_ptr);
      _ptr._0 = nullptr;
    }
  }
  CUDABackend(const CUDABackend& v) noexcept = delete;
  CUDABackend& operator=(const CUDABackend& obj) = delete;
  CUDABackend(CUDABackend&& obj) = default;
  CUDABackend& operator=(CUDABackend&& obj) = default;

  internal::native_methods::GainPtr sdp(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloCUDASDP(this->_ptr, foci, amps, size);
  }

  [[nodiscard]] internal::native_methods::GainPtr sdp_with_alpha(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloCUDASDPWithAlpha(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr sdp_with_repeat(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloCUDASDPWithRepeat(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr sdp_with_lambda(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloCUDASDPWithLambda(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr sdp_with_constraint(const internal::native_methods::GainPtr ptr,
                                                                      const AmplitudeConstraint v) const override {
    return AUTDGainHoloCUDASDPWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr evp(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloCUDAEVP(this->_ptr, foci, amps, size);
  }

  [[nodiscard]] internal::native_methods::GainPtr evp_with_gamma(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloCUDAEVPWithGamma(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr evp_with_constraint(const internal::native_methods::GainPtr ptr,
                                                                      const AmplitudeConstraint v) const override {
    return AUTDGainHoloCUDAEVPWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr gs(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloCUDAGS(this->_ptr, foci, amps, size);
  }

  [[nodiscard]] internal::native_methods::GainPtr gs_with_repeat(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloCUDAGSWithRepeat(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr gs_with_constraint(const internal::native_methods::GainPtr ptr,
                                                                     const AmplitudeConstraint v) const override {
    return AUTDGainHoloCUDAGSWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr gspat(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloCUDAGS(this->_ptr, foci, amps, size);
  }

  [[nodiscard]] internal::native_methods::GainPtr gspat_with_repeat(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloCUDAGSWithRepeat(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr gspat_with_constraint(const internal::native_methods::GainPtr ptr,
                                                                        const AmplitudeConstraint v) const override {
    return AUTDGainHoloCUDAGSWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr naive(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloCUDANaive(this->_ptr, foci, amps, size);
  }

  [[nodiscard]] internal::native_methods::GainPtr naive_with_constraint(const internal::native_methods::GainPtr ptr,
                                                                        const AmplitudeConstraint v) const override {
    return AUTDGainHoloCUDANaiveWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr lm(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloCUDALM(this->_ptr, foci, amps, size);
  }

  [[nodiscard]] internal::native_methods::GainPtr lm_with_eps1(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloCUDALMWithEps1(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr lm_with_eps2(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloCUDALMWithEps2(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr lm_with_tau(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloCUDALMWithTau(ptr, v);
  }

  [[nodiscard]] internal::native_methods::GainPtr lm_with_k_max(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloCUDALMWithKMax(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_initial(const internal::native_methods::GainPtr ptr, const double* v,
                                                    const uint64_t size) const override {
    return AUTDGainHoloCUDALMWithInitial(ptr, v, size);
  }

  [[nodiscard]] internal::native_methods::GainPtr lm_with_constraint(const internal::native_methods::GainPtr ptr,
                                                                     const AmplitudeConstraint v) const override {
    return AUTDGainHoloCUDALMWithConstraint(ptr, v.ptr());
  }
};

}  // namespace autd3::gain::holo
