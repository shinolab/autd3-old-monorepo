// File: focus.h
// Project: examples
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#define TRANS_SPACING_MM (10.16)
#define NUM_TRANS_X (18)
#define NUM_TRANS_Y (14)

void focus(void* autd) {
  AUTDSetSilencer(autd, 10, 4096);

  double x = TRANS_SPACING_MM * (((double)NUM_TRANS_X - 1.0) / 2.0);
  double y = TRANS_SPACING_MM * (((double)NUM_TRANS_Y - 1.0) / 2.0);
  double z = 150.0;

  void* g = NULL;
  AUTDGainFocus(&g, x, y, z, 1.0);

  void* m = NULL;
  AUTDModulationSine(&m, 150, 1.0, 0.5);

  AUTDSendHeaderBody(autd, m, g);

  AUTDDeleteGain(g);
  AUTDDeleteModulation(m);
}
