// This file was automatically generated from header file

#ifdef C2NIM
#  dynlib dll
#  cdecl
#  if defined(windows)
#    define dll "bin/autd3capi-link-bundle.dll"
#  elif defined(macosx)
#    define dll "bin/libautd3capi-link-bundle.dylib"
#  else
#    define dll "bin/libautd3capi-link-bundle.so"
#  endif
#endif

void AUTDLinkBundle(void** out, void** links, int32 n);
