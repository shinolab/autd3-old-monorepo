// File: boost_wrap.cpp
// Project: autd3-link-simulator
// Created Date: 25/04/2023
// Author: Shun Suzuki
// -----
// Last Modified: 25/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include "boost_wrap.h"

#include <boost/interprocess/managed_shared_memory.hpp>
#include <boost/interprocess/sync/interprocess_mutex.hpp>
#include <mutex>
#include <string>
#include <string_view>
#include <thread>

static constexpr std::string_view SHMEM_NAME{"autd3_simulator_shmem"};
static constexpr std::string_view SHMEM_MTX_NAME{"autd3_simulator_shmem_mtx"};
static constexpr std::string_view SHMEM_DATA_NAME{"autd3_simulator_shmem_ptr"};

boost::interprocess::managed_shared_memory boost_wrap_segment;
boost::interprocess::interprocess_mutex* boost_wrap_mtx{nullptr};
uint8_t* boost_wrap_ptr{nullptr};

bool shmem_create() {
  try {
    boost_wrap_segment = boost::interprocess::managed_shared_memory(boost::interprocess::open_only, std::string(SHMEM_NAME).c_str());
    boost_wrap_ptr = boost_wrap_segment.find<uint8_t>(std::string(SHMEM_DATA_NAME).c_str()).first;
    boost_wrap_mtx = boost_wrap_segment.find<boost::interprocess::interprocess_mutex>(std::string(SHMEM_MTX_NAME).c_str()).first;
    return true;
  } catch (std::exception&) {
    return false;
  }
}

bool shmem_copy_to(const uint8_t* data, const size_t size) {
  try {
    std::unique_lock lk(*boost_wrap_mtx);
    std::memcpy(boost_wrap_ptr, data, size);
    return true;
  } catch (std::exception&) {
    return false;
  }
}

bool shmem_copy_from(uint8_t* data, const size_t offset, const size_t size) {
  try {
    std::unique_lock lk(*boost_wrap_mtx);
    std::memcpy(data, boost_wrap_ptr + offset, size);
    return true;
  } catch (std::exception&) {
    return false;
  }
}
