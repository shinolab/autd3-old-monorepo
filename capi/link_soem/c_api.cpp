// File: c_api.cpp
// Project: link_soem
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 06/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "../base/wrapper_link.hpp"
#include "./soem_link.h"
#include "autd3/link/soem.hpp"
#include "custom_sink.hpp"
#include "spdlog/spdlog.h"

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

void AUTDLinkSOEM(void** out, const char* ifname, const uint16_t sync0_cycle, const uint16_t send_cycle, const bool freerun, void* on_lost,
                  const bool high_precision, const uint64_t state_check_interval) {
  auto& soem = autd3::link::SOEM()
                   .sync0_cycle(sync0_cycle)
                   .send_cycle(send_cycle)
                   .high_precision(high_precision)
                   .sync_mode(freerun ? autd3::link::SYNC_MODE::FREE_RUN : autd3::link::SYNC_MODE::DC)
                   .state_check_interval(std::chrono::milliseconds(state_check_interval));
  if (ifname != nullptr) soem.ifname(std::string(ifname));
  if (on_lost != nullptr) soem.on_lost([on_lost](const std::string& msg) { reinterpret_cast<OnLostCallback>(on_lost)(msg.c_str()); });

  auto soem_link = soem.build();
  auto* link = link_create(std::move(soem_link));
  *out = link;
}

void AUTDLinkSOEMSetLogLevel(const int32_t level) { spdlog::set_level(static_cast<spdlog::level::level_enum>(level)); }

void AUTDLinkSOEMSetDefaultLogger(void* out, void* flush) {
  auto custom_sink = std::make_shared<autd3::capi::custom_sink_mt>(out, flush);
  const auto logger = std::make_shared<spdlog::logger>("AUTD3 Logger", custom_sink);
  set_default_logger(logger);
}
