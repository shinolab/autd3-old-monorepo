// File: async.hpp
// Project: autd3
// Created Date: 08/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3 {
struct Async {};
inline Async async() { return {}; }
}  // namespace autd3
