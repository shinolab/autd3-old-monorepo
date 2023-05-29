// This file is autogenerated
using System;
using System.Runtime.InteropServices;

namespace AUTD3Sharp
{
    namespace NativeMethods
    {
        internal static class Simulator
        {
            private const string DLL = "autd3capi_simulator";

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDSimulator();

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDSimulatorPort(IntPtr simulator, ushort port);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDSimulatorWindowSize(IntPtr simulator, uint width, uint height);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDSimulatorVsync(IntPtr simulator, [MarshalAs(UnmanagedType.U1)] bool vsync);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDSimulatorGpuIdx(IntPtr simulator, int idx);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDSimulatorSettingsPath(IntPtr simulator, string path, byte[] err);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern int AUTDSimulatorRun(IntPtr simulator);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)][return: MarshalAs(UnmanagedType.U1)] public static extern bool AUTDSimulatorSaveSettings(IntPtr simulator, string path, byte[] err);
        }
    }
}
