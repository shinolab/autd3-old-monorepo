// File: backend_cuda.cpp
// Project: cuda
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#include "autd3/gain/backend_cuda.hpp"

#include <cuda_runtime_api.h>

#include <iostream>

#include "./kernel.h"

namespace autd3::gain::holo {

CUDABackend::CUDABackend(const int device_idx) { cudaSetDevice(device_idx); }

void CUDABackend::make_complex(const VectorXd& r, const VectorXd& i, VectorXc& c) {
  printf("CUDA backend\n");
  const auto rows = (uint32_t)c.size();
  constexpr uint32_t cols = 1;

  double *d_r, *d_i;
  cuDoubleComplex* d_c;
  cudaMalloc((void**)&d_r, sizeof(double) * rows);
  cudaMalloc((void**)&d_i, sizeof(double) * rows);
  cudaMalloc((void**)&d_c, sizeof(cuDoubleComplex) * rows);

  cudaMemcpy(d_r, r.data(), sizeof(double) * rows, cudaMemcpyHostToDevice);
  cudaMemcpy(d_i, i.data(), sizeof(double) * rows, cudaMemcpyHostToDevice);

  cu_make_complex(d_r, d_i, rows, cols, d_c);

  cudaMemcpy(c.data(), d_c, sizeof(cuDoubleComplex) * rows, cudaMemcpyDeviceToHost);

  cudaFree(d_r);
  cudaFree(d_i);
  cudaFree(d_c);
}

void CUDABackend::make_complex(const MatrixXd& r, const MatrixXd& i, MatrixXc& c) {
  printf("CUDA backend\n");
  const auto rows = (uint32_t)c.rows();
  const auto cols = (uint32_t)c.cols();
  cu_make_complex(r.data(), i.data(), rows, cols, (cuDoubleComplex*)c.data());
}

}  // namespace autd3::gain::holo
