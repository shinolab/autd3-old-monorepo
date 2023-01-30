/*
 * File: geometry_viewer.c
 * Project: example
 * Created Date: 10/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#include "geometry_viewer.h"

#include <stddef.h>

#include "autd3_c_api.h"

int main(void) {
  void* builder = NULL;
  void* geometry = NULL;

  AUTDCreateGeometryBuilder(&builder);
  AUTDAddDevice(builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
  AUTDBuildGeometry(&geometry, builder);

  AUTDExtraGeometryViewer(geometry, 800, 600, true, 0);

  return 0;
}
