// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.NativeMethods
{
    internal static class LinkSOEM
    {
        private const string DLL = "autd3capi-link-soem";

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern int AUTDGetAdapterPointer(out IntPtr @out);
        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGetAdapter(IntPtr pAdapter, int index, System.Text.StringBuilder? desc, System.Text.StringBuilder? name);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDFreeAdapterPointer(IntPtr pAdapter);
        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSOEM(out IntPtr @out, string ifname, ushort sync0Cycle, ushort sendCycle, [MarshalAs(UnmanagedType.U1)] bool freerun, IntPtr onLost, [MarshalAs(UnmanagedType.U1)] bool highPrecision);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
