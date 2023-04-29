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

void AUTDLinkRemoteSOEM(void** out, char* ip, uint16_t port);
void AUTDLinkRemoteSOEMServerIpAddr(void* remote_soem, char* server_ip_addr);
void AUTDLinkRemoteSOEMClientAmsNetId(void* remote_soem, char* client_ams_net_id);
void AUTDLinkRemoteSOEMLogLevel(void* remote_soem, int32_t level);
void AUTDLinkRemoteSOEMLogFunc(void* remote_soem, void* out_func, void* flush_func);
void AUTDLinkRemoteSOEMTimeout(void* remote_soem, uint64_t timeout_ns);
void AUTDLinkRemoteSOEMBuild(void** out, void* remote_soem);
