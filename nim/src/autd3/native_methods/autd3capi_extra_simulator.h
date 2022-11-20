// This file was automatically generated from header file

#ifdef C2NIM
#  dynlib dll
#  cdecl
#  if defined(windows)
#    define dll "bin/autd3capi-extra-simulator.dll"
#  elif defined(macosx)
#    define dll "bin/libautd3capi-extra-simulator.dylib"
#  else
#    define dll "bin/libautd3capi-extra-simulator.so"
#  endif
#endif

bool AUTDExtraSimulator(char* settings_path, bool vsync, int32 gpu_idx);
