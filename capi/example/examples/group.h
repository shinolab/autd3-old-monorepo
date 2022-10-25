// File: group.h
// Project: examples
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <math.h>

void* group(void* autd) {
  void* s = NULL;
  AUTDCreateSilencer(&s, 10, 4096);
  AUTDSend(autd, s, NULL);
  AUTDDeleteSilencer(s);

  double x = 90.0;
  double y = 70.0;

  void* g1 = NULL;
  AUTDGainFocus(&g1, x, y, 150.0, 1.0);

  void* g2 = NULL;
  AUTDGainBesselBeam(&g2, x, y, 0.0, 0.0, 0.0, 1.0, 13.0 / 180.0 * M_PI, 1.0);

  void* g = NULL;
  AUTDGainGrouped(&g, autd);

  AUTDGainGroupedAdd(g, 0, g1);
  AUTDGainGroupedAdd(g, 1, g2);

  void* m = NULL;
  AUTDModulationSine(&m, 150, 1.0, 0.5);

  AUTDSend(autd, m, g);

  AUTDDeleteGain(g);
  AUTDDeleteModulation(m);

  return autd;
}
