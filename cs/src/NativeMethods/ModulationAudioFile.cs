// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;
            
namespace AUTD3Sharp.NativeMethods
{
    internal static class ModulationAudioFile
    {
        const string DLL = "autd3capi-modulation-audio-file";

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDModulationRawPCM(out IntPtr mod, string filename, double samplingFreq, uint modSamplingFreqDiv);
        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDModulationWav(out IntPtr mod, string filename, uint modSamplingFreqDiv);
    }
}
