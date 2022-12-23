/*
 * File: remote_twincat.c
 * Project: example
 * Created Date: 16/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/12/2022
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
  const char* remote_ip = "";
  const char* remote_ams_net_id = "";
  const char* local_ams_net_id = "";

  AUTDCreateController(&cnt, 0);

  AUTDAddDevice(cnt, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

  AUTDLinkRemoteTwinCAT(&link, remote_ip, remote_ams_net_id, local_ams_net_id);

  if (!AUTDOpenController(cnt, link)) return -1;

  return run(cnt);
}
