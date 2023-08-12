// File: backend_cuda.hpp
// Project: gain
// Created Date: 08/06/2023
// Author: Shun Suzuki
// -----
// Last Modified: 11/08/2023
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
  CUDABackend() : Backend() {
    char err[256];
    _ptr = internal::native_methods::AUTDCUDABackend(err);
    if (_ptr._0 == nullptr) throw internal::AUTDException(err);
  }
  ~CUDABackend() override {
    if (_ptr._0 != nullptr) {
      internal::native_methods::AUTDDeleteCUDABackend(_ptr);
      _ptr._0 = nullptr;
    }
  }

  internal::native_methods::GainPtr sdp(const double* foci, const double* amps, const uint64_t size) const override {
    return internal::native_methods::AUTDGainHoloSDPCUDA(this->_ptr, foci, amps, size);
  }
  internal::native_methods::GainPtr sdp_with_alpha(internal::native_methods::GainPtr ptr, double v) const override {
    return internal::native_methods::AUTDGainHoloSDPWithAlphaCUDA(ptr, v);
  }
  internal::native_methods::GainPtr sdp_with_repeat(internal::native_methods::GainPtr ptr, uint32_t v) const override {
    return internal::native_methods::AUTDGainHoloSDPWithRepeatCUDA(ptr, v);
  }
  internal::native_methods::GainPtr sdp_with_lambda(internal::native_methods::GainPtr ptr, double v) const override {
    return internal::native_methods::AUTDGainHoloSDPWithLambdaCUDA(ptr, v);
  }
  internal::native_methods::GainPtr sdp_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const override {
    return internal::native_methods::AUTDGainHoloSDPWithConstraintCUDA(ptr, v.ptr());
  }

  internal::native_methods::GainPtr evp(const double* foci, const double* amps, const uint64_t size) const override {
    return internal::native_methods::AUTDGainHoloEVPCUDA(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr evp_with_gamma(internal::native_methods::GainPtr ptr, double v) const override {
    return internal::native_methods::AUTDGainHoloEVPWithGammaCUDA(ptr, v);
  }

  internal::native_methods::GainPtr evp_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const override {
    return internal::native_methods::AUTDGainHoloEVPWithConstraintCUDA(ptr, v.ptr());
  }

  internal::native_methods::GainPtr gs(const double* foci, const double* amps, const uint64_t size) const override {
    return internal::native_methods::AUTDGainHoloGSCUDA(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr gs_with_repeat(internal::native_methods::GainPtr ptr, uint32_t v) const override {
    return internal::native_methods::AUTDGainHoloGSWithRepeatCUDA(ptr, v);
  }

  internal::native_methods::GainPtr gs_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const override {
    return internal::native_methods::AUTDGainHoloGSWithConstraintCUDA(ptr, v.ptr());
  }

  internal::native_methods::GainPtr gspat(const double* foci, const double* amps, const uint64_t size) const override {
    return internal::native_methods::AUTDGainHoloGSCUDA(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr gspat_with_repeat(internal::native_methods::GainPtr ptr, uint32_t v) const override {
    return internal::native_methods::AUTDGainHoloGSWithRepeatCUDA(ptr, v);
  }

  internal::native_methods::GainPtr gspat_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const override {
    return internal::native_methods::AUTDGainHoloGSWithConstraintCUDA(ptr, v.ptr());
  }

  internal::native_methods::GainPtr naive(const double* foci, const double* amps, const uint64_t size) const override {
    return internal::native_methods::AUTDGainHoloNaiveCUDA(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr naive_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const override {
    return internal::native_methods::AUTDGainHoloNaiveWithConstraintCUDA(ptr, v.ptr());
  }

  internal::native_methods::GainPtr lm(const double* foci, const double* amps, const uint64_t size) const override {
    return internal::native_methods::AUTDGainHoloLMCUDA(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr lm_with_eps1(internal::native_methods::GainPtr ptr, double v) const override {
    return internal::native_methods::AUTDGainHoloLMWithEps1CUDA(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_eps2(internal::native_methods::GainPtr ptr, double v) const override {
    return internal::native_methods::AUTDGainHoloLMWithEps2CUDA(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_tau(internal::native_methods::GainPtr ptr, double v) const override {
    return internal::native_methods::AUTDGainHoloLMWithTauCUDA(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_kmax(internal::native_methods::GainPtr ptr, uint32_t v) const override {
    return internal::native_methods::AUTDGainHoloLMWithKMaxCUDA(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_initial(internal::native_methods::GainPtr ptr, const double* v, uint64_t size) const override {
    return internal::native_methods::AUTDGainHoloLMWithInitialCUDA(ptr, v, size);
  }

  internal::native_methods::GainPtr lm_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const override {
    return internal::native_methods::AUTDGainHoloLMWithConstraintCUDA(ptr, v.ptr());
  }
};

}  // namespace autd3::gain::holo
