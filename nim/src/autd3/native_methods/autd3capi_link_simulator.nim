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
proc AUTDLinkSimulator*(`out`: ptr pointer) {.cdecl, importc: "AUTDLinkSimulator",
    dynlib: dll.}
proc AUTDLinkSimulatorLogLevel*(simulator: pointer; level: int32) {.cdecl,
    importc: "AUTDLinkSimulatorLogLevel", dynlib: dll.}
proc AUTDLinkSimulatorLogFunc*(simulator: pointer; out_func: pointer;
                              flush_func: pointer) {.cdecl,
    importc: "AUTDLinkSimulatorLogFunc", dynlib: dll.}
proc AUTDLinkSimulatorTimeout*(simulator: pointer; timeout_ns: uint64) {.cdecl,
    importc: "AUTDLinkSimulatorTimeout", dynlib: dll.}
proc AUTDLinkSimulatorBuild*(`out`: ptr pointer; simulator: pointer) {.cdecl,
    importc: "AUTDLinkSimulatorBuild", dynlib: dll.}