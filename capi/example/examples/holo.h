// File: holo.h
// Project: examples
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include "holo_gain.h"

#define TRANS_SPACING_MM (10.16)
#define NUM_TRANS_X (18)
#define NUM_TRANS_Y (14)

typedef struct {
  const char* name;
  void* gain;
} Opt;

void* select_opt(void* backend) {
  printf_s("Select Optimization Method (default is SDP)\n");

  int32_t opt_size = 9;
  Opt* opts = (Opt*)malloc(opt_size * sizeof(Opt));

  void* constraint;
  AUTDAmplitudeConstraintUniform(&constraint, 1.0);

  int idx = 0;
  opts[idx].name = "SDP";
  AUTDGainHoloSDP(&opts[idx++].gain, backend, 1e-3, 0.9, 100, constraint);
  opts[idx].name = "EVD";
  AUTDGainHoloEVD(&opts[idx++].gain, backend, 1.0, constraint);
  opts[idx].name = "GS";
  AUTDGainHoloGS(&opts[idx++].gain, backend, 100, constraint);
  opts[idx].name = "GSPAT";
  AUTDGainHoloGSPAT(&opts[idx++].gain, backend, 100, constraint);
  opts[idx].name = "NAIVE";
  AUTDGainHoloNaive(&opts[idx++].gain, backend, constraint);
  opts[idx].name = "LM";
  AUTDGainHoloLM(&opts[idx++].gain, backend, 1e-8, 1e-8, 1e-3, 5, NULL, 0, constraint);
  opts[idx].name = "GaussNewton";
  AUTDGainHoloGaussNewton(&opts[idx++].gain, backend, 1e-6, 1e-6, 500, NULL, 0, constraint);
  opts[idx].name = "GradientDescent";
  AUTDGainHoloGradientDescent(&opts[idx++].gain, backend, 1e-6, 0.5, 2000, NULL, 0, constraint);
  opts[idx].name = "Greedy";
  AUTDGainHoloGreedy(&opts[idx++].gain, backend, 16, constraint);
  for (int32_t i = 0; i < opt_size; i++) printf_s("[%d]: %s\n", i, opts[i].name);

  AUTDDeleteAmplitudeConstraint(constraint);

  idx = 0;
  if (!scanf_s("%d", &idx)) idx = 3;
  (void)getchar();
  if (idx >= opt_size) idx = 3;

  for (int32_t i = 0; i < opt_size; i++) {
    if (i == idx) continue;
    AUTDDeleteGain(opts[i].gain);
  }

  return opts[idx].gain;
}

void holo(void* autd) {
  void* s = NULL;
  AUTDCreateSilencer(&s, 10, 4096);
  AUTDSendHeader(autd, s);
  AUTDDeleteSilencer(s);

  void* m = NULL;
  AUTDModulationSine(&m, 150, 1.0, 0.5);

  double x = TRANS_SPACING_MM * (((double)NUM_TRANS_X - 1.0) / 2.0);
  double y = TRANS_SPACING_MM * (((double)NUM_TRANS_Y - 1.0) / 2.0);
  double z = 150.0;

  void* backend = NULL;
  AUTDEigenBackend(&backend);

  void* g = select_opt(backend);

  AUTDGainHoloAdd(g, x + 30.0, y, z, 1.0);
  AUTDGainHoloAdd(g, x - 30.0, y, z, 1.0);

  AUTDSendHeaderBody(autd, m, g);

  AUTDDeleteGain(g);
  AUTDDeleteModulation(m);
  AUTDDeleteBackend(backend);
}
