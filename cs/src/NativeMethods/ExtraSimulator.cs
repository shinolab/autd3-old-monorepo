// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;
            
namespace AUTD3Sharp.NativeMethods
{
    internal static class ExtraSimulator
    {
        const string DLL = "autd3capi-extra-simulator";

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDExtraSimulator(string settingsPath, ushort port, string ip, [MarshalAs(UnmanagedType.U1)] bool vsync, int gpuIdx);
    }
}
