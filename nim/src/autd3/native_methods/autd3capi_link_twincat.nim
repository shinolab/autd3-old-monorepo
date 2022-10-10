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