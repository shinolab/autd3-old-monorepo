// File: trans_viewer.hpp
// Project: simulator
// Created Date: 03/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::extra::simulator {

class TransViewer {
 public:
  ~TransViewer() = default;
  TransViewer(const TransViewer& v) = delete;
  TransViewer& operator=(const TransViewer& obj) = delete;
  TransViewer(TransViewer&& obj) = default;
  TransViewer& operator=(TransViewer&& obj) = default;
}

}  // namespace autd3::extra::simulator
