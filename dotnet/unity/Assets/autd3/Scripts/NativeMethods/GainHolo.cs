// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.NativeMethods
{
    internal static class GainHolo
    {
        private const string DLL = "autd3capi-gain-holo";

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDDefaultBackend();

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainHoloSDP(IntPtr backend);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
