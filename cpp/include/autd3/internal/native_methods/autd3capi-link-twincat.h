#pragma once

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <cstdint>

namespace autd3::internal::native_methods {

struct LinkTwinCATBuilderPtr {
  void* _0;
};

struct LinkRemoteTwinCATBuilderPtr {
  void* _0;
};

extern "C" {

[[nodiscard]] LinkTwinCATBuilderPtr AUTDLinkTwinCAT();

[[nodiscard]]
LinkTwinCATBuilderPtr AUTDLinkTwinCATWithTimeout(LinkTwinCATBuilderPtr twincat,
                                                 uint64_t timeout_ns);

[[nodiscard]] LinkBuilderPtr AUTDLinkTwinCATIntoBuilder(LinkTwinCATBuilderPtr twincat);

[[nodiscard]]
LinkRemoteTwinCATBuilderPtr AUTDLinkRemoteTwinCAT(const char *server_ams_net_id,
                                                  char *err);

[[nodiscard]]
LinkRemoteTwinCATBuilderPtr AUTDLinkRemoteTwinCATWithServerIP(LinkRemoteTwinCATBuilderPtr twincat,
                                                              const char *addr);

[[nodiscard]]
LinkRemoteTwinCATBuilderPtr AUTDLinkRemoteTwinCATWithClientAmsNetId(LinkRemoteTwinCATBuilderPtr twincat,
                                                                    const char *id);

[[nodiscard]]
LinkRemoteTwinCATBuilderPtr AUTDLinkRemoteTwinCATWithTimeout(LinkRemoteTwinCATBuilderPtr twincat,
                                                             uint64_t timeout_ns);

[[nodiscard]] LinkBuilderPtr AUTDLinkRemoteTwinCATIntoBuilder(LinkRemoteTwinCATBuilderPtr twincat);

} // extern "C"

} // namespace autd3::internal::native_methods
