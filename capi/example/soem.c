/*
 * File: soem.c
 * Project: example
 * Created Date: 16/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/04/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#include <stdio.h>
#include <stdlib.h>

#include "autd3_c_api.h"
#include "runner.h"
#include "soem_link.h"

#ifdef _WIN32
__declspec(noreturn)
#else
_Noreturn
#endif
    void callback(char* msg) {
  printf("Link is lost\n");
  printf("%s\n", msg);
#ifdef __APPLE__
  exit(-1);
#else
  quick_exit(-1);
#endif
}

int main(void) {
  void* cnt = NULL;
  void* soem = NULL;
  void* link = NULL;
  void* builder = NULL;
  void* geometry = NULL;

  AUTDCreateGeometryBuilder(&builder);
  AUTDAddDevice(builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
  AUTDBuildGeometry(&geometry, builder);

  AUTDLinkSOEM(&soem);
  AUTDLinkSOEMIfname(soem, NULL);
  AUTDLinkSOEMBufSize(soem, 32);
  AUTDLinkSOEMSync0Cycle(soem, 2);
  AUTDLinkSOEMSendCycle(soem, 2);
  AUTDLinkSOEMFreerun(soem, true);
  AUTDLinkSOEMOnLost(soem, (void*)callback);
  AUTDLinkSOEMTimerStrategy(soem, 0);
  AUTDLinkSOEMStateCheckInterval(soem, 100);
  AUTDLinkSOEMLogLevel(soem, 2);
  AUTDLinkSOEMLogFunc(soem, NULL, NULL);
  AUTDLinkSOEMTimeout(soem, 20ULL * 1000 * 1000);
  AUTDLinkSOEMBuild(&link, soem);
  AUTDLinkSOEMDelete(soem);

  AUTDOpenController(&cnt, geometry, link);

  return run(cnt);
}
