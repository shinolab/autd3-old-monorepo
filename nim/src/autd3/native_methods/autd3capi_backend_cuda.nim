##  This file was automatically generated from header file

{.deadCodeElim: on.}
when defined(windows):
  const
    dll* = "bin/autd3capi-backend-cuda.dll"
elif defined(macosx):
  const
    dll* = "bin/libautd3capi-backend-cuda.dylib"
else:
  const
    dll* = "bin/libautd3capi-backend-cuda.so"
proc AUTDCUDABackend*(`out`: ptr pointer) {.cdecl, importc: "AUTDCUDABackend",
                                        dynlib: dll.}