// This file was automatically generated from header file

typedef char int8_t;
typedef unsigned char uint8_t;
typedef short int16_t;
typedef unsigned short uint16_t;
typedef int int32_t;
typedef unsigned int uint32_t;
#ifdef WIN32
typedef long long int64_t;
typedef unsigned long long uint64_t;
#else
typedef long int64_t;
typedef unsigned long uint64_t;
#endif

void AUTDLinkRemoteTwinCAT(void** out, char* server_ams_net_id);
void AUTDLinkRemoteTwinCATServerIpAddr(void* remote_twincat, char* server_ip_addr);
void AUTDLinkRemoteTwinCATClientAmsNetId(void* remote_twincat, char* client_ams_net_id);
void AUTDLinkRemoteTwinCATLogLevel(void* remote_twincat, int32_t level);
void AUTDLinkRemoteTwinCATLogFunc(void* remote_twincat, void* out_func, void* flush_func);
void AUTDLinkRemoteTwinCATTimeout(void* remote_twincat, uint64_t timeout_ns);
void AUTDLinkRemoteTwinCATBuild(void** out, void* remote_twincat);
