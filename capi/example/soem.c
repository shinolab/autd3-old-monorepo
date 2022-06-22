/*
 * File: soem.c
 * Project: example
 * Created Date: 16/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/06/2022
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

void callback(char* msg) {
  printf("Link is lost\n");
  printf("%s\n", msg);
  exit(-1);
}

int main() {
  void* cnt = NULL;
  AUTDCreateController(&cnt);

  AUTDAddDevice(cnt, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
  // AUTDAddDeviceQuaternion(cnt, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

  void* adapter_list = NULL;
  int32_t i;
  char name[128], desc[128];
  const int32_t adapter_list_size = AUTDGetAdapterPointer(&adapter_list);
  for (i = 0; i < adapter_list_size; i++) {
    AUTDGetAdapter(adapter_list, i, desc, name);
    printf("[%d]: %s, %s\n", i, desc, name);
  }
  printf("Choose number: ");
  if (!scanf("%d", &i)) return -1;
  (void)getchar();
  AUTDGetAdapter(adapter_list, i, desc, name);
  void* link = NULL;
  const int32_t device_num = AUTDNumDevices(cnt);
  AUTDLinkSOEM(&link, name, device_num, 2, (void*)callback, false);
  AUTDFreeAdapterPointer(adapter_list);

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
