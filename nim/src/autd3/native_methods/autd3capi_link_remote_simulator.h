// This file was automatically generated from header file

#ifdef C2NIM
#  dynlib dll
#  cdecl
#  if defined(windows)
#    define dll "bin/autd3capi-link-remote-simulator.dll"
#  elif defined(macosx)
#    define dll "bin/libautd3capi-link-remote-simulator.dylib"
#  else
#    define dll "bin/libautd3capi-link-remote-simulator.so"
#  endif
#endif

void AUTDLinkRemoteSimulator(void** out, char* ip, uint16 port);
void AUTDLinkRemoteSimulatorLogLevel(void* remote_simulator, int32 level);
void AUTDLinkRemoteSimulatorLogFunc(void* remote_simulator, void* out_func, void* flush_func);
void AUTDLinkRemoteSimulatorTimeout(void* remote_simulator, uint64 timeout_ns);
void AUTDLinkRemoteSimulatorBuild(void** out, void* remote_simulator);
