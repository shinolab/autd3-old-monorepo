// File: api_debug.h
// Project: examples
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 06/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

void api_debug(void* autd) {
  void* g = NULL;
  AUTDGainFocus(&g, 90, 70, 150, 1.0);
  void* m = NULL;
  printf("SineSquared\n");
  AUTDModulationSineSquared(&m, 150, 1.0, 0.5);

  AUTDSendHeaderBody(autd, m, g);
  AUTDDeleteGain(g);
  AUTDDeleteModulation(m);

  printf("press enter to test SineLegacy...\n");
  (void)getchar();

  AUTDModulationSineLegacy(&m, 150.0, 1.0, 0.5);
  AUTDSendHeader(autd, m);
  AUTDDeleteModulation(m);

  printf("press enter to test Square...\n");
  (void)getchar();

  AUTDModulationSquare(&m, 150, 0x00, 0xFF, 0.5);
  AUTDSendHeader(autd, m);

  AUTDModulationSetSamplingFrequencyDivision(m, 5);

  printf("Modulation API Test\n");
  printf("Modulation sampling frequency division: %d\n", AUTDModulationSamplingFrequencyDivision(m));
  printf("Modulation sampling frequency: %lf Hz\n", AUTDModulationSamplingFrequency(m));
  AUTDDeleteModulation(m);

  printf("STM API Test\n");
  void* stm = NULL;
  AUTDPointSTM(&stm);
  const int32_t point_num = 200;
  for (int32_t i = 0; i < point_num; i++) AUTDPointSTMAdd(stm, 0.0, 0.0, 0.0, 0);
  AUTDSTMSetFrequency(stm, 1.0);
  printf("Actual frequency is %lf Hz\n", AUTDSTMFrequency(stm));
  printf("Sampling frequency is %lf us\n", AUTDSTMSamplingFrequency(stm));
  printf("Sampling frequency division is %d\n", AUTDSTMSamplingFrequencyDivision(stm));

  printf("Set sampling frequency division to 100\n");
  AUTDSTMSetSamplingFrequencyDivision(stm, 100);
  printf("Actual frequency is %lf Hz\n", AUTDSTMFrequency(stm));
  printf("Sampling frequency is %lf us\n", AUTDSTMSamplingFrequency(stm));
  printf("Sampling frequency division is %d\n", AUTDSTMSamplingFrequencyDivision(stm));
  AUTDDeleteSTM(stm);

  printf("press enter to stop...\n");
  (void)getchar();

  AUTDGainNull(&g);
  AUTDSendBody(autd, g);

  AUTDDeleteGain(g);
}
