##  This file was automatically generated from header file

{.deadCodeElim: on.}
when defined(windows):
  const
    dll* = "bin/autd3capi-link-debug.dll"
elif defined(macosx):
  const
    dll* = "bin/libautd3capi-link-debug.dylib"
else:
  const
    dll* = "bin/libautd3capi-link-debug.so"
proc AUTDLinkDebug*(`out`: ptr pointer) {.cdecl, importc: "AUTDLinkDebug", dynlib: dll.}
proc AUTDLinkDebugSetLevel*(level: int32) {.cdecl, importc: "AUTDLinkDebugSetLevel",
    dynlib: dll.}