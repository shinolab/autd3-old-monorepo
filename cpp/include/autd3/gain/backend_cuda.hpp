// File: backend_cuda.hpp
// Project: gain
// Created Date: 08/06/2023
// Author: Shun Suzuki
// -----
// Last Modified: 08/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/gain/holo.hpp"
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
      AUTDDeleteCUDABackend(_ptr);
      _ptr._0 = nullptr;
    }
  }
  CUDABackend(const CUDABackend& v) noexcept = delete;
  CUDABackend& operator=(const CUDABackend& obj) = delete;
  CUDABackend(CUDABackend&& obj) = default;
  CUDABackend& operator=(CUDABackend&& obj) = default;

  internal::native_methods::GainPtr sdp(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloSDPCUDA(this->_ptr, foci, amps, size);
  }
  internal::native_methods::GainPtr sdp_with_alpha(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloSDPWithAlphaCUDA(ptr, v);
  }
  internal::native_methods::GainPtr sdp_with_repeat(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloSDPWithRepeatCUDA(ptr, v);
  }
  internal::native_methods::GainPtr sdp_with_lambda(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloSDPWithLambdaCUDA(ptr, v);
  }
  internal::native_methods::GainPtr sdp_with_constraint(const internal::native_methods::GainPtr ptr, const AmplitudeConstraint v) const override {
    return AUTDGainHoloSDPWithConstraintCUDA(ptr, v.ptr());
  }

  internal::native_methods::GainPtr evp(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloEVPCUDA(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr evp_with_gamma(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloEVPWithGammaCUDA(ptr, v);
  }

  internal::native_methods::GainPtr evp_with_constraint(const internal::native_methods::GainPtr ptr, const AmplitudeConstraint v) const override {
    return AUTDGainHoloEVPWithConstraintCUDA(ptr, v.ptr());
  }

  internal::native_methods::GainPtr gs(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloGSCUDA(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr gs_with_repeat(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloGSWithRepeatCUDA(ptr, v);
  }

  internal::native_methods::GainPtr gs_with_constraint(const internal::native_methods::GainPtr ptr, const AmplitudeConstraint v) const override {
    return AUTDGainHoloGSWithConstraintCUDA(ptr, v.ptr());
  }

  internal::native_methods::GainPtr gspat(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloGSCUDA(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr gspat_with_repeat(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloGSWithRepeatCUDA(ptr, v);
  }

  internal::native_methods::GainPtr gspat_with_constraint(const internal::native_methods::GainPtr ptr, const AmplitudeConstraint v) const override {
    return AUTDGainHoloGSWithConstraintCUDA(ptr, v.ptr());
  }

  internal::native_methods::GainPtr naive(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloNaiveCUDA(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr naive_with_constraint(const internal::native_methods::GainPtr ptr, const AmplitudeConstraint v) const override {
    return AUTDGainHoloNaiveWithConstraintCUDA(ptr, v.ptr());
  }

  internal::native_methods::GainPtr lm(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloLMCUDA(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr lm_with_eps1(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloLMWithEps1CUDA(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_eps2(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloLMWithEps2CUDA(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_tau(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloLMWithTauCUDA(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_k_max(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloLMWithKMaxCUDA(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_initial(const internal::native_methods::GainPtr ptr, const double* v,
                                                    const uint64_t size) const override {
    return AUTDGainHoloLMWithInitialCUDA(ptr, v, size);
  }

  internal::native_methods::GainPtr lm_with_constraint(const internal::native_methods::GainPtr ptr, const AmplitudeConstraint v) const override {
    return AUTDGainHoloLMWithConstraintCUDA(ptr, v.ptr());
  }
};

}  // namespace autd3::gain::holo
