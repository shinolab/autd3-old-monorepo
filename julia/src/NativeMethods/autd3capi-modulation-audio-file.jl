# This file was automatically generated from header file

module autd3capi_modulation_audio_file

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

const _dll = joinpath(@__DIR__, get_bin_path(), "bin", get_lib_prefix() * "autd3capi-modulation-audio-file" * get_lib_ext())

autd_modulation_raw_pcm(mod, filename, sampling_freq, mod_sampling_freq_div) = ccall((:AUTDModulationRawPCM, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Cstring, Float64, UInt32, ), mod, filename, sampling_freq, mod_sampling_freq_div);
autd_modulation_wav(mod, filename, mod_sampling_freq_div) = ccall((:AUTDModulationWav, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Cstring, UInt32, ), mod, filename, mod_sampling_freq_div);
end
