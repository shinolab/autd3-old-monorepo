// This file was automatically generated from header file

#ifdef C2NIM
#  dynlib dll
#  cdecl
#  if defined(windows)
#    define dll "bin/autd3capi-modulation-audio-file.dll"
#  elif defined(macosx)
#    define dll "bin/libautd3capi-modulation-audio-file.dylib"
#  else
#    define dll "bin/libautd3capi-modulation-audio-file.so"
#  endif
#endif

void AUTDModulationRawPCM(void** mod, char* filename, float64 sampling_freq, uint32 mod_sampling_freq_div);
void AUTDModulationWav(void** mod, char* filename, uint32 mod_sampling_freq_div);
