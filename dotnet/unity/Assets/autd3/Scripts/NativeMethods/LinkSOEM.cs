// This file is autogenerated
using System;
using System.Runtime.InteropServices;

namespace AUTD3Sharp
{
    namespace NativeMethods
    {
        internal static class LinkSOEM
        {
            private const string DLL = "autd3capi-link-soem";

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGetAdapterPointer(out uint len);

            [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGetAdapter(IntPtr adapters, uint idx, System.Text.StringBuilder desc, System.Text.StringBuilder name);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDFreeAdapterPointer(IntPtr adapters);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSOEM();

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSOEMSendCycle(IntPtr builder, ushort cycle);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSOEMSync0Cycle(IntPtr builder, ushort cycle);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSOEMBufSize(IntPtr builder, uint bufSize);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSOEMTimerStrategy(IntPtr builder, TimerStrategy timerStrategy);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSOEMSyncMode(IntPtr builder, SyncMode mode);

            [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSOEMIfname(IntPtr builder, string ifname);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSOEMStateCheckInterval(IntPtr builder, uint intervalMs);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSOEMOnLost(IntPtr builder, IntPtr onLostFunc);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSOEMLogLevel(IntPtr builder, Level level);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSOEMLogFunc(IntPtr builder, Level level, IntPtr outFunc, IntPtr flushFunc);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSOEMTimeout(IntPtr builder, ulong timeoutNs);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkSOEMBuild(IntPtr builder);

            [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkRemoteSOEM(string addr, ushort port);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkRemoteSOEMTimeout(IntPtr builder, ulong timeoutNs);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkRemoteSOEMBuild(IntPtr builder);
        }
    }

    public enum TimerStrategy : byte
    {
        Sleep = 0,
        NativeTimer = 1,
        BusyWait = 2,
    }

    public enum SyncMode : byte
    {
        FreeRun = 0,
        Dc = 1,
    }
}
