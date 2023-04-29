// This file was automatically generated from header file

#ifdef C2NIM
#  dynlib dll
#  cdecl
#  if defined(windows)
#    define dll "bin/autd3capi-link-soem.dll"
#  elif defined(macosx)
#    define dll "bin/libautd3capi-link-soem.dylib"
#  else
#    define dll "bin/libautd3capi-link-soem.so"
#  endif
#endif

int32 AUTDGetAdapterPointer(void** out);
void AUTDGetAdapter(void* p_adapter, int32 index, char* desc, char* name);
void AUTDFreeAdapterPointer(void* p_adapter);
void AUTDLinkSOEM(void** out);
void AUTDLinkSOEMIfname(void* soem, char* ifname);
void AUTDLinkSOEMBufSize(void* soem, uint64 buf_size);
void AUTDLinkSOEMSync0Cycle(void* soem, uint16 sync0_cycle);
void AUTDLinkSOEMSendCycle(void* soem, uint16 send_cycle);
void AUTDLinkSOEMFreerun(void* soem, bool freerun);
void AUTDLinkSOEMOnLost(void* soem, void* on_lost);
void AUTDLinkSOEMTimerStrategy(void* soem, uint8 timer_strategy);
void AUTDLinkSOEMStateCheckInterval(void* soem, uint64 state_check_interval);
void AUTDLinkSOEMLogLevel(void* soem, int32 level);
void AUTDLinkSOEMLogFunc(void* soem, void* out_func, void* flush_func);
void AUTDLinkSOEMTimeout(void* soem, uint64 timeout_ns);
void AUTDLinkSOEMBuild(void** out, void* soem);
