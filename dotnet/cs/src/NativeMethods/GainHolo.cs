// This file is autogenerated
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp
{
    namespace NativeMethods
    {
        internal static class GainHolo
        {
            private const string DLL = "autd3capi_gain_holo";

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern BackendPtr AUTDDefaultBackend();

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern ConstraintPtr AUTDGainHoloDotCareConstraint();

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern ConstraintPtr AUTDGainHoloNormalizeConstraint();

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern ConstraintPtr AUTDGainHoloUniformConstraint(double value);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern ConstraintPtr AUTDGainHoloClampConstraint(double minV, double maxV);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloSDP(BackendPtr backend, double[]? points, double[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloSDPWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloSDPWithAlpha(GainPtr holo, double alpha);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloSDPWithLambda(GainPtr holo, double lambda);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloSDPWithRepeat(GainPtr holo, uint repeat);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloEVP(BackendPtr backend, double[]? points, double[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloEVPWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloEVPWithGamma(GainPtr holo, double gamma);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloGS(BackendPtr backend, double[]? points, double[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloGSWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloGSWithRepeat(GainPtr holo, uint repeat);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloGSPAT(BackendPtr backend, double[]? points, double[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloGSPATWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloGSPATWithRepeat(GainPtr holo, uint repeat);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloNaive(BackendPtr backend, double[]? points, double[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloNaiveWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloGreedy(double[]? points, double[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloGreedyWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloGreedyWithPhaseDiv(GainPtr holo, uint div);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloLM(BackendPtr backend, double[]? points, double[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloLMWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloLMWithEps1(GainPtr holo, double eps);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloLMWithEps2(GainPtr holo, double eps);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloLMWithTau(GainPtr holo, double tau);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloLMWithKMax(GainPtr holo, uint kMax);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloLMWithInitial(GainPtr holo, double[]? initialPtr, ulong len);
        }
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct BackendPtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct ConstraintPtr
    {
        public IntPtr _0;
    }

}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif


