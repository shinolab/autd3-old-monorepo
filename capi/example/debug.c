/*
 * File: twincat.c
 * Project: example
 * Created Date: 16/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#include "autd3_c_api.h"
#include "debug_link.h"
#include "runner.h"

int main(void) {
  void* cnt = NULL;
  void* link = NULL;

  AUTDCreateController(&cnt);

  AUTDAddDevice(cnt, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

  AUTDLinkDebugSetLevel(1);

  AUTDLinkDebug(&link);

  AUTDOpenController(cnt, link);

  return run(cnt);
}
