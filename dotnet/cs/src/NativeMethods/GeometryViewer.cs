// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

namespace AUTD3Sharp
{
    namespace NativeMethods
    {
        internal static class GeometryViewer
        {
            private const string DLL = "autd3capi-geometry-viewer";

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGeometryViewer();

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGeometryViewerSize(IntPtr viewer, uint width, uint height);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGeometryViewerVsync(IntPtr viewer, [MarshalAs(UnmanagedType.U1)] bool vsync);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern int AUTDGeometryViewerRun(IntPtr viewer, IntPtr cnt);
    }
    }
}
