/*
 * File: kernel.cu
 * Project: cuda
 * Created Date: 13/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#include <cuda_runtime_api.h>

#include <complex>

#include "./kernel.h"

#ifdef AUTD3_USE_SINGLE_FLOAT
#define makeAUTDComplex make_cuComplex
#else
#define makeAUTDComplex make_cuDoubleComplex
#endif

namespace autd3 {
namespace gain {
namespace holo {

__device__ autd3_float_t absc2(const autd3_complex_t x) { return x.x * x.x + x.y * x.y; }
__device__ autd3_float_t absc(const autd3_complex_t x) { return sqrt(absc2(x)); }
__device__ autd3_complex_t conj(const autd3_complex_t a) { return makeAUTDComplex(a.x, -a.y); }
__device__ autd3_complex_t mulc(const autd3_complex_t a, const autd3_complex_t b) {
  return makeAUTDComplex(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

__global__ void abs_kernel(const autd3_complex_t* a, const uint32_t row, const uint32_t col, autd3_complex_t* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = makeAUTDComplex(absc(a[idx]), 0.0);
}
__global__ void abs_kernel(const autd3_complex_t* a, const uint32_t row, const uint32_t col, autd3_float_t* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = absc(a[idx]);
}

void cu_abs(const autd3_complex_t* a, const uint32_t row, const uint32_t col, autd3_complex_t* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  abs_kernel<<<grid, block>>>(a, row, col, b);
}
void cu_abs(const autd3_complex_t* a, const uint32_t row, const uint32_t col, autd3_float_t* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  abs_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void sqrt_kernel(const autd3_float_t* a, const uint32_t row, const uint32_t col, autd3_float_t* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = sqrt(a[idx]);
}

void cu_sqrt(const autd3_float_t* a, const uint32_t row, const uint32_t col, autd3_float_t* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  sqrt_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void conj_kernel(const autd3_complex_t* a, const uint32_t row, const uint32_t col, autd3_complex_t* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = conj(a[idx]);
}

void cu_conj(const autd3_complex_t* a, const uint32_t row, const uint32_t col, autd3_complex_t* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  conj_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void arg_kernel(const autd3_complex_t* a, const uint32_t row, const uint32_t col, autd3_complex_t* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  const autd3_float_t s = absc(a[idx]);
  const autd3_float_t x = a[idx].x / s;
  const autd3_float_t y = a[idx].y / s;
  b[idx] = makeAUTDComplex(x, y);
}

void cu_arg(const autd3_complex_t* a, const uint32_t row, const uint32_t col, autd3_complex_t* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  arg_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void reciprocal_kernel(const autd3_complex_t* a, const uint32_t row, const uint32_t col, autd3_complex_t* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  autd3_float_t s = absc2(a[idx]);
  const autd3_float_t x = a[idx].x / s;
  const autd3_float_t y = -a[idx].y / s;
  b[idx] = makeAUTDComplex(x, y);
}

void cu_reciprocal(const autd3_complex_t* a, const uint32_t row, const uint32_t col, autd3_complex_t* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  reciprocal_kernel<<<grid, block>>>(a, row, col, b);
}

__device__ autd3_complex_t expc(const autd3_complex_t x) {
  const autd3_float_t s = exp(x.x);
  const autd3_float_t r = cos(x.y);
  const autd3_float_t i = sin(x.y);
  return makeAUTDComplex(s * r, s * i);
}

__global__ void exp_kernel(const autd3_complex_t* a, const uint32_t row, const uint32_t col, autd3_complex_t* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = expc(a[idx]);
}

void cu_exp(const autd3_complex_t* a, const uint32_t row, const uint32_t col, autd3_complex_t* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  exp_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void pow_kernel(const autd3_float_t* a, const autd3_float_t p, const uint32_t row, const uint32_t col, autd3_float_t* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = pow(a[idx], p);
}

void cu_pow(const autd3_float_t* a, const autd3_float_t p, const uint32_t row, const uint32_t col, autd3_float_t* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  pow_kernel<<<grid, block>>>(a, p, row, col, b);
}

__global__ void imag_kernel(const autd3_complex_t* src, const uint32_t row, const uint32_t col, autd3_float_t* dst) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  dst[idx] = src[idx].y;
}

void cu_imag(const autd3_complex_t* src, const uint32_t row, const uint32_t col, autd3_float_t* dst) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  imag_kernel<<<grid, block>>>(src, row, col, dst);
}

__global__ void real_kernel(const autd3_complex_t* src, const uint32_t row, const uint32_t col, autd3_float_t* dst) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  dst[idx] = src[idx].x;
}

void cu_real(const autd3_complex_t* src, const uint32_t row, const uint32_t col, autd3_float_t* dst) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  real_kernel<<<grid, block>>>(src, row, col, dst);
}

__global__ void make_complex_kernel(const autd3_float_t* re, const autd3_float_t* im, const uint32_t row, const uint32_t col, autd3_complex_t* dst) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  dst[idx] = makeAUTDComplex(re[idx], im[idx]);
}

void cu_make_complex(const autd3_float_t* re, const autd3_float_t* im, const uint32_t row, const uint32_t col, autd3_complex_t* dst) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  make_complex_kernel<<<grid, block>>>(re, im, row, col, dst);
}

__global__ void set_diagonal_kernel(const autd3_complex_t* a, uint32_t row, uint32_t col, autd3_complex_t* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = xi == yi ? a[xi] : makeAUTDComplex(0.0, 0.0);
}

void cu_set_diagonal(const autd3_complex_t* a, const uint32_t row, const uint32_t col, autd3_complex_t* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  set_diagonal_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void get_diagonal_kernel(const autd3_complex_t* a, uint32_t row, uint32_t col, autd3_complex_t* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  if (xi == yi) {
    unsigned int idx = yi + xi * row;
    b[xi] = a[idx];
  }
}

void cu_get_diagonal(const autd3_complex_t* a, const uint32_t row, const uint32_t col, autd3_complex_t* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  get_diagonal_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void get_diagonal_kernel(const autd3_float_t* a, uint32_t row, uint32_t col, autd3_float_t* b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  if (xi == yi) {
    unsigned int idx = yi + xi * row;
    b[xi] = a[idx];
  }
}

void cu_get_diagonal(const autd3_float_t* a, const uint32_t row, const uint32_t col, autd3_float_t* b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  get_diagonal_kernel<<<grid, block>>>(a, row, col, b);
}

__global__ void hadamard_product_kernel(const autd3_complex_t* a, const autd3_complex_t* b, const uint32_t row, const uint32_t col,
                                        autd3_complex_t* c) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  c[idx] = mulc(a[idx], b[idx]);
}

void cu_hadamard_product(const autd3_complex_t* a, const autd3_complex_t* b, const uint32_t row, const uint32_t col, autd3_complex_t* c) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  hadamard_product_kernel<<<grid, block>>>(a, b, row, col, c);
}

__global__ void calc_singular_inv_kernel(autd3_float_t* d_s, uint32_t row, uint32_t col, autd3_float_t alpha, autd3_complex_t* p_singular_inv) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  if (xi == yi)
    p_singular_inv[yi + xi * row] = makeAUTDComplex(d_s[xi] / (d_s[xi] * d_s[xi] + alpha), 0.0);
  else
    p_singular_inv[yi + xi * row] = makeAUTDComplex(0.0, 0.0);
}

void cu_calc_singular_inv(autd3_float_t* d_s, const uint32_t row, const uint32_t col, const autd3_float_t alpha, autd3_complex_t* p_singular_inv) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  calc_singular_inv_kernel<<<grid, block>>>(d_s, row, col, alpha, p_singular_inv);
}

__global__ void calc_singular_inv_kernel(autd3_float_t* d_s, uint32_t row, uint32_t col, autd3_float_t alpha, autd3_float_t* p_singular_inv) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  if (xi == yi)
    p_singular_inv[yi + xi * row] = d_s[xi] / (d_s[xi] * d_s[xi] + alpha);
  else
    p_singular_inv[yi + xi * row] = 0.0;
}

void cu_calc_singular_inv(autd3_float_t* d_s, const uint32_t row, const uint32_t col, const autd3_float_t alpha, autd3_float_t* p_singular_inv) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  calc_singular_inv_kernel<<<grid, block>>>(d_s, row, col, alpha, p_singular_inv);
}

__global__ void col_sum_kernel(const autd3_float_t* din, uint32_t m, uint32_t n, autd3_float_t* dout) {
  extern __shared__ autd3_float_t smem[];

  uint32_t row = blockIdx.y * blockDim.y + threadIdx.y;
  if (row >= m) return;

  uint32_t tid = threadIdx.x;
  uint32_t i = blockIdx.x * (blockDim.x * 2) + threadIdx.x;
  autd3_float_t local_sum = i < n ? din[i * m + row] : 0;
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

void cu_reduce_col(const autd3_float_t* mat, const uint32_t m, const uint32_t n, autd3_float_t* result, autd3_float_t* buffer) {
  dim3 block(BLOCK_SIZE / 2, 1, 1);
  dim3 grid((n - 1) / BLOCK_SIZE + 1, m, 1);
  col_sum_kernel<<<grid, block, BLOCK_SIZE * sizeof(autd3_float_t)>>>(mat, m, n, buffer);
  col_sum_kernel<<<dim3(1, m, 1), dim3(max((grid.x + 1) / 2, 1), 1, 1), max(grid.x, 2) * sizeof(autd3_float_t)>>>(buffer, m, grid.x, result);
}

}  // namespace holo
}  // namespace gain
}  // namespace autd3
