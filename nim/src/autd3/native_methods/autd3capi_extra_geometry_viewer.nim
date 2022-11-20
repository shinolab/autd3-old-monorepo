##  This file was automatically generated from header file

{.deadCodeElim: on.}
when defined(windows):
  const
    dll* = "bin/autd3capi-extra-geometry-viewer.dll"
elif defined(macosx):
  const
    dll* = "bin/libautd3capi-extra-geometry-viewer.dylib"
else:
  const
    dll* = "bin/libautd3capi-extra-geometry-viewer.so"
proc AUTDExtraGeometryViewer*(cnt: pointer; width: int32; height: int32; vsync: bool;
                             gpu_idx: int32): bool {.cdecl,
    importc: "AUTDExtraGeometryViewer", dynlib: dll.}