// This file was automatically generated from header file

#ifdef C2NIM
#  dynlib dll
#  cdecl
#  if defined(windows)
#    define dll "bin/autd3capi-extra-geometry-viewer.dll"
#  elif defined(macosx)
#    define dll "bin/libautd3capi-extra-geometry-viewer.dylib"
#  else
#    define dll "bin/libautd3capi-extra-geometry-viewer.so"
#  endif
#endif

bool AUTDExtraGeometryViewer(void* geometry, int32 width, int32 height, bool vsync, int32 gpu_idx);
