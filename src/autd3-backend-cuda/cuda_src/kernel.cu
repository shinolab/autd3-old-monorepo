/*
 * File: kernel.cu
 * Project: cuda_src
 * Created Date: 06/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#include <cuComplex.h>

#include <cstdint>

#ifdef AUTD3_USE_SINGLE_FLOAT
#define makeAUTDComplex make_cuComplex
#else
#define makeAUTDComplex make_cuDoubleComplex
#endif

#ifdef AUTD3_USE_SINGLE_FLOAT
typedef float autd3_float_t;
typedef cuComplex autd3_complex_t;
#else
typedef double autd3_float_t;
typedef cuDoubleComplex autd3_complex_t;
#endif

__device__ autd3_float_t absc2(const autd3_complex_t x) { return x.x * x.x + x.y * x.y; }
__device__ autd3_float_t absc(const autd3_complex_t x) { return sqrt(absc2(x)); }
__device__ autd3_complex_t conj(const autd3_complex_t a) { return makeAUTDComplex(a.x, -a.y); }
__device__ autd3_complex_t mulc(const autd3_complex_t a, const autd3_complex_t b) {
  return makeAUTDComplex(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}
__device__ autd3_complex_t mulcr(const autd3_complex_t a, const autd3_float_t b) { return makeAUTDComplex(a.x * b, a.y * b); }

__device__ autd3_complex_t divcr(const autd3_complex_t x, const autd3_float_t y) {
  const autd3_float_t r = x.x / y;
  const autd3_float_t i = x.y / y;
  return makeAUTDComplex(r, i);
}

__global__ void cu_gs_normalize_kernel(const autd3_complex_t *x, uint32_t n, autd3_complex_t *y) {
  unsigned int i = blockDim.x * blockIdx.x + threadIdx.x;
  if (i >= n) return;
  y[i] = mulcr(divcr(y[i], absc(y[i])), x[i].x);
}

__global__ void cu_gspat_normalize_kernel(const autd3_complex_t *x, const autd3_complex_t *y, uint32_t n, autd3_complex_t *z) {
  unsigned int i = blockDim.x * blockIdx.x + threadIdx.x;
  if (i >= n) return;
  z[i] = mulcr(divcr(y[i], absc(y[i])), x[i].x);
}

__global__ void cu_gspat_normalize2_kernel(const autd3_complex_t *x, const autd3_complex_t *y, uint32_t n, autd3_complex_t *z) {
  unsigned int i = blockDim.x * blockIdx.x + threadIdx.x;
  if (i >= n) return;
  z[i] = mulcr(divcr(y[i], absc2(y[i])), x[i].x * x[i].x);
}

__global__ void get_diagonal_kernel(const autd3_complex_t *a, uint32_t row, uint32_t col, autd3_complex_t *b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  if (xi == yi) {
    unsigned int idx = yi + xi * row;
    b[xi] = a[idx];
  }
}

__global__ void get_diagonal_kernel(const autd3_float_t *a, uint32_t row, uint32_t col, autd3_float_t *b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  if (xi == yi) {
    unsigned int idx = yi + xi * row;
    b[xi] = a[idx];
  }
}

__global__ void set_diagonal_kernel_c(const autd3_complex_t *a, uint32_t n, autd3_complex_t *b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  if (xi >= n) return;
  unsigned int idx = xi + xi * n;
  b[idx] = a[xi];
}

__global__ void set_diagonal_kernel(const autd3_float_t *a, uint32_t n, autd3_float_t *b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  if (xi >= n) return;
  unsigned int idx = xi + xi * n;
  b[idx] = a[xi];
}

__global__ void reciprocal_kernel(const autd3_complex_t *a, const uint32_t row, const uint32_t col, autd3_complex_t *b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  autd3_float_t s = absc2(a[idx]);
  const autd3_float_t x = a[idx].x / s;
  const autd3_float_t y = -a[idx].y / s;
  b[idx] = makeAUTDComplex(x, y);
}

__global__ void hadamard_product_kernel(const autd3_complex_t *a, const autd3_complex_t *b, const uint32_t row, const uint32_t col,
                                        autd3_complex_t *c) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  c[idx] = mulc(a[idx], b[idx]);
}

__global__ void abs_kernel(const autd3_complex_t *a, const uint32_t row, const uint32_t col, autd3_float_t *b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = absc(a[idx]);
}

__global__ void sqrt_kernel(const autd3_float_t *a, const uint32_t row, const uint32_t col, autd3_float_t *b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = sqrt(a[idx]);
}

__global__ void make_complex_kernel(const autd3_float_t *re, const autd3_float_t *im, const uint32_t row, const uint32_t col, autd3_complex_t *dst) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  dst[idx] = makeAUTDComplex(re[idx], im[idx]);
}

__global__ void pow_kernel(const autd3_float_t *a, const autd3_float_t p, const uint32_t row, const uint32_t col, autd3_float_t *b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = pow(a[idx], p);
}

__global__ void conj_kernel(const autd3_complex_t *a, const uint32_t row, const uint32_t col, autd3_complex_t *b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = conj(a[idx]);
}

__global__ void calc_singular_inv_kernel(autd3_float_t *d_s, uint32_t row, uint32_t col, autd3_float_t alpha, autd3_complex_t *p_singular_inv) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  if (xi == yi)
    p_singular_inv[yi + xi * row] = makeAUTDComplex(d_s[xi] / (d_s[xi] * d_s[xi] + alpha), 0.0);
  else
    p_singular_inv[yi + xi * row] = makeAUTDComplex(0.0, 0.0);
}

__device__ autd3_complex_t expc(const autd3_complex_t x) {
  const autd3_float_t s = exp(x.x);
  const autd3_float_t r = cos(x.y);
  const autd3_float_t i = sin(x.y);
  return makeAUTDComplex(s * r, s * i);
}

__global__ void exp_kernel(const autd3_complex_t *a, const uint32_t row, const uint32_t col, autd3_complex_t *b) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  b[idx] = expc(a[idx]);
}

__global__ void real_kernel(const autd3_complex_t *src, const uint32_t row, const uint32_t col, autd3_float_t *dst) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  dst[idx] = src[idx].x;
}
__global__ void imag_kernel(const autd3_complex_t *src, const uint32_t row, const uint32_t col, autd3_float_t *dst) {
  unsigned int xi = blockIdx.x * blockDim.x + threadIdx.x;
  unsigned int yi = blockIdx.y * blockDim.y + threadIdx.y;
  if (xi >= col || yi >= row) return;

  unsigned int idx = yi + xi * row;
  dst[idx] = src[idx].y;
}

__global__ void col_sum_kernel(const autd3_float_t *din, uint32_t m, uint32_t n, autd3_float_t *dout) {
  uint32_t row = blockIdx.y * blockDim.y + threadIdx.y;
  if (row >= m) return;
  autd3_float_t sum = 0;
  for (uint32_t col = 0; col < n; col++) sum += din[col * m + row];
  dout[row] = sum;
}

#ifdef __cplusplus
extern "C" {
#endif

#define BLOCK_SIZE (32)

void cu_gs_normalize(const autd3_complex_t *x, const uint32_t len, autd3_complex_t *y) {
  unsigned int blocksPerGrid = (len + BLOCK_SIZE - 1) / BLOCK_SIZE;
  cu_gs_normalize_kernel<<<blocksPerGrid, BLOCK_SIZE>>>(x, len, y);
}

void cu_gspat_normalize(const autd3_complex_t *x, const autd3_complex_t *y, const uint32_t len, autd3_complex_t *z) {
  unsigned int blocksPerGrid = (len + BLOCK_SIZE - 1) / BLOCK_SIZE;
  cu_gspat_normalize_kernel<<<blocksPerGrid, BLOCK_SIZE>>>(x, y, len, z);
}

void cu_gspat_normalize2(const autd3_complex_t *x, const autd3_complex_t *y, const uint32_t len, autd3_complex_t *z) {
  unsigned int blocksPerGrid = (len + BLOCK_SIZE - 1) / BLOCK_SIZE;
  cu_gspat_normalize2_kernel<<<blocksPerGrid, BLOCK_SIZE>>>(x, y, len, z);
}

void cu_get_diagonal(const autd3_float_t *a, const uint32_t row, const uint32_t col, autd3_float_t *b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  get_diagonal_kernel<<<grid, block>>>(a, row, col, b);
}

void cu_get_diagonal_c(const autd3_complex_t *a, const uint32_t row, const uint32_t col, autd3_complex_t *b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  get_diagonal_kernel<<<grid, block>>>(a, row, col, b);
}

void cu_set_diagonal(const autd3_float_t *a, const uint32_t n, autd3_float_t *b) {
  dim3 block(BLOCK_SIZE * BLOCK_SIZE, 1, 1);
  dim3 grid((n - 1) / (BLOCK_SIZE * BLOCK_SIZE) + 1, 1, 1);
  set_diagonal_kernel<<<grid, block>>>(a, n, b);
}

void cu_set_diagonal_c(const autd3_complex_t *a, const uint32_t n, autd3_complex_t *b) {
  dim3 block(BLOCK_SIZE * BLOCK_SIZE, 1, 1);
  dim3 grid((n - 1) / (BLOCK_SIZE * BLOCK_SIZE) + 1, 1, 1);
  set_diagonal_kernel_c<<<grid, block>>>(a, n, b);
}

void cu_reciprocal(const autd3_complex_t *a, const uint32_t row, const uint32_t col, autd3_complex_t *b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  reciprocal_kernel<<<grid, block>>>(a, row, col, b);
}

void cu_hadamard_product(const autd3_complex_t *a, const autd3_complex_t *b, const uint32_t row, const uint32_t col, autd3_complex_t *c) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  hadamard_product_kernel<<<grid, block>>>(a, b, row, col, c);
}

void cu_abs(const autd3_complex_t *a, const uint32_t row, const uint32_t col, autd3_float_t *b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  abs_kernel<<<grid, block>>>(a, row, col, b);
}

void cu_sqrt(const autd3_float_t *a, const uint32_t row, const uint32_t col, autd3_float_t *b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  sqrt_kernel<<<grid, block>>>(a, row, col, b);
}

void cu_make_complex(const autd3_float_t *re, const autd3_float_t *im, const uint32_t row, const uint32_t col, autd3_complex_t *dst) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  make_complex_kernel<<<grid, block>>>(re, im, row, col, dst);
}

void cu_pow(const autd3_float_t *a, const autd3_float_t p, const uint32_t row, const uint32_t col, autd3_float_t *b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  pow_kernel<<<grid, block>>>(a, p, row, col, b);
}

void cu_conj(const autd3_complex_t *a, const uint32_t row, const uint32_t col, autd3_complex_t *b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  conj_kernel<<<grid, block>>>(a, row, col, b);
}

void cu_calc_singular_inv(autd3_float_t *d_s, const uint32_t row, const uint32_t col, const autd3_float_t alpha, autd3_complex_t *p_singular_inv) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  calc_singular_inv_kernel<<<grid, block>>>(d_s, row, col, alpha, p_singular_inv);
}

void cu_exp(const autd3_complex_t *a, const uint32_t row, const uint32_t col, autd3_complex_t *b) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  exp_kernel<<<grid, block>>>(a, row, col, b);
}

void cu_real(const autd3_complex_t *src, const uint32_t row, const uint32_t col, autd3_float_t *dst) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  real_kernel<<<grid, block>>>(src, row, col, dst);
}

void cu_imag(const autd3_complex_t *src, const uint32_t row, const uint32_t col, autd3_float_t *dst) {
  dim3 block(BLOCK_SIZE, BLOCK_SIZE, 1);
  dim3 grid((col - 1) / BLOCK_SIZE + 1, (row - 1) / BLOCK_SIZE + 1, 1);
  imag_kernel<<<grid, block>>>(src, row, col, dst);
}

void cu_reduce_col(const autd3_float_t *mat, const uint32_t m, const uint32_t n, autd3_float_t *result) {
  dim3 block(1, BLOCK_SIZE * BLOCK_SIZE, 1);
  dim3 grid(1, (m - 1) / (BLOCK_SIZE * BLOCK_SIZE) + 1, 1);
  col_sum_kernel<<<grid, block>>>(mat, m, n, result);
}

#ifdef __cplusplus
}
#endif
