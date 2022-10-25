// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.NativeMethods
{
    internal static class LinkSimulator
    {
        private const string DLL = "autd3capi-link-simulator";

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSimulator(out IntPtr @out, ushort port, string ipAddr);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
