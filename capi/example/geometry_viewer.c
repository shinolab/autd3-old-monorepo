/*
 * File: geometry_viewer.c
 * Project: example
 * Created Date: 10/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#include "geometry_viewer.h"

#include <stddef.h>

#include "autd3_c_api.h"

int main() {
  void* cnt = NULL;
  AUTDCreateController(&cnt);

  AUTDAddDevice(cnt, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

  AUTDExtraGeometryViewer(cnt, 800, 600, true, 0);

  return 0;
}
