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
proc AUTDLinkSOEM*(`out`: ptr pointer; ifname: cstring; buf_size: uint64;
                  sync0_cycle: uint16; send_cycle: uint16; freerun: bool;
                  on_lost: pointer; timer_strategy: uint8;
                  state_check_interval: uint64; level: int32; out_func: pointer;
                  flush_func: pointer) {.cdecl, importc: "AUTDLinkSOEM", dynlib: dll.}