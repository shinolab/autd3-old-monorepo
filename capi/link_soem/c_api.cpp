// File: c_api.cpp
// Project: link_soem
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "../../src/spdlog.hpp"
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

typedef void (*OutCallback)(const char*);
typedef void (*FlushCallback)();

void AUTDLinkSOEM(void** out) { *out = new autd3::link::SOEM; }
void AUTDLinkSOEMIfname(void* soem, const char* ifname) {
  if (ifname != nullptr) static_cast<autd3::link::SOEM*>(soem)->ifname(std::string(ifname));
}
void AUTDLinkSOEMBufSize(void* soem, const uint64_t buf_size) { static_cast<autd3::link::SOEM*>(soem)->buf_size(buf_size); }
void AUTDLinkSOEMSync0Cycle(void* soem, const uint16_t sync0_cycle) { static_cast<autd3::link::SOEM*>(soem)->sync0_cycle(sync0_cycle); }
void AUTDLinkSOEMSendCycle(void* soem, const uint16_t send_cycle) { static_cast<autd3::link::SOEM*>(soem)->send_cycle(send_cycle); }
void AUTDLinkSOEMFreerun(void* soem, const bool freerun) {
  static_cast<autd3::link::SOEM*>(soem)->sync_mode(freerun ? autd3::link::SyncMode::FreeRun : autd3::link::SyncMode::DC);
}
void AUTDLinkSOEMOnLost(void* soem, void* on_lost) {
  if (on_lost != nullptr)
    static_cast<autd3::link::SOEM*>(soem)->on_lost([on_lost](const std::string& msg) { reinterpret_cast<OnLostCallback>(on_lost)(msg.c_str()); });
}
void AUTDLinkSOEMTimerStrategy(void* soem, const uint8_t timer_strategy) {
  static_cast<autd3::link::SOEM*>(soem)->timer_strategy(static_cast<autd3::TimerStrategy>(timer_strategy));
}
void AUTDLinkSOEMStateCheckInterval(void* soem, const uint64_t state_check_interval) {
  static_cast<autd3::link::SOEM*>(soem)->state_check_interval(std::chrono::milliseconds(state_check_interval));
}
void AUTDLinkSOEMLogLevel(void* soem, const int32_t level) {
  static_cast<autd3::link::SOEM*>(soem)->debug_level(static_cast<autd3::driver::DebugLevel>(level));
}
void AUTDLinkSOEMLogFunc(void* soem, void* out_func, void* flush_func) {
  if (out_func != nullptr && flush_func != nullptr)
    static_cast<autd3::link::SOEM*>(soem)->debug_log_func(
        [out_func](const std::string& msg) { reinterpret_cast<OutCallback>(out_func)(msg.c_str()); },
        [flush_func] { reinterpret_cast<FlushCallback>(flush_func)(); });
}
void AUTDLinkSOEMTimeout(void* soem, const uint64_t timeout_ns) {
  static_cast<autd3::link::SOEM*>(soem)->timeout(std::chrono::nanoseconds(timeout_ns));
}
void AUTDLinkSOEMBuild(void** out, void* soem) {
  auto* link = link_create(static_cast<autd3::link::SOEM*>(soem)->build());
  *out = link;
}
void AUTDLinkSOEMDelete(void* soem) { delete static_cast<autd3::link::SOEM*>(soem); }
