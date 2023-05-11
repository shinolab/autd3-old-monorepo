// File: holo_gain.h
// Project: gain_holo
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 02/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "../base/header.hpp"

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD void AUTDEigenBackend(OUT void** out);
EXPORT_AUTD void AUTDDeleteBackend(IN const void* backend);
EXPORT_AUTD void AUTDGainHoloSDP(OUT void** gain, IN const void* backend, IN autd3_float_t alpha, IN autd3_float_t lambda, IN uint64_t repeat);
EXPORT_AUTD void AUTDGainHoloEVP(OUT void** gain, IN const void* backend, IN autd3_float_t gamma);
EXPORT_AUTD void AUTDGainHoloNaive(OUT void** gain, IN const void* backend);
EXPORT_AUTD void AUTDGainHoloGS(OUT void** gain, IN const void* backend, IN uint64_t repeat);
EXPORT_AUTD void AUTDGainHoloGSPAT(OUT void** gain, IN const void* backend, IN uint64_t repeat);
EXPORT_AUTD void AUTDGainHoloLM(OUT void** gain, IN const void* backend, IN autd3_float_t eps_1, IN autd3_float_t eps_2, IN autd3_float_t tau,
                                IN uint64_t k_max, IN const autd3_float_t* initial, IN int32_t initial_size);
EXPORT_AUTD void AUTDGainHoloGreedy(OUT void** gain, IN const void* backend, IN int32_t phase_div);
EXPORT_AUTD void AUTDGainHoloLSSGreedy(OUT void** gain, IN const void* backend, IN int32_t phase_div);
EXPORT_AUTD void AUTDGainHoloAPO(OUT void** gain, IN const void* backend, IN autd3_float_t eps, IN autd3_float_t lambda, IN int32_t k_max,
                                 IN int32_t line_search_max);
EXPORT_AUTD void AUTDGainHoloAdd(IN void* gain, IN autd3_float_t x, IN autd3_float_t y, IN autd3_float_t z, IN autd3_float_t amp);
EXPORT_AUTD void AUTDConstraintDontCare(OUT void** constraint);
EXPORT_AUTD void AUTDConstraintNormalize(OUT void** constraint);
EXPORT_AUTD void AUTDConstraintUniform(OUT void** constraint, IN autd3_float_t value);
EXPORT_AUTD void AUTDConstraintClamp(OUT void** constraint);
EXPORT_AUTD void AUTDSetConstraint(IN void* gain, IN void* constraint);
#ifdef __cplusplus
}
#endif
