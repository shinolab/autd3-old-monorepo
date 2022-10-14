// This file was automatically generated from header file

#ifdef C2NIM
#  dynlib dll
#  cdecl
#  if defined(windows)
#    define dll "bin/autd3capi-link-debug.dll"
#  elif defined(macosx)
#    define dll "bin/libautd3capi-link-debug.dylib"
#  else
#    define dll "bin/libautd3capi-link-debug.so"
#  endif
#endif

void AUTDLinkDebug(void** out);
void AUTDLinkDebugSetLevel(int32 level);
