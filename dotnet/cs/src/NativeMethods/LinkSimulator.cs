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

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSimulator(ushort port);

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSimulatorAddr(IntPtr builder, string addr);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSimulatorTimeout(IntPtr builder, ulong timeoutNs);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSimulatorBuild(IntPtr builder);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
