/*
 * File: kernel.cu
 * Project: cuda
 * Created Date: 13/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/05/2022
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

__device__ cuDoubleComplex conj(cuDoubleComplex a) { return make_cuDoubleComplex(a.x, -a.y); }

__global__ void conj_kernel(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  int xi = blockIdx.x * blockDim.x + threadIdx.x;
  int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  int idx = yi + xi * row;
  b[idx] = conj(a[idx]);
}

void cu_conj(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((row - 1) / BLOCK_SIZE + 1, (col - 1) / BLOCK_SIZE + 1, 1);
  conj_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void set_diagonal_kernel(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b) {
  int xi = blockIdx.x * blockDim.x + threadIdx.x;
  int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  int idx = yi + xi * row;
  b[idx] = xi == yi ? a[xi] : make_cuDoubleComplex(0.0, 0.0);
}

void cu_set_diagonal(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((row - 1) / BLOCK_SIZE + 1, (col - 1) / BLOCK_SIZE + 1, 1);
  set_diagonal_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void calc_singular_inv_kernel(double* d_s, uint32_t row, uint32_t col, double alpha, cuDoubleComplex* p_singular_inv) {
  int xi = blockIdx.x * blockDim.x + threadIdx.x;
  int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  if (xi == yi)
    p_singular_inv[yi + xi * row] = make_cuDoubleComplex(d_s[xi] / (d_s[xi] * d_s[xi] + alpha), 0.0);
  else
    p_singular_inv[yi + xi * row] = make_cuDoubleComplex(0.0, 0.0);
}

void cu_calc_singular_inv(double* d_s, uint32_t row, uint32_t col, double alpha, cuDoubleComplex* p_singular_inv) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((row - 1) / BLOCK_SIZE + 1, (col - 1) / BLOCK_SIZE + 1, 1);
  calc_singular_inv_kernel<<<grid, block>>>(d_s, row, col, alpha, p_singular_inv);
}

}  // namespace holo
}  // namespace gain
}  // namespace autd3
