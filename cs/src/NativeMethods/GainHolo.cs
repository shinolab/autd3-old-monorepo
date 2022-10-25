// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.NativeMethods
{
    internal static class GainHolo
    {
        private const string DLL = "autd3capi-gain-holo";

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDEigenBackend(out IntPtr @out);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDDeleteBackend(IntPtr backend);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloSDP(out IntPtr gain, IntPtr backend, double alpha, double lambda, ulong repeat);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloEVD(out IntPtr gain, IntPtr backend, double gamma);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloNaive(out IntPtr gain, IntPtr backend);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloGS(out IntPtr gain, IntPtr backend, ulong repeat);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloGSPAT(out IntPtr gain, IntPtr backend, ulong repeat);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloLM(out IntPtr gain, IntPtr backend, double eps1, double eps2, double tau, ulong kMax, double[]? initial, int initialSize);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloGreedy(out IntPtr gain, IntPtr backend, int phaseDiv);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloLSSGreedy(out IntPtr gain, IntPtr backend, int phaseDiv);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloAPO(out IntPtr gain, IntPtr backend, double eps, double lambda, int kMax, int lineSearchMax);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloAdd(IntPtr gain, double x, double y, double z, double amp);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDConstraintDontCare(out IntPtr constraint);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDConstraintNormalize(out IntPtr constraint);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDConstraintUniform(out IntPtr constraint, double value);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDConstraintClamp(out IntPtr constraint);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDSetConstraint(IntPtr gain, IntPtr constraint);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
