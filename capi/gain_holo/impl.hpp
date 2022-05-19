// File: impl.hpp
// Project: gain_holo
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 20/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <utility>
#include <vector>

#include "../base/wrapper.hpp"
#include "./holo_gain.h"
#include "autd3/gain/backend.hpp"
#include "autd3/gain/holo.hpp"

void AUTDEigenBackend(void** out) {
  auto* b = backend_create(autd3::gain::holo::EigenBackend::create());
  *out = b;
}

void AUTDDeleteBackend(const void* backend) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  backend_delete(b);
}

void AUTDAmplitudeConstraintDontCate(void** out) { *out = new autd3::gain::holo::DontCare; }
void AUTDAmplitudeConstraintNormalize(void** out) { *out = new autd3::gain::holo::Normalize; }
void AUTDAmplitudeConstraintUniform(void** out, const double value) { *out = new autd3::gain::holo::Uniform(value); }
void AUTDAmplitudeConstraintClamp(void** out) { *out = new autd3::gain::holo::Clamp; }

void AUTDDeleteAmplitudeConstraint(void* constraint) {
  const auto* wrapper = static_cast<const autd3::gain::holo::AmplitudeConstraint*>(constraint);
  delete wrapper;
}

void AUTDGainHoloSDP(void** gain, const void* backend, const double alpha, const double lambda, const uint64_t repeat, const void* constraint) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::SDP<T>(b->ptr);
  g->alpha = alpha;
  g->lambda = lambda;
  g->repeat = repeat;
  g->constraint = *static_cast<const autd3::gain::holo::AmplitudeConstraint*>(constraint);
  *gain = g;
}

void AUTDGainHoloEVD(void** gain, const void* backend, const double gamma, const void* constraint) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::EVD<T>(b->ptr);
  g->gamma = gamma;
  g->constraint = *static_cast<const autd3::gain::holo::AmplitudeConstraint*>(constraint);
  *gain = g;
}

void AUTDGainHoloNaive(void** gain, const void* backend, const void* constraint) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::Naive<T>(b->ptr);
  g->constraint = *static_cast<const autd3::gain::holo::AmplitudeConstraint*>(constraint);
  *gain = g;
}

void AUTDGainHoloGS(void** gain, const void* backend, const uint64_t repeat, const void* constraint) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::GS<T>(b->ptr);
  g->repeat = repeat;
  g->constraint = *static_cast<const autd3::gain::holo::AmplitudeConstraint*>(constraint);
  *gain = g;
}

void AUTDGainHoloGSPAT(void** gain, const void* backend, const uint64_t repeat, const void* constraint) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::GSPAT<T>(b->ptr);
  g->repeat = repeat;
  g->constraint = *static_cast<const autd3::gain::holo::AmplitudeConstraint*>(constraint);
  *gain = g;
}

void AUTDGainHoloLM(void** gain, const void* backend, const double eps_1, const double eps_2, const double tau, const uint64_t k_max,
                    const double* initial, const int32_t initial_size, const void* constraint) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  std::vector<double> initial_;
  initial_.reserve(initial_size);
  for (auto i = 0; i < initial_size; i++) initial_.emplace_back(initial[i]);

  auto* g = new autd3::gain::holo::LM(b->ptr);
  g->eps_1 = eps_1;
  g->eps_2 = eps_2;
  g->tau = tau;
  g->k_max = k_max;
  g->initial = std::move(initial_);
  g->constraint = *static_cast<const autd3::gain::holo::AmplitudeConstraint*>(constraint);
  *gain = g;
}

void AUTDGainHoloGaussNewton(void** gain, const void* backend, const double eps_1, const double eps_2, const uint64_t k_max, const double* initial,
                             const int32_t initial_size, const void* constraint) {
  const auto b = static_cast<const BackendWrapper*>(backend);

  std::vector<double> initial_;
  initial_.reserve(initial_size);
  for (auto i = 0; i < initial_size; i++) initial_.emplace_back(initial[i]);

  auto* g = new autd3::gain::holo::GaussNewton<T>(b->ptr);
  g->eps_1 = eps_1;
  g->eps_2 = eps_2;
  g->k_max = k_max;
  g->initial = std::move(initial_);
  g->constraint = *static_cast<const autd3::gain::holo::AmplitudeConstraint*>(constraint);
  *gain = g;
}
void AUTDGainHoloGradientDescent(void** gain, const void* backend, const double eps, const double step, const uint64_t k_max, const double* initial,
                                 const int32_t initial_size, const void* constraint) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  std::vector<double> initial_;
  initial_.reserve(initial_size);
  for (auto i = 0; i < initial_size; i++) initial_.emplace_back(initial[i]);
  auto* g = new autd3::gain::holo::GradientDescent<T>(b->ptr);
  g->eps = eps;
  g->k_max = k_max;
  g->step = step;
  g->initial = std::move(initial_);
  g->constraint = *static_cast<const autd3::gain::holo::AmplitudeConstraint*>(constraint);
  *gain = g;
}

void AUTDGainHoloGreedy(void** gain, const void* backend, const int32_t phase_div, const void* constraint) {
  const auto b = static_cast<const BackendWrapper*>(backend);
  auto* g = new autd3::gain::holo::Greedy<T>(b->ptr);
  g->phase_div = phase_div;
  g->constraint = *static_cast<const autd3::gain::holo::AmplitudeConstraint*>(constraint);
  *gain = g;
}

void AUTDGainHoloAdd(void* gain, const double x, const double y, const double z, const double amp) {
  auto* g = static_cast<autd3::gain::holo::Holo<T>*>(gain);
  g->add_focus(autd3::core::Vector3(x, y, z), amp);
}
