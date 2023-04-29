// This file was automatically generated from header file

#ifdef C2NIM
#  dynlib dll
#  cdecl
#  if defined(windows)
#    define dll "bin/autd3capi-link-twincat.dll"
#  elif defined(macosx)
#    define dll "bin/libautd3capi-link-twincat.dylib"
#  else
#    define dll "bin/libautd3capi-link-twincat.so"
#  endif
#endif

void AUTDLinkTwinCAT(void** out);
void AUTDLinkTwinCATLogLevel(void* twincat, int32 level);
void AUTDLinkTwinCATLogFunc(void* twincat, void* out_func, void* flush_func);
void AUTDLinkTwinCATTimeout(void* twincat, uint64 timeout_ns);
void AUTDLinkTwinCATBuild(void** out, void* twincat);
