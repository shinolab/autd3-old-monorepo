# This file was automatically generated from header file

module autd3capi_link_simulator

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

const _dll = joinpath(@__DIR__, get_bin_path(), "bin", get_lib_prefix() * "autd3capi-link-simulator" * get_lib_ext())

autd_link_simulator(out) = ccall((:AUTDLinkSimulator, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), out);
autd_link_simulator_log_level(simulator, level) = ccall((:AUTDLinkSimulatorLogLevel, _dll), Cvoid, (Ptr{Cvoid}, Int32, ), simulator, level);
autd_link_simulator_log_func(simulator, out_func, flush_func) = ccall((:AUTDLinkSimulatorLogFunc, _dll), Cvoid, (Ptr{Cvoid}, Ptr{Cvoid}, Ptr{Cvoid}, ), simulator, out_func, flush_func);
autd_link_simulator_timeout(simulator, timeout_ns) = ccall((:AUTDLinkSimulatorTimeout, _dll), Cvoid, (Ptr{Cvoid}, UInt64, ), simulator, timeout_ns);
autd_link_simulator_build(out, simulator) = ccall((:AUTDLinkSimulatorBuild, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, ), out, simulator);
end
