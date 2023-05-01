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

void AUTDLinkRemoteSimulator(void** out, char* ip, uint16_t port);
void AUTDLinkRemoteSimulatorLogLevel(void* remote_simulator, int32_t level);
void AUTDLinkRemoteSimulatorLogFunc(void* remote_simulator, void* out_func, void* flush_func);
void AUTDLinkRemoteSimulatorTimeout(void* remote_simulator, uint64_t timeout_ns);
void AUTDLinkRemoteSimulatorBuild(void** out, void* remote_simulator);
