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
#include "runner.h"
#include "twincat_link.h"

int main(void) {
  void* cnt = NULL;
  void* link = NULL;

  AUTDCreateController(&cnt);

  AUTDAddDevice(cnt, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

  AUTDLinkTwinCAT(&link);

  AUTDOpenController(cnt, link);

  return run(cnt);
}
