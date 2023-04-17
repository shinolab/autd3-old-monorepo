// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.NativeMethods
{
    internal static class LinkRemoteSOEM
    {
        private const string DLL = "autd3capi-link-remote-soem";

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteSOEM(out IntPtr @out, string ip, ushort port, ulong timeoutNs);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
