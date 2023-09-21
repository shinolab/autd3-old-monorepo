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
        internal static class BackendCUDA
        {
            private const string DLL = "autd3capi_backend_cuda";

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern BackendPtr AUTDCUDABackend(byte[] err);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDCUDABackendDelete(BackendPtr backend);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDASDP(BackendPtr backend, float[]? points, float[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDASDPWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDASDPWithAlpha(GainPtr holo, float alpha);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDASDPWithLambda(GainPtr holo, float lambda);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDASDPWithRepeat(GainPtr holo, uint repeat);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDAEVP(BackendPtr backend, float[]? points, float[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDAEVPWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDAEVPWithGamma(GainPtr holo, float gamma);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDAGS(BackendPtr backend, float[]? points, float[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDAGSWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDAGSWithRepeat(GainPtr holo, uint repeat);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDAGSPAT(BackendPtr backend, float[]? points, float[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDAGSPATWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDAGSPATWithRepeat(GainPtr holo, uint repeat);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDANaive(BackendPtr backend, float[]? points, float[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDANaiveWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDAGreedy(float[]? points, float[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDAGreedyWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDAGreedyWithPhaseDiv(GainPtr holo, uint div);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDALM(BackendPtr backend, float[]? points, float[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDALMWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDALMWithEps1(GainPtr holo, float eps);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDALMWithEps2(GainPtr holo, float eps);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDALMWithTau(GainPtr holo, float tau);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDALMWithKMax(GainPtr holo, uint kMax);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern GainPtr AUTDGainHoloCUDALMWithInitial(GainPtr holo, float[]? initialPtr, ulong len);
        }
    }

}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif


