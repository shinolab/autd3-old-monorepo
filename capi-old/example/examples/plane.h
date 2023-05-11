// File: plane.h
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

void* plane(void* autd) {
  void* s = NULL;
  AUTDCreateSilencer(&s, 10, 4096);
  AUTDSend(autd, s, NULL, 20ULL * 1000ULL * 1000ULL);
  AUTDDeleteSilencer(s);

  void* g = NULL;
  AUTDGainPlaneWave(&g, 0, 0, 1, 1.0);

  void* m = NULL;
  AUTDModulationSine(&m, 150, 1.0, 0.5);

  AUTDSend(autd, m, g, 20ULL * 1000ULL * 1000ULL);

  AUTDDeleteGain(g);
  AUTDDeleteModulation(m);

  return autd;
}
