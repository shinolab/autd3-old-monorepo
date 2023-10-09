// File: nop.hpp
// Project: link
// Created Date: 09/10/2023
// Author: Shun Suzuki
// -----
// Last Modified: 09/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::link {

/**
 * @brief Link which does nothing
 *
 */
class Nop {
 public:
  class Builder final : public internal::LinkBuilder {
    friend class Nop;

    explicit Builder() : LinkBuilder() {}

   public:
    internal::native_methods::LinkBuilderPtr ptr() const override { return internal::native_methods::AUTDLinkNop(); }
  };

  static Builder builder() { return Builder(); }

  Nop() = delete;
};
}  // namespace autd3::link
