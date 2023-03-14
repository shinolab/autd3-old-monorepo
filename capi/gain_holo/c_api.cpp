// File: c_api.cpp
// Project: gain_holo
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 14/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <utility>
#include <vector>

#include "../base/wrapper.hpp"
#include "./holo_gain.h"
#include "autd3/gain/backend.hpp"
#include "autd3/gain/holo.hpp"

void AUTDEigenBackend(void** out) {
  auto* b = backend_create(autd3::gain::holo::EigenBackend().build());
  *out = b;
}

void AUTDDeleteBackend(const void* backend) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  backend_delete(b);
}

void AUTDGainHoloSDP(void** gain, const void* backend, const autd3_float_t alpha, const autd3_float_t lambda, const uint64_t repeat) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::SDP(b->ptr);
  g->alpha = alpha;
  g->lambda = lambda;
  g->repeat = static_cast<size_t>(repeat);
  *gain = g;
}

void AUTDGainHoloEVP(void** gain, const void* backend, const autd3_float_t gamma) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::EVP(b->ptr);
  g->gamma = gamma;
  *gain = g;
}

void AUTDGainHoloNaive(void** gain, const void* backend) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::Naive(b->ptr);
  *gain = g;
}

void AUTDGainHoloGS(void** gain, const void* backend, const uint64_t repeat) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::GS(b->ptr);
  g->repeat = static_cast<size_t>(repeat);
  *gain = g;
}

void AUTDGainHoloGSPAT(void** gain, const void* backend, const uint64_t repeat) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::GSPAT(b->ptr);
  g->repeat = static_cast<size_t>(repeat);
  *gain = g;
}

void AUTDGainHoloLM(void** gain, const void* backend, const autd3_float_t eps_1, const autd3_float_t eps_2, const autd3_float_t tau,
                    const uint64_t k_max, const autd3_float_t* initial, const int32_t initial_size) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  std::vector<autd3_float_t> initial_;
  initial_.reserve(initial_size);
  std::copy_n(initial, initial_size, std::back_inserter(initial_));

  auto* g = new autd3::gain::holo::LM(b->ptr);
  g->eps_1 = eps_1;
  g->eps_2 = eps_2;
  g->tau = tau;
  g->k_max = static_cast<size_t>(k_max);
  g->initial = std::move(initial_);
  *gain = g;
}

void AUTDGainHoloGreedy(void** gain, const void* backend, const int32_t phase_div) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::Greedy(b->ptr);
  g->phase_div = phase_div;
  *gain = g;
}

void AUTDGainHoloLSSGreedy(void** gain, const void* backend, const int32_t phase_div) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::LSSGreedy(b->ptr);
  g->phase_div = phase_div;
  *gain = g;
}

void AUTDGainHoloAPO(void** gain, const void* backend, const autd3_float_t eps, const autd3_float_t lambda, const int32_t k_max,
                     const int32_t line_search_max) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::APO(b->ptr);
  g->eps = eps;
  g->lambda = lambda;
  g->k_max = k_max;
  g->line_search_max = line_search_max;
  *gain = g;
}

void AUTDGainHoloAdd(void* gain, const autd3_float_t x, const autd3_float_t y, const autd3_float_t z, const autd3_float_t amp) {
  auto* g = static_cast<autd3::gain::holo::Holo*>(gain);
  g->add_focus(autd3::core::Vector3(x, y, z), amp);
}

void AUTDConstraintDontCare(void** constraint) {
  auto* c = constraint_create(std::make_unique<autd3::gain::holo::DontCare>());
  *constraint = c;
}
void AUTDConstraintNormalize(void** constraint) {
  auto* c = constraint_create(std::make_unique<autd3::gain::holo::Normalize>());
  *constraint = c;
}
void AUTDConstraintUniform(void** constraint, autd3_float_t value) {
  auto* c = constraint_create(std::make_unique<autd3::gain::holo::Uniform>(value));
  *constraint = c;
}
void AUTDConstraintClamp(void** constraint) {
  auto* c = constraint_create(std::make_unique<autd3::gain::holo::Clamp>());
  *constraint = c;
}
void AUTDSetConstraint(void* gain, void* constraint) {
  auto* g = static_cast<autd3::gain::holo::Holo*>(gain);
  auto* c = static_cast<ConstraintWrapper*>(constraint);
  g->constraint = std::move(c->ptr);
  constraint_delete(c);
}
