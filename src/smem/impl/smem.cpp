// File: smem.cpp
// Project: impl
// Created Date: 27/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 21/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#ifdef SMEM_HEADER_ONLY
#define SMEM_HEADER_ONLY_INLINE inline
#else
#define SMEM_HEADER_ONLY_INLINE
#endif

#include "smem.hpp"

#include <sstream>
#include <system_error>

#ifdef _WIN32

SMEM_HEADER_ONLY_INLINE void smem::SMem::create(const std::string& name, const size_t size) {
  _handle = ::CreateFileMapping(INVALID_HANDLE_VALUE, nullptr, PAGE_READWRITE, 0, static_cast<DWORD>(size), name.c_str());
  if (_handle == nullptr) {
    const auto err = GetLastError();
    LPVOID msg{};
    const auto n = FormatMessage(FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS, nullptr, err, 0,
                                 reinterpret_cast<LPTSTR>(&msg), 0, nullptr);
    TCHAR buffer[512];
    std::char_traits<TCHAR>::copy(buffer, static_cast<TCHAR*>(msg), n);
    throw std::runtime_error(buffer);
  }
  _size = size;
}

SMEM_HEADER_ONLY_INLINE void* smem::SMem::map() {
  _ptr = ::MapViewOfFile(_handle, FILE_MAP_WRITE | FILE_MAP_READ, 0, 0, static_cast<DWORD>(_size));
  return _ptr;
}

SMEM_HEADER_ONLY_INLINE void smem::SMem::unmap() {
  if (_ptr == nullptr) return;
  ::UnmapViewOfFile(_ptr);
  _ptr = nullptr;
}

SMEM_HEADER_ONLY_INLINE void smem::SMem::close() {
  if (_handle == nullptr) return;
  ::CloseHandle(_handle);
  _handle = nullptr;
  _size = 0;
}

SMEM_HEADER_ONLY_INLINE smem::SMem::~SMem() {}

#else

#include <sys/ipc.h>
#include <sys/shm.h>
#include <sys/stat.h>
#include <sys/types.h>

SMEM_HEADER_ONLY_INLINE void smem::SMem::create(const std::string& name, const size_t size) {
  const char* home_path = getenv("HOME");
  std::stringstream ss;
  ss << home_path << "/" << name;
  _key_path = ss.str();
  FILE* fp;
  fp = ::fopen(_key_path.c_str(), "w");
  ::fclose(fp);
  const int id = 97;
  const key_t key = ::ftok(_key_path.c_str(), id);
  if (key == -1) throw std::runtime_error(std::to_string(errno) + ": Failed to get key");

  _seg_id = shmget(key, size, IPC_CREAT | IPC_EXCL | S_IRUSR | S_IWUSR);
  if (_seg_id != -1) return;

  if (errno == 17) {
    _seg_id = shmget(key, 0, 0);
    if (_seg_id == -1) throw std::runtime_error(std::to_string(errno) + ": Failed to get shared memory");
  } else {
    throw std::runtime_error(std::to_string(errno) + ": Failed to create shared memory");
  }
}

SMEM_HEADER_ONLY_INLINE void* smem::SMem::map() {
  _ptr = shmat(_seg_id, 0, 0);
  return _ptr;
}

SMEM_HEADER_ONLY_INLINE void smem::SMem::unmap() {
  if (_ptr == nullptr) return;
  shmdt(_ptr);
  _ptr = nullptr;
}

SMEM_HEADER_ONLY_INLINE void smem::SMem::close() {
  if (_seg_id == -1) return;
  shmctl(_seg_id, IPC_RMID, nullptr);
  _seg_id = -1;
  ::remove(_key_path.c_str());
}

SMEM_HEADER_ONLY_INLINE smem::SMem::~SMem() {}

#endif
