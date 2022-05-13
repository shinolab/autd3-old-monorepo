/*
 * File: kernel.cu
 * Project: cuda
 * Created Date: 13/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 *
 */

#include <cuda_runtime_api.h>

#include <complex>

#include "./kernel.h"

namespace autd3 {
namespace gain {
namespace holo {

__global__ void make_complex_kernel(const double* r, const double* i, const uint32_t row, const uint32_t col, cuDoubleComplex* c) {
  int xi = blockIdx.x * blockDim.x + threadIdx.x;
  int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= row || yi >= col) return;

  int idx = xi + yi * row;
  c[idx] = make_cuDoubleComplex(r[idx], i[idx]);
}

void cu_make_complex(const double* r, const double* i, const uint32_t row, const uint32_t col, cuDoubleComplex* c) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((row - 1) / BLOCK_SIZE + 1, (col - 1) / BLOCK_SIZE + 1, 1);
  make_complex_kernel<<<grid, block>>>(r, i, row, col, c);
}

}  // namespace holo
}  // namespace gain
}  // namespace autd3
