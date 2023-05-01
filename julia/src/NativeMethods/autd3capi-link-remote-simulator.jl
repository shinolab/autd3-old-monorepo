# This file was automatically generated from header file

module autd3capi_link_remote_simulator

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

const _dll = joinpath(@__DIR__, get_bin_path(), "bin", get_lib_prefix() * "autd3capi-link-remote-simulator" * get_lib_ext())

autd_link_remote_simulator(out, ip, port) = ccall((:AUTDLinkRemoteSimulator, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Cstring, UInt16, ), out, ip, port);
autd_link_remote_simulator_log_level(remote_simulator, level) = ccall((:AUTDLinkRemoteSimulatorLogLevel, _dll), Cvoid, (Ptr{Cvoid}, Int32, ), remote_simulator, level);
autd_link_remote_simulator_log_func(remote_simulator, out_func, flush_func) = ccall((:AUTDLinkRemoteSimulatorLogFunc, _dll), Cvoid, (Ptr{Cvoid}, Ptr{Cvoid}, Ptr{Cvoid}, ), remote_simulator, out_func, flush_func);
autd_link_remote_simulator_timeout(remote_simulator, timeout_ns) = ccall((:AUTDLinkRemoteSimulatorTimeout, _dll), Cvoid, (Ptr{Cvoid}, UInt64, ), remote_simulator, timeout_ns);
autd_link_remote_simulator_build(out, remote_simulator) = ccall((:AUTDLinkRemoteSimulatorBuild, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, ), out, remote_simulator);
end
