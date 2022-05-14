// File: kernel.h
// Project: cuda
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 14/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <cuComplex.h>

#include <cstdint>

namespace autd3 {
namespace gain {
namespace holo {

constexpr uint32_t BLOCK_SIZE = 32;

void cu_abs(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b);
void cu_conj(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b);
void cu_arg(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b);
void cu_reciprocal(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b);

void cu_set_diagonal(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b);
void cu_get_diagonal(const cuDoubleComplex* a, uint32_t row, uint32_t col, cuDoubleComplex* b);

void cu_hadamard_product(const cuDoubleComplex* a, const cuDoubleComplex* b, uint32_t row, uint32_t col, cuDoubleComplex* c);

void cu_calc_singular_inv(double* d_s, uint32_t row, uint32_t col, double alpha, cuDoubleComplex* p_singular_inv);
}  // namespace holo
}  // namespace gain
}  // namespace autd3
