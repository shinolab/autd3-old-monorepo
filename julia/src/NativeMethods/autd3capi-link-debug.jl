# This file was automatically generated from header file

module autd3capi_link_debug

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

const _dll = joinpath(@__DIR__, get_bin_path(), "bin", get_lib_prefix() * "autd3capi-link-debug" * get_lib_ext())

autd_link_debug(out, level, out_func, flush_func) = ccall((:AUTDLinkDebug, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Int32, Ptr{Cvoid}, Ptr{Cvoid}, ), out, level, out_func, flush_func);
end
