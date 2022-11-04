/*
 * File: remote_soem.c
 * Project: example
 * Created Date: 03/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#include <stdio.h>
#include <stdlib.h>

#include "autd3_c_api.h"
#include "remote_soem_link.h"
#include "runner.h"

int main() {
  void* cnt = NULL;
  void* link = NULL;
  const char* ip = "";
  const uint16_t port = 50632;

  AUTDCreateController(&cnt);

  AUTDAddDevice(cnt, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

  AUTDLinkRemoteSOEM(&link, ip, port);

  if (!AUTDOpenController(cnt, link)) {
    const int32_t error_size = AUTDGetLastError(NULL);
    char* error = malloc(error_size);
    AUTDGetLastError(error);
    printf("%s\n", error);
    free(error);
    return -1;
  }

  return run(cnt);
}
