##  This file was automatically generated from header file

{.deadCodeElim: on.}
when defined(windows):
  const
    dll* = "bin/autd3capi-link-twincat.dll"
elif defined(macosx):
  const
    dll* = "bin/libautd3capi-link-twincat.dylib"
else:
  const
    dll* = "bin/libautd3capi-link-twincat.so"
proc AUTDLinkTwinCAT*(`out`: ptr pointer) {.cdecl, importc: "AUTDLinkTwinCAT",
                                        dynlib: dll.}
proc AUTDLinkTwinCATLogLevel*(twincat: pointer; level: int32) {.cdecl,
    importc: "AUTDLinkTwinCATLogLevel", dynlib: dll.}
proc AUTDLinkTwinCATLogFunc*(twincat: pointer; out_func: pointer; flush_func: pointer) {.
    cdecl, importc: "AUTDLinkTwinCATLogFunc", dynlib: dll.}
proc AUTDLinkTwinCATTimeout*(twincat: pointer; timeout_ns: uint64) {.cdecl,
    importc: "AUTDLinkTwinCATTimeout", dynlib: dll.}
proc AUTDLinkTwinCATBuild*(`out`: ptr pointer; twincat: pointer) {.cdecl,
    importc: "AUTDLinkTwinCATBuild", dynlib: dll.}