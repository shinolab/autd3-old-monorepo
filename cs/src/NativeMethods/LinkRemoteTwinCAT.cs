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

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteTwinCAT(out IntPtr @out, string serverAmsNetId);
        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteTwinCATServerIpAddr(IntPtr remoteTwinCAT, string serverIpAddr);
        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteTwinCATClientAmsNetId(IntPtr remoteTwinCAT, string clientAmsNetId);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteTwinCATLogLevel(IntPtr remoteTwinCAT, int level);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteTwinCATLogFunc(IntPtr remoteTwinCAT, IntPtr outFunc, IntPtr flushFunc);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteTwinCATTimeout(IntPtr remoteTwinCAT, ulong timeoutNs);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteTwinCATBuild(out IntPtr @out, IntPtr remoteTwinCAT);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
