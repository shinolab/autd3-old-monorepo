// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.NativeMethods
{
    internal static class ModulationAudioFile
    {
        private const string DLL = "autd3capi-modulation-audio-file";

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDModulationRawPCM(out IntPtr mod, string filename, float samplingFreq, uint modSamplingFreqDiv);
        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDModulationWav(out IntPtr mod, string filename, uint modSamplingFreqDiv);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
