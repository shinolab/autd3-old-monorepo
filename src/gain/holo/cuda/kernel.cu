/*
 * File: kernel.cu
 * Project: cuda
 * Created Date: 13/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/05/2022
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

__device__ double absc2(const cuDoubleComplex x) { return x.x * x.x + x.y * x.y; }
__device__ double absc(const cuDoubleComplex x) { return sqrt(absc2(x)); }
__device__ cuDoubleComplex conj(const cuDoubleComplex a) { return make_cuDoubleComplex(a.x, -a.y); }
__device__ cuDoubleComplex mulc(const cuDoubleComplex a, const cuDoubleComplex b) {
  return make_cuDoubleComplex(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

__global__ void abs_kernel(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = make_cuDoubleComplex(absc(a[idx]), 0.0);
}
__global__ void abs_kernel(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, double* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = absc(a[idx]);
}

void cu_abs(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  abs_kernel<<<grid, block>>>(a, row, col, b);
}
void cu_abs(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, double* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  abs_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void sqrt_kernel(const double* a, const uint32_t row, const uint32_t col, double* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = sqrt(a[idx]);
}

void cu_sqrt(const double* a, const uint32_t row, const uint32_t col, double* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  sqrt_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void conj_kernel(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = conj(a[idx]);
}

void cu_conj(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  conj_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void arg_kernel(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  const double s = absc(a[idx]);
  const double x = a[idx].x / s;
  const double y = a[idx].y / s;
  b[idx] = make_cuDoubleComplex(x, y);
}

void cu_arg(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  arg_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void reciprocal_kernel(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  double s = absc2(a[idx]);
  const double x = a[idx].x / s;
  const double y = -a[idx].y / s;
  b[idx] = make_cuDoubleComplex(x, y);
}

void cu_reciprocal(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  reciprocal_kernel<<<grid, block>>>(a, row, col, b);
}

__device__ cuDoubleComplex expc(const cuDoubleComplex x) {
  const double s = exp(x.x);
  const double r = cos(x.y);
  const double i = sin(x.y);
  return make_cuDoubleComplex(s * r, s * i);
}

__global__ void exp_kernel(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = expc(a[idx]);
}

void cu_exp(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  exp_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void pow_kernel(const double* a, const double p, const uint32_t row, const uint32_t col, double* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = pow(a[idx], p);
}

void cu_pow(const double* a, const double p, const uint32_t row, const uint32_t col, double* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  pow_kernel<<<grid, block>>>(a, p, row, col, b);
}

__global__ void imag_kernel(const cuDoubleComplex* src, const uint32_t row, const uint32_t col, double* dst) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  dst[idx] = src[idx].y;
}

void cu_imag(const cuDoubleComplex* src, const uint32_t row, const uint32_t col, double* dst) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  imag_kernel<<<grid, block>>>(src, row, col, dst);
}

__global__ void real_kernel(const cuDoubleComplex* src, const uint32_t row, const uint32_t col, double* dst) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  dst[idx] = src[idx].x;
}

void cu_real(const cuDoubleComplex* src, const uint32_t row, const uint32_t col, double* dst) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  real_kernel<<<grid, block>>>(src, row, col, dst);
}

__global__ void make_complex_kernel(const double* re, const double* im, const uint32_t row, const uint32_t col, cuDoubleComplex* dst) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  dst[idx] = make_cuDoubleComplex(re[idx], im[idx]);
}

void cu_make_complex(const double* re, const double* im, const uint32_t row, const uint32_t col, cuDoubleComplex* dst) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  make_complex_kernel<<<grid, block>>>(re, im, row, col, dst);
}

__global__ void set_diagonal_kernel(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = xi == yi ? a[xi] : make_cuDoubleComplex(0.0, 0.0);
}

void cu_set_diagonal(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  set_diagonal_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void get_diagonal_kernel(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  if (xi == yi) {
    unsigned int idx = yi + xi * row;
    b[xi] = a[idx];
  }
}

void cu_get_diagonal(const cuDoubleComplex* a, const uint32_t row, const uint32_t col, cuDoubleComplex* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  get_diagonal_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void get_diagonal_kernel(const double* a, uint32_t row, uint32_t col, double* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  if (xi == yi) {
    unsigned int idx = yi + xi * row;
    b[xi] = a[idx];
  }
}

void cu_get_diagonal(const double* a, const uint32_t row, const uint32_t col, double* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  get_diagonal_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void hadamard_product_kernel(const cuDoubleComplex* a, const cuDoubleComplex* b, const uint32_t row, const uint32_t col,
                                        cuDoubleComplex* c) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  c[idx] = mulc(a[idx], b[idx]);
}

void cu_hadamard_product(const cuDoubleComplex* a, const cuDoubleComplex* b, const uint32_t row, const uint32_t col, cuDoubleComplex* c) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  hadamard_product_kernel<<<grid, block>>>(a, b, row, col, c);
}

__global__ void calc_singular_inv_kernel(double* d_s, uint32_t row, uint32_t col, double alpha, cuDoubleComplex* p_singular_inv) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  if (xi == yi)
    p_singular_inv[yi + xi * row] = make_cuDoubleComplex(d_s[xi] / (d_s[xi] * d_s[xi] + alpha), 0.0);
  else
    p_singular_inv[yi + xi * row] = make_cuDoubleComplex(0.0, 0.0);
}

void cu_calc_singular_inv(double* d_s, const uint32_t row, const uint32_t col, const double alpha, cuDoubleComplex* p_singular_inv) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  calc_singular_inv_kernel<<<grid, block>>>(d_s, row, col, alpha, p_singular_inv);
}

__global__ void calc_singular_inv_kernel(double* d_s, uint32_t row, uint32_t col, double alpha, double* p_singular_inv) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  if (xi == yi)
    p_singular_inv[yi + xi * row] = d_s[xi] / (d_s[xi] * d_s[xi] + alpha);
  else
    p_singular_inv[yi + xi * row] = 0.0;
}

void cu_calc_singular_inv(double* d_s, const uint32_t row, const uint32_t col, const double alpha, double* p_singular_inv) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  calc_singular_inv_kernel<<<grid, block>>>(d_s, row, col, alpha, p_singular_inv);
}

__global__ void col_sum_kernel(const double* din, uint32_t m, uint32_t n, double* dout) {
  extern __shared__ double smem[];

  uint32_t row = blockIdx.y * blockDim.y + threadIdx.y;
  if (row >= m) return;

  uint32_t tid = threadIdx.x;
  uint32_t i = blockIdx.x * (blockDim.x * 2) + threadIdx.x;
  double local_sum = i < n ? din[i * m + row] : 0;
  if (i + blockDim.x < n) {
    local_sum += din[(i + blockDim.x) * m + row];
  }
  smem[tid] = local_sum;
  __syncthreads();

  for (unsigned int s = blockDim.x >> 1; s > 32; s >>= 1) {
    if (tid < s) {
      smem[tid] = local_sum = local_sum + smem[tid + s];
    }
    __syncthreads();
  }
  if (tid < 32) {
    if (blockDim.x >= 64) {
      local_sum += smem[tid + 32];
    }
    for (int offset = 32 >> 1; offset > 0; offset >>= 1) {
      local_sum += __shfl_down_sync(0xffffffff, local_sum, offset);
    }
  }
  if (tid == 0) {
    dout[blockIdx.x * m + row] = local_sum;
  }
}

void cu_reduce_col(const double* mat, const uint32_t m, const uint32_t n, double* result, double* buffer) {
  dim3 block(BLOCK_SIZE / 2, 1, 1);
  dim3 grid((n - 1) / BLOCK_SIZE + 1, m, 1);
  col_sum_kernel<<<grid, block, BLOCK_SIZE * sizeof(double)>>>(mat, m, n, buffer);
  col_sum_kernel<<<dim3(1, m, 1), dim3(max((grid.x + 1) / 2, 1), 1, 1), max(grid.x, 2) * sizeof(double)>>>(buffer, m, grid.x, result);
}

}  // namespace holo
}  // namespace gain
}  // namespace autd3
