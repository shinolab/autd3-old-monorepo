##  This file was automatically generated from header file

{.deadCodeElim: on.}
when defined(windows):
  const
    dll* = "bin/autd3capi-link-remote-soem.dll"
elif defined(macosx):
  const
    dll* = "bin/libautd3capi-link-remote-soem.dylib"
else:
  const
    dll* = "bin/libautd3capi-link-remote-soem.so"
proc AUTDLinkRemoteSOEM*(`out`: ptr pointer; ip: cstring; port: uint16;
                        timeout_ns: uint64) {.cdecl, importc: "AUTDLinkRemoteSOEM",
    dynlib: dll.}