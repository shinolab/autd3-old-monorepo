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

void AUTDLinkSimulator(void** out, uint16 port, char* ip_addr);
