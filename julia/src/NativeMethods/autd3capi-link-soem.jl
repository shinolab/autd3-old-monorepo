# This file was automatically generated from header file

module autd3capi_link_soem

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

const _dll = joinpath(@__DIR__, get_bin_path(), "bin", get_lib_prefix() * "autd3capi-link-soem" * get_lib_ext())

autd_get_adapter_pointer(out) = ccall((:AUTDGetAdapterPointer, _dll), Int32, (Ref{Ptr{Cvoid}}, ), out);
autd_get_adapter(p_adapter, index, desc, name) = ccall((:AUTDGetAdapter, _dll), Cvoid, (Ptr{Cvoid}, Int32, Ptr{UInt8}, Ptr{UInt8}, ), p_adapter, index, desc, name);
autd_free_adapter_pointer(p_adapter) = ccall((:AUTDFreeAdapterPointer, _dll), Cvoid, (Ptr{Cvoid}, ), p_adapter);
autd_link_soem(out, ifname, sync0_cycle, send_cycle, freerun, on_lost, high_precision, state_check_interval) = ccall((:AUTDLinkSOEM, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Cstring, UInt16, UInt16, Bool, Ptr{Cvoid}, Bool, UInt64, ), out, ifname, sync0_cycle, send_cycle, freerun, on_lost, high_precision, state_check_interval);
autd_link_soem_set_log_level(level) = ccall((:AUTDLinkSOEMSetLogLevel, _dll), Cvoid, (Int32, ), level);
autd_link_soem_set_default_logger(out, flush) = ccall((:AUTDLinkSOEMSetDefaultLogger, _dll), Cvoid, (Ptr{Cvoid}, Ptr{Cvoid}, ), out, flush);
end
