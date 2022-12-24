// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.NativeMethods
{
    internal static class ExtraGeometryViewer
    {
        private const string DLL = "autd3capi-extra-geometry-viewer";

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)][return: MarshalAs(UnmanagedType.U1)] public static extern bool AUTDExtraGeometryViewer(IntPtr cnt, int width, int height, [MarshalAs(UnmanagedType.U1)] bool vsync, int gpuIdx);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
