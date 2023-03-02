// File: holo.h
// Project: examples
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 02/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "holo_gain.h"

typedef struct {
  const char* name;
  void* gain;
} Opt;

void* select_opt(void* backend) {
  printf("Select Optimization Method (default is SDP)\n");

  int32_t opt_size = 7;
  Opt* opts = (Opt*)malloc(opt_size * sizeof(Opt));

  int idx = 0;
  opts[idx].name = "SDP";
  AUTDGainHoloSDP(&opts[idx++].gain, backend, 1e-3, 0.9, 100);
  opts[idx].name = "EVP";
  AUTDGainHoloEVP(&opts[idx++].gain, backend, 1.0);
  opts[idx].name = "GS";
  AUTDGainHoloGS(&opts[idx++].gain, backend, 100);
  opts[idx].name = "GSPAT";
  AUTDGainHoloGSPAT(&opts[idx++].gain, backend, 100);
  opts[idx].name = "NAIVE";
  AUTDGainHoloNaive(&opts[idx++].gain, backend);
  opts[idx].name = "LM";
  AUTDGainHoloLM(&opts[idx++].gain, backend, 1e-8, 1e-8, 1e-3, 5, NULL, 0);
  opts[idx].name = "Greedy";
  AUTDGainHoloGreedy(&opts[idx++].gain, backend, 16);
  for (int32_t i = 0; i < opt_size; i++) printf("[%d]: %s\n", i, opts[i].name);

  idx = 0;
  if (!scanf("%d", &idx)) idx = 3;
  (void)getchar();
  if (idx >= opt_size) idx = 3;

  for (int32_t i = 0; i < opt_size; i++) {
    if (i == idx) continue;
    AUTDDeleteGain(opts[i].gain);
  }

  return opts[idx].gain;
}

void* holo(void* autd) {
  void* s = NULL;
  AUTDCreateSilencer(&s, 10, 4096);
  AUTDSend(autd, s, NULL);
  AUTDDeleteSilencer(s);

  void* m = NULL;
  AUTDModulationSine(&m, 150, 1.0, 0.5);

  double x = 90.0;
  double y = 70.0;
  double z = 150.0;

  void* backend = NULL;
  AUTDEigenBackend(&backend);

  void* g = select_opt(backend);
  void* constraint = NULL;
  AUTDConstraintUniform(&constraint, 1.0);
  AUTDSetConstraint(g, constraint);

  AUTDGainHoloAdd(g, x + 30.0, y, z, 1.0);
  AUTDGainHoloAdd(g, x - 30.0, y, z, 1.0);

  AUTDSend(autd, m, g);

  AUTDDeleteGain(g);
  AUTDDeleteModulation(m);
  AUTDDeleteBackend(backend);

  return autd;
}
