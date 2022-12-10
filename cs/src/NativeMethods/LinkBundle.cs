// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.NativeMethods
{
    internal static class LinkBundle
    {
        private const string DLL = "autd3capi-link-bundle";

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkBundle(out IntPtr @out, IntPtr[]? links, int n);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
