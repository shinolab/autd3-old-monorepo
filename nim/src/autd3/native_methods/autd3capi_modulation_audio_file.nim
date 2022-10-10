##  This file was automatically generated from header file

{.deadCodeElim: on.}
when defined(windows):
  const
    dll* = "bin/autd3capi-modulation-audio-file.dll"
elif defined(macosx):
  const
    dll* = "bin/libautd3capi-modulation-audio-file.dylib"
else:
  const
    dll* = "bin/libautd3capi-modulation-audio-file.so"
proc AUTDModulationRawPCM*(`mod`: ptr pointer; filename: cstring;
                          sampling_freq: float64; mod_sampling_freq_div: uint32) {.
    cdecl, importc: "AUTDModulationRawPCM", dynlib: dll.}
proc AUTDModulationWav*(`mod`: ptr pointer; filename: cstring;
                       mod_sampling_freq_div: uint32) {.cdecl,
    importc: "AUTDModulationWav", dynlib: dll.}