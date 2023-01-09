// File: bundle.hpp
// Project: link
// Created Date: 16/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <utility>
#include <vector>

#include "autd3/core/link.hpp"

namespace autd3::link {
/**
 * @brief Link for bundling some Links up
 */
class Bundle {
 public:
  using value_type = core::LinkPtr;

  /**
   * @brief Create Bundle link
   */
  [[nodiscard]] core::LinkPtr build();

  /**
   * @brief Bundle new link
   */
  Bundle& link(core::LinkPtr link) {
    _links.emplace_back(std::move(link));
    return *this;
  }

  void push_back(core::LinkPtr link) { _links.emplace_back(std::move(link)); }

  /**
   * @brief Constructor
   */
  explicit Bundle(core::LinkPtr link) { _links.emplace_back(std::move(link)); }

  ~Bundle() = default;
  Bundle(const Bundle& v) noexcept = delete;
  Bundle& operator=(const Bundle& obj) = delete;
  Bundle(Bundle&& obj) = delete;
  Bundle& operator=(Bundle&& obj) = delete;

 private:
  std::vector<core::LinkPtr> _links;
};
}  // namespace autd3::link
