// File: smem.hpp
// Project: smem
// Created Date: 27/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#ifdef _WIN32
#include <windows.h>
#endif

#include <string>

namespace smem {

class SMem {
 public:
  SMem() noexcept = default;
  SMem(const SMem& v) noexcept = default;
  SMem& operator=(const SMem& obj) = default;
  SMem(SMem&& obj) = default;
  SMem& operator=(SMem&& obj) = default;
  ~SMem();
  void create(const std::string& name, size_t size);
  void* map();
  void unmap();
  void close();

 protected:
#ifdef _WIN32
  HANDLE _handle{};
#else
  int _seg_id{-1};
  std::string _key_path;
#endif

  void* _ptr{};
  size_t _size{};
};

}  // namespace smem

#ifdef SMEM_HEADER_ONLY
#include "impl/smem.cpp"
#endif
