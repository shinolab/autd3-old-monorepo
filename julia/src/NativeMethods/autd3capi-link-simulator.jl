# This file was automatically generated from header file

module autd3capi_link_simulator

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

const _dll = joinpath(@__DIR__, "bin", get_lib_prefix() * "autd3capi-link-simulator" * get_lib_ext())

autd_link_simulator(out, port, ip_addr) = ccall((:AUTDLinkSimulator, _dll), Cvoid, (Ref{Ptr{Cvoid}}, UInt16, Cstring, ), out, port, ip_addr);
end
