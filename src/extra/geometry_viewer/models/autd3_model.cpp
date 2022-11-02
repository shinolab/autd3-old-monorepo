// File: autd3_model.cpp
// Project: models
// Created Date: 18/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3_model.hpp"

uint8_t model_data[] = {
#include "AUTD3.glb.txt"
};
unsigned model_size = sizeof model_data;
