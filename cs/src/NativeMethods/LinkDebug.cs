// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;
            
namespace AUTD3Sharp.NativeMethods
{
    internal static class LinkDebug
    {
        const string DLL = "autd3capi-link-debug";

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkDebug(out IntPtr @out);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkDebugSetLevel(int level);
    }
}
