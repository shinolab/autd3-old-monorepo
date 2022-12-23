// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.NativeMethods
{
    internal static class LinkRemoteTwinCAT
    {
        private const string DLL = "autd3capi-link-remote-twincat";

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteTwinCAT(out IntPtr @out, string serverIpAddr, string serverAmsNetId, string clientAmsNetId);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
