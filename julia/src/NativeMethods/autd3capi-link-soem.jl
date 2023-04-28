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
autd_link_soem(out) = ccall((:AUTDLinkSOEM, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), out);
autd_link_soem_ifname(soem, ifname) = ccall((:AUTDLinkSOEMIfname, _dll), Cvoid, (Ptr{Cvoid}, Cstring, ), soem, ifname);
autd_link_soem_buf_size(soem, buf_size) = ccall((:AUTDLinkSOEMBufSize, _dll), Cvoid, (Ptr{Cvoid}, UInt64, ), soem, buf_size);
autd_link_soem_sync_0_cycle(soem, sync0_cycle) = ccall((:AUTDLinkSOEMSync0Cycle, _dll), Cvoid, (Ptr{Cvoid}, UInt16, ), soem, sync0_cycle);
autd_link_soem_send_cycle(soem, send_cycle) = ccall((:AUTDLinkSOEMSendCycle, _dll), Cvoid, (Ptr{Cvoid}, UInt16, ), soem, send_cycle);
autd_link_soem_freerun(soem, freerun) = ccall((:AUTDLinkSOEMFreerun, _dll), Cvoid, (Ptr{Cvoid}, Bool, ), soem, freerun);
autd_link_soem_on_lost(soem, on_lost) = ccall((:AUTDLinkSOEMOnLost, _dll), Cvoid, (Ptr{Cvoid}, Ptr{Cvoid}, ), soem, on_lost);
autd_link_soem_timer_strategy(soem, timer_strategy) = ccall((:AUTDLinkSOEMTimerStrategy, _dll), Cvoid, (Ptr{Cvoid}, UInt8, ), soem, timer_strategy);
autd_link_soem_state_check_interval(soem, state_check_interval) = ccall((:AUTDLinkSOEMStateCheckInterval, _dll), Cvoid, (Ptr{Cvoid}, UInt64, ), soem, state_check_interval);
autd_link_soem_log_level(soem, level) = ccall((:AUTDLinkSOEMLogLevel, _dll), Cvoid, (Ptr{Cvoid}, Int32, ), soem, level);
autd_link_soem_log_func(soem, out_func, flush_func) = ccall((:AUTDLinkSOEMLogFunc, _dll), Cvoid, (Ptr{Cvoid}, Ptr{Cvoid}, Ptr{Cvoid}, ), soem, out_func, flush_func);
autd_link_soem_timeout(soem, timeout_ns) = ccall((:AUTDLinkSOEMTimeout, _dll), Cvoid, (Ptr{Cvoid}, UInt64, ), soem, timeout_ns);
autd_link_soem_build(out, soem) = ccall((:AUTDLinkSOEMBuild, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, ), out, soem);
end
