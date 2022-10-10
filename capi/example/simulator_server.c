/*
 * File: simulator_server.c
 * Project: example
 * Created Date: 10/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#include "simulator.h"

int main() {
  const char* settings = "settings.json";

  AUTDExtraSimulator(settings);

  return 0;
}
