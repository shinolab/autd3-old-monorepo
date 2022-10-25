// This file was automatically generated from header file

#ifdef C2NIM
#  dynlib dll
#  cdecl
#  if defined(windows)
#    define dll "bin/autd3capi-gain-holo.dll"
#  elif defined(macosx)
#    define dll "bin/libautd3capi-gain-holo.dylib"
#  else
#    define dll "bin/libautd3capi-gain-holo.so"
#  endif
#endif

void AUTDEigenBackend(void** out);
void AUTDDeleteBackend(void* backend);
void AUTDGainHoloSDP(void** gain, void* backend, float64 alpha, float64 lambda, uint64 repeat);
void AUTDGainHoloEVD(void** gain, void* backend, float64 gamma);
void AUTDGainHoloNaive(void** gain, void* backend);
void AUTDGainHoloGS(void** gain, void* backend, uint64 repeat);
void AUTDGainHoloGSPAT(void** gain, void* backend, uint64 repeat);
void AUTDGainHoloLM(void** gain, void* backend, float64 eps_1, float64 eps_2, float64 tau, uint64 k_max, float64* initial, int32 initial_size);
void AUTDGainHoloGreedy(void** gain, void* backend, int32 phase_div);
void AUTDGainHoloLSSGreedy(void** gain, void* backend, int32 phase_div);
void AUTDGainHoloAPO(void** gain, void* backend, float64 eps, float64 lambda, int32 k_max, int32 line_search_max);
void AUTDGainHoloAdd(void* gain, float64 x, float64 y, float64 z, float64 amp);
void AUTDConstraintDontCare(void** constraint);
void AUTDConstraintNormalize(void** constraint);
void AUTDConstraintUniform(void** constraint, float64 value);
void AUTDConstraintClamp(void** constraint);
void AUTDSetConstraint(void* gain, void* constraint);
