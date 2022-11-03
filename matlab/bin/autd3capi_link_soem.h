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

int32_t AUTDGetAdapterPointer(void** out);
void AUTDGetAdapter(void* p_adapter, int32_t index, int8_t* desc, int8_t* name);
void AUTDFreeAdapterPointer(void* p_adapter);
void AUTDLinkSOEM(void** out, char* ifname, uint16_t sync0_cycle, uint16_t send_cycle, bool freerun, void* on_lost, bool high_precision, uint64_t state_check_interval);
void AUTDLinkSOEMSetLogLevel(int32_t level);
void AUTDLinkSOEMSetDefaultLogger(void* out, void* flush);
