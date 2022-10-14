/*
 * File: soem.c
 * Project: example
 * Created Date: 16/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/10/2022
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

int main() {
  void* cnt = NULL;
  void* link = NULL;
  AUTDCreateController(&cnt);

  AUTDAddDevice(cnt, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
  // AUTDAddDeviceQuaternion(cnt, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

  // void* adapter_list = NULL;
  // int32_t i;
  // char name[128], desc[128];
  // const int32_t adapter_list_size = AUTDGetAdapterPointer(&adapter_list);
  // for (i = 0; i < adapter_list_size; i++) {
  //   AUTDGetAdapter(adapter_list, i, desc, name);
  //   printf("[%d]: %s, %s\n", i, desc, name);
  // }
  // printf("Choose number: ");
  // if (!scanf("%d", &i)) return -1;
  // (void)getchar();
  // AUTDGetAdapter(adapter_list, i, desc, name);
  // AUTDFreeAdapterPointer(adapter_list);
  AUTDLinkSOEM(&link, NULL, 1, 1, false, (void*)callback, false);

  if (!AUTDOpenController(cnt, link) || !AUTDIsOpen(cnt)) {
    const int32_t error_size = AUTDGetLastError(NULL);
    char* error = malloc(error_size);
    AUTDGetLastError(error);
    printf("%s\n", error);
    free(error);
    return -1;
  }

  AUTDSetCheckTrials(cnt, 50);

  return run(cnt);
}
