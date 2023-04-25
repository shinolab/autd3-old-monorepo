// File: boost_wrap.h
// Project: autd3-link-simulator
// Created Date: 25/04/2023
// Author: Shun Suzuki
// -----
// Last Modified: 25/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#ifndef _BOOST_WRAP_H
#define _BOOST_WRAP_H

#ifdef __cplusplus
#include <cstdint>
#else
#include <stdbool.h>
#include <stdint.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

bool shmem_create();
bool shmem_copy_to(const uint8_t* data, size_t size);
bool shmem_copy_from(uint8_t* data, size_t offset, size_t size);

#ifdef __cplusplus
}
#endif

#endif /* _BOOST_WRAP_H */
