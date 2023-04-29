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
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSOEM(out IntPtr @out);
        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSOEMIfname(IntPtr soem, string ifname);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSOEMBufSize(IntPtr soem, ulong bufSize);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSOEMSync0Cycle(IntPtr soem, ushort sync0Cycle);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSOEMSendCycle(IntPtr soem, ushort sendCycle);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSOEMFreerun(IntPtr soem, [MarshalAs(UnmanagedType.U1)] bool freerun);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSOEMOnLost(IntPtr soem, IntPtr onLost);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSOEMTimerStrategy(IntPtr soem, byte timerStrategy);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSOEMStateCheckInterval(IntPtr soem, ulong stateCheckInterval);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSOEMLogLevel(IntPtr soem, int level);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSOEMLogFunc(IntPtr soem, IntPtr outFunc, IntPtr flushFunc);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSOEMTimeout(IntPtr soem, ulong timeoutNs);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkSOEMBuild(out IntPtr @out, IntPtr soem);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
