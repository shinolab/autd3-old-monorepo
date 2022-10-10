# This file was automatically generated from header file

module autd3capi_backend_cuda

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

const _dll = joinpath(@__DIR__, "bin", get_lib_prefix() * "autd3capi-backend-cuda" * get_lib_ext())

autdcuda_backend(out) = ccall((:AUTDCUDABackend, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), out);
end
