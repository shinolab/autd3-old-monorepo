##  This file was automatically generated from header file

{.deadCodeElim: on.}
when defined(windows):
  const
    dll* = "bin/autd3capi-link-simulator.dll"
elif defined(macosx):
  const
    dll* = "bin/libautd3capi-link-simulator.dylib"
else:
  const
    dll* = "bin/libautd3capi-link-simulator.so"
proc AUTDLinkSimulator*(`out`: ptr pointer; port: uint16; ip_addr: cstring) {.cdecl,
    importc: "AUTDLinkSimulator", dynlib: dll.}