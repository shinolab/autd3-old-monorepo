// File: link.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 24/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

template <class T>
concept link_builder = requires(T t) {
  typename T::Link;
  { t.ptr() } -> std::same_as<native_methods::LinkBuilderPtr>;
};

}  // namespace autd3::internal
