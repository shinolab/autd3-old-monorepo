/*
 * File: soem.c
 * Project: example
 * Created Date: 16/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/01/2023
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
  void* link = NULL;
  AUTDCreateController(&cnt);

  AUTDAddDevice(cnt, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

  AUTDLinkSOEM(&link, NULL, 2, 2, false, (void*)callback, false, 100, 2, NULL, NULL);

  AUTDOpenController(cnt, link);

  AUTDSetAckCheckTimeout(cnt, 20LL * 1000 * 1000);

  return run(cnt);
}
