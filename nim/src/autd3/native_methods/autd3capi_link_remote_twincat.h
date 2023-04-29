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

void AUTDLinkRemoteTwinCAT(void** out, char* server_ams_net_id);
void AUTDLinkRemoteTwinCATServerIpAddr(void* remote_twincat, char* server_ip_addr);
void AUTDLinkRemoteTwinCATClientAmsNetId(void* remote_twincat, char* client_ams_net_id);
void AUTDLinkRemoteTwinCATLogLevel(void* remote_twincat, int32 level);
void AUTDLinkRemoteTwinCATLogFunc(void* remote_twincat, void* out_func, void* flush_func);
void AUTDLinkRemoteTwinCATTimeout(void* remote_twincat, uint64 timeout_ns);
void AUTDLinkRemoteTwinCATBuild(void** out, void* remote_twincat);
