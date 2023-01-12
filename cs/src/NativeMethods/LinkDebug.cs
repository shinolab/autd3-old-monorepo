// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.NativeMethods
{
    internal static class LinkDebug
    {
        private const string DLL = "autd3capi-link-debug";

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkDebug(out IntPtr @out, int level, IntPtr outFunc, IntPtr flushFunc);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
