// File: point_stm.h
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

#include <math.h>

#define TRANS_SPACING_MM (10.16)
#define NUM_TRANS_X (18)
#define NUM_TRANS_Y (14)

void point_stm(void* autd) {
  AUTDSetSilencer(autd, 10, 4096);

  double x = TRANS_SPACING_MM * (((double)NUM_TRANS_X - 1.0) / 2.0);
  double y = TRANS_SPACING_MM * (((double)NUM_TRANS_Y - 1.0) / 2.0);
  double z = 150.0;

  void* stm = NULL;
  AUTDPointSTM(&stm);

  const int32_t point_num = 200;
  for (int32_t i = 0; i < point_num; i++) {
    const double radius = 30.0;
    const double theta = 2.0 * M_PI * (double)i / (double)point_num;
    AUTDPointSTMAdd(stm, x + radius * cos(theta), y + radius * sin(theta), z, 0);
  }

  const double actual_freq = AUTDSTMSetFrequency(stm, 1.0);
  printf_s("Actual frequency is %lf Hz\n", actual_freq);

  void* m = NULL;
  AUTDModulationStatic(&m, 1.0);

  AUTDSendHeaderBody(autd, m, stm);

  AUTDDeleteSTM(stm);
  AUTDDeleteModulation(m);
}
