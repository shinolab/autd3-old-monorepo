// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.NativeMethods
{
    internal static class ExtraSimulator
    {
        private const string DLL = "autd3capi-extra-simulator";

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)][return: MarshalAs(UnmanagedType.U1)] public static extern bool AUTDExtraSimulator(string settingsPath, [MarshalAs(UnmanagedType.U1)] bool vsync, int gpuIdx);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
