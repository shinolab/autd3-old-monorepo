#pragma once

#include <AdsDef.h>

#ifdef __cplusplus
extern "C" {
#endif
long AdsCPortOpenEx();
long AdsCPortCloseEx(long port);
long AdsCSyncReadReqEx2(long port, const AmsAddr* pAddr, uint32_t indexGroup, uint32_t indexOffset, uint32_t bufferLength, void* buffer,
                        uint32_t* bytesRead);

long AdsCSyncWriteReqEx(long port, const AmsAddr* pAddr, uint32_t indexGroup, uint32_t indexOffset, uint32_t bufferLength, const void* buffer);
void AdsCSetLocalAddress(AmsNetId ams);
long AdsCAddRoute(AmsNetId ams, const char* ip);
#ifdef __cplusplus
}
#endif
