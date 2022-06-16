// File: c_api.cpp
// Project: gain_holo
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <utility>
#include <vector>

#include "../base/wrapper.hpp"
#include "./holo_gain.h"
#include "autd3/core/geometry/dynamic_transducer.hpp"
#include "autd3/gain/backend.hpp"
#include "autd3/gain/holo.hpp"

using T = autd3::core::DynamicTransducer;

void AUTDEigenBackend(void** out) {
  auto* b = backend_create(autd3::gain::holo::EigenBackend::create());
  *out = b;
}

void AUTDDeleteBackend(const void* backend) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  backend_delete(b);
}

void AUTDGainHoloSDP(void** gain, const void* backend, const double alpha, const double lambda, const uint64_t repeat) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::SDP<T>(b->ptr);
  g->alpha = alpha;
  g->lambda = lambda;
  g->repeat = repeat;
  *gain = g;
}

void AUTDGainHoloEVD(void** gain, const void* backend, const double gamma) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::EVD<T>(b->ptr);
  g->gamma = gamma;
  *gain = g;
}

void AUTDGainHoloNaive(void** gain, const void* backend) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::Naive<T>(b->ptr);
  *gain = g;
}

void AUTDGainHoloGS(void** gain, const void* backend, const uint64_t repeat) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::GS<T>(b->ptr);
  g->repeat = repeat;
  *gain = g;
}

void AUTDGainHoloGSPAT(void** gain, const void* backend, const uint64_t repeat) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::GSPAT<T>(b->ptr);
  g->repeat = repeat;
  *gain = g;
}

void AUTDGainHoloLM(void** gain, const void* backend, const double eps_1, const double eps_2, const double tau, const uint64_t k_max,
                    const double* initial, const int32_t initial_size) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  std::vector<double> initial_;
  initial_.reserve(initial_size);
  for (auto i = 0; i < initial_size; i++) initial_.emplace_back(initial[i]);

  auto* g = new autd3::gain::holo::LM<T>(b->ptr);
  g->eps_1 = eps_1;
  g->eps_2 = eps_2;
  g->tau = tau;
  g->k_max = k_max;
  g->initial = std::move(initial_);
  *gain = g;
}

void AUTDGainHoloGreedy(void** gain, const void* backend, const int32_t phase_div) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::Greedy<T>(b->ptr);
  g->phase_div = phase_div;
  *gain = g;
}

void AUTDGainHoloAdd(void* gain, const double x, const double y, const double z, const double amp) {
  auto* g = static_cast<autd3::gain::holo::Holo<T>*>(gain);
  g->add_focus(autd3::core::Vector3(x, y, z), amp);
}

void AUTDSetConstraint(void* gain, const int32_t type, void* param) {
  auto* g = static_cast<autd3::gain::holo::Holo<T>*>(gain);
  switch (type) {
    case 0:
      g->constraint = autd3::gain::holo::DontCare();
      break;
    case 1:
      g->constraint = autd3::gain::holo::Normalize();
      break;
    case 2:
      g->constraint = autd3::gain::holo::Uniform(*static_cast<double*>(param));
      break;
    case 3:
      g->constraint = autd3::gain::holo::Clamp();
      break;
    default:
      break;
  }
}

void AUTDSetModeHolo(const uint8_t mode) { T::mode() = static_cast<autd3::core::TransducerMode>(mode); }
