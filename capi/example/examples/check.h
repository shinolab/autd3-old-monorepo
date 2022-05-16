// File: check.h
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

#include <windows.h>

void check(void* autd) {
  int32_t num_devices = AUTDNumDevices(autd);
  printf_s("===== Device informations =====\n");
  for (int32_t i = 0; i < num_devices; i++) {
    double x, y, z;
    AUTDTransPosition(autd, i, 0, &x, &y, &z);
    printf_s("[%d]: Origin = (%lf, %lf, %lf)\n", i, x, y, z);
    AUTDTransXDirection(autd, i, 0, &x, &y, &z);
    printf_s("[%d]: X = (%lf, %lf, %lf)\n", i, x, y, z);
    AUTDTransYDirection(autd, i, 0, &x, &y, &z);
    printf_s("[%d]: Y = (%lf, %lf, %lf)\n", i, x, y, z);
    AUTDTransZDirection(autd, i, 0, &x, &y, &z);
    printf_s("[%d]: Z = (%lf, %lf, %lf)\n", i, x, y, z);
  }
  printf_s("\n");

  printf_s("===== Flags =====\n");

  AUTDSetReadsFPGAInfo(autd, true);
  AUTDSetCheckAck(autd, false);
  AUTDSetForceFan(autd, false);

  bool is_force_fan = AUTDGetForceFan(autd);
  bool is_reads_fpga_info = AUTDGetReadsFPGAInfo(autd);
  bool is_check_ack = AUTDGetCheckAck(autd);

  printf_s("Is force fan: %d\n", is_force_fan);
  printf_s("Is reads FPGA info: %d\n", is_reads_fpga_info);
  printf_s("Is check ack: %d\n", is_check_ack);
  printf_s("\n");

  printf_s("===== Properties =====\n");

  AUTDSetAttenuation(autd, 0.0);
  printf_s("Attenuation coefficient %lf [Np/mm]\n", AUTDGetAttenuation(autd));
  printf_s("\n");

  printf_s("===== FPGA informations =====\n");

  uint8_t* infos = malloc(num_devices);
  AUTDGetFPGAInfo(autd, infos);
  for (int32_t i = 0; i < num_devices; i++) {
    printf_s("[%d]: Is fan running : %d\n", i, infos[i]);
  }
  printf_s("\n");

  printf_s("press any key to force fan...");
  (void)getchar();

  AUTDSetForceFan(autd, true);
  AUTDUpdateFlags(autd);

  Sleep(100);

  AUTDGetFPGAInfo(autd, infos);
  for (int32_t i = 0; i < num_devices; i++) {
    printf_s("[%d]: Is fan running : %d\n", i, infos[i]);
  }
  printf_s("\n");

  printf_s("press any key to stop fan...");
  (void)getchar();

  AUTDSetForceFan(autd, false);
  AUTDUpdateFlags(autd);

  free(infos);
}
