// File: api_debug.h
// Project: examples
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

void api_debug(void* autd) {
  void* g = NULL;
  AUTDGainFocus(&g, 90, 70, 150, 1.0);
  void* m = NULL;
  printf_s("SineSquared\n");
  AUTDModulationSineSquared(&m, 150, 1.0, 0.5);

  AUTDSendHeaderBody(autd, m, g);
  AUTDDeleteGain(g);
  AUTDDeleteModulation(m);

  printf_s("press enter to test SineLegacy...\n");
  (void)getchar();

  AUTDModulationSineLegacy(&m, 150.0, 1.0, 0.5);
  AUTDSendHeader(autd, m);
  AUTDDeleteModulation(m);

  printf_s("press enter to test Square...\n");
  (void)getchar();

  AUTDModulationSquare(&m, 150, 0x00, 0xFF, 0.5);
  AUTDSendHeader(autd, m);

  AUTDModulationSetSamplingFrequencyDivision(m, 5);

  printf_s("Modulation API Test\n");
  printf_s("Modulation sampling frequency division: %ld\n", AUTDModulationSamplingFrequencyDivision(m));
  printf_s("Modulation sampling frequency: %lf Hz\n", AUTDModulationSamplingFrequency(m));
  AUTDDeleteModulation(m);

  printf_s("STM API Test\n");
  void* stm = NULL;
  AUTDPointSTM(&stm);
  const int32_t point_num = 200;
  for (int32_t i = 0; i < point_num; i++) AUTDPointSTMAdd(stm, 0.0, 0.0, 0.0, 0);
  AUTDSTMSetFrequency(stm, 1.0);
  printf_s("Actual frequency is %lf Hz\n", AUTDSTMFrequency(stm));
  printf_s("Sampling frequency is %lf us\n", AUTDSTMSamplingFrequency(stm));
  printf_s("Sampling frequency division is %ld\n", AUTDSTMSamplingFrequencyDivision(stm));

  printf_s("Set sampling frequency division to 100\n");
  AUTDSTMSetSamplingFrequencyDivision(stm, 100);
  printf_s("Actual frequency is %lf Hz\n", AUTDSTMFrequency(stm));
  printf_s("Sampling frequency is %lf us\n", AUTDSTMSamplingFrequency(stm));
  printf_s("Sampling frequency division is %ld\n", AUTDSTMSamplingFrequencyDivision(stm));
  AUTDDeleteSTM(stm);

  printf_s("press enter to stop...\n");
  (void)getchar();

  AUTDGainNull(&g);
  AUTDSendBody(autd, g);

  AUTDDeleteGain(g);
}
