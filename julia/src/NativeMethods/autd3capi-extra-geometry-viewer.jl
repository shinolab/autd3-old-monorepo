# This file was automatically generated from header file

module autd3capi_extra_geometry_viewer

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

const _dll = joinpath(@__DIR__, get_bin_path(), "bin", get_lib_prefix() * "autd3capi-extra-geometry-viewer" * get_lib_ext())

autd_extra_geometry_viewer(geometry, width, height, vsync, gpu_idx) = ccall((:AUTDExtraGeometryViewer, _dll), Bool, (Ptr{Cvoid}, Int32, Int32, Bool, Int32, ), geometry, width, height, vsync, gpu_idx);
end
