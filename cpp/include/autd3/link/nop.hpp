// File: nop.hpp
// Project: link
// Created Date: 09/10/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {
class ControllerBuilder;
}

namespace autd3::link {

/**
 * @brief Link which does nothing
 *
 */
class Nop {
  explicit Nop(internal::native_methods::LinkPtr) {}

 public:
  class Builder final {
    friend class Nop;
    friend class internal::ControllerBuilder;

    Builder() {}

    [[nodiscard]] Nop resolve_link(const internal::native_methods::LinkPtr link) const { return Nop{link}; }

   public:
    using Link = Nop;

    [[nodiscard]] internal::native_methods::LinkBuilderPtr ptr() const { return internal::native_methods::AUTDLinkNop(); }
  };

  static Builder builder() { return Builder(); }
};
}  // namespace autd3::link
