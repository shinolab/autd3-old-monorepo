// File: check.h
// Project: examples
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#ifdef WIN32
#include <Windows.h>
#else
#include <unistd.h>
#endif

void* check(void* autd) {
  int32_t num_devices = AUTDNumDevices(autd);
  printf("===== Device informations =====\n");
  for (int32_t i = 0; i < num_devices; i++) {
    double x, y, z;
    AUTDTransPosition(autd, i, 0, &x, &y, &z);
    printf("[%d]: Origin = (%lf, %lf, %lf)\n", i, x, y, z);
    AUTDTransXDirection(autd, i, 0, &x, &y, &z);
    printf("[%d]: X = (%lf, %lf, %lf)\n", i, x, y, z);
    AUTDTransYDirection(autd, i, 0, &x, &y, &z);
    printf("[%d]: Y = (%lf, %lf, %lf)\n", i, x, y, z);
    AUTDTransZDirection(autd, i, 0, &x, &y, &z);
    printf("[%d]: Z = (%lf, %lf, %lf)\n", i, x, y, z);
  }
  printf("\n");

  printf("===== Flags =====\n");

  AUTDSetReadsFPGAInfo(autd, true);
  AUTDSetCheckTrials(autd, 20);
  AUTDSetSendInterval(autd, 2);
  AUTDSetForceFan(autd, false);

  bool is_force_fan = AUTDGetForceFan(autd);
  bool is_reads_fpga_info = AUTDGetReadsFPGAInfo(autd);
  int32_t check_trials = AUTDGetCheckTrials(autd);
  int32_t send_interval = AUTDGetSendInterval(autd);

  printf("Is force fan: %d\n", is_force_fan);
  printf("Is reads FPGA info: %d\n", is_reads_fpga_info);
  printf("Check trials: %d\n", check_trials);
  printf("Send interval: %d\n", send_interval);
  printf("\n");

  printf("===== Properties =====\n");

  AUTDSetAttenuation(autd, 0.0);
  printf("Attenuation coefficient %lf [Np/mm]\n", AUTDGetAttenuation(autd));
  printf("\n");

  printf("===== FPGA informations =====\n");

  uint8_t* infos = malloc(num_devices);
  AUTDGetFPGAInfo(autd, infos);
  for (int32_t i = 0; i < num_devices; i++) {
    printf("[%d]: Is fan running : %d\n", i, infos[i]);
  }
  printf("\n");

  printf("press any key to force fan...");
  (void)getchar();

  AUTDSetForceFan(autd, true);
  AUTDUpdateFlags(autd);

#ifdef WIN32
  Sleep(100);
#else
  usleep(100 * 1000);
#endif

  AUTDGetFPGAInfo(autd, infos);
  for (int32_t i = 0; i < num_devices; i++) {
    printf("[%d]: Is fan running : %d\n", i, infos[i]);
  }
  printf("\n");

  printf("press any key to stop fan...");
  (void)getchar();

  AUTDSetForceFan(autd, false);
  AUTDUpdateFlags(autd);

  free(infos);

  return autd;
}
