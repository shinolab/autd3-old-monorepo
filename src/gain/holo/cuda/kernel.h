// File: kernel.h
// Project: cuda
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 23/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cuComplex.h>

#include <cstdint>

#ifdef AUTD3_USE_SINGLE_FLOAT
typedef float autd3_float_t;
typedef cuComplex autd3_complex_t;
#else
typedef double autd3_float_t;
typedef cuDoubleComplex autd3_complex_t;
#endif

namespace autd3 {
namespace gain {
namespace holo {

constexpr uint32_t BLOCK_SIZE = 32;

void cu_abs(const autd3_complex_t* a, uint32_t row, uint32_t col, autd3_float_t* b);
void cu_abs(const autd3_complex_t* a, uint32_t row, uint32_t col, autd3_complex_t* b);
void cu_sqrt(const autd3_float_t* a, uint32_t row, uint32_t col, autd3_float_t* b);
void cu_conj(const autd3_complex_t* a, uint32_t row, uint32_t col, autd3_complex_t* b);
void cu_arg(const autd3_complex_t* a, uint32_t row, uint32_t col, autd3_complex_t* b);
void cu_reciprocal(const autd3_complex_t* a, uint32_t row, uint32_t col, autd3_complex_t* b);
void cu_exp(const autd3_complex_t* a, uint32_t row, uint32_t col, autd3_complex_t* b);
void cu_pow(const autd3_float_t* a, autd3_float_t p, uint32_t row, uint32_t col, autd3_float_t* b);

void cu_real(const autd3_complex_t* src, uint32_t row, uint32_t col, autd3_float_t* dst);
void cu_imag(const autd3_complex_t* src, uint32_t row, uint32_t col, autd3_float_t* dst);
void cu_make_complex(const autd3_float_t* re, const autd3_float_t* im, uint32_t row, uint32_t col, autd3_complex_t* dst);

void cu_set_diagonal(const autd3_complex_t* a, uint32_t row, uint32_t col, autd3_complex_t* b);
void cu_get_diagonal(const autd3_complex_t* a, uint32_t row, uint32_t col, autd3_complex_t* b);
void cu_get_diagonal(const autd3_float_t* a, uint32_t row, uint32_t col, autd3_float_t* b);

void cu_hadamard_product(const autd3_complex_t* a, const autd3_complex_t* b, uint32_t row, uint32_t col, autd3_complex_t* c);

void cu_calc_singular_inv(autd3_float_t* d_s, uint32_t row, uint32_t col, autd3_float_t alpha, autd3_complex_t* p_singular_inv);
void cu_calc_singular_inv(autd3_float_t* d_s, uint32_t row, uint32_t col, autd3_float_t alpha, autd3_float_t* p_singular_inv);

void cu_reduce_col(const autd3_float_t* mat, uint32_t m, uint32_t n, autd3_float_t* result, autd3_float_t* buffer);
}  // namespace holo
}  // namespace gain
}  // namespace autd3
