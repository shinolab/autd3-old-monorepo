/*
 * File: simulator_client.c
 * Project: example
 * Created Date: 10/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/04/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#include "autd3_c_api.h"
#include "runner.h"
#include "simulator_link.h"

int main(void) {
  void* cnt = NULL;
  void* link = NULL;
  void* builder = NULL;
  void* geometry = NULL;

  AUTDCreateGeometryBuilder(&builder);
  AUTDAddDevice(builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
  AUTDBuildGeometry(&geometry, builder);

  AUTDLinkSimulator(&builder);
  AUTDLinkSimulatorBuild(&link, builder);

  AUTDOpenController(&cnt, geometry, link);

  return run(cnt);
}
