// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

namespace AUTD3Sharp
{
    namespace NativeMethods
    {
        internal static class ModulationAudioFile
        {
            private const string DLL = "autd3capi-modulation-audio-file";

            [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDModulationWav(string path, System.Text.StringBuilder err);
    }
    }
}
