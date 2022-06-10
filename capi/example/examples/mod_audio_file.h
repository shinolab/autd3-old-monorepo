// File: mod_audio_file.h
// Project: examples
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 10/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "audio_file_modulation.h"

#define TRANS_SPACING_MM (10.16)
#define NUM_TRANS_X (18)
#define NUM_TRANS_Y (14)

void mod_audio_file(void* autd) {
  void* s = NULL;
  AUTDCreateSilencer(&s, 10, 4096);
  AUTDSend(autd, s, NULL);
  AUTDDeleteSilencer(s);

  double x = TRANS_SPACING_MM * (((double)NUM_TRANS_X - 1.0) / 2.0);
  double y = TRANS_SPACING_MM * (((double)NUM_TRANS_Y - 1.0) / 2.0);
  double z = 150.0;

  void* g = NULL;
  AUTDGainFocus(&g, x, y, z, 1.0);

  printf("===== Wav =====\n");

  char path[256];
  sprintf(path, "%s/%s", AUTD3_RESOURCE_PATH, "sin150.wav");
  void* mw = NULL;
  AUTDModulationWav(&mw, path, 40960);

  AUTDSend(autd, mw, g);

  AUTDDeleteModulation(mw);

  printf("press any key to start RawPCM test...\n");
  (void)getchar();
  printf("===== RawPCM =====\n");

  sprintf(path, "%s/%s", AUTD3_RESOURCE_PATH, "sin150.dat");
  void* mr = NULL;
  AUTDModulationRawPCM(&mr, path, 40e3, 40960);

  AUTDSend(autd, mr, g);

  AUTDDeleteGain(g);
  AUTDDeleteModulation(mr);
}
