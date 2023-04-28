##  This file was automatically generated from header file

{.deadCodeElim: on.}
when defined(windows):
  const
    dll* = "bin/autd3capi-link-remote-twincat.dll"
elif defined(macosx):
  const
    dll* = "bin/libautd3capi-link-remote-twincat.dylib"
else:
  const
    dll* = "bin/libautd3capi-link-remote-twincat.so"
proc AUTDLinkRemoteTwinCAT*(`out`: ptr pointer; server_ams_net_id: cstring) {.cdecl,
    importc: "AUTDLinkRemoteTwinCAT", dynlib: dll.}
proc AUTDLinkRemoteTwinCATServerIpAddr*(remote_twincat: pointer;
                                       server_ip_addr: cstring) {.cdecl,
    importc: "AUTDLinkRemoteTwinCATServerIpAddr", dynlib: dll.}
proc AUTDLinkRemoteTwinCATClientAmsNetId*(remote_twincat: pointer;
    client_ams_net_id: cstring) {.cdecl,
                                importc: "AUTDLinkRemoteTwinCATClientAmsNetId",
                                dynlib: dll.}
proc AUTDLinkRemoteTwinCATLogLevel*(remote_twincat: pointer; level: int32) {.cdecl,
    importc: "AUTDLinkRemoteTwinCATLogLevel", dynlib: dll.}
proc AUTDLinkRemoteTwinCATLogFunc*(remote_twincat: pointer; out_func: pointer;
                                  flush_func: pointer) {.cdecl,
    importc: "AUTDLinkRemoteTwinCATLogFunc", dynlib: dll.}
proc AUTDLinkRemoteTwinCATTimeout*(remote_twincat: pointer; timeout_ns: uint64) {.
    cdecl, importc: "AUTDLinkRemoteTwinCATTimeout", dynlib: dll.}
proc AUTDLinkRemoteTwinCATBuild*(`out`: ptr pointer; remote_twincat: pointer) {.cdecl,
    importc: "AUTDLinkRemoteTwinCATBuild", dynlib: dll.}