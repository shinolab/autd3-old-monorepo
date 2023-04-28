# This file was automatically generated from header file

module autd3capi_link_remote_soem

function get_bin_path()
if Sys.iswindows()
    return "win-x64"
elseif Sys.isapple()
    return "macos-universal"
elseif Sys.islinux()
    return "linux-x64"
end
end

function get_lib_ext()
if Sys.iswindows()
    return ".dll"
elseif Sys.isapple()
    return ".dylib"
elseif Sys.islinux()
    return ".so"
end
end

function get_lib_prefix()
if Sys.iswindows()
    return ""
else
    return "lib"
end
end

const _dll = joinpath(@__DIR__, get_bin_path(), "bin", get_lib_prefix() * "autd3capi-link-remote-soem" * get_lib_ext())

autd_link_remote_soem(out, ip, port) = ccall((:AUTDLinkRemoteSOEM, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Cstring, UInt16, ), out, ip, port);
autd_link_remote_soem_server_ip_addr(remote_soem, server_ip_addr) = ccall((:AUTDLinkRemoteSOEMServerIpAddr, _dll), Cvoid, (Ptr{Cvoid}, Cstring, ), remote_soem, server_ip_addr);
autd_link_remote_soem_client_ams_net_id(remote_soem, client_ams_net_id) = ccall((:AUTDLinkRemoteSOEMClientAmsNetId, _dll), Cvoid, (Ptr{Cvoid}, Cstring, ), remote_soem, client_ams_net_id);
autd_link_remote_soem_log_level(remote_soem, level) = ccall((:AUTDLinkRemoteSOEMLogLevel, _dll), Cvoid, (Ptr{Cvoid}, Int32, ), remote_soem, level);
autd_link_remote_soem_log_func(remote_soem, out_func, flush_func) = ccall((:AUTDLinkRemoteSOEMLogFunc, _dll), Cvoid, (Ptr{Cvoid}, Ptr{Cvoid}, Ptr{Cvoid}, ), remote_soem, out_func, flush_func);
autd_link_remote_soem_timeout(remote_soem, timeout_ns) = ccall((:AUTDLinkRemoteSOEMTimeout, _dll), Cvoid, (Ptr{Cvoid}, UInt64, ), remote_soem, timeout_ns);
autd_link_remote_soem_build(out, remote_soem) = ccall((:AUTDLinkRemoteSOEMBuild, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, ), out, remote_soem);
end
