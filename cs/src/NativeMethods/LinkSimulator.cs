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

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSimulator(out IntPtr @out);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSimulatorLogLevel(IntPtr simulator, int level);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSimulatorLogFunc(IntPtr simulator, IntPtr outFunc, IntPtr flushFunc);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSimulatorTimeout(IntPtr simulator, ulong timeoutNs);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSimulatorBuild(out IntPtr @out, IntPtr simulator);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
