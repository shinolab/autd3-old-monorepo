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

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteSOEM(out IntPtr @out, string ip, ushort port);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteSOEMLogLevel(IntPtr remoteSOEM, int level);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteSOEMLogFunc(IntPtr remoteSOEM, IntPtr outFunc, IntPtr flushFunc);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteSOEMTimeout(IntPtr remoteSOEM, ulong timeoutNs);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteSOEMBuild(out IntPtr @out, IntPtr remoteSOEM);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
