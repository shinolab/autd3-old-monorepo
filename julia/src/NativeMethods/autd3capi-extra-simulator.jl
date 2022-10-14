# This file was automatically generated from header file

module autd3capi_extra_simulator

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

const _dll = joinpath(@__DIR__, get_bin_path(), "bin", get_lib_prefix() * "autd3capi-extra-simulator" * get_lib_ext())

autd_extra_simulator(settings_path, port, ip, vsync, gpu_idx) = ccall((:AUTDExtraSimulator, _dll), Cvoid, (Cstring, UInt16, Cstring, Bool, Int32, ), settings_path, port, ip, vsync, gpu_idx);
end
