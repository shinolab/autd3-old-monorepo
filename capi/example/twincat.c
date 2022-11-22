/*
 * File: twincat.c
 * Project: example
 * Created Date: 16/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#include "autd3_c_api.h"
#include "runner.h"
#include "twincat_link.h"

int main() {
  void* cnt = NULL;
  void* link = NULL;

  AUTDCreateController(&cnt, 0);

  AUTDAddDevice(cnt, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

  AUTDLinkTwinCAT(&link);

  if (!AUTDOpenController(cnt, link)) return -1;

  return run(cnt);
}
