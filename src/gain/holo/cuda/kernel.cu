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

__device__ double absc2(cuDoubleComplex x) { return x.x * x.x + x.y * x.y; }
__device__ double absc(cuDoubleComplex x) { return sqrt(absc2(x)); }
__device__ cuDoubleComplex conj(cuDoubleComplex a) { return make_cuDoubleComplex(a.x, -a.y); }
__device__ cuDoubleComplex mulc(cuDoubleComplex a, cuDoubleComplex b) { return make_cuDoubleComplex(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x); }

__global__ void abs_kernel(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  int xi = blockIdx.x * blockDim.x + threadIdx.x;
  int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  int idx = yi + xi * row;
  b[idx] = make_cuDoubleComplex(absc(a[idx]), 0.0);
}

void cu_abs(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  abs_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void conj_kernel(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  int xi = blockIdx.x * blockDim.x + threadIdx.x;
  int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  int idx = yi + xi * row;
  b[idx] = conj(a[idx]);
}

void cu_conj(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  conj_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void arg_kernel(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  int xi = blockIdx.x * blockDim.x + threadIdx.x;
  int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  int idx = yi + xi * row;
  double s = absc(a[idx]);
  double x = a[idx].x / s;
  double y = a[idx].y / s;
  b[idx] = make_cuDoubleComplex(x, y);
}

void cu_arg(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  arg_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void reciprocal_kernel(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  int xi = blockIdx.x * blockDim.x + threadIdx.x;
  int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  int idx = yi + xi * row;
  double s = absc2(a[idx]);
  double x = a[idx].x / s;
  double y = -a[idx].y / s;
  b[idx] = make_cuDoubleComplex(x, y);
}

void cu_reciprocal(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  reciprocal_kernel<<<grid, block>>>(a, row, col, b);
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
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  set_diagonal_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void hadamard_product_kernel(const cuDoubleComplex* a, const cuDoubleComplex* b, const uint32_t row, const uint32_t col,
                                        cuDoubleComplex* c) {
  int xi = blockIdx.x * blockDim.x + threadIdx.x;
  int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  int idx = yi + xi * row;
  c[idx] = mulc(a[idx], b[idx]);
}

void cu_hadamard_product(const cuDoubleComplex* a, const cuDoubleComplex* b, uint32_t row, uint32_t col, cuDoubleComplex* c) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  hadamard_product_kernel<<<grid, block>>>(a, b, row, col, c);
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
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  calc_singular_inv_kernel<<<grid, block>>>(d_s, row, col, alpha, p_singular_inv);
}

__global__ void col_sum_kernel(const cuDoubleComplex* din, uint32_t m, uint32_t n, cuDoubleComplex* dout) {
  extern __shared__ double smem[];

  uint32_t row = blockIdx.y * blockDim.y + threadIdx.y;
  if (row >= m) return;

  uint32_t tid = threadIdx.x;
  uint32_t i = blockIdx.x * (blockDim.x * 2) + threadIdx.x;
  double local_sum_r = (i < n) ? din[i * m + row].x : 0;
  double local_sum_i = (i < n) ? din[i * m + row].y : 0;
  if (i + blockDim.x < n) {
    local_sum_r += din[(i + blockDim.x) * m + row].x;
    local_sum_i += din[(i + blockDim.x) * m + row].y;
  }
  smem[2 * tid] = local_sum_r;
  smem[2 * tid + 1] = local_sum_i;
  __syncthreads();

  for (unsigned int s = blockDim.x >> 1; s > 32; s >>= 1) {
    if (tid < s) {
      smem[2 * tid] = local_sum_r = local_sum_r + smem[2 * (tid + s)];
      smem[2 * tid + 1] = local_sum_i = local_sum_i + smem[2 * (tid + s) + 1];
    }
    __syncthreads();
  }
  if (tid < 32) {
    if (blockDim.x >= 64) {
      local_sum_r += smem[2 * (tid + 32)];
      local_sum_i += smem[2 * (tid + 32) + 1];
    }
    for (int offset = 32 >> 1; offset > 0; offset >>= 1) {
      local_sum_r += __shfl_down_sync(0xffffffff, local_sum_r, offset);
      local_sum_i += __shfl_down_sync(0xffffffff, local_sum_i, offset);
    }
  }
  if (tid == 0) {
    dout[blockIdx.x * m + row].x = local_sum_r;
    dout[blockIdx.x * m + row].y = local_sum_i;
  }
}

void cu_reduce_col(const cuDoubleComplex* mat, uint32_t m, uint32_t n, cuDoubleComplex* result, cuDoubleComplex* buffer) {
  dim3 block(BLOCK_SIZE / 2, 1, 1);
  dim3 grid((n - 1) / BLOCK_SIZE + 1, m, 1);

  col_sum_kernel<<<grid, block, BLOCK_SIZE * sizeof(cuDoubleComplex)>>>(mat, m, n, buffer);
  col_sum_kernel<<<dim3(1, m, 1), dim3(max((grid.x + 1) / 2, 1), 1, 1), max(grid.x, 2) * sizeof(cuDoubleComplex)>>>(buffer, m, grid.x, result);
}

}  // namespace holo
}  // namespace gain
}  // namespace autd3
