// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;
            
namespace AUTD3Sharp.NativeMethods
{
    internal static class ExtraGeometryViewer
    {
        const string DLL = "autd3capi-extra-geometry-viewer";

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDExtraGeometryViewer(IntPtr cnt, int width, int height, [MarshalAs(UnmanagedType.U1)] bool vsync, string model, string font, int gpuIdx);
    }
}
