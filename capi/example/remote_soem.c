/*
 * File: remote_soem.c
 * Project: example
 * Created Date: 03/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#include "autd3_c_api.h"
#include "remote_soem_link.h"
#include "runner.h"

int main(void) {
  void* cnt = NULL;
  void* link = NULL;
  void* builder = NULL;
  void* geometry = NULL;
  const char* ip = "";
  const uint16_t port = 50632;

  AUTDCreateGeometryBuilder(&builder);
  AUTDAddDevice(builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
  AUTDBuildGeometry(&geometry, builder);

  AUTDLinkRemoteSOEM(&link, ip, port);

  AUTDOpenController(&cnt, geometry, link);

  return run(cnt);
}
