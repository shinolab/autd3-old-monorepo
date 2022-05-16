// File: c_api.cpp
// Project: link_soem
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#include "../base/wrapper_link.hpp"
#include "./soem_link.h"
#include "autd3/link/soem.hpp"

typedef struct {
  std::vector<autd3::link::EtherCATAdapter> adapters;
} EtherCATAdaptersWrapper;

inline EtherCATAdaptersWrapper* ether_cat_adapters_create(const std::vector<autd3::link::EtherCATAdapter>& adapters) {
  return new EtherCATAdaptersWrapper{adapters};
}
inline void ether_cat_adapters_delete(const EtherCATAdaptersWrapper* ptr) { delete ptr; }

int32_t AUTDGetAdapterPointer(void** out) {
  const auto adapters = autd3::link::SOEM::enumerate_adapters();
  *out = ether_cat_adapters_create(adapters);
  return static_cast<int32_t>(adapters.size());
}
void AUTDGetAdapter(void* p_adapter, const int32_t index, char* desc, char* name) {
  const auto* wrapper = static_cast<EtherCATAdaptersWrapper*>(p_adapter);
  const auto& desc_ = wrapper->adapters[index].desc;
  const auto& name_ = wrapper->adapters[index].name;
  std::char_traits<char>::copy(desc, desc_.c_str(), desc_.size() + 1);
  std::char_traits<char>::copy(name, name_.c_str(), name_.size() + 1);
}
void AUTDFreeAdapterPointer(void* p_adapter) {
  const auto* wrapper = static_cast<EtherCATAdaptersWrapper*>(p_adapter);
  ether_cat_adapters_delete(wrapper);
}

void AUTDLinkSOEM(void** out, const char* ifname, const int32_t device_num, const uint16_t cycle_ticks, void* on_lost, const bool high_precision) {
  auto soem_link = autd3::link::SOEM(std::string(ifname), static_cast<size_t>(device_num))
                       .cycle_ticks(cycle_ticks)
                       .high_precision(high_precision)
                       .on_lost([on_lost](const std::string& msg) { reinterpret_cast<OnLostCallback>(on_lost)(msg.c_str()); })
                       .build();
  auto* link = link_create(std::move(soem_link));
  *out = link;
}
