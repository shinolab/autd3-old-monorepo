##  This file was automatically generated from header file

{.deadCodeElim: on.}
when defined(windows):
  const
    dll* = "bin/autd3capi-link-remote-soem.dll"
elif defined(macosx):
  const
    dll* = "bin/libautd3capi-link-remote-soem.dylib"
else:
  const
    dll* = "bin/libautd3capi-link-remote-soem.so"
proc AUTDLinkRemoteSOEM*(`out`: ptr pointer; ip: cstring; port: uint16) {.cdecl,
    importc: "AUTDLinkRemoteSOEM", dynlib: dll.}
proc AUTDLinkRemoteSOEMServerIpAddr*(remote_soem: pointer; server_ip_addr: cstring) {.
    cdecl, importc: "AUTDLinkRemoteSOEMServerIpAddr", dynlib: dll.}
proc AUTDLinkRemoteSOEMClientAmsNetId*(remote_soem: pointer;
                                      client_ams_net_id: cstring) {.cdecl,
    importc: "AUTDLinkRemoteSOEMClientAmsNetId", dynlib: dll.}
proc AUTDLinkRemoteSOEMLogLevel*(remote_soem: pointer; level: int32) {.cdecl,
    importc: "AUTDLinkRemoteSOEMLogLevel", dynlib: dll.}
proc AUTDLinkRemoteSOEMLogFunc*(remote_soem: pointer; out_func: pointer;
                               flush_func: pointer) {.cdecl,
    importc: "AUTDLinkRemoteSOEMLogFunc", dynlib: dll.}
proc AUTDLinkRemoteSOEMTimeout*(remote_soem: pointer; timeout_ns: uint64) {.cdecl,
    importc: "AUTDLinkRemoteSOEMTimeout", dynlib: dll.}
proc AUTDLinkRemoteSOEMBuild*(`out`: ptr pointer; remote_soem: pointer) {.cdecl,
    importc: "AUTDLinkRemoteSOEMBuild", dynlib: dll.}