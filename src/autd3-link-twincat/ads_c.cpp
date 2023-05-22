// File: AdsLib_c copy.h
// Project: autd3-link-twincat
// Created Date: 22/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 22/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include "ads_c.h"

#include <AdsLib.h>

long AdsCPortOpenEx() { return AdsPortOpenEx(); }

long AdsCPortCloseEx(long port) { return AdsPortCloseEx(port); }

long AdsCSyncReadReqEx2(long port, const AmsAddr* pAddr, uint32_t indexGroup, uint32_t indexOffset, uint32_t bufferLength, void* buffer,
                        uint32_t* bytesRead) {
  return AdsSyncReadReqEx2(port, pAddr, indexGroup, indexOffset, bufferLength, buffer, bytesRead);
}

long AdsCSyncWriteReqEx(long port, const AmsAddr* pAddr, uint32_t indexGroup, uint32_t indexOffset, uint32_t bufferLength, const void* buffer) {
  return AdsSyncWriteReqEx(port, pAddr, indexGroup, indexOffset, bufferLength, buffer);
}

void AdsCSetLocalAddress(AmsNetId ams) { AdsSetLocalAddress(ams); }

long AdsCAddRoute(AmsNetId ams, const char* ip) { return AdsAddRoute(ams, ip); }
