// File: check.h
// Project: examples
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 01/02/2023
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
  void* geometry = NULL;
  AUTDGetGeometry(&geometry, autd);

  int32_t num_transducers = AUTDNumTransducers(geometry);
  printf("===== Transducer informations =====\n");
  for (int32_t i = 0; i < num_transducers; i++) {
    double x, y, z;
    AUTDTransPosition(geometry, i, &x, &y, &z);
    printf("[%d]: Origin = (%lf, %lf, %lf)\n", i, x, y, z);
    AUTDTransXDirection(geometry, i, &x, &y, &z);
    printf("[%d]: X = (%lf, %lf, %lf)\n", i, x, y, z);
    AUTDTransYDirection(geometry, i, &x, &y, &z);
    printf("[%d]: Y = (%lf, %lf, %lf)\n", i, x, y, z);
    AUTDTransZDirection(geometry, i, &x, &y, &z);
    printf("[%d]: Z = (%lf, %lf, %lf)\n", i, x, y, z);
  }
  printf("\n");

  printf("===== Flags =====\n");

  AUTDSetReadsFPGAInfo(autd, true);
  AUTDSetAckCheckTimeout(autd, 20LL * 1000 * 1000);
  AUTDSetSendInterval(autd, 1LL * 1000 * 1000);
  AUTDSetForceFan(autd, false);

  bool is_force_fan = AUTDGetForceFan(autd);
  bool is_reads_fpga_info = AUTDGetReadsFPGAInfo(autd);
  uint64_t timeout = AUTDGetAckCheckTimeout(autd);
  uint64_t send_interval = AUTDGetSendInterval(autd);

  printf("Is force fan: %d\n", is_force_fan);
  printf("Is reads FPGA info: %d\n", is_reads_fpga_info);
#if defined(WIN32) || defined(__APPLE__)
  printf("Check timeout: %llu\n", timeout);
  printf("Send interval: %llu\n", send_interval);
#else
  printf("Check timeout: %lu\n", timeout);
  printf("Send interval: %lu\n", send_interval);
#endif
  printf("\n");

  printf("===== Properties =====\n");

  AUTDSetAttenuation(geometry, 0.0);
  printf("Attenuation coefficient %lf [Np/mm]\n", AUTDGetAttenuation(geometry));
  printf("\n");

  printf("===== FPGA informations =====\n");

  int32_t num_devices = AUTDNumDevices(geometry);
  uint8_t* infos = malloc(num_devices);
  AUTDGetFPGAInfo(autd, infos);
  for (int32_t i = 0; i < num_devices; i++) {
    printf("[%d]: Is fan running : %d\n", i, infos[i]);
  }
  printf("\n");

  printf("press any key to force fan...");
  (void)getchar();

  AUTDSetForceFan(autd, true);

  void* update_flag;
  AUTDUpdateFlags(&update_flag);
  AUTDSendSpecial(autd, update_flag);
  AUTDDeleteSpecialData(update_flag);

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

  AUTDUpdateFlags(&update_flag);
  AUTDSendSpecial(autd, update_flag);
  AUTDDeleteSpecialData(update_flag);

  free(infos);

  return autd;
}
