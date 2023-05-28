#pragma once

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <cstdint>

namespace autd3::internal::native_methods {

extern "C" {

[[nodiscard]] void* AUTDGeometryViewer();

[[nodiscard]] void* AUTDGeometryViewerSize(void* viewer, uint32_t width, uint32_t height);

[[nodiscard]] void* AUTDGeometryViewerVsync(void* viewer, bool vsync);

[[nodiscard]] int32_t AUTDGeometryViewerRun(void* viewer, void* cnt);

} // extern "C"

} // namespace autd3::internal::native_methods
