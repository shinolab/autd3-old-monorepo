// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;
            
namespace AUTD3Sharp.NativeMethods
{
    internal static class BackendCUDA
    {
        const string DLL = "autd3capi-backend-cuda";

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDCUDABackend(out IntPtr @out);
    }
}
