##  This file was automatically generated from header file

{.deadCodeElim: on.}
when defined(windows):
  const
    dll* = "bin/autd3capi-extra-simulator.dll"
elif defined(macosx):
  const
    dll* = "bin/libautd3capi-extra-simulator.dylib"
else:
  const
    dll* = "bin/libautd3capi-extra-simulator.so"
proc AUTDExtraSimulator*(settings_path: cstring; vsync: bool; gpu_idx: int32) {.cdecl,
    importc: "AUTDExtraSimulator", dynlib: dll.}