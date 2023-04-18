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
void AUTDLinkSOEM(void** out);
void AUTDLinkSOEMIfname(void* soem, char* ifname);
void AUTDLinkSOEMBufSize(void* soem, uint64_t buf_size);
void AUTDLinkSOEMSync0Cycle(void* soem, uint16_t sync0_cycle);
void AUTDLinkSOEMSendCycle(void* soem, uint16_t send_cycle);
void AUTDLinkSOEMFreerun(void* soem, bool freerun);
void AUTDLinkSOEMOnLost(void* soem, void* on_lost);
void AUTDLinkSOEMTimerStrategy(void* soem, uint8_t timer_strategy);
void AUTDLinkSOEMStateCheckInterval(void* soem, uint64_t state_check_interval);
void AUTDLinkSOEMLogLevel(void* soem, int32_t level);
void AUTDLinkSOEMLogFunc(void* soem, void* out_func, void* flush_func);
void AUTDLinkSOEMTimeout(void* soem, uint64_t timeout_ns);
void AUTDLinkSOEMBuild(void** out, void* soem);
void AUTDLinkSOEMDelete(void* soem);
