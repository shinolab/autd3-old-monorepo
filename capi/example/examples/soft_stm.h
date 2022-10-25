// File: point_stm.h
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

void* soft_stm(void* autd) {
  void* s = NULL;
  AUTDCreateSilencer(&s, 0xFFFF, 4096);
  AUTDSend(autd, s, NULL);
  AUTDDeleteSilencer(s);

  void* m = NULL;
  AUTDModulationStatic(&m, 1.0);
  AUTDSend(autd, m, NULL);
  AUTDDeleteModulation(m);

  double x = 90.0;
  double y = 70.0;
  double z = 150.0;

  void* stm = NULL;
  AUTDSoftwareSTM(&stm);

  const int32_t point_num = 200;
  for (int32_t i = 0; i < point_num; i++) {
    const double radius = 30.0;
    const double theta = 2.0 * M_PI * (double)i / (double)point_num;
    void* g = NULL;
    AUTDGainFocus(&g, x + radius * cos(theta), y + radius * sin(theta), z, 1.0);
    AUTDSoftwareSTMAdd(stm, g);
  }

  const double actual_freq = AUTDSoftwareSTMSetFrequency(stm, 1.0);
  printf("Actual frequency is %lf Hz\n", actual_freq);

  void* handle = NULL;
  AUTDSoftwareSTMStart(&handle, stm, autd);

  printf("press any key to stop stm...");
  (void)getchar();

  AUTDSoftwareSTMFinish(&autd, handle);

  AUTDDeleteSoftwareSTM(stm);

  return autd;
}
