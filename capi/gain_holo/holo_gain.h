// File: holo_gain.h
// Project: gain_holo
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include "../base/header.h"

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD void AUTDEigenBackend(void** out);
EXPORT_AUTD void AUTDDeleteBackend(const void* backend);
EXPORT_AUTD void AUTDGainHoloSDP(void** gain, const void* backend, double alpha, double lambda, uint64_t repeat);
EXPORT_AUTD void AUTDGainHoloEVD(void** gain, const void* backend, double gamma);
EXPORT_AUTD void AUTDGainHoloNaive(void** gain, const void* backend);
EXPORT_AUTD void AUTDGainHoloGS(void** gain, const void* backend, uint64_t repeat);
EXPORT_AUTD void AUTDGainHoloGSPAT(void** gain, const void* backend, uint64_t repeat);
EXPORT_AUTD void AUTDGainHoloLM(void** gain, const void* backend, double eps_1, double eps_2, double tau, uint64_t k_max, const double* initial,
                                int32_t initial_size);
EXPORT_AUTD void AUTDGainHoloGaussNewton(void** gain, const void* backend, double eps_1, double eps_2, uint64_t k_max, const double* initial,
                                         int32_t initial_size);
EXPORT_AUTD void AUTDGainHoloGradientDescent(void** gain, const void* backend, double eps, double step, uint64_t k_max, const double* initial,
                                             int32_t initial_size);
EXPORT_AUTD void AUTDGainHoloGreedy(void** gain, const void* backend, int32_t phase_div);
EXPORT_AUTD void AUTDGainHoloAdd(void* gain, double x, double y, double z, double amp);
EXPORT_AUTD void AUTDSetConstraint(void* gain, int32_t type, void* param);
EXPORT_AUTD void AUTDSetModeHolo(int32_t mode);
#ifdef __cplusplus
}
#endif
