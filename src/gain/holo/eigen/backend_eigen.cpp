// File: backend_eigen.cpp
// Project: eigen
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#include "autd3/gain/backend.hpp"

namespace autd3::gain::holo {

void EigenBackend::make_complex(const VectorXd& r, const VectorXd& i, VectorXc& c) {
  printf("Eigen backend\n");
  c.real() = r;
  c.imag() = i;
}

void EigenBackend::make_complex(const MatrixXd& r, const MatrixXd& i, MatrixXc& c) {
  printf("Eigen backend\n");
  c.real() = r;
  c.imag() = i;
}

}  // namespace autd3::gain::holo
