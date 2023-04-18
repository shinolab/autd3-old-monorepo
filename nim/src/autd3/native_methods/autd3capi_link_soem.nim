##  This file was automatically generated from header file

{.deadCodeElim: on.}
when defined(windows):
  const
    dll* = "bin/autd3capi-link-soem.dll"
elif defined(macosx):
  const
    dll* = "bin/libautd3capi-link-soem.dylib"
else:
  const
    dll* = "bin/libautd3capi-link-soem.so"
proc AUTDGetAdapterPointer*(`out`: ptr pointer): int32 {.cdecl,
    importc: "AUTDGetAdapterPointer", dynlib: dll.}
proc AUTDGetAdapter*(p_adapter: pointer; index: int32; desc: cstring; name: cstring) {.
    cdecl, importc: "AUTDGetAdapter", dynlib: dll.}
proc AUTDFreeAdapterPointer*(p_adapter: pointer) {.cdecl,
    importc: "AUTDFreeAdapterPointer", dynlib: dll.}
proc AUTDLinkSOEM*(`out`: ptr pointer) {.cdecl, importc: "AUTDLinkSOEM", dynlib: dll.}
proc AUTDLinkSOEMIfname*(soem: pointer; ifname: cstring) {.cdecl,
    importc: "AUTDLinkSOEMIfname", dynlib: dll.}
proc AUTDLinkSOEMBufSize*(soem: pointer; buf_size: uint64) {.cdecl,
    importc: "AUTDLinkSOEMBufSize", dynlib: dll.}
proc AUTDLinkSOEMSync0Cycle*(soem: pointer; sync0_cycle: uint16) {.cdecl,
    importc: "AUTDLinkSOEMSync0Cycle", dynlib: dll.}
proc AUTDLinkSOEMSendCycle*(soem: pointer; send_cycle: uint16) {.cdecl,
    importc: "AUTDLinkSOEMSendCycle", dynlib: dll.}
proc AUTDLinkSOEMFreerun*(soem: pointer; freerun: bool) {.cdecl,
    importc: "AUTDLinkSOEMFreerun", dynlib: dll.}
proc AUTDLinkSOEMOnLost*(soem: pointer; on_lost: pointer) {.cdecl,
    importc: "AUTDLinkSOEMOnLost", dynlib: dll.}
proc AUTDLinkSOEMTimerStrategy*(soem: pointer; timer_strategy: uint8) {.cdecl,
    importc: "AUTDLinkSOEMTimerStrategy", dynlib: dll.}
proc AUTDLinkSOEMStateCheckInterval*(soem: pointer; state_check_interval: uint64) {.
    cdecl, importc: "AUTDLinkSOEMStateCheckInterval", dynlib: dll.}
proc AUTDLinkSOEMLogLevel*(soem: pointer; level: int32) {.cdecl,
    importc: "AUTDLinkSOEMLogLevel", dynlib: dll.}
proc AUTDLinkSOEMLogFunc*(soem: pointer; out_func: pointer; flush_func: pointer) {.
    cdecl, importc: "AUTDLinkSOEMLogFunc", dynlib: dll.}
proc AUTDLinkSOEMTimeout*(soem: pointer; timeout_ns: uint64) {.cdecl,
    importc: "AUTDLinkSOEMTimeout", dynlib: dll.}
proc AUTDLinkSOEMBuild*(`out`: ptr pointer; soem: pointer) {.cdecl,
    importc: "AUTDLinkSOEMBuild", dynlib: dll.}
proc AUTDLinkSOEMDelete*(soem: pointer) {.cdecl, importc: "AUTDLinkSOEMDelete",
                                       dynlib: dll.}