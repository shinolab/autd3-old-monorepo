// File: bessel.h
// Project: examples
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <math.h>

void* bessel(void* autd) {
  void* s = NULL;
  AUTDCreateSilencer(&s, 10, 4096);
  AUTDSend(autd, s, NULL, 20 * 1000 * 1000);
  AUTDDeleteSilencer(s);

  double x = 90.0;
  double y = 70.0;
  double z = 0;

  void* g = NULL;
  AUTDGainBesselBeam(&g, x, y, z, 0.0, 0.0, 1.0, 13.0 / 180.0 * M_PI, 1.0);

  void* m = NULL;
  AUTDModulationSine(&m, 150, 1.0, 0.5);

  AUTDSend(autd, m, g, 20 * 1000 * 1000);

  AUTDDeleteGain(g);
  AUTDDeleteModulation(m);

  return autd;
}
