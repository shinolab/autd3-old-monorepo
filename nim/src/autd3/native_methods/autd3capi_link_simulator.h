// This file was automatically generated from header file

#ifdef C2NIM
#  dynlib dll
#  cdecl
#  if defined(windows)
#    define dll "bin/autd3capi-link-simulator.dll"
#  elif defined(macosx)
#    define dll "bin/libautd3capi-link-simulator.dylib"
#  else
#    define dll "bin/libautd3capi-link-simulator.so"
#  endif
#endif

void AUTDLinkSimulator(void** out);
void AUTDLinkSimulatorLogLevel(void* simulator, int32 level);
void AUTDLinkSimulatorLogFunc(void* simulator, void* out_func, void* flush_func);
void AUTDLinkSimulatorTimeout(void* simulator, uint64 timeout_ns);
void AUTDLinkSimulatorBuild(void** out, void* simulator);
