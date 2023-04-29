# This file was automatically generated from header file

module autd3capi_link_remote_twincat

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

const _dll = joinpath(@__DIR__, get_bin_path(), "bin", get_lib_prefix() * "autd3capi-link-remote-twincat" * get_lib_ext())

autd_link_remote_twin_cat(out, server_ams_net_id) = ccall((:AUTDLinkRemoteTwinCAT, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Cstring, ), out, server_ams_net_id);
autd_link_remote_twin_cat_server_ip_addr(remote_twincat, server_ip_addr) = ccall((:AUTDLinkRemoteTwinCATServerIpAddr, _dll), Cvoid, (Ptr{Cvoid}, Cstring, ), remote_twincat, server_ip_addr);
autd_link_remote_twin_cat_client_ams_net_id(remote_twincat, client_ams_net_id) = ccall((:AUTDLinkRemoteTwinCATClientAmsNetId, _dll), Cvoid, (Ptr{Cvoid}, Cstring, ), remote_twincat, client_ams_net_id);
autd_link_remote_twin_cat_log_level(remote_twincat, level) = ccall((:AUTDLinkRemoteTwinCATLogLevel, _dll), Cvoid, (Ptr{Cvoid}, Int32, ), remote_twincat, level);
autd_link_remote_twin_cat_log_func(remote_twincat, out_func, flush_func) = ccall((:AUTDLinkRemoteTwinCATLogFunc, _dll), Cvoid, (Ptr{Cvoid}, Ptr{Cvoid}, Ptr{Cvoid}, ), remote_twincat, out_func, flush_func);
autd_link_remote_twin_cat_timeout(remote_twincat, timeout_ns) = ccall((:AUTDLinkRemoteTwinCATTimeout, _dll), Cvoid, (Ptr{Cvoid}, UInt64, ), remote_twincat, timeout_ns);
autd_link_remote_twin_cat_build(out, remote_twincat) = ccall((:AUTDLinkRemoteTwinCATBuild, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, ), out, remote_twincat);
end
