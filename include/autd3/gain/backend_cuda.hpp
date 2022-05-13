// File: backend.hpp
// Project: gain
// Created Date: 10/09/2021
// Author: Shun Suzuki
// -----
// Last Modified: 13/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#pragma once

#include "backend.hpp"

namespace autd3::gain::holo {

/**
 * \brief Backend for HoloGain
 */
class CUDABackend final : public Backend {
 public:
  CUDABackend(int device_idx = 0);
  ~CUDABackend() override = default;
  CUDABackend(const CUDABackend& v) noexcept = default;
  CUDABackend& operator=(const CUDABackend& obj) = default;
  CUDABackend(CUDABackend&& obj) = default;
  CUDABackend& operator=(CUDABackend&& obj) = default;

  void make_complex(const VectorXd& r, const VectorXd& i, VectorXc& c) override;
  void make_complex(const MatrixXd& r, const MatrixXd& i, MatrixXc& c) override;

  static BackendPtr create() { return std::make_shared<CUDABackend>(); }
};

}  // namespace autd3::gain::holo
