// File: kernel.h
// Project: cuda
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cuComplex.h>

#include <cstdint>

namespace autd3 {
namespace gain {
namespace holo {

constexpr uint32_t BLOCK_SIZE = 32;

void cu_abs(const cuDoubleComplex* a, uint32_t row, uint32_t col, double* b);
void cu_abs(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b);
void cu_sqrt(const double* a, uint32_t row, uint32_t col, double* b);
void cu_conj(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b);
void cu_arg(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b);
void cu_reciprocal(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b);
void cu_exp(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b);
void cu_pow(const double* a, double p, uint32_t row, uint32_t col, double* b);

void cu_real(const cuDoubleComplex* src, uint32_t row, uint32_t col, double* dst);
void cu_imag(const cuDoubleComplex* src, uint32_t row, uint32_t col, double* dst);
void cu_make_complex(const double* re, const double* im, uint32_t row, uint32_t col, cuDoubleComplex* dst);

void cu_set_diagonal(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b);
void cu_get_diagonal(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b);
void cu_get_diagonal(const double* a, uint32_t row, uint32_t col, double* b);

void cu_hadamard_product(const cuDoubleComplex* a, const cuDoubleComplex* b, uint32_t row, uint32_t col, cuDoubleComplex* c);

void cu_calc_singular_inv(double* d_s, uint32_t row, uint32_t col, double alpha, cuDoubleComplex* p_singular_inv);
void cu_calc_singular_inv(double* d_s, uint32_t row, uint32_t col, double alpha, double* p_singular_inv);

void cu_reduce_col(const double* mat, uint32_t m, uint32_t n, double* result, double* buffer);
}  // namespace holo
}  // namespace gain
}  // namespace autd3
