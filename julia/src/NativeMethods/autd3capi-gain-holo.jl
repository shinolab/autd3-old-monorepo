# This file was automatically generated from header file

module autd3capi_gain_holo

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

const _dll = joinpath(@__DIR__, get_bin_path(), "bin", get_lib_prefix() * "autd3capi-gain-holo" * get_lib_ext())

autd_eigen_backend(out) = ccall((:AUTDEigenBackend, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), out);
autd_delete_backend(backend) = ccall((:AUTDDeleteBackend, _dll), Cvoid, (Ptr{Cvoid}, ), backend);
autd_gain_holo_sdp(gain, backend, alpha, lambda, repeat) = ccall((:AUTDGainHoloSDP, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, Float64, Float64, UInt64, ), gain, backend, alpha, lambda, repeat);
autd_gain_holo_evp(gain, backend, gamma) = ccall((:AUTDGainHoloEVP, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, Float64, ), gain, backend, gamma);
autd_gain_holo_naive(gain, backend) = ccall((:AUTDGainHoloNaive, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, ), gain, backend);
autd_gain_holo_gs(gain, backend, repeat) = ccall((:AUTDGainHoloGS, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, UInt64, ), gain, backend, repeat);
autd_gain_holo_gspat(gain, backend, repeat) = ccall((:AUTDGainHoloGSPAT, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, UInt64, ), gain, backend, repeat);
autd_gain_holo_lm(gain, backend, eps_1, eps_2, tau, k_max, initial, initial_size) = ccall((:AUTDGainHoloLM, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, Float64, Float64, Float64, UInt64, Array{Float64,1}, Int32, ), gain, backend, eps_1, eps_2, tau, k_max, initial, initial_size);
autd_gain_holo_greedy(gain, backend, phase_div) = ccall((:AUTDGainHoloGreedy, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, Int32, ), gain, backend, phase_div);
autd_gain_holo_lss_greedy(gain, backend, phase_div) = ccall((:AUTDGainHoloLSSGreedy, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, Int32, ), gain, backend, phase_div);
autd_gain_holo_apo(gain, backend, eps, lambda, k_max, line_search_max) = ccall((:AUTDGainHoloAPO, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Ptr{Cvoid}, Float64, Float64, Int32, Int32, ), gain, backend, eps, lambda, k_max, line_search_max);
autd_gain_holo_add(gain, x, y, z, amp) = ccall((:AUTDGainHoloAdd, _dll), Cvoid, (Ptr{Cvoid}, Float64, Float64, Float64, Float64, ), gain, x, y, z, amp);
autd_constraint_dont_care(constraint) = ccall((:AUTDConstraintDontCare, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), constraint);
autd_constraint_normalize(constraint) = ccall((:AUTDConstraintNormalize, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), constraint);
autd_constraint_uniform(constraint, value) = ccall((:AUTDConstraintUniform, _dll), Cvoid, (Ref{Ptr{Cvoid}}, Float64, ), constraint, value);
autd_constraint_clamp(constraint) = ccall((:AUTDConstraintClamp, _dll), Cvoid, (Ref{Ptr{Cvoid}}, ), constraint);
autd_set_constraint(gain, constraint) = ccall((:AUTDSetConstraint, _dll), Cvoid, (Ptr{Cvoid}, Ptr{Cvoid}, ), gain, constraint);
end
