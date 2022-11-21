/*
 * File: twincat.c
 * Project: example
 * Created Date: 16/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#include <stdio.h>
#include <stdlib.h>

#include "autd3_c_api.h"
#include "debug_link.h"
#include "runner.h"

int main() {
  void* cnt = NULL;
  void* link = NULL;

  AUTDCreateController(&cnt, 0);

  AUTDAddDevice(cnt, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

  AUTDLinkDebugSetLevel(1);

  AUTDLinkDebug(&link);

  if (!AUTDOpenController(cnt, link)) return -1;

  return run(cnt);
}
