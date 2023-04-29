// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.NativeMethods
{
    internal static class LinkTwinCAT
    {
        private const string DLL = "autd3capi-link-twincat";

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkTwinCAT(out IntPtr @out);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkTwinCATLogLevel(IntPtr twincat, int level);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkTwinCATLogFunc(IntPtr twincat, IntPtr outFunc, IntPtr flushFunc);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkTwinCATTimeout(IntPtr twincat, ulong timeoutNs);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkTwinCATBuild(out IntPtr @out, IntPtr twincat);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
