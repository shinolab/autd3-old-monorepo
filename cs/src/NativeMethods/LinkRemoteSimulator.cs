// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.NativeMethods
{
    internal static class LinkRemoteSimulator
    {
        private const string DLL = "autd3capi-link-remote-simulator";

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteSimulator(out IntPtr @out, string ip, ushort port);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteSimulatorLogLevel(IntPtr remoteSimulator, int level);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteSimulatorLogFunc(IntPtr remoteSimulator, IntPtr outFunc, IntPtr flushFunc);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteSimulatorTimeout(IntPtr remoteSimulator, ulong timeoutNs);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteSimulatorBuild(out IntPtr @out, IntPtr remoteSimulator);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
