# This file was automatically generated from header file

module autd3capi_link_twincat

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

const _dll = joinpath(@__DIR__, get_bin_path(), "bin", get_lib_prefix() * "autd3capi-link-twincat" * get_lib_ext())

autd_link_twin_cat(out) = ccall((:AUTDLinkTwinCAT, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), out);
autd_link_twin_cat_log_level(twincat, level) = ccall((:AUTDLinkTwinCATLogLevel, _dll), Cvoid, (Ptr{Cvoid}, Int32, ), twincat, level);
autd_link_twin_cat_log_func(twincat, out_func, flush_func) = ccall((:AUTDLinkTwinCATLogFunc, _dll), Cvoid, (Ptr{Cvoid}, Ptr{Cvoid}, Ptr{Cvoid}, ), twincat, out_func, flush_func);
autd_link_twin_cat_timeout(twincat, timeout_ns) = ccall((:AUTDLinkTwinCATTimeout, _dll), Cvoid, (Ptr{Cvoid}, UInt64, ), twincat, timeout_ns);
autd_link_twin_cat_build(out, twincat) = ccall((:AUTDLinkTwinCATBuild, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, ), out, twincat);
end
