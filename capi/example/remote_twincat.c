/*
 * File: remote_twincat.c
 * Project: example
 * Created Date: 16/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#include "autd3_c_api.h"
#include "remote_twincat_link.h"
#include "runner.h"

int main(void) {
  void* cnt = NULL;
  void* link = NULL;
  void* builder = NULL;
  void* geometry = NULL;
  const char* remote_ip = "";
  const char* remote_ams_net_id = "";
  const char* local_ams_net_id = "";

  AUTDCreateGeometryBuilder(&builder);
  AUTDAddDevice(builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
  AUTDBuildGeometry(&geometry, builder);

  AUTDLinkRemoteTwinCAT(&link, remote_ip, remote_ams_net_id, local_ams_net_id);

  AUTDOpenController(&cnt, geometry, link);

  return run(cnt);
}
