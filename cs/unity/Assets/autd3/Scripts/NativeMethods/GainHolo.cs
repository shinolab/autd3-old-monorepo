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
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloSDP(out IntPtr gain, IntPtr backend, float alpha, float lambda, ulong repeat);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloEVP(out IntPtr gain, IntPtr backend, float gamma);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloNaive(out IntPtr gain, IntPtr backend);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloGS(out IntPtr gain, IntPtr backend, ulong repeat);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloGSPAT(out IntPtr gain, IntPtr backend, ulong repeat);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloLM(out IntPtr gain, IntPtr backend, float eps1, float eps2, float tau, ulong kMax, float[]? initial, int initialSize);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloGreedy(out IntPtr gain, IntPtr backend, int phaseDiv);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloLSSGreedy(out IntPtr gain, IntPtr backend, int phaseDiv);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloAPO(out IntPtr gain, IntPtr backend, float eps, float lambda, int kMax, int lineSearchMax);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloAdd(IntPtr gain, float x, float y, float z, float amp);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDConstraintDontCare(out IntPtr constraint);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDConstraintNormalize(out IntPtr constraint);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDConstraintUniform(out IntPtr constraint, float value);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDConstraintClamp(out IntPtr constraint);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDSetConstraint(IntPtr gain, IntPtr constraint);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
