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

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern ConstraintPtr AUTDGainHoloConstraintDotCare();

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern ConstraintPtr AUTDGainHoloConstraintNormalize();

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern ConstraintPtr AUTDGainHoloConstraintUniform(float value);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern ConstraintPtr AUTDGainHoloConstraintClamp(float minV, float maxV);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloEVP(BackendPtr backend, float[]? points, float[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloEVPWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloEVPWithGamma(GainPtr holo, float gamma);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloGreedy(float[]? points, float[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloGreedyWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloGreedyWithPhaseDiv(GainPtr holo, uint div);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloGS(BackendPtr backend, float[]? points, float[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloGSWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloGSWithRepeat(GainPtr holo, uint repeat);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloGSPAT(BackendPtr backend, float[]? points, float[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloGSPATWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloGSPATWithRepeat(GainPtr holo, uint repeat);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloLM(BackendPtr backend, float[]? points, float[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloLMWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloLMWithEps1(GainPtr holo, float eps);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloLMWithEps2(GainPtr holo, float eps);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloLMWithTau(GainPtr holo, float tau);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloLMWithKMax(GainPtr holo, uint kMax);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloLMWithInitial(GainPtr holo, float[]? initialPtr, ulong len);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloNaive(BackendPtr backend, float[]? points, float[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloNaiveWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern BackendPtr AUTDNalgebraBackend();

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern void AUTDDeleteNalgebraBackend(BackendPtr backend);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloSDP(BackendPtr backend, float[]? points, float[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloSDPWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloSDPWithAlpha(GainPtr holo, float alpha);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloSDPWithLambda(GainPtr holo, float lambda);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloSDPWithRepeat(GainPtr holo, uint repeat);
        }
    }

}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif


