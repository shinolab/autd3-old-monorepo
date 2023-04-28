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

void AUTDLinkSimulator(void** out);
void AUTDLinkSimulatorLogLevel(void* simulator, int32_t level);
void AUTDLinkSimulatorLogFunc(void* simulator, void* out_func, void* flush_func);
void AUTDLinkSimulatorTimeout(void* simulator, uint64_t timeout_ns);
void AUTDLinkSimulatorBuild(void** out, void* simulator);
