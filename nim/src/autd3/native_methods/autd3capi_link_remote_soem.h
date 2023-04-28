// This file was automatically generated from header file

#ifdef C2NIM
#  dynlib dll
#  cdecl
#  if defined(windows)
#    define dll "bin/autd3capi-link-remote-soem.dll"
#  elif defined(macosx)
#    define dll "bin/libautd3capi-link-remote-soem.dylib"
#  else
#    define dll "bin/libautd3capi-link-remote-soem.so"
#  endif
#endif

void AUTDLinkRemoteSOEM(void** out, char* ip, uint16 port);
void AUTDLinkRemoteSOEMServerIpAddr(void* remote_soem, char* server_ip_addr);
void AUTDLinkRemoteSOEMClientAmsNetId(void* remote_soem, char* client_ams_net_id);
void AUTDLinkRemoteSOEMLogLevel(void* remote_soem, int32 level);
void AUTDLinkRemoteSOEMLogFunc(void* remote_soem, void* out_func, void* flush_func);
void AUTDLinkRemoteSOEMTimeout(void* remote_soem, uint64 timeout_ns);
void AUTDLinkRemoteSOEMBuild(void** out, void* remote_soem);
