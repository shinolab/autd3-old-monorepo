// This file was automatically generated from header file

#ifdef C2NIM
#  dynlib dll
#  cdecl
#  if defined(windows)
#    define dll "bin/autd3capi-link-remote-twincat.dll"
#  elif defined(macosx)
#    define dll "bin/libautd3capi-link-remote-twincat.dylib"
#  else
#    define dll "bin/libautd3capi-link-remote-twincat.so"
#  endif
#endif

void AUTDLinkRemoteTwinCAT(void** out, char* server_ip_addr, char* server_ams_net_id, char* client_ams_net_id);
